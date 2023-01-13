mod routes;
mod tls;

use std::net::SocketAddr;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Compress;
use tracing::{error, info};

#[actix_web::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let cert_chain = match tls::load_cert_chain("cert.pem") {
    Ok(cert_chain) => cert_chain,
    Err(err) => return error!("could not load cert chain: {}", err),
  };

  let private_key = match tls::load_private_key("key.pem") {
    Ok(private_key) => private_key,
    Err(err) => return error!("could not load private key: {}", err),
  };

  let config = match tls::build_config(cert_chain, private_key) {
    Ok(config) => config,
    Err(err) => return error!("could not build config: {}", err),
  };

  let factory = || {
    let files = Files::new("/static", "static")
      .path_filter(|path, _| path.extension().is_some())
      .use_last_modified(false)
      .disable_content_disposition();

    App::new()
      .wrap(Compress::default())
      .service(files)
      .service(web::resource("/").route(web::to(routes::index)))
      .service(web::resource("/privacy").route(web::to(routes::privacy)))
      .service(web::resource("/terms").route(web::to(routes::terms)))
      .service(web::resource("/invite").route(web::to(routes::invite)))
      .service(web::resource("/support").route(web::to(routes::support)))
      .service(web::resource("/premium").route(web::to(routes::premium)))
      .service(web::resource("/commands").route(web::to(routes::commands)))
      .default_service(web::to(routes::default))
  };

  let addr = SocketAddr::from(([0, 0, 0, 0], 443));
  let server = match HttpServer::new(factory).bind_rustls(addr, config) {
    Ok(server) => server.run(),
    Err(err) => return error!("could not bind: {}", err),
  };

  info!("listening on {}", addr);

  if let Err(err) = server.await {
    error!("{}", err);
  }
}
