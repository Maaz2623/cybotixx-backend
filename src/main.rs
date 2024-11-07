use std::env;
mod types;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use mongodb::{Client, Collection};
use types::Forum;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Index page working.")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv
    let uri = match env::var("MONGO_URI") {
        Ok(v) => v.to_string(),
        Err(_) => "Error connecting to mongodb".to_string()
    };

    let client = Client::with_uri_str(uri).await.unwrap();
    let db = client.database("cybotixx");
    let main_db = Data::new(db);
    let _forums_collection: Collection<Forum> = main_db.collection("forums");


    HttpServer::new(move || {
        App::new().app_data(main_db.clone()).service(hello)
    }).bind(("localhost", 5000))?.run().await

    
}