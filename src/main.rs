mod system;
mod controllers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    // Данные для работы системы
    let mut system = system::System::inic().unwrap();
    // Адрес прослушивания
    let addr = system.configs.listen.get_addr();

    HttpServer::new(||{
        App::new()
            // Главная | DESKTOP
            .configure(controllers::desktop::desktop::config)

            //
            // media файлы CSS|JS|Images
            .service(actix_files::Files::new("/media", "media/"))
            // Фавиконка
            .route("/favicon.ico", actix_web::web::get().to(controllers::other::favicon))
            // Страница по умолчанию | 404
            .default_service( actix_web::web::get().to(controllers::other::page_404) )
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