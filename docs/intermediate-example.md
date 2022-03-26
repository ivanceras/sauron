## A counter example

For our intermediate example, we will use the classic `counter` example.

We will start by creating a new rust library project called `counter`.
```sh
cargo new --lib counter
```
Modiy the `Cargo.toml` to be a library of crate-type "cdylib", since we are compiling it for a web application.
Then add `sauron` to our dependency.

```toml
[package]
name = "counter"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
sauron = "0.49"
```

In this example we will show 3 buttons where can click to increment/decrement and reset the count.
We will start by defining our model. Remove the contents of `src/lib.rs` put this code.

```rust
use sauron::prelude::*;

struct App {
    count: i32,
}

impl App {
    fn new() -> Self {
        App { count: 0 }
    }
}
```

We also added a function `new` to create our initial state of `App` which starts with count `0`.

Next, we define out set of actions that our application will have. We will implement this using an `enum` with name `Msg`.

```rust
enum Msg {
    Increment,
    Decrement,
    Reset,
}
```

Next, we implement the `Application` trait for our model `App`. This is a way to define how our app should be displayed on page with the `view` method.
Additionally, this also let us define how the `App` modifies its model with the `update` method.
We also specify `Msg` to the `Application` trait which tells what type of messages that will be passed from events in the `view`, which also corresponds to
the type we send into the `update` method.

Append this code to `src/lib.rs`.

```rust
impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <button class="count" on_click=|_|{Msg::Reset} >{text(self.count)}</button>
                <input type="button"
                    value="-"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        Cmd::none()
    }
}
```
The `view` method creates a html node tree.
Notice, that the return type is `Node<Msg>` which means it creates a html node where any of its member html elements has an event listener
that can emit a `Msg` messages to the program handler.
When a user clicks on the `+` button, this will emit an `Msg::Increment` message.

The `update` method accepts `Msg` as an argument and modify our model `App` depending on the variant of our `Msg`.
For now, we return `Cmd::none()` at the end of the update method. `Cmd` is a way to tell the program to execute something, we will cover this in a more advanced example.

Next, we define an entry point for our `wasm` web app.
This is done by anotating a public function with `#[wasm_bindgen(start)]`.

Append this code to `src/lib.rs`.

```rust
#[wasm_bindgen(start)]
pub fn start() {
    Program::mount_to_body(App::new());
}
```
Inside this function, we define how our app is to be mounted into our `index.html` page.
In this case we mount it to the `body` of our html page.
The [`Program`](https://docs.rs/sauron/latest/sauron/struct.Program.html) object provides other methods on how to mount your application into the document.

Next, we need to link our application in a html page.

Put this in `index.html` file at the project base folder.
You can put your styles as usual.

```html

<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <title>Counter</title>
    <style type="text/css">
        body { font-family: verdana, arial, monospace; }
        main {
            width:30px;
            height: 100px;
            margin:auto;
            text-align: center;
        }
        input, .count{
            font-size: 40px;
            padding: 30px;
            margin: 30px;
        }
    </style>
    <script type=module>
        import init from './pkg/counter.js';
        await init().catch(console.error);
    </script>
  </head>
  <body>
  </body>
</html>
```

Compile the web app by issuing the command:

```shell
wasm-pack build --release --target=web
```
Finally, serve the files using `basic-http-server`

```shell
basic-http-server
```

By default, the page is served in port `4000`
Navigate your browser to http://127.0.0.1:400 to see the app.
