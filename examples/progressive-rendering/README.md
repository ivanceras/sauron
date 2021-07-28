# Progressive rendering example

This example showcases sauron's server-side rendering capability.
This also showcase `jss!` macro which lets you easily compose css style dynamically in the server.
The goal of this setup is to have a consistent look when the page is rendered from the server and even after the dynamic part is loaded and re-rendered on the client.
This is accomplished by rendering the page with the same data in the server as in the client.


## Quick start

Make sure you have the installed pre-requisite.
Go to [rustup.rs](http://rustup.rs) if you haven't installed rust.
Then we need to add `wasm32-unknown-unknown` to our target list.
Install also `wasm-pack`. To easily compile and package our client rust code into `wasm`.

```sh
 rustup target add wasm32-unknown-unknown
 cargo install wasm-pack
```

```sh
git clone --depth=1 https://github.com/ivanceras/sauron.git
cd examples/progressive-rendering
./serve.sh
```
Open [http://localhost:3030](http://localhost:3030)

## What's going on?

in `server/src/main.rs`, url is rounted into 5 main paths
- root
    - ie: `http://localhost:3030/`
    - This serves the index page.
- api calls
    - ie: `http://localhost:3030/api/`
    - Example calls: `/api/Foo Bar`
    - This just respond serialized json `Data` based on the supplied `name`.
- static files
    - ie: `/pkg/client.js`
    - This serves the static files in `/pkg` directory where the compiled client files are located.
- page with named parameter
    - ie: `/Foo Bar`
    - This serves the index page, but with the name rendered in it.
- form submit
    - ie: `/?name=Foo Bar`
    - This extracts the name value in the form submitted and render the page supplied with the `name`.
    - Form submit will only activate if the client has no javascript enabled.
    - You can test this by installing an add-on like `noscript` to disable the javascript in your browser.

### Index page
When the user navigates to `http://localhost:3030/`. An html file is served by the war server.
The whole page is served in `index` function found in `server/src/page.rs`
The `index` function takes `App` as a parameter, this contains the data we need to render the page.
Since `App` struct is a sauron `Component`, we can call the `view` function on it, which returns a `Node<Msg>`.
We then inject this view into the body of our generated html. Take note of the `{view}` notation.

### Client
To use the same state we have in the server, we can derive a `serialized_state` from the app by serializing the `App` into json.
This `serialized_state` is then passed in the `main` function of client code which will be executed, right after the page is loaded in the browser.
The `main` function in `client/src/lib.rs` is the code that will be called when the script has loaded.
From there, we can recreate the `App` by deserializing the `serialized_state`. Our `App` is a component in `sauron` which we then can mount into the an anchor element in the document.
In this case, we just replace the `<main>..</main` element in the page. All the state changes, diffing, and patches is handled by `sauron` framework.

### Api call
The api call is routed to `/api`, and is followed with a String type.
This returns a json derived from the supplied name.
Example:

`/api/Foo Bar`

```
{"length":7,"modified_name":"FOO BAR"}
```
This `/api` route is used in the client when the user clicks on the `Okay!` submit button.
There are actually 2 possible scenarios that can happen here.

1. If javascript is enabled in the browser.
    - We hooked into the form `on_submit` event and immediately call on `prevent_default()` to prevent the browser on submitting the form
    and instead, we pass on `Msg::QueryAPI` which in turn execute an http fetch to the server.
2. If there is no javascript capability in the browser
    - The form submit action will execute since there was no javascript to cancel it with `prevent_default`.
    - The form data will be submitted to the server and will be served in the `submit` route, which is expecting a form data.

### Submit form
The server has a route to `submit` which expects data triggered by submit form from the client.
We then extract the value of `name` from the `HashMap`. This `name` is then used as argument to `render_page` to render the page
with using data from the the submitted name.


