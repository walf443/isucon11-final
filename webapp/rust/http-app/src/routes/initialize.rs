use actix_web::{web, HttpResponse};
use futures::StreamExt;
use isucholar_core::{ASSIGNMENTS_DIRECTORY, INIT_DATA_DIRECTORY, SQL_DIRECTORY};
use isucholar_http_core::responses::error::SqlxError;
use sqlx::Executor;

#[derive(Debug, serde::Serialize)]
struct InitializeResponse {
    language: &'static str,
}

// POST /initialize 初期化エンドポイント
pub async fn initialize(pool: web::Data<sqlx::MySqlPool>) -> actix_web::Result<HttpResponse> {
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
