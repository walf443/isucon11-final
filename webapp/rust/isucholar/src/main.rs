use actix_web::web;
use actix_web::HttpResponse;
use futures::StreamExt as _;
use isucholar::routes::course_routes::add_course::add_course;
use isucholar::routes::course_routes::get_classes::get_classes;
use isucholar::routes::course_routes::get_course_detail::get_course_detail;
use isucholar::routes::course_routes::search_courses::search_courses;
use isucholar::routes::course_routes::set_course_status::set_course_status;
use isucholar::routes::course_routes::submit_assignment::submit_assignment;
use isucholar::routes::user_routes::{
    get_grades, get_me, get_registered_courses, register_courses,
};
use sqlx::Arguments as _;
use sqlx::Executor as _;

const SQL_DIRECTORY: &str = "../sql/";
const ASSIGNMENTS_DIRECTORY: &str = "../assignments/";
const INIT_DATA_DIRECTORY: &str = "../data/";
const SESSION_NAME: &str = "isucholar_rust";
const MYSQL_ERR_NUM_DUPLICATE_ENTRY: u16 = 1062;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();

    let mysql_config = sqlx::mysql::MySqlConnectOptions::new()
        .host(
            &std::env::var("MYSQL_HOSTNAME")
                .ok()
                .unwrap_or_else(|| "127.0.0.1".to_owned()),
        )
        .port(
            std::env::var("MYSQL_PORT")
                .ok()
                .and_then(|port_str| port_str.parse().ok())
                .unwrap_or(3306),
        )
        .username(
            &std::env::var("MYSQL_USER")
                .ok()
                .unwrap_or_else(|| "isucon".to_owned()),
        )
        .password(
            &std::env::var("MYSQL_PASS")
                .ok()
                .unwrap_or_else(|| "isucon".to_owned()),
        )
        .database(
            &std::env::var("MYSQL_DATABASE")
                .ok()
                .unwrap_or_else(|| "isucholar".to_owned()),
        )
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .after_connect(|conn| {
            Box::pin(async move {
                conn.execute("set time_zone = '+00:00'").await?;
                Ok(())
            })
        })
        .connect_with(mysql_config)
        .await
        .expect("failed to connect db");

    let mut session_key = b"trapnomura".to_vec();
    session_key.resize(32, 0);

    let server = actix_web::HttpServer::new(move || {
        let users_api = web::scope("/users")
            .route("/me", web::get().to(get_me))
            .route("/me/courses", web::get().to(get_registered_courses))
            .route("/me/courses", web::put().to(register_courses))
            .route("/me/grades", web::get().to(get_grades));

        let courses_api = web::scope("/courses")
            .route("", web::get().to(search_courses))
            .service(
                web::resource("")
                    .guard(actix_web::guard::Post())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(add_course),
            )
            .route("/{course_id}", web::get().to(get_course_detail))
            .service(
                web::resource("/{course_id}/status")
                    .guard(actix_web::guard::Put())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(set_course_status),
            )
            .route("/{course_id}/classes", web::get().to(get_classes))
            .service(
                web::resource("/{course_id}/classes")
                    .guard(actix_web::guard::Post())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(add_class),
            )
            .route(
                "/{course_id}/classes/{class_id}/assignments",
                web::post().to(submit_assignment),
            )
            .service(
                web::resource("/{course_id}/classes/{class_id}/assignments/scores")
                    .guard(actix_web::guard::Put())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(register_scores),
            )
            .service(
                web::resource("/{course_id}/classes/{class_id}/assignments/export")
                    .guard(actix_web::guard::Get())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(download_submitted_assignments),
            );

        let announcements_api = web::scope("/announcements")
            .route("", web::get().to(get_announcement_list))
            .service(
                web::resource("")
                    .guard(actix_web::guard::Post())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(add_announcement),
            )
            .route("/{announcement_id}", web::get().to(get_announcement_detail));

        actix_web::App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                actix_session::CookieSession::signed(&session_key)
                    .secure(false)
                    .name(SESSION_NAME)
                    .max_age(3600),
            )
            .route("/initialize", web::post().to(initialize))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .service(
                web::scope("/api")
                    .wrap(isucholar::middleware::IsLoggedIn)
                    .service(users_api)
                    .service(courses_api)
                    .service(announcements_api),
            )
    });
    if let Some(l) = listenfd::ListenFd::from_env().take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind((
            "0.0.0.0",
            std::env::var("PORT")
                .ok()
                .and_then(|port_str| port_str.parse().ok())
                .unwrap_or(7000),
        ))?
    }
    .run()
    .await
}

#[derive(Debug)]
struct SqlxError(sqlx::Error);
impl std::fmt::Display for SqlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl actix_web::ResponseError for SqlxError {
    fn error_response(&self) -> HttpResponse {
        log::error!("{}", self);
        HttpResponse::InternalServerError()
            .content_type(mime::TEXT_PLAIN)
            .body(format!("SQLx error: {:?}", self.0))
    }
}

#[derive(Debug, serde::Serialize)]
struct InitializeResponse {
    language: &'static str,
}

// POST /initialize 初期化エンドポイント
async fn initialize(pool: web::Data<sqlx::MySqlPool>) -> actix_web::Result<HttpResponse> {
    let files = ["1_schema.sql", "2_init.sql", "3_sample.sql"];
    for file in files {
        let data = tokio::fs::read_to_string(format!("{}{}", SQL_DIRECTORY, file)).await?;
        let mut stream = pool.execute_many(data.as_str());
        while let Some(result) = stream.next().await {
            result.map_err(SqlxError)?;
        }
    }

    if !tokio::process::Command::new("rm")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("-rf")
        .arg(ASSIGNMENTS_DIRECTORY)
        .status()
        .await?
        .success()
    {
        return Err(actix_web::error::ErrorInternalServerError(""));
    }
    if !tokio::process::Command::new("cp")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("-r")
        .arg(INIT_DATA_DIRECTORY)
        .arg(ASSIGNMENTS_DIRECTORY)
        .status()
        .await?
        .success()
    {
        return Err(actix_web::error::ErrorInternalServerError(""));
    }

    Ok(HttpResponse::Ok().json(InitializeResponse { language: "rust" }))
}

fn get_user_info(session: actix_session::Session) -> actix_web::Result<(String, String, bool)> {
    let user_id = session.get("userID")?;
    if user_id.is_none() {
        return Err(actix_web::error::ErrorInternalServerError(
            "failed to get userID from session",
        ));
    }
    let user_name = session.get("userName")?;
    if user_name.is_none() {
        return Err(actix_web::error::ErrorInternalServerError(
            "failed to get userName from session",
        ));
    }
    let is_admin = session.get("isAdmin")?;
    if is_admin.is_none() {
        return Err(actix_web::error::ErrorInternalServerError(
            "failed to get isAdmin from session",
        ));
    }
    Ok((user_id.unwrap(), user_name.unwrap(), is_admin.unwrap()))
}

#[derive(Debug, PartialEq, Eq)]
enum UserType {
    Student,
    Teacher,
}
impl sqlx::Type<sqlx::MySql> for UserType {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        str::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <&str>::compatible(ty)
    }
}
impl<'r> sqlx::Decode<'r, sqlx::MySql> for UserType {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match <&'r str>::decode(value)? {
            "student" => Ok(Self::Student),
            "teacher" => Ok(Self::Teacher),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}
impl<'q> sqlx::Encode<'q, sqlx::MySql> for UserType {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        match *self {
            Self::Teacher => "teacher",
            Self::Student => "student",
        }
        .encode_by_ref(buf)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: String,
    code: String,
    name: String,
    hashed_password: Vec<u8>,
    #[sqlx(rename = "type")]
    type_: UserType,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CourseType {
    LiberalArts,
    MajorSubjects,
}
impl sqlx::Type<sqlx::MySql> for CourseType {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        str::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <&str>::compatible(ty)
    }
}
impl<'r> sqlx::Decode<'r, sqlx::MySql> for CourseType {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match <&'r str>::decode(value)? {
            "liberal-arts" => Ok(Self::LiberalArts),
            "major-subjects" => Ok(Self::MajorSubjects),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}
impl<'q> sqlx::Encode<'q, sqlx::MySql> for CourseType {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        match *self {
            Self::LiberalArts => "liberal-arts",
            Self::MajorSubjects => "major-subjects",
        }
        .encode_by_ref(buf)
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}
impl sqlx::Type<sqlx::MySql> for DayOfWeek {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        str::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <&str>::compatible(ty)
    }
}
impl<'r> sqlx::Decode<'r, sqlx::MySql> for DayOfWeek {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match <&'r str>::decode(value)? {
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}
impl<'q> sqlx::Encode<'q, sqlx::MySql> for DayOfWeek {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        match *self {
            Self::Monday => "monday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
            Self::Thursday => "thursday",
            Self::Friday => "friday",
        }
        .encode_by_ref(buf)
    }
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CourseStatus {
    Registration,
    InProgress,
    Closed,
}
impl sqlx::Type<sqlx::MySql> for CourseStatus {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        str::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <&str>::compatible(ty)
    }
}
impl<'r> sqlx::Decode<'r, sqlx::MySql> for CourseStatus {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match <&'r str>::decode(value)? {
            "registration" => Ok(Self::Registration),
            "in-progress" => Ok(Self::InProgress),
            "closed" => Ok(Self::Closed),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}
impl<'q> sqlx::Encode<'q, sqlx::MySql> for CourseStatus {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        match *self {
            Self::Registration => "registration",
            Self::InProgress => "in-progress",
            Self::Closed => "closed",
        }
        .encode_by_ref(buf)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct Course {
    id: String,
    code: String,
    #[sqlx(rename = "type")]
    type_: CourseType,
    name: String,
    description: String,
    credit: u8,
    period: u8,
    day_of_week: DayOfWeek,
    teacher_id: String,
    keywords: String,
    status: CourseStatus,
}

// ---------- Public API ----------

#[derive(Debug, serde::Deserialize)]
struct LoginRequest {
    code: String,
    password: String,
}

// POST /login ログイン
async fn login(
    session: actix_session::Session,
    pool: web::Data<sqlx::MySqlPool>,
    req: web::Json<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    let user: Option<User> = sqlx::query_as("SELECT * FROM `users` WHERE `code` = ?")
        .bind(&req.code)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(SqlxError)?;
    if user.is_none() {
        return Err(actix_web::error::ErrorUnauthorized(
            "Code or Password is wrong.",
        ));
    }
    let user = user.unwrap();

    if !bcrypt::verify(
        &req.password,
        &String::from_utf8(user.hashed_password).unwrap(),
    )
    .map_err(actix_web::error::ErrorInternalServerError)?
    {
        return Err(actix_web::error::ErrorUnauthorized(
            "Code or Password is wrong.",
        ));
    }

    if let Some(user_id) = session.get::<String>("userID")? {
        if user_id == user.id {
            return Err(actix_web::error::ErrorBadRequest(
                "You are already logged in.",
            ));
        }
    }

    session.insert("userID", user.id)?;
    session.insert("userName", user.name)?;
    session.insert("isAdmin", user.type_ == UserType::Teacher)?;
    Ok(HttpResponse::Ok().finish())
}

// POST /logout ログアウト
async fn logout(session: actix_session::Session) -> actix_web::Result<HttpResponse> {
    session.purge();
    Ok(HttpResponse::Ok().finish())
}

// ---------- Users API ----------

#[derive(Debug, serde::Serialize)]
struct GetRegisteredCourseResponseContent {
    id: String,
    name: String,
    teacher: String,
    period: u8,
    day_of_week: DayOfWeek,
}

#[derive(Debug, sqlx::FromRow)]
struct Class {
    id: String,
    course_id: String,
    part: u8,
    title: String,
    description: String,
    submission_closed: bool,
}

#[derive(Debug, Default, serde::Serialize)]
struct Summary {
    credits: i64,
    gpa: f64,
    gpa_t_score: f64, // 偏差値
    gpa_avg: f64,     // 平均値
    gpa_max: f64,     // 最大値
    gpa_min: f64,     // 最小値
}

#[derive(Debug, serde::Serialize)]
struct CourseResult {
    name: String,
    code: String,
    total_score: i64,
    total_score_t_score: f64, // 偏差値
    total_score_avg: f64,     // 平均値
    total_score_max: i64,     // 最大値
    total_score_min: i64,     // 最小値
    class_scores: Vec<ClassScore>,
}

#[derive(Debug, serde::Serialize)]
struct ClassScore {
    class_id: String,
    title: String,
    part: u8,
    score: Option<i64>, // 0~100点
    submitters: i64,    // 提出した学生数
}

// ---------- Courses API ----------

#[derive(Debug, serde::Deserialize)]
struct AddClassRequest {
    part: u8,
    title: String,
    description: String,
}

#[derive(Debug, serde::Serialize)]
struct AddClassResponse {
    class_id: String,
}

// POST /api/courses/{course_id}/classes 新規講義(&課題)追加
async fn add_class(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
    req: web::Json<AddClassRequest>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let course: Option<Course> = isucholar::db::fetch_optional_as(
        sqlx::query_as("SELECT * FROM `courses` WHERE `id` = ? FOR SHARE").bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if course.is_none() {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }
    let course = course.unwrap();
    if course.status != CourseStatus::InProgress {
        return Err(actix_web::error::ErrorBadRequest(
            "This course is not in-progress.",
        ));
    }

    let class_id = isucholar::util::new_ulid().await;
    let result = sqlx::query("INSERT INTO `classes` (`id`, `course_id`, `part`, `title`, `description`) VALUES (?, ?, ?, ?, ?)")
        .bind(&class_id)
        .bind(course_id)
        .bind(&req.part)
        .bind(&req.title)
        .bind(&req.description)
        .execute(&mut tx)
        .await;
    if let Err(e) = result {
        let _ = tx.rollback().await;
        if let sqlx::error::Error::Database(ref db_error) = e {
            if let Some(mysql_error) =
                db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
            {
                if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                    let class: Class = sqlx::query_as(
                        "SELECT * FROM `classes` WHERE `course_id` = ? AND `part` = ?",
                    )
                    .bind(course_id)
                    .bind(&req.part)
                    .fetch_one(pool.as_ref())
                    .await
                    .map_err(SqlxError)?;
                    if req.title != class.title || req.description != class.description {
                        return Err(actix_web::error::ErrorConflict(
                            "A class with the same part already exists.",
                        ));
                    } else {
                        return Ok(
                            HttpResponse::Created().json(AddClassResponse { class_id: class.id })
                        );
                    }
                }
            }
        }
        return Err(SqlxError(e).into());
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Created().json(AddClassResponse { class_id }))
}

#[derive(Debug, serde::Deserialize)]
struct AssignmentPath {
    course_id: String,
    class_id: String,
}

#[derive(Debug, serde::Deserialize)]
struct Score {
    user_code: String,
    score: i64,
}

// PUT /api/courses/{course_id}/classes/{class_id}/assignments/scores 採点結果登録
async fn register_scores(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<AssignmentPath>,
    req: web::Json<Vec<Score>>,
) -> actix_web::Result<HttpResponse> {
    let class_id = &path.class_id;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let submission_closed: Option<bool> = isucholar::db::fetch_optional_scalar(
        sqlx::query_scalar("SELECT `submission_closed` FROM `classes` WHERE `id` = ? FOR SHARE")
            .bind(class_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if let Some(submission_closed) = submission_closed {
        if !submission_closed {
            return Err(actix_web::error::ErrorBadRequest(
                "This assignment is not closed yet.",
            ));
        }
    } else {
        return Err(actix_web::error::ErrorNotFound("No such class."));
    }

    for score in req.into_inner() {
        sqlx::query("UPDATE `submissions` JOIN `users` ON `users`.`id` = `submissions`.`user_id` SET `score` = ? WHERE `users`.`code` = ? AND `class_id` = ?")
            .bind(&score.score)
            .bind(&score.user_code)
            .bind(class_id)
            .execute(&mut tx)
            .await
            .map_err(SqlxError)?;
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, sqlx::FromRow)]
struct Submission {
    user_id: String,
    user_code: String,
    file_name: String,
}

// GET /api/courses/{course_id}/classes/{class_id}/assignments/export 提出済みの課題ファイルをzip形式で一括ダウンロード
async fn download_submitted_assignments(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<AssignmentPath>,
) -> actix_web::Result<actix_files::NamedFile> {
    let class_id = &path.class_id;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let class_count: i64 = isucholar::db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `classes` WHERE `id` = ? FOR UPDATE")
            .bind(class_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if class_count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such class."));
    }
    let submissions: Vec<Submission> = sqlx::query_as(concat!(
        "SELECT `submissions`.`user_id`, `submissions`.`file_name`, `users`.`code` AS `user_code`",
        " FROM `submissions`",
        " JOIN `users` ON `users`.`id` = `submissions`.`user_id`",
        " WHERE `class_id` = ?",
    ))
    .bind(class_id)
    .fetch_all(&mut tx)
    .await
    .map_err(SqlxError)?;

    let zip_file_path = format!("{}{}.zip", ASSIGNMENTS_DIRECTORY, class_id);
    create_submissions_zip(&zip_file_path, class_id, &submissions).await?;

    sqlx::query("UPDATE `classes` SET `submission_closed` = true WHERE `id` = ?")
        .bind(class_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(actix_files::NamedFile::open(&zip_file_path)?)
}

async fn create_submissions_zip(
    zip_file_path: &str,
    class_id: &str,
    submissions: &[Submission],
) -> std::io::Result<()> {
    let tmp_dir = format!("{}{}/", ASSIGNMENTS_DIRECTORY, class_id);
    tokio::process::Command::new("rm")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("-rf")
        .arg(&tmp_dir)
        .status()
        .await?;
    tokio::process::Command::new("mkdir")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg(&tmp_dir)
        .status()
        .await?;

    // ファイル名を指定の形式に変更
    for submission in submissions {
        tokio::process::Command::new("cp")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg(&format!(
                "{}{}-{}.pdf",
                ASSIGNMENTS_DIRECTORY, class_id, submission.user_id
            ))
            .arg(&format!(
                "{}{}-{}",
                tmp_dir, submission.user_code, submission.file_name
            ))
            .status()
            .await?;
    }

    // -i 'tmp_dir/*': 空zipを許す
    tokio::process::Command::new("zip")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("-j")
        .arg("-r")
        .arg(zip_file_path)
        .arg(&tmp_dir)
        .arg("-i")
        .arg(&format!("{}*", tmp_dir))
        .status()
        .await?;
    Ok(())
}

// ---------- Announcement API ----------

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
struct AnnouncementWithoutDetail {
    id: String,
    course_id: String,
    course_name: String,
    title: String,
    unread: bool,
}

#[derive(Debug, serde::Serialize)]
struct GetAnnouncementsResponse {
    unread_count: i64,
    announcements: Vec<AnnouncementWithoutDetail>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct GetAnnouncementsQuery {
    course_id: Option<String>,
    page: Option<String>,
}

// GET /api/announcements お知らせ一覧取得
async fn get_announcement_list(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    params: web::Query<GetAnnouncementsQuery>,
    request: actix_web::HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let mut query = concat!(
        "SELECT `announcements`.`id`, `courses`.`id` AS `course_id`, `courses`.`name` AS `course_name`, `announcements`.`title`, NOT `unread_announcements`.`is_deleted` AS `unread`",
        " FROM `announcements`",
        " JOIN `courses` ON `announcements`.`course_id` = `courses`.`id`",
        " JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`",
        " JOIN `unread_announcements` ON `announcements`.`id` = `unread_announcements`.`announcement_id`",
        " WHERE 1=1",
    ).to_owned();
    let mut args = sqlx::mysql::MySqlArguments::default();

    if let Some(ref course_id) = params.course_id {
        query.push_str(" AND `announcements`.`course_id` = ?");
        args.add(course_id);
    }

    query.push_str(concat!(
        " AND `unread_announcements`.`user_id` = ?",
        " AND `registrations`.`user_id` = ?",
        " ORDER BY `announcements`.`id` DESC",
        " LIMIT ? OFFSET ?",
    ));
    args.add(&user_id);
    args.add(&user_id);

    let page = if let Some(ref page_str) = params.page {
        match page_str.parse() {
            Ok(page) if page > 0 => page,
            _ => return Err(actix_web::error::ErrorBadRequest("Invalid page.")),
        }
    } else {
        1
    };
    let limit = 20;
    let offset = limit * (page - 1);
    // limitより多く上限を設定し、実際にlimitより多くレコードが取得できた場合は次のページが存在する
    args.add(limit + 1);
    args.add(offset);

    let mut announcements: Vec<AnnouncementWithoutDetail> = sqlx::query_as_with(&query, args)
        .fetch_all(&mut tx)
        .await
        .map_err(SqlxError)?;

    let unread_count: i64 = isucholar::db::fetch_one_scalar(
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM `unread_announcements` WHERE `user_id` = ? AND NOT `is_deleted`",
        )
        .bind(&user_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    let uri = request.uri();
    let mut params = params.into_inner();
    let mut links = Vec::new();
    if page > 1 {
        params.page = Some(format!("{}", page - 1));
        links.push(format!(
            "<{}?{}>; rel=\"prev\"",
            uri.path(),
            serde_urlencoded::to_string(&params)?
        ));
    }
    if announcements.len() as i64 > limit {
        params.page = Some(format!("{}", page + 1));
        links.push(format!(
            "<{}?{}>; rel=\"next\"",
            uri.path(),
            serde_urlencoded::to_string(&params)?
        ));
    }

    if announcements.len() as i64 == limit + 1 {
        announcements.truncate(announcements.len() - 1);
    }

    // 対象になっているお知らせが0件の時は空配列を返却

    let mut builder = HttpResponse::Ok();
    if !links.is_empty() {
        builder.insert_header((actix_web::http::header::LINK, links.join(",")));
    }
    Ok(builder.json(GetAnnouncementsResponse {
        unread_count,
        announcements,
    }))
}

#[derive(Debug, sqlx::FromRow)]
struct Announcement {
    id: String,
    course_id: String,
    title: String,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
struct AddAnnouncementRequest {
    id: String,
    course_id: String,
    title: String,
    message: String,
}

// POST /api/announcements 新規お知らせ追加
async fn add_announcement(
    pool: web::Data<sqlx::MySqlPool>,
    req: web::Json<AddAnnouncementRequest>,
) -> actix_web::Result<HttpResponse> {
    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let count: i64 = isucholar::db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ?").bind(&req.course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    let result = sqlx::query(
        "INSERT INTO `announcements` (`id`, `course_id`, `title`, `message`) VALUES (?, ?, ?, ?)",
    )
    .bind(&req.id)
    .bind(&req.course_id)
    .bind(&req.title)
    .bind(&req.message)
    .execute(&mut tx)
    .await;
    if let Err(e) = result {
        let _ = tx.rollback().await;
        if let sqlx::error::Error::Database(ref db_error) = e {
            if let Some(mysql_error) =
                db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
            {
                if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                    let announcement: Announcement =
                        sqlx::query_as("SELECT * FROM `announcements` WHERE `id` = ?")
                            .bind(&req.id)
                            .fetch_one(pool.as_ref())
                            .await
                            .map_err(SqlxError)?;
                    if announcement.course_id != req.course_id
                        || announcement.title != req.title
                        || announcement.message != req.message
                    {
                        return Err(actix_web::error::ErrorConflict(
                            "An announcement with the same id already exists.",
                        ));
                    } else {
                        return Ok(HttpResponse::Created().finish());
                    }
                }
            }
        }
        return Err(SqlxError(e).into());
    }

    let targets: Vec<User> = sqlx::query_as(concat!(
        "SELECT `users`.* FROM `users`",
        " JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
        " WHERE `registrations`.`course_id` = ?",
    ))
    .bind(&req.course_id)
    .fetch_all(&mut tx)
    .await
    .map_err(SqlxError)?;

    for user in targets {
        sqlx::query(
            "INSERT INTO `unread_announcements` (`announcement_id`, `user_id`) VALUES (?, ?)",
        )
        .bind(&req.id)
        .bind(user.id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Created().finish())
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
struct AnnouncementDetail {
    id: String,
    course_id: String,
    course_name: String,
    title: String,
    message: String,
    unread: bool,
}

// GET /api/announcements/{announcement_id} お知らせ詳細取得
async fn get_announcement_detail(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    announcement_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let announcement_id = &announcement_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let announcement: Option<AnnouncementDetail> = isucholar::db::fetch_optional_as(
        sqlx::query_as(concat!(
                "SELECT `announcements`.`id`, `courses`.`id` AS `course_id`, `courses`.`name` AS `course_name`, `announcements`.`title`, `announcements`.`message`, NOT `unread_announcements`.`is_deleted` AS `unread`",
                " FROM `announcements`",
                " JOIN `courses` ON `courses`.`id` = `announcements`.`course_id`",
                " JOIN `unread_announcements` ON `unread_announcements`.`announcement_id` = `announcements`.`id`",
                " WHERE `announcements`.`id` = ?",
                " AND `unread_announcements`.`user_id` = ?",
        )).bind(announcement_id).bind(&user_id),
        &mut tx
    )
    .await
    .map_err(SqlxError)?;
    if announcement.is_none() {
        return Err(actix_web::error::ErrorNotFound("No such announcement."));
    }
    let announcement = announcement.unwrap();

    let registration_count: i64 = isucholar::db::fetch_one_scalar(
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM `registrations` WHERE `course_id` = ? AND `user_id` = ?",
        )
        .bind(&announcement.course_id)
        .bind(&user_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if registration_count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such announcement."));
    }

    sqlx::query("UPDATE `unread_announcements` SET `is_deleted` = true WHERE `announcement_id` = ? AND `user_id` = ?")
        .bind(announcement_id)
        .bind(&user_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().json(announcement))
}
