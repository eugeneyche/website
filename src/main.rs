extern crate actix_web;
extern crate chrono;
extern crate env_logger;
extern crate log;
extern crate pulldown_cmark;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate walkdir;

mod post;

use actix_web::{fs, http, middleware, server, App, Json, HttpRequest, Result};
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct AppState {
    post_store: post::Store,
}

fn crate_path<'a, P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
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

fn main() {
    env_logger::init();
    let argv: Vec<String> = std::env::args().collect();
    let state = AppState {post_store: post::Store::new(&argv[1])};
    server::new(move || {
        App::with_state(state.clone())
            .middleware(middleware::Logger::default())
            .handler("/static", fs::StaticFiles::new(&crate_path("static/dist")).expect("can serve statics"))
            .resource("/api/list_posts", |r| r.method(http::Method::POST).f(list_posts))
            .resource("/{path:.*}", |r| r.method(http::Method::GET).f(index))
            .finish()
    })
    .bind("127.0.0.1:8000").expect("can bind to socket")
    .workers(16)
    .run();
}    
