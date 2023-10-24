use actix_web::{HttpResponse, HttpRequest, web};

use crate::server::{AppState};

//
pub async fn head(_req: HttpRequest, _data: web::Data<AppState>) -> HttpResponse {
    //HttpResponse::Ok()
    HttpResponse::NoContent().into()
}

//
pub async fn get(_req: HttpRequest, _data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("METHOD GET\n",))
}

//
pub async fn post(_req: HttpRequest, _data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("METHOD POST\n",))
}

//
pub async fn put(_req: HttpRequest, _data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("METHOD PUT\n",))
}

//
pub async fn delete(_req: HttpRequest, _data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("METHOD DELETE\n",))
}

// EOF
