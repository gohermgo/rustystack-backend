use actix_web::dev::Server;
use actix_web::{
    body::MessageBody, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer,
};
use std::net::TcpListener;
pub const PUBLIC_IP: &str = "127.0.0.1";
pub fn site_uri() -> String {
    format!("http://{}/", &PUBLIC_IP)
}
pub const URI_ROOT: &str = "127.0.0.1:8000";
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn greet(req: HttpRequest) -> HttpResponse {
    if let Some(name) = req.match_info().get("name") {
        HttpResponse::with_body(StatusCode::OK, format!("Hello {}!", &name).boxed())
    } else {
        HttpResponse::with_body(StatusCode::OK, String::from("Hello world!").boxed())
    }
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/greet", web::get().to(greet))
    })
    .listen(listener)?
    // .bind(PUBLIC_IP)?
    .run();
    Ok(server)
}
