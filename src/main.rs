use axum::{
    extract::Multipart,
    routing::{get, post},
    http::StatusCode,
    Json,
    Router,
};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use chrono::Utc;
use std::path::Path;
use serde::{Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/upload", post(upload_files));

    let host_and_port: String = String::from("0.0.0.0:3050");

    let listener = tokio::net::TcpListener::bind(host_and_port.clone()).await.unwrap();
    
    println!("Server is listening on: {}", host_and_port);
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "<h1>Yeah, the application is working! :D :)</h1>"
}

async fn upload_files(mut multipart: Multipart) -> (StatusCode, Json<FilesUploadedMessage>) {
    // Creates a string with Year/Month/Day
    let date_dir: String = Utc::now().format("%Y/%m/%d").to_string();
    let upload_dir: String = format!("./uploads/{}/", date_dir);

    // Creates the directory if it doesn't exist
    if !Path::new(&upload_dir).exists() {
        fs::create_dir_all(&upload_dir).await.unwrap();
    }

    // Processes each field in the multipart form
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(file_name) = field.file_name() {
            let file_name: String = file_name.to_string();
            let file_path: String = format!("{}{}", upload_dir, file_name);

            // Extracts the file data
            let data = field.bytes().await.unwrap();

            // Saves the file
            let mut file = File::create(&file_path).await.unwrap();
            file.write_all(&data).await.unwrap();

            println!("File '{}' uploaded successfully!", file_name);
        }
    }

    let response = FilesUploadedMessage{ message:format!("The files were uploaded to: {}", upload_dir)};
    (StatusCode::CREATED, Json(response))
}

#[derive(Serialize)]
struct FilesUploadedMessage {
    message: String,
}