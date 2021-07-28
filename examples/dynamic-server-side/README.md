# Dynamic server-side example

This example showcases serving of the index and it's style dynamically.
This also showcase `jss!` macro which lets you easily compose css style dynamically in the server.

## Quick start

```sh
git clone --depth=1 https://github.com/ivanceras/sauron.git
cd examples/dynamic-server-side
./serve.sh
```
Open [http://localhost:3030](http://localhost:3030)

## Explanation
in `server/src/main.rs`, url is rounted into 3 paths
- api calls
- static files
- page with named parameter

The api call is rounted to `/api` this is called from the client when a user
clicks on the button to change the name. This, in turn will execute the function `fake_api_call` which returns a data struct.
This function could be doing a database lookup, but for simplicity we will return the name calculating it's length and capitalized the letters.

