## General

 - [X] Add a prelude to import all html tags, svg tags, html attributes, svg attributes that doesn't conflicts
 - [X] Make a module that isolate the `with-dom` features.
 - [X] Rework the dumb patch taking the advantage of feature gating the dom capability.
 - [ ] Add documentation to functions
 - [ ] Loosen the lifetime requirement of the `Fn` inside `Callback` from `'static` to a generic one eg: `'c`
 - [ ] Deprecate the tag macro since it complicates the conflict in reexporting the functions
     - ie: `style!` as a tag, `style!` macro for attributes, `style` as attribute call.
 - [ ] Change the README example to use the node macro syntax
     - rename the old `minimal` to `minimal-alt` and use the `node-macro-syntax` in `minimal` example
 - [X] Move `sauron-syntax` into `html2sauron` project


## Features
- [ ] Storage service (May not be needed since the user can directly use web-sys)
- [X] Fetch service
- [ ] Url change service
- [ ] re-think about the `sauron-core` features:
    - [ ] `with-dom` when used in client-side, default
    - [ ] `with-ssr` when used in server-side rendering, mutually exlusive to `with-dom`
    - [ ] `no_request_animation_frame` this should be additive
- [X] `with-markdown`
    - [X] Add sanitation to markdown parser, use `ammonia` crate
    - [X] expose the `sauron-md` as `sauron::markdown` module, behind a feature flag
- [ ] Add example using markdown

## Performance
- [ ] Fix the reported issues with benchmarks
- [ ] Create a new benchmark for the js-comprehensive-benchmark suite
    - [link](https://github.com/krausest/js-framework-benchmark)

