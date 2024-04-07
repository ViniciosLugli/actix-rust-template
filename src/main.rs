extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
	info!("Hello, world!");

	HttpResponse::Ok().json(serde_json::json!({
		"message": "Hello, world!"
	}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenvy::dotenv().ok();
	pretty_env_logger::init();

	HttpServer::new(|| App::new().wrap(Logger::default()).service(index)).bind(("127.0.0.1", 3000))?.run().await
}

#[cfg(test)]
mod tests {
	use actix_web::{http::header::ContentType, test, App};

	use super::*;

	#[actix_web::test]
	async fn test_index() {
		let app = test::init_service(App::new().service(index)).await;
		let req = test::TestRequest::default().insert_header(ContentType::plaintext()).to_request();
		let resp = test::call_service(&app, req).await;

		assert!(resp.status().is_success());

		let json: serde_json::Value = test::read_body_json(resp).await;
		assert_eq!(
			json,
			serde_json::json!({
				"message": "Hello, world!"
			})
		);
	}
}
