# Features

## Write style in rust, as though they were rust code.

You can feature gate individual property attribute in your style.
For example, if you want to outline the some elements with obvious color when debugging.

```rust
jss!{
    ".shape_buffer": {
        position: "absolute",
        top: 0,
        left: 0,
        #[cfg(feature = "with-debug")]
        border: "1px solid red",
    },

    ".shape_buffer .bounds": {
        position: "absolute",
        #[cfg(feature = "with-debug")]
        border: "1px solid blue",
    },
}
```
