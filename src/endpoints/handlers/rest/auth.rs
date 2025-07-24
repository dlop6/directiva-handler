use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

/// Request para login de directivos
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Request para registro de directivos
#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub nombre: String,
    pub cargo: String,
}

/// Response de autenticación siguiendo patrón general-handler
#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub usuario_id: Option<String>,
}

/// Handler para login de directivos
pub async fn user_login(req: web::Json<LoginRequest>) -> Result<HttpResponse> {
    // TODO: Implementar validación real contra Redis usando SHA256
    let response = AuthResponse {
        success: true,
        message: "Login exitoso - Directivo autorizado".to_string(),
        token: Some("sha256_token_placeholder".to_string()),
        usuario_id: Some("directivo_001".to_string()),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Handler para registro de directivos
pub async fn user_sign_up(req: web::Json<SignUpRequest>) -> Result<HttpResponse> {
    // TODO: Implementar registro real en Redis usando SHA256
    let response = AuthResponse {
        success: true,
        message: "Directivo registrado exitosamente".to_string(),
        token: Some("sha256_token_placeholder".to_string()),
        usuario_id: Some("directivo_nuevo".to_string()),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

/// Configuración de rutas de autenticación para directivos
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(user_sign_up))
            .route("/login", web::post().to(user_login)),
    );
}
