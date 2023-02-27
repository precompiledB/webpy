use std::{process::Command, sync::{Arc, RwLock}};

use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    routing::{get, get_service, post},
    Router, Extension,
};
use tokio::io::AsyncReadExt;
use tower_http::{add_extension::AddExtensionLayer, services::{ServeDir, ServeFile}};
use tower::{ServiceBuilder};

use shared_structs::tasks;

#[tokio::main]
async fn main() {
    // build our application with a single route
    tracing_subscriber::fmt::init();

    let f = |path| {
        get_service(ServeFile::new(path)).handle_error(|error: std::io::Error| async move {
            println!("ERR {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        })
    };

    let d = |path| {
        get_service(ServeDir::new(path)).handle_error(|error: std::io::Error| async move {
            println!("ERR {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        })
    };

    let mut counter = Arc::new(RwLock::new(0));

    // our router
    let app = Router::new()
        .route("/", f("webpy/dist/index.html"))
        .route("/assets/next_assignment", get(next_file))
            .layer(ServiceBuilder::new()
                .layer(AddExtensionLayer::new(counter)))
        .route("/assets/:name", get(give_file))
        .route("/execute_python/", post(exe_py))
        .route("/:name", d("./webpy/dist/"));

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn next_file(Extension(counter): Extension<Arc<RwLock<u32>>>) -> String {
    let mut data = String::new();
    
    {
        let cnt = counter.try_read().unwrap();
        let path = format!("./webpy/assets/task{}.toml", *cnt);
        tokio::fs::File::open(std::path::Path::new(&path))
            .await
            .unwrap()
            .read_to_string(&mut data)
            .await
            .unwrap();
    }
    {
        let mut cnt = counter.write().unwrap();
        *cnt += 1;
    }

    data
}

async fn give_file(Path(path): Path<String>) -> String {
    let mut data = String::new();
    tokio::fs::File::open(std::path::Path::new("./webpy/assets").join(path))
        .await
        .unwrap()
        .read_to_string(&mut data)
        .await
        .unwrap();
    data
}

enum ConsoleOutput {
    Err(String),
    Suc(String),
    Inf(String), // arbitrary info for user
}

impl ConsoleOutput {
    fn to_str(self) -> String {
        match self {
            ConsoleOutput::Err(err) => format!("Err:{err}"),
            ConsoleOutput::Suc(suc) => format!("Suc:{suc}"),
            ConsoleOutput::Inf(inf) => format!("Inf:{inf}"),
        }
    }
}

// TODO: remove error prone error handling
async fn exe_py(payload: String) -> String {
    use std::io::{Read, Write};
    use ConsoleOutput::*;

    let mut file = tempfile::NamedTempFile::new().expect("Cannot create temp file");

    file.write_all(payload.as_bytes())
        .expect("Cannot write in tempfile");

    let path = file.into_temp_path();

    let cmd = Command::new("python")
        .arg("webserver/pyenv/load_tester.py")
        .arg(format!("{}", path.display()))
        .output()
        .expect("Couldn't execute python");

    if !cmd.status.success() {
        let error_msg = String::from_utf8(cmd.stderr).expect("No UTF8");
        println!("{error_msg}");
        // TODO: Remove file path
        Err(error_msg)
    } else {
        let succes_msg = String::from_utf8(cmd.stdout).expect("No UTF8");
        Suc(succes_msg)
    }
    .to_str()
}
