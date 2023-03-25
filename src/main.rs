mod apis;
mod jwt_handler;
mod user;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

#[derive(Debug, Clone)]
pub struct ServerState {
    pool: MySqlPool,
}

/// is used to allow cors requests
#[actix_web::options("/{tail:.*}")]
async fn cors() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Loads the .env file as a hashtable

    // Loading the entries of the .env file
    let serv_address = env::var("SERVER_ADDRESS").expect("no SERVER_ADDRESS in .env file");
    let serv_port = env::var("SERVER_PORT")
        .expect("no SERVER_PORT in .env file")
        .parse::<u16>()
        .expect("SERVER_PORT in .env file is not a number");
    let db_url = env::var("DATABASE_URL").expect("no DATABASE_URL in .env file");

    // Creating sql pool of connections
    let pool = MySqlPool::connect(&db_url)
        .await
        .expect("Cannot connect to database");

    // Creating global app state
    let server_state = ServerState { pool };

    // Starting the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_state.clone()))
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "http://localhost:5173"))
                    .add(("Access-Control-Allow-Headers", "*")),
            )
            .service(
                web::scope("/auth")
                    .service(apis::authentication::login)
                    .service(apis::authentication::refresh),
            )
            .service(web::scope("/admin").service(apis::admin::get_user))
            .service(cors)
    })
    .bind((serv_address, serv_port))?
    .run()
    .await?;

    Ok(())
}
