/// Главная старница
pub mod desktop {
    use actix_web::{
        HttpResponse,
        web,
        web::ServiceConfig
    };
    // Роутинг
    pub fn config(cfg: &mut ServiceConfig){
        cfg.service(
            web::resource("/")
                .wrap(actix_web::middleware::DefaultHeaders::new().header("v3tracking_service", "desktop"))
                .wrap(actix_web::middleware::NormalizePath)
                .route(web::get().to(|| HttpResponse::Ok().body("test")))
                // .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
        );
    }
}