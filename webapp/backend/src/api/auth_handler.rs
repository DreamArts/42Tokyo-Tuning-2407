use crate::domains::auth_service::AuthService;
use crate::domains::dto::auth::{LoginRequestDto, LogoutRequestDto, RegisterRequestDto};
use crate::errors::AppError;
use crate::repositories::auth_repository::AuthRepositoryImpl;
use actix_web::{web, HttpResponse};

pub async fn register_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<RegisterRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service
        .register_user(&req.username, &req.password, &req.role, req.area_id)
        .await
    {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(err) => Err(err),
    }
}

pub async fn login_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<LoginRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service.login_user(&req.username, &req.password).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(err) => Err(err),
    }
}

pub async fn logout_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<LogoutRequestDto>,
) -> Result<HttpResponse, AppError> {
    match service.logout_user(&req.session_token).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Ok().finish()),
    }
}

pub async fn user_profile_image_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let width = 500;
    let height = 500;
    let user_id = path.into_inner();
    let profile_image_byte = service
        .get_resized_profile_image_byte(user_id, width, height)
        .await?;
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(profile_image_byte))
}
