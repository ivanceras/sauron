# Getting started

Let's build a simple web application with sauron.
This will just display a text "hello" in the page.

## Pre-requisite

Make sure you have all the prerequisite installed.
- rust and cargo
- wasm-pack
- basic-http-server

If you haven't installed rust, head over to https://rustup.rs/ to install it.
Once rust is installed, the package manager called `cargo` is automatically available to be used.
We can then easily install the other 2 pre-requisite since they all published as executable in `crates.io`.

We will be using `wasm-pack` to facilitate compiling rust code into webassembly.
It will generate the necessary javascript shims, optimize the output and put then in the `./pkg` directory.

```sh
cargo install wasm-pack
```

We also use `basic-http-server` to easily serve static files locally.
```sh
cargo install basic-http-server
```

## Creating a new project
We will create a new project called `hello`.
```
cargo new --lib hello
```
This will create a new folder `hello` with set of files necessary to be compiled as a rust project.
Try to compile this project to test if we installed rust correctly.
```
cd hello
cargo build
```

If you look at `Cargo.toml`, this is what you should see.

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[dependencies]
```


Take note of the package name as the filenames of the compiled binary be derived from it.
There is also `src/lib.rs` which has a stub code on it.
In summary there is only 2 files needed to create a minimum rust crate: `Cargo.toml` and `src/lib.rs`.

## Using sauron
Since we are making a web application we need to specify in `Cargo.toml` that this crate needs to be compiled as 'cdylib'.

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
```

Next, we need to add sauron as our dependency.

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
sauron = "0.49"
```
Next, we modify `src/lib.rs` with our application code.
This will just display a "hello" text inside a paragraph.

```rust
use sauron::prelude::*;

struct App;

impl Application<()> for App {
    fn view(&self) -> Node<()> {
        node! {
            <p>
                "hello"
            </p>
        }
    }

    fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::mount_to_body(App);
}
```
Take notice of the `view` method. Here we are using `node!` macro which takes html-like syntax to be used to display our app.
We implement the `Application` trait for our `App` so that we can implement the required methods necessary to tell sauron how out app behaves.

To compile, we issue the command:
```shell
wasm-pack build --release --target=web
```
As mentioned earlier,`wasm-pack` helps us simplify the process of compiling rust for targetting web applications.
A folder `./pkg` is then created inside our project. This will contain the resulting compiled files.
We only pay attention to the 2 files, named derived from the given package name `<package_name>.js` and `<package_name>_bg.wasm`.
In our case, it will be `hello.js` and `hello_bg.wasm`.

We need to reference this file in our page. Let's create `index.html` in our project.
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <script type=module>
      import init from './pkg/hello.js';
      init().catch(console.error);
    </script>
  </body>
</html>
```
Take note, that we are using `<script type=module>`.
Another thing to take note is that we referencing `./pkg/hello.js` from the `./pkg` folder.
If you changed the package name of the crate, you will also need to change the filename here.

Recompile our webapp, issue this command everytime you have changes to the rust code.
```shell
wasm-pack build --release --target=web
```

Finally, we serve the files using `basic-http-server`
```shell
basic-http-server
```
By default, it serves the page in port `4000`
Navigate to http://127.0.0.1:4000 to see the 'hello' message.
There you have it, you've built the bare minimum web application using sauron.

Well, the result is pretty underwhelming. We could just create a completely static html page with "hello" on it.
Head over to [`intermediate example`]("./intermediate-example.md") where sauron really shines.
