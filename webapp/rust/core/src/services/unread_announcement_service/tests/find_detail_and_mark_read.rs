use crate::models::announcement_detail::AnnouncementDetail;
use crate::repos::error::ReposError::TestError;
use crate::services::error::Result;
use crate::services::unread_announcement_service::tests::S;
use crate::services::unread_announcement_service::UnreadAnnouncementServiceImpl;

#[tokio::test]
#[should_panic(expected = "ReposError(TestError)")]
async fn find_announcement_detail_by_announcement_id_and_user_id_in_tx_err() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
        .returning(|_, _, _| Err(TestError));

    service.find_detail_and_mark_read("", "").await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "AnnouncementNotFound")]
async fn find_announcement_detail_by_announcement_id_and_user_id_in_tx_none() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
        .returning(|_, _, _| Ok(None));

    service.find_detail_and_mark_read("", "").await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "ReposError(TestError)")]
async fn exist_by_user_id_and_course_id_in_tx_err() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
        .returning(|_, _, _| {
            Ok(Some(AnnouncementDetail {
                id: "".to_string(),
                course_id: "".to_string(),
                course_name: "".to_string(),
                title: "".to_string(),
                message: "".to_string(),
                unread: false,
            }))
        });

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id_in_tx()
        .returning(|_, _, _| Err(TestError));

    service.find_detail_and_mark_read("", "").await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "AnnouncementNotFound")]
async fn exist_by_user_id_and_course_id_in_tx_false() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
        .returning(|_, _, _| {
            Ok(Some(AnnouncementDetail {
                id: "".to_string(),
                course_id: "".to_string(),
                course_name: "".to_string(),
                title: "".to_string(),
                message: "".to_string(),
                unread: false,
            }))
        });

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id_in_tx()
        .returning(|_, _, _| Ok(false));

    service.find_detail_and_mark_read("", "").await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "ReposError(TestError)")]
async fn mark_read_failed() -> () {
    let mut service = S::new().await;
    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
        .returning(|_, _, _| {
            Ok(Some(AnnouncementDetail {
                id: "".to_string(),
                course_id: "".to_string(),
                course_name: "".to_string(),
                title: "".to_string(),
                message: "".to_string(),
                unread: false,
            }))
        });

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id_in_tx()
        .returning(|_, _, _| Ok(true));

    service
        .unread_announcement_repo
        .expect_mark_read()
        .returning(|_, _, _| Err(TestError));

    service.find_detail_and_mark_read("", "").await.unwrap();
}

#[tokio::test]
async fn success_case() -> Result<()> {
    let mut service = S::new().await;
    let expected = AnnouncementDetail {
        id: "aid".to_string(),
        course_id: "course_id".to_string(),
        course_name: "course_name".to_string(),
        title: "title".to_string(),
        message: "message".to_string(),
        unread: true,
    };
    let detail = expected.clone();

    service
        .unread_announcement_repo
        .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
        .withf(|_, aid, user_id| aid == "aid" && user_id == "user_id")
        .returning(move |_, _, _| Ok(Some(detail.clone())));

    service
        .registration_repo
        .expect_exist_by_user_id_and_course_id_in_tx()
        .withf(|_, user_id, course_id| user_id == "user_id" && course_id == "course_id")
        .returning(|_, _, _| Ok(true));

    service
        .unread_announcement_repo
        .expect_mark_read()
        .withf(|_, aid, user_id| aid == "aid" && user_id == "user_id")
        .returning(|_, _, _| Ok(()));

    let detail = service
        .find_detail_and_mark_read("aid", "user_id")
        .await
        .unwrap();

    assert_eq!(detail, expected);

    Ok(())
}