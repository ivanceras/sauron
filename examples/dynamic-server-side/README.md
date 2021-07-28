# Dynamic server-side example

This example showcases serving of the index and it's style dynamically.
This also showcase `jss!` macro which lets you easily compose css style dynamically in the server.
The goal of this setup is to have a constant look when the page is rendered from the server and even after the dynamic part is loaded and re-rendered on the client.
This is accomplished by rendering the page with the same data in the server as in the client.

The other advantage is that the page will still work even if you disable javascript.

## Quick start

```sh
git clone --depth=1 https://github.com/ivanceras/sauron.git
cd examples/dynamic-server-side
./serve.sh
```
Open [http://localhost:3030](http://localhost:3030)

## Explanation
in `server/src/main.rs`, url is rounted into 4 main paths
- root
    - ie: `http://localhost:3030/`
    - This serves the index page
- api calls
    - ie: `http://localhost:3030/api/`
- static files
    - ie: `/pkg/client.js`
- page with named parameter
    - ie: `/Foo Bar`

The api call is rounted to `/api` this is called from the client when a user
clicks on the button to change the name. This, in turn will execute the function `fake_api_call` which returns a data struct.
This function could be doing a database lookup, but for simplicity we will return the name calculating it's length and capitalized the letters.


The `index` page is served and is build in `page.rs` module which is then routed in `named` and `root` route.
The function `index` in page module takes `App` as a parameter. This `App` contains all the data we need to render the page.
First, we can generate a `view` of the `App`. We can then inject the view in the body of the html.
Secondly, we can derive a `serialized_state` from the app by serializing the `App` into json.
This `serialized_state` will be used in the `main` function in the client which will be executed when the page is completely loaded in the browser.



