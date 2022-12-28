use crate::routes::{health_check, subscribe};
use std::net::TcpListener;

use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        // HttpServer::new does not take App as argument - it wants a closure that returns an App struct.
        // This is to support actix-web’s runtime model: actix-web will spin up a worker process for each available
        // core on your machine.
        // Each worker runs its own copy of the application built by HttpServer calling the very same closure that
        // HttpServer::new takes as argument.
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
