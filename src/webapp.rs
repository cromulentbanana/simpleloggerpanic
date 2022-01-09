use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use std::{thread, time};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/ping")]
async fn ping(data: web::Data<AppState>) -> impl Responder {
    let delay = time::Duration::from_secs(data.artificial_latency_seconds.into());
    let response: serde_json::Value = serde_json::json!({"response":"hi"});
    thread::sleep(delay);
    HttpResponse::Ok().body(response)
}

#[derive(Clone)]
struct AppState {
    artificial_latency_seconds: u16,
}

#[actix_web::main]
pub async fn main(host: std::net::IpAddr, port: u16, delay: u16) -> std::io::Result<()> {
    let data = AppState {
        artificial_latency_seconds: delay,
    };

    HttpServer::new(move || App::new().data(data.clone()).service(echo).service(ping))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
