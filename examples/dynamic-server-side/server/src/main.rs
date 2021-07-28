use warp::{Filter, http::Response};
use serde_json;
use client::{App, Data, FetchStatus};
use sauron::prelude::*;
use std::net::SocketAddr;
use std::collections::HashMap;
use percent_encoding::percent_decode_str;

mod page;

// Replace this with whatever data you're actually trying to return
fn fake_api_call(name: String) -> Data {
    Data {
        length: name.len(),
        modified_name: name.to_uppercase(),
    }
}

// path relative to the working directory when you run the server binary
const PKG_DIR: &str = "client/pkg";
const FAVICON_FILE: &str = "client/favicon.ico";
const DEFAULT_NAME: &str = "Ferris";


#[tokio::main]
async fn main() {
    let api_call = warp::path!("api" / String)
        .map(move|name:String| {
            let name = percent_decode_str(&name).decode_utf8_lossy().to_string();
            serde_json::to_string(&fake_api_call(name)).unwrap()
        });

    // The compiled javascript and wasm in the client.
    let pkg_files = warp::path("pkg")
        .and(warp::fs::dir(PKG_DIR));

    let favicon = warp::path("favicon.ico").and(warp::fs::file(FAVICON_FILE));

    let render_page = |name: String| {
        // initialize blank app state
        let mut app = App::new();
        // Fetch API data for the argument and stuff it into the app
        app.name = name.clone();
        let api_data = fake_api_call(name);
        app.data = FetchStatus::Complete(api_data);

        let rendered_index_page = page::index(&app).render_to_string();

        Response::builder().body(rendered_index_page)
    };

    // Render paths that include a name argument
    let named = warp::path!(String)
        .map(move|name: String| {
            let name = percent_decode_str(&name).decode_utf8_lossy().to_string();
            render_page(name)
        });

    // Render paths that don't include a name with a default
    let root = warp::path::end()
        .map(move || render_page(DEFAULT_NAME.to_string()));

    // When the user has no javascript enabled, we render the page
    // with the name extracted from the form.
    // The html form is in [`App.view`](client/src/lib.rs) where the name is the text input named `name`.
    let submit = warp::body::form()
        .map(move|value_map:HashMap<String,String>|{
            let mut name = if let Some(name) = value_map.get("name"){
                name.to_string()
            }else{
                DEFAULT_NAME.to_string()
            };

            if name.trim().is_empty(){
                name = DEFAULT_NAME.to_string();
            }
            render_page(name)
        });

    let routes = warp::get().and(
        root
        .or(favicon)
        .or(named)
        .or(api_call)
        .or(pkg_files)
    ).or(submit);


    let socket: SocketAddr = ([127, 0, 0, 1], 3030).into();
    println!("serve at http://{}:{}", socket.ip(), socket.port());
    warp::serve(routes)
        .run(socket)
        .await;
}
