## Migration guide from 0.60.0 to 0.61.0

- `MSG` is now an associated type to `Application`
- `Cmd<Self>` should now be just `Cmd<Msg>`

old:
```rust
impl Application<Msg> for App{
    
    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        ...
    }
}
```
new:
```rust
impl Application for App {
    type MSG = Msg;

    fn update(&mut self, msg: Msg) -> Cmd<Msg>{
        ...
    }
}
```

