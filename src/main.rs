use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Result};
use askama::Template;
use futures::{StreamExt, TryStreamExt};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> Result<HttpResponse> {
    let html = IndexTemplate.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let upload_dir = "./uploads";
    std::fs::create_dir_all(upload_dir).ok();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_path = format!("{}/uploaded_file.jpg", upload_dir);
        let mut file_bytes = Vec::new();

        // Accumulate the chunks into a Vec<u8>
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file_bytes.extend_from_slice(&data);
        }

        // Write the accumulated bytes to the file
        web::block(move || std::fs::write(file_path, file_bytes)).await?;
    }

    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("File uploaded successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
