use warp::{Filter, http::Response};
use client::{App, Data};
use sauron::prelude::*;
use std::net::SocketAddr;
use std::collections::HashMap;
use percent_encoding::percent_decode_str;

mod page;

// path relative to the working directory when you run the server binary
const PKG_DIR: &str = "client/pkg";
const FAVICON_FILE: &str = "client/favicon.ico";
const DEFAULT_NAME: &str = "Ferris";
const DEFAULT_PORT: u16 = 3030;

// Replace this with whatever data you're actually trying to return
// This is normally something where the client side can not do
// such as accessing a database.
fn fake_api_call(name: String) -> Data {
    Data {
        length: name.len(),
        modified_name: name.to_uppercase(),
    }
}


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
        // Fetch API data for the argument and stuff it into the app
        let api_data = fake_api_call(name.clone());
        let app = App::with_name_and_data(&name, api_data);

        let rendered_index_page = page::index(&app).render_to_string_pretty();

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

    // These are the example url paths
    // GET
    //   /
    //   /favicon.ico
    //   /api
    //   /Foo Bar
    //   /pkg/client.js
    //
    // POST
    //   /?name=Foo Bar
    let routes = warp::get().and(
        root
        .or(favicon)
        .or(api_call)
        .or(named)
        .or(pkg_files)
    ).or(submit);


    let port = if let Ok(port) = std::env::var("PORT") {
        if let Ok(port) = port.parse::<u16>() {
            port
        } else {
            DEFAULT_PORT
        }
    } else {
        DEFAULT_PORT
    };

    let socket: SocketAddr = ([0, 0, 0, 0], port).into();
    println!("serve at http://{}:{}", socket.ip(), socket.port());
    warp::serve(routes)
        .run(socket)
        .await;
}
