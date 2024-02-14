## General

 - [X] Add a prelude to import all html tags, svg tags, html attributes, svg attributes that doesn't conflicts
 - [X] Make a module that isolate the `with-dom` features.
 - [X] Rework the dumb patch taking the advantage of feature gating the dom capability.
 - [ ] Add documentation to functions
     - Add examples to usage of methods in `Program`.
 - [ ] Loosen the lifetime requirement of the `Fn` inside `Callback` from `'static` to a generic one eg: `'c`
     - [X] Done in `mt-dom` branch: non-static-lifetime
 - [X] Deprecate the tag macro since it complicates the conflict in reexporting the functions
     - ie: `style!` as a tag, `style!` macro for attributes, `style` as attribute call.
 - [X] Change the README example to use the node macro syntax
     - rename the old `minimal` to `minimal-alt` and use the `node-macro-syntax` in `minimal` example
 - [X] Move `sauron-syntax` into `html2sauron` project
 - ~~[ ] Expose Cmd,Component outside of `with-dom` feature gate~~
     - This would allow a total isomorphic app reusing the components
     - ~~[ ] Make an equivalent for Program(client-side updater) for use in server-side~~
       ~~ - ie: ServerRender, where Msg could be passed as a data to hydrate the view (template) before sending to the client~~
     - We don't need to use the update function in server-side rendering. We set the state of the app by instatiating the app with appropriate data.
 - [X] Fix the render function where attributes of the same name not merged
 - [X] Change type of tag, attribute_name, style keys from `&'static str` to `&'a str`
     - This will remove the need for hardcode HTML_STYLES lookup, which could be a performance penalty
 - [ ] Add the RealWorld example
     - Use the elm base code https://github.com/rtfeldman/elm-spa-example
 ~~- [ ] **breaking** ~put back `style` as a normal attribute~, to avoid possible confusion to new users.
     - Cancelled, since style is treated differently in attributes.
     ~~
 - [X] **breaking** merge `Browser` to `Window`.
 - [ ] Add `and_then`, `sequence` to `Cmd` to perform a task after the preceding Cmd succeeds.
 - [ ] Create a document on why it is needed for events such as `on_click` to have a copy of the variables outside of its environment.
 - [X] Rethink on the naming of Component, SimpleComponent, SubComponent.
     - Component is actually Application since it the app that is manipulated by the program(executor).
     - Other names: ~~Root~~ Application, Component, ~~Control,~~ Widget
 - [X] Merge `init` and `init_with_program`
    - It make sense to make `init` mutable so the developer can manipulate the component at initialization stage.
    - [X] Make call to `init` right after the Application is mounted into the `DOM`, rather than before.
    - [X] Simplify `Application::init` to not have access to `Program` since it can return a `Cmd`. It can however, modify itself.
 - [X] Rename the type alias `Callback` into `EventCallback` or `Listener`.
      This way, we can use the more generic `Callback` in Components and in `Cmd`.
      - [X] Recreate Callback from a clean state, with no TypeId and used it in `Cmd`.
      - [X] Listener will have it's own dedidate struct with the TypeId.
      - [X] Use `Callback` in `Cmd`
 - [ ] Component system declared in view.
    - The current system needs to store all state of the Application and its member sub components, regardless if they are specific to the Aplication or not.
    - Some component will have properties that the App don't need to store.
    - To do this, we need to create higher level macro and function which includes Component to be a node variant.
        ```rust
        enum Msg{
            FuiButtonMsg(fui_button::Msg),
        }

        fn view(&self) -> Node<Widget>{
            <div class="wrapper">
                <FuiButton on_click=|_|Msg::BtnClicked style="full"/>
            </div>
        }
        ```

        leaf.rs
        ```rust
        pub enum Leaf<MSG>{
            Text(String),
            <...>
            Component(Box<dyn Component>),
            <...>
        }
        ```

        attribute_value.rs
        ```rust
        pub enum AttributeValue<MSG>{
            Simple(Value),
            CompProperties(<ComponentProperties>),
        }
        ```
    - The `Application` don't have to store the state of `FuiButton` component, it will be stored into the `Program` object.
        - Issue how will be map the Msg of the sub component to convert it into the Msg of the main `Application`?
    - [X] Merge the Container and Component which the view is now requires to have children components
    - [X] Add a CustomElement trait which facilitates the component to be a custom element
    - [X] Rethink of the sauron-component-macro
        - [X] Redo it, maybe we don't need it and then manually implement all the Components
        - ~~[ ] Make Application trait for internal usage only~~
- [ ] Make Http api pass a decoder function
- [ ] Additional to the dispatching of mount event.
    - [X] on_mount
         - on_will_mount
    - [ ] on_dismount
         - on_will_dismount
- [X] Make the mount event be wrap as a real event, this way we can dispatch it in the real dom instead of from the virtual node
    ```javascript
        let mount_event = new Event("mount");
        elm.dispatchEvent(mount_event);
    ```
- [X] Call set_attribute in addition to setting the special attributes such as `value`, `checked`, this should trigger the `attribute_changed` callback in web components
- [ ] The attribute_changed method in CustomElement should return an `MSG` which will be dispatched in the WebComponent struct.
- [X] There is conflict with the use of `style`
    - `style!` macro
    - `style` attribute function
    - `style` method in Application, Component, Container
    - `style` html tag
    Maybe use `css` or `stylesheet` as the method name.
    ```rust
    fn css(&self) -> Vec<String>{
    }
    fn stylesheet(&self) -> Vec<String>{
    }
    ```
- [ ] Make the compilation error in `jss!`, `style!`, more informative


## Internal
- ~~[ ] Find a way to map `Cmd<APP,MSG>` to `Cmd<APP2, MSG2>`~~
        ~~ie: `Cmd<ChildApp, ChildMsg>` to `Cmd<App, Msg>`
        This is needed since `Cmd` from `update` function of sub components
        are not dispatched in the program. Only the top level
        component `Cmd` can be dispatched~~
    - ~~[ ] Find a way to map `Program<APP,MSG>` to `Program<APP2,MSG2>`~~
        - ~~[X] map `DomUpdater<MSG>` to `DomUpdater<MSG2>`~~
        -  ~~Issue mapping fields of Program that are in `Rc<RefCell>` seems not that simple~~
           ~~ as the Rc value of dom_updater is to be borrowed and will have a borrow checker issue~~
- [X] Merge `Program` and `DomUpdater`
    - Issue: DomUpdater has multiple fields, which would then be wrap with `Rc<RefCell>` individually
- ~~[ ] Change the `'static` of trait implementation by specifying the lifetime
        - ref: https://stackoverflow.com/questions/52187644/lifetime-must-be-valid-for-the-static-lifetime-so-that-the-types-are-compatible~~
- [X] Get rid of test_fixtures and move it to test directory
- [ ] Make each component have a reference to the root dom where it is mounted.
    - This will make local state changes to the component easier to do, as opposed to diffing the whole DOM tree.
- [X] Unify the code of Program replace_mount, append_mount
- ~~[ ] replace the request_animation_frame with the code from `execute_request_animation` frame~~
- [ ] Create a function to derive Component name from the struct name of the Component
    and preprocess the jss with it before injecting it to the main program
- [X] Clean up `CreateNode`
    - no need to wrap `Node` and `Element` instead just return them as created with their `closures`
- [X] Cmd should include a `should_update: bool` field which indicates if the update should be made or not
        - Cmd{ commands:Vec<..>,should_update }
        - Cmd::noop() // no update operation
        - Fixed in `0.36.0`
- [X] Remove the Dispatch trait and pass Program as it is in `dom_updater` and `apply_patches` module
    - There is only one implementation of `Dispatch` trait anyway, that is `Program`
    - Dispatch serve its purpose to make the code less clutter, by passing arguments around with less generics.
- [X] ISSUE: sauron `node!` macro doesn't work on svg tags since it is using only `html_element` function which `namespace` is not supplied.
    - Fixed in `0.35.0` by checking whether a tag has a namespace.
- [X] Change `program: Option<&DSP>` to just `program: &DSP` since there program is needed everywhere.
- [X] Improve `sauron-node-macro` to call on the equivalent html function instead of `elment_ns`.
    - This would improve performance since the function already has the information whether or not it has namespace or not.
    - Mitigated with the use of `Lazy` `HashSet` look up in `sauron-parse` for faster lookup.
    - Further improved using `sauron-parse` by resolving the value of `self-closing` and `namespace` at compile time in the `node` macro.
- [X] Create a hashed collections in `sauron-parse` to optimize lookup of tags for namespace or self-closing
    - Created a fast lookup for `is_self_closing(tag)` amd `tag_namespace`.
- [ ] old elements that has an event attached has no way of knowing the equivalent new element has the same event attached as their callbacks are clone of closures inside of Rc
    and no 2 closures are the same even if the have the same code.
- ~~[ ] add a conditional function for event attribute that if any of the other attribute is changed the event will have to be remove and re-attach.
    - This is to mitigate the aggressive recycling of nodes which we skipp diffing for event listeners for performance reasons, as it is impractical to
        reattach event listener at every render cycle.~~
    - This has been solved by using the `TypeId` of the closure of the callback.
- [X] Remove NodeIdx traversal and also remove NodeIdx in `mt-dom` TreePath, as traversal path prove to be correct.
- [X] Maybe Remove the style functionality as Components and Applications can manipulate the style in the document directly
    - [X] Change style that it returns only a `String` instead of `Vec<String>`.
    - [X] The injected style shall have a class name equal to the the type_id of the `APP`.
- [X] Add `maybe_attr(name: &str, value: Option<Value>)` to set the attribute if there is a value. empty otherwise.
- [X] Centralize the handling of attributes tha has a state such as `value`, `checked`,.
- [X] Issue with not finding the nodes to be patched
    - This issue manifested in `performance-test-sauron` repo
    - Suspecting it has to do with `mount_node` and `root_node` as replace and append could have a different behavior in the 2.
    - Solved by: using mutable reference to the `root_node` rather than a mutable reference to a clond one.
- [X] Rethink about the `replace_mount` in Program
    - It is useful for replacing the preload spinner when the application is finished loading
    - [X] Have an enum for mount action
        ```rust
            enum MountAction{
                /// append as child to the target mount
                Append,
                /// clear any child to the target mount then append
                ClearAppend,
                /// replace the target mount with the root node
                Replace
            }
        ```
    - [X] Mount event should have a reference to the `host_node` and the `root_node`
            - host_node is the node where the view is mounted, usually the parent
            - in case of replace host_node is the same as the root_node.
- [X] Maybe we don't need the `async` in update.
- ~[ ] Refactor Program that will have to use less of `Rc<RefCell<>>`, by having an inner structure which is wrapped into `Rc<RefCell<>>`~
    - this is not possible because we need to update each field separately, and borrowing the inner program state will disallow borrowing other fields.
- [X] BUG: if the `dispatch_inner` is not called in a callback which is `request_animation_frame` or `request_idle_callback`
    - This will cause the `dispatch_mount` event to dispatch before the `root_node` is set in the program when the program is to be mounted
    - Note the `dispatch_mount` is triggered when the view has `on_mount` event.
    - [X] mitigation: make the `dispatch_inner` spawn in a thead either via callback, or `spawn_local` from `wasm_bindgen_futures`.
- [X] Tighten visibility of objects that are not meant to be `pub`
    - [X] some fields in `Program`
    - [X] struct types that are not meant to be public
- [X] Use the deadline object in `request_idle_callback_with_deadline`, instead of just `f64`, which calculates the remaining time manually
- [X] Migrate to rstml, since syn-rsx is unmaintained.
- [X] Remove `Dispatch` and just pass `Program` around
- [X] Make an alternative to `Effects` and `Cmd` that can be used in `Component`.
    - call it `Task` a wrapper to a future, will resolve into MSG which will then be dispatched into the program
    - does not have access to program for dispatching
- [X] Remove the use of `Closure::forget()`
- [X] Refactor `ActiveClosure` to use
      - add a field `dom_closures` in `Program` which stores all closure for a certain Element
      - all other closures is stored in `active_closure`
      ```rust
      closure_id_counter: usize,
      type ActiveClosure: BTreeMap<usize, Closure>;
      ```
- [X] unify the `Program::add_event_listener` which attach the event to window
    and the `dom_node::add_event_listener_callback` usage used in `set_element_attributes`

    ```rust
        Program::add_event_listener(&self, target_element: EventTarget, event_listeners).
    ```
- [X] Make the svg attributes follow `snake_case` convention
    - `viewBox` -> `view_box`
    - `preserveAspectRatio` -> `preserve_aspect_ratio`
- [X] As an alternative to Task where `Component` can not use `Cmd`, due to it referencing Program,
    we can instead return listeners.
    - window listeners
    - document listener
    ```rust
    struct GlobalListener{
        window_listeners: Vec<Attribute<MSG>>;
        document_listeners: Vec<Attribute<MSG>>;
    }
    ```
    add these events:
    - `on_interval(|i32|{})` for attaching interval in the Window
    Http can be done with task
- [ ] Make `Sub` as counterpart to `Cmd`
    - We can use `Sub` in the `Component`
    ```rust
     fn on_resize(&self) -> Sub<Msg>{
     }
    ```
- [ ] Bring back CreatedNode maybe with a different name: `DomNode` which wraps the `Node` or `Element` along with it's closures from event listener
    - These are then saved into the `Program` where when removed or dropped, it will also drop the associated closures with them, thereby simplifying the code.
    - Right now, we are attaching a `vdom-data` attribute for nodes that have listeners
- [ ] Make use of `Arc<RwLock>` to check if can solve copying the `APP` via `transmute_copy`.
    - See if there are performance penalty


## Features
- [X] Storage service (May not be needed since the user can directly use web-sys)
    - [X] using wasm-bindgen directly will remove the need for Storage service wrapper
- [X] Fetch service
- [X] Url change service
    - using wasm-bindgen directly eliminates the need for Url change service wrapper
- [X] re-think about the `sauron-core` features:
    - [X] `with-dom` when used in client-side, default
    - ~~[]`with-ssr` when used in server-side rendering, mutually exlusive to `with-dom`~~
        - Server-side rendering is implicit when target is not wasm.
    - [X] `no_request_animation_frame` this should be additive
        -  crate is now using `with-request-animation` feature
- [X] `with-markdown`
    - [X] Add sanitation to markdown parser, use `ammonia` crate
    - [X] expose the `sauron-md` as `sauron::markdown` module, behind a feature flag
- [X] Add example using markdown
- [X] Make use of `serde_json` to parse `style` into components
- [X] Add an example where a program is a custom html element, that way sauron could be used as a way to migrate parts of an existing html/js code base.
    - [X] Custom element which is defiend as a web component where it can be used by some other Application.
    - [ ] The App should be serializable and each of the fields will become an html attribute which
    - [X] There is an issue with the patch not being able to find the element to be patch when using custom element
        due to the reason that the root_node stored in the dom updater is not the first element of the view, but rather
        the root_node in the dom updater is the first element of the view.
        The old technique was the replace the root node with the created first element but this is not ideal when used for custom_element since we need to get the attributes from the custom element
        Possible solution:
            - Add a mount_node to the dom_updater alongside with the root_node
    - [X] custom element would be appended to the `shadowRoot`
        - [X] Usage of custom element inside another sauron component should skip the custom-element internal DOM elements
    - [ ] custom element should also need access to the `textContent` of the tag for further processing
- [X] Properly trigger the MountEvent at the appending of the component to the DOM
    - Right now, it is triggered when the virtual Node is created into a real Node.
- [X] Maybe rename `#[web_component]` macro to `#[custom_element]`
    - Also `WebComponent` to `CustomElementWrapper`


## Performance
- [X] Fix the reported issues with benchmarks
    - fixed by setting the target to web when building the wasm
- [X] Create a new benchmark for the js-comprehensive-benchmark suite
    - [link](https://github.com/krausest/js-framework-benchmark)
    - Initial attempt https://github.com/ivanceras/performance-test-sauron
- [X] Use Weak pointer for program instead of Rc where strong reference is not needed.
       - Program stays as long as the user is using the app.
- [X] Add `Program::batch_dispatch(&self, msgs: Vec<MSG>)` to call update on each of the messages before
    calling on the view, this would improve performance when there are multiple messages to be dispatched to the application
    - [X] implemented with `dispatch_multiple`
- [X] Make a benchmark for building views with more than 2000 nodes, like a text-editor.
    - There is a huge performance regression in between 0.40 and 0.42
    - [X] It was cause by jss `style!` macro where the lookup for style name is recreated everytime,
          due to the use of `const` instead of `static` in a `once_cell::Lazy` declaration. This is fixed in `jss 0.3.3`
- [ ] Create the dom nodes in depth-first-traversal
- [X] Make a pending patches in the DomUpdater which stops applying remaining patches when the time remaining for the deadline is up.
    - There is a problem with storing the patch in a struct which will need to have explicit lifetime
    - This can not also be stored in a global variable since there is a generic MSG
    - Solution:
        - Make an owned Patch which has no reference to the node
          ```rust
            struct OwnedPatch{
                tag: Option<TAG>,
                node: Node<'a,...>,
            }
          ```
        - Store the patches as Closures, so as to get away with the generics
            - but then there will be error can be shared between threads because closure is not Send
        - [X] Make a DomPatch which a DOM version of the patch with the target node and created node
- [ ] Find a way to break up building the view into multiple frames, since view can take a long time to build
- [ ] Find a way to break up diffing the current vdom and the new vdom as they can also take a bit of long time as well.
- [ ] Add benchmark function for using CACHE_ELEMENT and not

## Maintenance
- [X] Move `sauron-markdown` into it's own repo, for keeping sauron slim.
- [X] Move `jss` into a new crate `sauron-jss` for keeping sauron slim.
    - [x] Use [json](https://github.com/maciejhirsz/json-rust) crate for `jss`.
        - The quote on keys are optional, so this is good for use in writing css.
- [X] Enumerate the exported modules and structs in prelude instead of just using glob(ie: *).
- [X] Fix the data-viewer example to use Components on the views rather than Application
- [X] Revisit and use style_name identifier in usage of jss in examples
- [X] Move `html::units` to `jss` crate
- [X] Rename `DomUpdater` to `DomPatcher`.
    - [X] move apply_patches into `DomPatcher`.
- [X] Rename `CreatedNode` to `DomNode`.
    - [X] Maybe completely remove CreatedNode
- [X] Move fields from `DomUpdater` into `Program` such as
     - [X] current_vdom
     - [X] root_node,
     - [X] active_closures,
     - [X] pending_patches
- [X] Remove the use of `wee_alloc` crate
- [X] sauron-core and jss should have a different version of Value, where jss value can be converted into.
    - `Value` struct needs to reside here, since it is a corner-stone data structure and used eveywhere.
    - Maybe `Value` should be in a very common crate. Say `sauron-common`.
- [ ] Use `xtask` for scripting: building, checking, publishing, running the examples
- [ ] Use `{ workspace = true }` to common dependencies for easieir maintenance work.

## Bug
- [X] When 2 nodes with multiple similar keys, multiple replace node patch is generated. But it couldn't seem to find the correct target element.
     or the target element has no parent, therefore can not replace/insert the node.
     - This is solved by getting the type_id of the closure.
- [X] Add more test for recycled nodes with keys
- [X] When 2 text are next to each other, the second text will become a comment
- [X] Runtime errors when using fragments
- [ ] usage of `classes_flag` seems to be broken with complext trait requirement.
    - This should work very simply `classes_flag([("todo", true), ("editor", is_editing)])`

## Limitations
- In rust, no two closures, even if identical, have the same type. Therefore closure can not be check for equality.
    - In sauron node are matched and reused aggressively, except when the keys are different then the node is discarded.
    - If we don't reuse nodes with event listeners aggressively, then we would have a performance penalty, since every recreation of the node with event listener will have to discard and create a new one for it, even if it is matching itself.
    - Adding `key` attribute provides a good trade off.
