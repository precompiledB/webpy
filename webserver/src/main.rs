use std::{
    process::Command,
    sync::{Arc, RwLock},
};

use axum::{
    body::Body,
    extract::{Path, Query},
    http::{Request, StatusCode},
    routing::{get, get_service, post},
    Extension, Router,
};
use tokio::io::AsyncReadExt;
use tower::ServiceBuilder;
use tower_http::{
    add_extension::AddExtensionLayer,
    services::{ServeDir, ServeFile},
};

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

    // our router
    let app = Router::new()
        .route("/", f("webpy/dist/index.html"))
        .route("/assets/:name", get(give_file))
        .route("/execute_python/", post(exe_py))
        .route("/:name", d("./webpy/dist/"));

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn give_file(Path(path): Path<String>) -> String {
    let mut data = String::new();
    tokio::fs::File::open(std::path::Path::new("webserver/assets").join(path))
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

#[derive(serde::Deserialize, Debug)]
struct CurrentData {
    assignment: i32,
    task: i32,
}

// TODO: remove error prone error handling
async fn exe_py(Query(CurrentData { assignment, task }): Query<CurrentData>, payload: String) -> String {
    use std::io::{Read, Write};
    use ConsoleOutput::*;

    println!("Query: assignemnt {assignment} & lesson {task}");

    let mut file = tempfile::NamedTempFile::new().expect("Cannot create temp file");

    dbg!(&payload);

    file.write_all(payload.as_bytes())
        .expect("Cannot write in tempfile");

    let path = file.into_temp_path();

    let cmd = Command::new("python")
        .arg("webserver/pyenv/load_tester.py")
        .arg(format!("{}", path.display()))
        .arg(format!("{assignment}_{task}"))
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
