/// contains the rest apis that are consumed by the client
mod apis;

/// contains representations of database models
mod model;

/// does nothing for now
mod email;

/// manages the jwt creation and verification
mod jwt_handler;

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
            .service(
                web::scope("/admin")
                    .service(apis::admin::get_user)
                    .service(apis::admin::get_users)
                    .service(apis::admin::create_user)
                    .service(apis::admin::create_users)
                    .service(apis::admin::update_user)
                    .service(apis::admin::delete_user)
                    .service(apis::admin::create_virtual_platform),
            )
            .service(
                web::scope("/vice-doyen")
                    .service(apis::vice_doyen::get_modules)
                    .service(apis::vice_doyen::create_module)
                    .service(apis::vice_doyen::delete_module)
                    .service(apis::vice_doyen::get_possible_applicants)
                    .service(apis::vice_doyen::get_current_applicants)
                    .service(apis::vice_doyen::get_applicant)
                    .service(apis::vice_doyen::affect_applicant)
                    .service(apis::vice_doyen::delete_applicant)
                    .service(apis::vice_doyen::get_announcement)
                    .service(apis::vice_doyen::create_announcement)
                    .service(apis::vice_doyen::delete_announcement)
                    .service(apis::vice_doyen::create_session)
                    .service(apis::vice_doyen::get_session)
                    .service(apis::vice_doyen::get_sessions)
                    .service(apis::vice_doyen::update_session)
                    .service(apis::vice_doyen::delete_session),
            )
            .service(
                web::scope("/account")
                    .service(apis::account::change_email)
                    .service(apis::account::change_password),
            )
            .service(cors)
    })
    .bind((serv_address, serv_port))?
    .run()
    .await?;

    Ok(())
}
