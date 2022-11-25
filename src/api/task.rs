use actix_web::{get, post, put, web, HttpResponse, Responder};

#[get("/task/{task_id}")]
pub async fn get_task() -> impl Responder {
    HttpResponse::Ok()
}