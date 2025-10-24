use actix_web::{ get, HttpResponse, Responder };

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}
