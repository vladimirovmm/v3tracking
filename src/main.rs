mod system;
use actix_web::{App, HttpServer, HttpRequest, Responder};

async fn default_page(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Данные для работы системы
    let mut system = system::System::inic().unwrap();
    // Адрес прослушивания
    let addr = system.configs.listen.get_addr();

    HttpServer::new(||{
        App::new()
            // Стартовая панель/Главная страница
            // .configure(controllers::admin::panel::Panel::config_routes)
            .route("/", actix_web::web::get().to(default_page))
        })
        // Количество потоков
        .workers(system.configs.listen.get_workers() )
        // Количество одноврименных соединений.
        // если поставить 1 то только один пользователь сможет пользоваться сайтом
        // пока его соединение не закроется
        .maxconn(system.configs.listen.get_maxconn())
        .maxconnrate(system.configs.listen.get_maxconnrate())
        // Время жизни соединения
        .keep_alive(system.configs.listen.get_keep_alive())
        // Время ожидания соединения
        .client_timeout(system.configs.listen.get_client_timeout())
        // Врем выполнения
        .client_shutdown(system.configs.listen.get_client_shutdown())
        // Адрес прослушивания
        .bind(addr.as_str())?
        .run()
        .await
}