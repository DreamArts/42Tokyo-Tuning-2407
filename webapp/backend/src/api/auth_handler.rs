use crate::domains::auth_service::AuthService;
use crate::domains::dto::auth::{LoginRequestDto, LogoutRequestDto, RegisterRequestDto};
use crate::errors::AppError;
use crate::repositories::auth_repository::AuthRepositoryImpl;
use actix_web::{web, HttpResponse};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROFILE_IMAGE_CACHE: Arc<RwLock<HashMap<i32, Vec<u8>>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub async fn register_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<RegisterRequestDto>,
) -> Result<HttpResponse, AppError> {
    let response = service
        .register_user(&req.username, &req.password, &req.role, req.area_id)
        .await?;
    Ok(HttpResponse::Created().json(response))
}

pub async fn login_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<LoginRequestDto>,
) -> Result<HttpResponse, AppError> {
    let response = service
        .login_user(&req.username, &req.password)
        .await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn logout_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    req: web::Json<LogoutRequestDto>,
) -> Result<HttpResponse, AppError> {
    service
        .logout_user(&req.session_token)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .or_else(|_| Ok(HttpResponse::Ok().finish()))  // Handle logout errors gracefully
}

pub async fn user_profile_image_handler(
    service: web::Data<AuthService<AuthRepositoryImpl>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Check if the image is cached
    if let Some(image) = {
        let cache = PROFILE_IMAGE_CACHE.read().await;
        cache.get(&user_id).cloned()
    } {
        return Ok(HttpResponse::Ok()
            .content_type("image/png")
            .body(image));
    }

    // Fetch the image if not cached
    let profile_image_byte = service.get_resized_profile_image_byte(user_id).await?;

    // Cache the image
    {
        let mut cache = PROFILE_IMAGE_CACHE.write().await;
        cache.insert(user_id, profile_image_byte.to_vec());
    }

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(profile_image_byte))
}
