use isucholar_core::models::user::UserID;

pub fn get_user_info(session: actix_session::Session) -> actix_web::Result<(UserID, String, bool)> {
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
    Ok((
        UserID::new(user_id.unwrap()),
        user_name.unwrap(),
        is_admin.unwrap(),
    ))
}
