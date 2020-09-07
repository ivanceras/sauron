use warp::{Filter, http::Response};
use serde_json;
use client::{App, Data, FetchStatus};
use sauron::prelude::*;

fn fake_api_call(name: String) -> Data {
    Data {
        length: name.len(),
        modified_name: name.to_uppercase(),
    }
}

const STATIC_DIR: &str = "client/static";
static INDEX_HTML: &'static str = include_str!("../../client/static/index.html");

#[tokio::main]
async fn main() {
    let api_call = warp::path!("api" / String)
        .map(|name| {
            serde_json::to_string(&fake_api_call(name)).unwrap()
        });
    
    let static_files = warp::path("static")
        .and(warp::fs::dir(STATIC_DIR));

    let render_page = |name: String| {
        let mut buffer = String::new();
        let mut app = App::new();
        app.name = name.clone();

        let api_data = fake_api_call(name);
        app.data = FetchStatus::Complete(api_data);

        let serialized_state = serde_json::to_string(&app).unwrap();

        let node = app.view();
        node.render(&mut buffer).expect("must render");

        let rendered_index_page = INDEX_HTML
            .replace("<main></main>", &buffer)
            .replace("main(``);", &format!("main(`{}`);", serialized_state));
        Response::builder().body(rendered_index_page)
    };

    let named = warp::path!(String)
        .map(render_page);

    let root = warp::path::end()
        .map(move || render_page(String::from("Ferris")));

    let routes = warp::get().and(
        root
        .or(named)
        .or(api_call)
        .or(static_files)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}