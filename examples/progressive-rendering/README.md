# Progressive rendering example

This example showcases sauron's both server-side and client-side rendering capability.
The goal of this setup is to have a consistent look when the page loaded from the server and when it is re-rendered in the client.
This is accomplished by rendering the page with the same data in the server as in the client.


## Quick start

Make sure you have the installed pre-requisite.
Go to [rustup.rs](http://rustup.rs) if you haven't installed rust.
Then we need to add `wasm32-unknown-unknown` to our target list.
Install also `wasm-pack`, to simplfiy our workflow in compiling and packaging our client rust code into `wasm`.

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
Please take a look at the code of this example as you follow along this README guide.

In [`server/src/main.rs`](https://github.com/ivanceras/sauron/blob/master/examples/progressive-rendering/server/src/main.rs), we use `warp` for our server.
The url is routed into 5 main paths:
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
When the user navigates to `http://localhost:3030/`. An html file is served by the web server.
The whole page created in `index` function found in [`server/src/page.rs`](https://github.com/ivanceras/sauron/blob/master/examples/progressive-rendering/server/src/page.rs)
The `index` function takes `App` as a parameter, this contains the data we need to render the page.
Since `App` struct is a sauron `Component`, we can call the `view` function on it, which returns a `Node<Msg>`.
We then inject this view into the body of our generated html. Take note of the `{view}` notation.

### Client
To use the same state we have in the server, we can derive a `serialized_state` from the app by serializing the `App` into json.
This `serialized_state` is then passed in the `main` function of client code which will be executed, right after the page is loaded in the browser.
The `main` function in [`client/src/lib.rs`](https://github.com/ivanceras/sauron/blob/master/examples/progressive-rendering/client/src/lib.rs) is the code that will be called when the script has loaded.
From there, we can recreate the `App` by deserializing the `serialized_state`. Our `App` is a [`Component`](https://docs.rs/sauron/0.34.0/sauron/trait.Component.html) in `sauron` which we then can mount into the an anchor element in the document.
In this case, we just replace the `<main>..</main` element in the page. All the state changes, diffing, and patches is handled by `sauron` framework.

### Api call
The api call is routed to `/api`, and is followed with a String type.
This returns a json derived from the supplied name.
Example:

`/api/Foo Bar`

```json
{"length":7,"modified_name":"FOO BAR"}
```
This `/api` route is called from the client when the user clicks on the `Okay!` submit button.
There are actually 2 possible scenarios that can happen here.

1. If javascript is enabled in the browser.
    - We hooked into the form `on_submit` event and immediately call on `prevent_default()` to prevent the browser on submitting the form
    and instead, we pass on `Msg::QueryAPI` which in turn execute an http fetch to the server.
2. If there is no javascript capability in the browser
    - The form will submit to the server, since there is no way to cancel it with `prevent_default`.

### Submit form
The server has a submit route which expects a form data, where we specify to map it into a `HashMap<String,String>`.
We then extract the value of `name` from the `HashMap`. This `name` is then used as argument to `render_page`, which return an html file with the supplied `name` rendered on it.


### Final thoughts
We can now finally use rust to efficiently serve progressive webapps.
Sauron can be use to render the view either via client of via server whichever is convenient or available.
Here we demonstrated, that the page can still work even if the javascript is disabled in the browser, by rendering the page server-side.
