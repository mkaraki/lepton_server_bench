use std::fs::File;
use lepton_jpeg::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestData {
    i: String,
}

#[get("/")]
async fn index(info: web::Query<RequestData>) -> impl Responder {
    let mut file = File::open("images/".to_owned() + &info.i).unwrap();
    let mut output_buffer: Vec<u8> = Vec::new();

    match decode_lepton(
        &mut file,
        &mut output_buffer,
        1,
    ) {
        Ok(_) => {
            HttpResponse::Ok().content_type("image/jpeg").body(output_buffer)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Failed to decode")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}