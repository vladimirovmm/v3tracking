use actix_web::Responder;
use actix_files::NamedFile;

/// Фавиконка
pub async fn favicon() -> actix_web::Result<NamedFile> { actix_web::Result::Ok(NamedFile::open("media/favicon.ico")?) }
/// 404 cтраница
pub async fn page_404() -> actix_web::Result<NamedFile> { actix_web::Result::Ok(NamedFile::open("media/template/page/404.html")?) }
