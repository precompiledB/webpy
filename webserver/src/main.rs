use std::process::Command;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};
use tokio::io::AsyncReadExt;
use tower_http::services::{ServeDir, ServeFile};

mod tasks;

#[tokio::main]
async fn main() {
    let test: tasks::Assignment = tasks::Assignment::from_toml(r#"
        description = "hello"
        status = "Current"

        [[Task]]
        description = "task1"
        info = "no"
        help = "some help text"
        template = "print('hello world')"
        status = "Current"

        [[Task]]
        description = "task2"
        info = "huh?"
        help = "some help text"
        template = "print('hello world')"
        status = "Current"

    "#).unwrap();

    println!("{:#?}", test);

    // build our application with a single route
    //tracing_subscriber::fmt::init();
    let f = |path| {
        get_service(ServeFile::new(path)).handle_error(|error: std::io::Error| async move {
            eprintln!("ERR {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        })
    };

    // our router
    let app = Router::new()
        .route("/", f("webpy/index.html"))
        .route("/DEBUG", f("webpy/ace_edit.html")) // TODO: remove
        .route("/favicon.ico", f("webpy/favicon.ico"))
        .route("/style.css", f("webpy/style.css"))
        .route("/pkg/webpy_bg.wasm", f("webpy/pkg/webpy_bg.wasm"))
        .route("/pkg/webpy.js", f("webpy/pkg/webpy.js"))
        .route("/assets/:name", get(give_file))
        .route("/execute_python/", post(exe_py));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
    }.to_str()
    

    // if let Ok(mut child) = Command::new("python")
    //     .arg("test")// format!("{}", path.display())
    //     .spawn()
    // {
    //     child.wait().expect("command wasn't running");
    //     println!("Child has finished its execution!");
    //     let mut stdout = child.stdout.take().expect("Cannot get stdout");
    //     let mut buf = String::new();
    //     stdout.read_to_string(&mut buf).expect("Cannot read from string");
    //     buf
    // } else {
    //     println!("ls command didn't start");
    //     "ERROR".into()
    // }
}
