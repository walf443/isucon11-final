use crate::models::announcement::AnnouncementID;
use crate::models::announcement_detail::AnnouncementDetail;
use crate::models::course::CourseID;
use crate::models::user::UserID;
use crate::repos::error::ReposError::TestError;
use crate::services::error::Result;
use crate::services::unread_announcement_service::tests::S;
use crate::services::unread_announcement_service::UnreadAnnouncementServiceImpl;
use fake::{Fake, Faker};

#[tokio::test]
#[should_panic(expected = "ReposError(TestError)")]
async fn find_announcement_detail_by_announcement_id_and_user_id_err() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id()
        .returning(|_, _, _| Err(TestError));

    let aid: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();

    service
        .find_detail_and_mark_read(&aid, &user_id)
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "AnnouncementNotFound")]
async fn find_announcement_detail_by_announcement_id_and_user_id_none() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id()
        .returning(|_, _, _| Ok(None));

    let aid: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();
    service
        .find_detail_and_mark_read(&aid, &user_id)
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "ReposError(TestError)")]
async fn exist_by_user_id_and_course_id_err() -> () {
    let mut service = S::new().await;

    let aid: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();

    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id()
        .returning(|_, _, _| {
            Ok(Some(AnnouncementDetail {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                course_name: "".to_string(),
                title: "".to_string(),
                message: "".to_string(),
                unread: false,
            }))
        });

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id()
        .returning(|_, _, _| Err(TestError));

    service
        .find_detail_and_mark_read(&aid, &user_id)
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "AnnouncementNotFound")]
async fn exist_by_user_id_and_course_id_false() -> () {
    let aid: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();

    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id()
        .returning(|_, _, _| {
            Ok(Some(AnnouncementDetail {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                course_name: "".to_string(),
                title: "".to_string(),
                message: "".to_string(),
                unread: false,
            }))
        });

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id()
        .returning(|_, _, _| Ok(false));

    service
        .find_detail_and_mark_read(&aid, &user_id)
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic(expected = "ReposError(TestError)")]
async fn mark_read_failed() -> () {
    let aid: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();

    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id()
        .returning(|_, _, _| {
            Ok(Some(AnnouncementDetail {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                course_name: "".to_string(),
                title: "".to_string(),
                message: "".to_string(),
                unread: false,
            }))
        });

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id()
        .returning(|_, _, _| Ok(true));

    service
        .unread_announcement_repo
        .expect_mark_read()
        .returning(|_, _, _| Err(TestError));

    service
        .find_detail_and_mark_read(&aid, &user_id)
        .await
        .unwrap();
}

#[tokio::test]
async fn success_case() -> Result<()> {
    let aid: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();

    let mut service = S::new().await;
    let expected = AnnouncementDetail {
        id: AnnouncementID::new("aid".to_string()),
        course_id: CourseID::new("course_id".to_string()),
        course_name: "course_name".to_string(),
        title: "title".to_string(),
        message: "message".to_string(),
        unread: true,
    };
    let detail = expected.clone();

    let announcement_id = aid.clone();
    let uid = user_id.clone();
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id()
        .withf(move |_, aid, user_id| {
            aid.to_string() == announcement_id.to_string() && user_id.to_string() == uid.to_string()
        })
        .returning(move |_, _, _| Ok(Some(detail.clone())));

    let uid = user_id.clone();
    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id()
        .withf(move |_, user_id, course_id| {
            user_id.to_string() == uid.to_string()
                && course_id.to_string() == "course_id".to_string()
        })
        .returning(|_, _, _| Ok(true));

    let announcement_id = aid.clone();
    let uid = user_id.clone();
    service
        .unread_announcement_repo
        .expect_mark_read()
        .withf(move |_, aid, user_id| {
            aid.to_string() == announcement_id.to_string() && user_id.to_string() == uid.to_string()
        })
        .returning(|_, _, _| Ok(()));

    let detail = service
        .find_detail_and_mark_read(&aid, &user_id)
        .await
        .unwrap();

    assert_eq!(detail, expected);

    Ok(())
}
