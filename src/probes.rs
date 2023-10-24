use actix_web::{HttpResponse};

pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("healthz\n")
}
pub async fn livez() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("livez\n")
}

pub async fn readyz() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("readyz\n")
}
