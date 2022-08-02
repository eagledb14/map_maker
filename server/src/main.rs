use actix_web::{get, web, Responder, HttpServer, App};
use map::Map;

mod map;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let map = web::Data::new(Map::new());  



    HttpServer::new(move || {
        App::new()
        .app_data(map.clone())
        .service(pushed)
        .service(increment)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[get("/")]
async fn pushed(map: web::Data<Map>) -> impl Responder {
    map.push(1).await;
    println!("{:?}", map.map().await);
    format!("pushed")
}

#[get("/inc")]
async fn increment(map: web::Data<Map>) -> impl Responder {
    map.inc().await;
    format!("increment")
}

