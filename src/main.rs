use actix_web::{web, App, HttpServer, web::get};

use std::io::Result;
use std::sync::Mutex;
use actix_web::web::Data;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    let counter = Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", get().to(index))
    })
        .bind("localhost:8000")?
        .run()
        .await
}
