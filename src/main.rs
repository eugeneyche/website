extern crate actix_web;
extern crate chrono;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate pulldown_cmark;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate walkdir;

mod post;

use actix_web::{fs, http, middleware, server, App, Json, HttpRequest, Result};
use std::path::{Path, PathBuf};
use std::fs::File;


fn crate_path<'a, P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}


#[derive(Clone)]
struct AppState {
    post_store: post::Store,
}


fn index(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(&crate_path("static/index.html"))?)
}


#[derive(Serialize)]
struct ListPostsResp {
    posts: Vec<post::Post>,
}


fn list_posts(req: &HttpRequest<AppState>) -> Json<ListPostsResp> {
    Json(ListPostsResp {posts: req.state().post_store.list_posts()})
}



fn run_app(address: &str, posts_root_path: &str, num_workers: usize) -> Result<(), &'static str> {
    info!("Starting server");
    let state = AppState {
        post_store: post::Store::new(posts_root_path),
    };
    server::new(move || {
        info!("Initialing app worker");
        App::with_state(state.clone())
            .middleware(middleware::Logger::new("%a %r %{User-Agent}i"))
            .handler("/static", fs::StaticFiles::new(&crate_path("static/dist")).expect("can serve statics"))
            .resource("/api/list_posts", |r| r.method(http::Method::POST).f(list_posts))
            .resource("/{path:.*}", |r| r.method(http::Method::GET).f(index))
            .finish()
    })
        .bind(address).map_err(|_| "Cannot to bind to address")?
        .workers(num_workers)
        .run();
    info!("Terminating server");
    Ok(())
}


#[derive(Deserialize)]
struct AppConfig {
    address: String,
    posts_root_path: String,
    num_workers: Option<usize>,
}


fn main() {
    env_logger::init();

    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 2 {
        eprintln!("Missing argument <config path>.");
        return;
    }
    let file = match File::open(&argv[1]) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open config file at \"{}\".", argv[1]);
            return
        }
    };
    let config: AppConfig = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to parse config: {:?}", err);
            return
        }
    };
    match run_app(
        &config.address,
        &config.posts_root_path,
        config.num_workers.unwrap_or(4),
    ) {
        Err(message) => {
            eprintln!("Error occured while initializing: {}", message);
        },
        _ => (),
    }
}
