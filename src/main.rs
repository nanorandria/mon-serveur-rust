#[macro_use] extern crate rocket;

use rocket::fs::{NamedFile, FileServer, relative};
use rocket::http::ContentType;
use rocket::data::{Data, ByteUnit};
use rocket::serde::json::Json;
use rocket::State;
use std::path::PathBuf;
use uuid::Uuid;
use std::sync::Arc;

struct AppState {
    host: String,
}

#[derive(serde::Serialize)]
struct UploadResponse {
    url: String,
}

#[post("/upload", data = "<data>")]
async fn upload(
    content_type: &ContentType,
    data: Data<'_>,
    state: &State<Arc<AppState>>
) -> Json<UploadResponse> {
    let id = Uuid::new_v4().to_string();
    let filename = format!("/tmp/{}.jpg", id);

    if let Ok(mut file) = tokio::fs::File::create(&filename).await {
        let _ = data.open(ByteUnit::from_bytes(10 * 1024 * 1024)).stream_to(&mut file).await;
    }

    Json(UploadResponse {
        url: format!("{}/temp/{}.jpg", state.host, id),
    })
}

#[get("/temp/<filename>")]
async fn temp_file(filename: String) -> Option<NamedFile> {
    let path = format!("/tmp/{}", filename);
    NamedFile::open(PathBuf::from(path)).await.ok()
}

#[launch]
fn rocket() -> _ {
    let host = std::env::var("HOST_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());

    rocket::build()
        .manage(Arc::new(AppState { host }))
        .mount("/", routes![upload, temp_file])
        .mount("/static", FileServer::from(relative!("static")))
}
