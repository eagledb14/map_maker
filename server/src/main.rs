use actix_web::{get, web, Responder, HttpResponse, HttpServer, App};
use map::Map;

mod map;
mod noise;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let map = web::Data::new(Map::new());  

    println!("Listening on port: 8080");

    HttpServer::new(move || {
        App::new()
        .app_data(map.clone())
        .service(pushed)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[get("/")]
async fn pushed(map: web::Data<Map>) -> impl Responder {
    let map = map.map().await;
    println!("{:?}", map);
    HttpResponse::Ok().json(map)
}


