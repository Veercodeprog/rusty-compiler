use actix_web::{web, App, HttpRequest, HttpServer};
use gcc::Build;

async fn submit_code(req: HttpRequest, body: web::Bytes) -> String {
    let code = String::from_utf8_lossy(&body).to_string();

    // Validate the code, ensuring it meets your requirements
    if code.is_empty() {
        return "Code is empty".to_string();
    }

    // Create a temporary directory to store the code
    let temp_dir = tempdir::TempDir::new("code_submission").unwrap();
    let source_file_path = temp_dir.path().join("code.c");

    // Write the code to the source file
    std::fs::write(&source_file_path, &code).unwrap();

    // Use gcc crate to compile the code
    let build_result = Build::new()
        .file(&source_file_path)
        .out_dir(&temp_dir.path())
        .compile("code");

    // Check if the compilation was successful
    if build_result.is_err() {
        return "Code compilation failed".to_string();
    }

    // Execute the compiled binary and capture the output
    let output = std::process::Command::new(temp_dir.path().join("code"))
        .output()
        .expect("Failed to execute the code");

    // Capture the output and return it
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    format!("Output:\n{}\nErrors:\n{}", stdout, stderr)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/submit", web::post().to(submit_code))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
