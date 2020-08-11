## General

 - [X] Add a prelude to import all html tags, svg tags, html attributes, svg attributes that doesn't conflicts
 - [X] Make a module that isolate the `with-dom` features.
 - [X] Rework the dumb patch taking the advantage of feature gating the dom capability.
 - [ ] Add documentation to functions
 - [ ] Loosen the lifetime requirement of the `Fn` inside `Callback` from `'static` to a generic one eg: `'c`
 - [ ] Deprecate the tag macro since it complicates the conflict in reexporting the functions
     - ie: `style!` as a tag, `style!` macro for attributes, `style` as attribute call.
 - [ ] Change the README example to use the node macro syntax

## Features
- [ ] Storage service (May not be needed since the user can directly use web-sys)
- [X] Fetch service
- [ ] Url change service
