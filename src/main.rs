use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use context_analyzer::{
    db,
    three_grams::routers::{self, AppData},
};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

/// The main function of the application.
///
/// This function initializes the application and starts the server.
///
/// # Returns
///
/// A `std::io::Result` containing the result of the function.
///
/// # Errors
///
/// If the server fails to start, an error is returned.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let session = match db::init().await {
        Ok(session) => session,
        Err(e) => {
            eprintln!("{}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to connect to ScyllaDB",
            ));
        }
    };

    let data = Data::new(AppData {
        scy_session: session,
    });

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .configure(routers::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
