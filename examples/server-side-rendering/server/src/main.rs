use warp::{Filter, http::Response};
use serde_json;
use client::{App, Data, FetchStatus};
use sauron::prelude::*;

// Replace this with whatever data you're actually trying to return
fn fake_api_call(name: String) -> Data {
    Data {
        length: name.len(),
        modified_name: name.to_uppercase(),
    }
}

// path relative to the working directory when you run the server binary
const STATIC_DIR: &str = "client/static";
// path relative to this file
static INDEX_HTML: &'static str = include_str!("../../client/static/index.html");

#[tokio::main]
async fn main() {
    let api_call = warp::path!("api" / String)
        .map(|name| {
            serde_json::to_string(&fake_api_call(name)).unwrap()
        });
    
    // Static assets: CSS, JS, etc.
    let static_files = warp::path("static")
        .and(warp::fs::dir(STATIC_DIR));

    let render_page = |name: String| {
        // initialize blank app state
        let mut app = App::new();

        // Fetch API data for the argument and stuff it into the app
        app.name = name.clone();
        let api_data = fake_api_call(name);
        app.data = FetchStatus::Complete(api_data);

        // Serialize the state 
        let serialized_state = serde_json::to_string(&app).unwrap();

        // Render the app into a String buffer
        let node = app.view();
        let mut buffer = String::new();
        node.render(&mut buffer).expect("must render");

        // Render the page
        let rendered_index_page = INDEX_HTML
            .replace("<main></main>", &buffer)
            // pass the serialized state as argument to the main() function, defined in client/src/lib.rs
            .replace("main(``);", &format!("main(`{}`);", serialized_state));
        Response::builder().body(rendered_index_page)
    };

    // Render paths that include a name argument
    let named = warp::path!(String)
        .map(render_page);

    // Render paths that don't include a name with a default
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