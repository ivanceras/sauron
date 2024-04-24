# Changelog

## 0.61.7
- remove support for `custom_element` as its functionality is superseeded with `stateful_component`
- remove parent in DomNode as to simplify code

## 0.61.6
- remove template and use-template feature gate as it is not adding performance enhancements

## 0.61.5
- rename `safe_html` to `raw_html`
    - decode html entities before parsing and converting into node
- optimize dispatching of mount event only to element that has attached event listener to on_mount
- make `stateful_component` patches work on its own dom tree

## 0.61.4
- make `use-template` and `use-skipdiff` fail safe if the template and skipdiff is not used in the view
- feat: feature gate `with-trace` for determining how much time is spent in section of making stateful component

## 0.61.3 
- more clippy lint fixes
- improvements on the examples

## 0.61.2
- use `with-lookup` feature by default

## 0.61.0
- add template system, skip_diff
- web_sys::Node is now wrapped with DomNode, the event listeners
  is now managed from its containing DomNode, instead of from the Program.
  when a DomNode is removed and goes out of scope, so does its associated event closures are dropped
- `Cmd<Msg>` can now be mapped into `Effects<Msg,..>` and vice versa
- The `mt-dom` crate is now part of `core` in the `vdom` module. This change is necessary
   to improve code coherence and simplicity, lesser number of generic types has to be passed arround.
- remove `inner_html` as a function to set html via html attributes.
    - This cause breakage in tracking the DOMTree as new nodes could be inserted dynamically without the runtime knowing it.
- Rename `safe_html` to `symbol` as its usage is intended for html entities such as `&nbsp;` `&quote;` etc.
- A `safe_html` function is added and use `html-parser` to parse dynamic html and convert it into a safe dom-tree.


## 0.60.7
- feat: add selectionchange event and document_event_listener

## 0.60.6
- feat: Add a new crate html-parser for dynamically parsing html content

## 0.60.5
-  feat: add rgba css function

## 0.60.4
- fix: check only for non-matching property when the property name is not quoted.
  - quoted property name in `style!` macro will be an escape hatch for few cases such as the use of prefixes, ie: "-webkit, -ms, -moz"
- add `Program::clear_mount` as alias to `clear_append_to_mount`, add `clear_mount_to_body`

## 0.60.3
- return `Option` for style property lookup instead of panicking.
- Make `style!` provides a helpful error when property names used is not found in the lookup.

## 0.60.2
- improve the Input event
    - InputEvent now call on `value()` to get the value
- update to mt-dom 0.59.1 to force crate user to use this version, since it has a fix of critical bug
- feat: allow annotation to style properties
    - add test for annotated styles

## 0.60.1
- feat: add lazy_view_if function to lazily evaluate the node function only when the flag is true
- fix: web_component, use Program clone for attribute_change
- fix: Program has to be wrap with ManuallyDrop so as not to drop it at the start function
- feat: **breaking** all references to Program is now Weak and will go out of scope, and so an AppLoader is needed
    - Fixed with using `ManuallyDrop`.
- fix: test, use update_dom_with_vdom instead of just setting the current vdom using set_current_dom function
- fix: remove set_current_dom function which has a different functionality with app_context.set_current_dom
- use mutable methods in app_context
- docs: add a Note on why the function update_dom_with_vdom is not unified into update_dom
- refactor: use mutable function when applicable
- fix: progressive rendering example
- refactor: make dispatching messages and updating dom mutable
- feat: include strong_count and weak_count to the measurements
- add a test-case for issue#75

## 0.60.0
- fix: remove the auto-implementation fo `Container+WebComponent` as it conflicts with other usage
- feat: add utility method to the component to create a consistent namespace class names and selector for building view and css nicely
- remove the style attribute function, since it conflicts with the name of the style! macro which is more useful than the removed one
- fix: comparison of Value for string variants to compare stringly types
- feat: make use of `classes_flag` in `todomvc` for more pleasant syntax
- feat: **breaking** migrated `jss` from independent `jss` crate back to sauron
- feat: improvement on the ergonomics of writing web_components
- feat: improvement on registration of web component, as this eliminates the need to attaching component to the Window object

## 0.59.2
- refactor: remove the `use-snippets` feature, since we are  using `js_sys::Function`

## 0.59.1
- feat: add `as_str` method for returning the str value of an attribute

## 0.59.0
- refactor: **breaking** change the methods which extracts inner values to not panic but instead return `Option`
    - `unwrap_text` -> `as_text`
    - `unwrap_safe_html` -> `as_safe_html`
- refactor: remove the use of unnecessary map_callback, since it can be done using map_msg since Fn pointer can just be clone
- refactor: use the clear_children function
- patching a document fragment other than in root node should not happen
- refactor: convert the shell scripts to justfile
- refactor: use raf for doing patches, use ric for anything else
- feat: add a request_idle_callback to make ric work in safari
- feat: expose a reference to the underlying field in Program's app_context
- fix: move `data` attribute out of the commons, since it is commonly used in local variables
- fix: `web_component` init method
- feat: add a conversion from Task to Effects
- fix: to the new version of sauron, improve code for interactive example
- feat: **breaking** `Effects` now wraps `Task` which could now accept ready MSG or an Future MSG
    - **breaking** change Component signature which returns Effects on the init function
    - **breaking** change Application signature which returns Cmd on the init function, to match that of  the Component
- fix clippy, no need to collect iterators
- fix: add `'static` constraint to `XMSG` in components
- refactor: such that borrows end at line, which open for a more flexible data structure for the `AppContext`
- refactor: rename `dispatch_multiple` to `push_msgs` in `AppContext`
- refactor: use update_app as function name for updating the app
- refactor: simplify the batching of `pending_cmds` code
- inject the css before mounting the App to the dom
- feat: create `AppContext` module which moves app, current_vdom,  pending_msgs, pending_cmds, which pertains to the state to the Application's model

## 0.58.0
- use the latest published version of mt-dom
- refactor: move `fragment` into `mt-dom,` so it can be diff there, therefore will still be performant when in used in Application view
- fix: **breaking** expose `delay` function, in replacement to `async_delay`
- fix: issue with `fragment` node and `node_list` when used in the top-level view of the app
- refactor: **breaking** swap names for `custom_element` and `web_component` since it is more concise to swap the two

## 0.57.5
- update to `mt-dom` `0.57.0`
- feat: add method as_text to NodeTrait for extracting optional str from a leaf text node
- remove too many logging in `define_custom_element`
- `custom_element` should include the `web_component` macro
- don't lock third-party dependencies to latest version, otherwise, it will disallow other dependent crate to use older version of the mentioned third-party dep causing failure to select version of dependencies
- chore: remove unnecessary logging
- fix: `custom_element` Modifier arg
- feat: improvement on `Modifier` and `Measurement` to contain the name as tag
- refactor: improvement on the data structure of `ActiveClosure`
- improve code for attaching `node_closures`

## 0.57.4

- make `init` and `stylsheet` an optional method for `Application` and `Component` trait.
- update mt-dom version to 0.56.1

## 0.57.3
- update to mt-dom `0.56.0`, which contains bug fixes when swapping rows with keyed elements
- new PatchVariant in `mt-dom` which has `MoveBeforeNode` and `MoveAfterNode` for moving existing keyed elements

## 0.57.2
- allow users to surround braces on closure in html events. ie: `on_click={|_|..}`

## 0.57.1
- update dependencies

## 0.57.0
 - docs: add warnings on the usage of Fragment
 - docs: put quote on words that has underscore on it
 - refactor: migrate to `rstml` since `syn-rsx` is not maintained anymore
    - also migrate to `syn` `2.0`.
 - feat: **breaking** change the data type for the old_value and new_value in the attribute_changed function, since everything is a string in the attribute value
 - feat: include the `custom_tag` in the `web_component` macro
 - refactor: restructure the declaration of html and svg tag such that the module is wrapping the declaration instead of inside the macro code

## 0.56.0
- fix: changes in `mt-dom` 0.55.0 where the method names are shorter and concise
- feat: **breaking** add a new static method for Application, and Components: `stylesheet` which returns a static stylesheet, make `style` optional
- feat: linking of `define_custom_element` behind `use-snippets` module
- feat: add an alternative way of making registration of custom element work in sauron using wasm_bindgen and it's limitations
- feat: put the `custom_element` functionality behind `custom_element` feature gate
- feat: put `regsiter_custom_element` behind a feature gate
- refactor: simplify the resize task to not use Rc and RefCell
- restructure exports and prelude
- fix: refine and sanitize the export of modules in sauron
- refactor: improve and unify `add_event_listener`
- fix: remove use of `HtmlMenuItemElement` since it is already deprecated
- call `update_dom` after calling on the program `connected_callback`
- refactor: improve the function signature for `attribute_changed`
- refactor: **breaking** use snake_case for svg tags and attributes to fit rust conventions
- fix: use a different generic type to the external in `Effect::localize` function, this way, the user don't have to unzip the Msg
- fix: remove the use of wee_alloc
- refactor: move some `window` functions into `util` module
- refactor: remove the use `Closure::forget` in adding event listeners, and store the closures into the Program as `node_closures`
- refactor: put the creating of closure in place rather than calling create_closure_wrap
- use `Closure::new` instead of `Closure::wrap` since they have the same effect, only thinner and less complex
- refactor: modularize rif,raf,and timeout into their corresponding modules, now returns the Handle which contains the closure
- refactor: rename active_closures to node_closures since it is only storing the closures used in node elements
- rename `created_node` to `dom_node` module
- refactor: remove unnecessary wrapper `CreatedNode`
- move `apply_dom_patch` to `dom_patch` module, but still under `Program` implementation
- append the `active_closures` directly to the `Program` instead of storing it in `CreatedNode`
- join styles with blank instead of new_line since the new version of jss has added the necessary lines
- style now returns `Vec<String>` to make it ergonomic when dealing with Component that has multiple styles
- feature: add a conversion of `Window::on_resize` to , a variant which returns a `Task<MSG>`, instead of `Cmd`
- **breaking** change the return type of init in `Application` to `Vec<Cmd>`
- feature: add utility function to inject style to document head
- feature: add `Task`

## 0.55.1
- fix: call `dispatch_mount_event` to other patching variants, other than just the `Append` variant

## 0.55.0
- **breaking** rename crate `sauron-node-macro` into `sauron-macro`, since it contains some other macro other than `node!` ie: `#[web_component]`
- organized the with-dom feature
- remove `Dispatch` trait, and just pass the `Program` around, since `Program` is the only implementation of `Dispatch` trait.
- use a more concise method names in `Http` module
- refactor: put allow(unused) directive to some feature gated function
- remove focused_node, to slim down and simplify the framework code
- remove on_enter event since it can easily be done in the user application
- feature: add a new attribute macro: #[web_component] to easily derive a WebComponent
- add getting InputEvent from HtmlElement which is used in CustomElement
- add `Widget` trait, which is a simpler version of `Component`.
- move `CustomElement` to `custom_element` module
- add `map_external` method to Effects
- remove unused trait method `attributes_for_mount` in `CustomElement` trait

## 0.54.1
- bump up version such that `sauron-node-macro` is using `sauron-core` v0.54.1 as well.

## 0.54.0
- fix: make clippy happy, use bump up version of mt-dom
- fix to suit changes in mt-dom where replacement uses multiple nodes, fix tests
- add composed to InputEvent, modify the custom_element example to demonstrate input event
- call on `set_attribute`, in order to trigger attribute_change value for special attributes such as `value`, `checked`, `open`, `disabled`  etc
- fix: put debug logging behind feature patches
- add improvement to MountEvent dispatching it as a native web event
- refine the appending of child nodes and combine them into a function that calls the dispatch_mount event together upon appending
- simplify the `register_custom_element` function, to just accept the 2 args instead of 3.
    - add custom_tag function to CustomElement trait
    - This includes the custom-tag and the CustomElement wrapper, and removes the never changing HTML as base class
    - add improvements on custom_elements, auto-registering when used in function call to form a widget, attributes should also be a function inside the module
    - move the register function as a static member of the WidgetCustomElement
- add more util functions for asynchronous execution
- use edition 2021 for the example projects

## 0.53.0
- **breaking**: fix glob imports conflict, remove prelude.
    - the html tags and attributes should be explicity imported as well
    - this is necessary since rust version 1.70 don't allow importing names that conflicts other names in the same glob, in the case of html and svg attributes, there are conflicting names such as: title, slot, data, style
        old
        ```rust
        use sauron::prelude::*,
        ```
        new
        ```rust
        use sauron::{*, html::*, html::attributes::*, html::events::*}
        ```
        this is neccessary, since a lot of tags and attributes could be comming from html or svg, if you are working with svg elements
        ```rust
        use sauron::{*, svg::*, svg::attributes::*, html::events::*}
        ```
        if you are using both html and svg in the same module, you can glob imports the html, and explcitly specify the svg attribtues.
        ```rust
        use sauron::{*, html::*, html::attributes::*, html::events::*, svg::attributes::{xmlns, cx, cy, r, points, d}}
        ```
- fix: put the logging of dispatch time in measure and debug feature flag
- feat: enable scroll related objects from web_sys
- feat: convert on_focus and on_blur to use FocusEvent
- refactor: make the request_animation_callback, request_idle_callback and async_delay more readable by unifying the code that was scattered into small little functions
- feat: make the cache_elements readonly, not using refcell as there is no need to add the other elements
- feat: add caching of created element for faster creation using clone_node
- feat: add with-interning feature to make transfering faster from rust to js
- cargo fmt using stable features with default values

## 0.52.0
- refactor: make visibility private on fields of program that is not needed to be accessible outside of the module
- refactor: use `target_element` instead of `target_node` in `DomPatch`
- refactor: make `DomPatch` use less repeated fields in all variants by making it use struct with `PatchVariant` as the enum
- feat: Program api such as `replace_mount` and `append_to_mount` is now clearly defined with explcity use of enum for `MountAction` and `MounTarget`, rather than just boolean for `is_shadow` or `is_replace`.
- fix: improve the lifecycle logic of custom elements
- refactor: organize code in updating of dom
- feat: expose `with-raf` as feature in sauron, which the dispatch function cal use `requestAnimationFrame` callback
- feat: add support for `touch_event` in events module
- feat: export `wasm_bindgen_futures` from sauron
- fix: add optional deadline to applying of msgs and patches functions
- fix: add code for ensuring the state of the Program is correct
- refactor: make `dispatch_dom_changes` a sync function since it does not need to be async, add feature gates to function calls
- refactor: make `pending_cmd` as state in the program, which can be handled later on
- refactor: create a `request_idle_callback_with_deadline` in dom util, rename some functions, add early return when there is no pending patches, pending msgs
- refactor: use `BTreeMap` and `BTreeSet` as alternative to `HashMap` and `HashSet` which seems to work faster than the later
- fix: remove the Already borrowed error when setting the root_node, refactor the DomPatch apply method into the Program
- refactor: make conversion of attribute value from various types idiomatic
- feat: expose the `with-ric` (requestIdleCallback) feature in sauron
- refactor: merge DomUpdater into Program to simplify and unify all the central logic of the APP lifecycle in one place
- refactor: remove apply_patches and distribute the functions into CreatedNode and DomUpdater
- fix: polish the open attribute and the toggle event used in `<details>` element
- feat: improvement to the processing of `pending_patches`, remove the throttling feature
- feat: add DomPatch as an intermediate Patch with no lifetime reference for processing the patches in a controlled way to minimize sluggishness of the app
- refactor: mutate the active_closures in place rather than re-assigning it.
- fix: remove commented out examples
- feat: split the `dispatch_inner` method into 2 stages methods: the app updates and the dom updates
- feat: add `skip_criteria`, an improvement on the skip attribute
- refactor: make `batch_msg` method accepts IntoIterator rathern than vec

## 0.51.0
 - breaking: remove the use of sauron-component-macro
 - fix: use mt-dom 0.21.0
 - feat: add a function to check whether a tag is a custom-element, add append_child to Container trait
 - feat: improve the implementation for macro for implementing components
 - fix: remove #[wasm_bindgen] anotation for register, since it will conflict with other functions for wasm
 - feat: expose jss_ns_pretty
 - feat: add node_list
 - feature: add Cmd::async function to create cmd from Future types
 - refactor: more simplification to Http module by using async/await with JsFutures from wasm-bindgen-futures
 - refactor: use wasm-bindgen-future, async and await instead of Closures

## 0.50.6
- feat: add utility function `async_delay` for putting delay inside of async functions

## 0.50.5
- feat: remove the feature `with-request-animation-frame` as it is now always used by default

## 0.50.4
- fix: the passed root_node must point to the original root_node, as there is a possibility that the top level node will be replaced with a new top-level view node
- refactor: reuse the call to other function in creating request animation call

## 0.50.3
- Add `key!` macro which combines `key` and `format!` macro to easily format key attribute value.

## 0.50.2
- Add `Progam::dispatch_with_delay` to dispatch an MSG at a later time.

## 0.50.1
- bump `mt-dom` to 0.20.0

## 0.50.0
- **BREAKING** make the `Cmd::append` accept `&mut self` and modify it rather then return a Self
- Add support for defining custom elements a.k.a web components
    - [DateTime widget example](https://github.com/ivanceras/sauron/tree/master/examples/custom-element)
- Add support for creating fragment in a node
The `node!` macro can now be used to return a view which uses multiple top level tags.
Example:
```rust
node!{
    <h1>"Header"</h1>
    <main>"The content goes here"</main>
}
```

## 0.49.3
- support setting the attribute value for;
    - HtmlLiElement
    - HtmlMeterElement
    - HtmlMenuItemElement
    - HtmlProgressElement
    - HtmlParamElement
    - HtmlOutputElement
    - HtmlDataElement

## 0.49.2
- fix: a bug, closures for the elements created in InsertBeforeNode and InsertAfterNode variant for Patch is not added to the active closures

## 0.49.1
- Fix the ordering of inserting nodes when applying patch InsertAfterNode.

## 0.49.0
- fix assertion in apply_patch for InsertNode variants

## 0.48.0
- Remove the branch for patching the variant of Patch ReplaceLeaf since it is superseeded by ReplaceNode variant.
- Fix all the test including the wasm test this time

## 0.47.0
- Use TreePath for finding and traversing nodes.

## 0.46.1
- bump minor last release was using sauron-core 0.45 for sauron-node-macro

## 0.46.0
- change in mt-dom TreePath where root node is now at [], while first element in the dom is at [0].

## 0.45.2
- change in Patch variants InsertNode -> InsertBeforeNode, added InsertAfterNode.

## 0.45.1
- limit the export for `units` so as not to conflict with html tags and attributes

## 0.45.0
- The Leaf node of vdom formerly integerated in mt-dom::Node is now defined in the crate.
- `Value` and `units` is now from `jss` crate.
    - `Value` enum don't have bytes anymore since it is not useful in dom context

## 0.44.0
- Refactor on apply_patches due to changes in mt-dom (collapsing the Patch struct)

## 0.43.11
-  Correctly set value on HtmlSelectElement and HtmlOptionElement

## 0.43.10
- Add `unzip` and `extend` method to `Effects`

## 0.43.9
- Add support for `<select>` element in converting to event into `InputEvent`.
- Add example using `custom-elements`.

## 0.43.8
 - Add support for `disabled` attribute to be explicitly called on their corresponding DOM html element functions to explicitly set the boolean value.
     - This allows the users to use `disabled=true` when using the `node!` macro with the intuitive expected result as opposed to the default value in html which only checks
        for the presence of the attribute.
 - Improve the `Render::render_to_string` function to skip attributes: `open`, `checked`, `disabled` when the value evaluates to false as the mere presence of this attribute alone will
    make the attribute the same as evaluated to true. We mitigate this counter-intuitive behavior of the browser such that we use the 2 states of the boolean value, where `true` will
    make the attribute be in effect, while `false` skip the attribute and/or render the element attribute in the opposite state.
 - Improve `Cmd` accumulation in dispatch inner using functional code

## 0.43.7
- Add support for details/disclosure element and its open attribute
- Add toggle event.

## 0.43.6
 - Add `Dispatch::dispatch_multiple` to dispatch multiple MSG at once, which add performance benefit of not having the compute the view in between updates,
    but only when all of the MSG has been dispatched

## 0.43.5
- Change `Cmd` such that it uses `FnOnce`, which doesn't require `MSG` to be a `Clone`.

## 0.43.4
- Remove `log::trace` on `with-measure` feature flag, it is not needed anymore since the details is already passed in the Measurements object

## 0.43.3
- Call only measurements only when `with-measure` feature flag is enabled.
- Use of `IntoIterator` for attributes and children of the html nodes, which allow the usage of either array or vec for a more cleaner view
    - Improve readability on examples to array of attributes and children nodes for better readability
- Move effects to be under `dom` module instead of under `component` module
- increase the sleep timer to give time for crates to reflect the dep


## 0.43.2
- Use comment for the `view_if` instead of a blank span
- Improve code for handling TextNodes, hacks to create comment as separator is now removed
- Use thread_local for caching Window, Document and NodeId counter
- Add modifier for Effects
- Make the `on_scroll` work for window as well
- Rename Generics PMSG (Parent Msg) to XMSG (External Msg)

## 0.43.1
- Fix handling of units with multiple value

## 0.43.0
- Add `Effects::append_local` to easily chain local Msg into existing effects.
- Remove anything that has to do with `node_idx` since `mt-dom` don't use it anymore
    - We are now using `TreePath` which provides a better way to traverse the DOM tree.
- Include the total number of patches applied on the `Measurements`.
- Cleanup `Cmd` by moving the accessory fields into a `Modifier` struct
- Update to jss 0.4
- **breaking** Rename `Effects{follow_ups, effects}` to a more descriptive name `Effects{local, external}`
- **breaking** Remove `Cmd::map_msg` and `from_effects` as their functionality is moved into `Effects`.
- **breaking** Rename `Effects::follow_through` to `Effects::localize` since it maps the `external` fields into msg that is compatible with local messages and merge them.
- Change the `node_to_find` function to use `BTreeMap` so that their arrangement will be ordered/consistent, hence easier to debug
- Improve debugging InsertNode where it can NOT find the parent node
- Modify the `style` method in `Application` such that it returns a `String` rather than `Vec<String>`.
       `Component`, `Container` and `View` can now have `style` method
- Add `maybe_attr` utility helper function in attributes
- Add `Cmd::append` method to easily add more cmd to existing cmd

## 0.42.0
- shorten the `data-vdom-id` that we used as marker in the DOM element
- Improve calling on emit, The input don't need to implement Into, so we can call it right away
- Use Callback in Cmd
    - Repurpose the old Callback for Listener so it can contain additional fields such as TypeId of the arguments and return type of the Fn it contains,
        A new generic Callback with only Fn as the content has now a new place in dom::callback::Callback module, this would be used for lightweight operations such as callbacks in the components
    - Rename the type alias of Callback<Event,MSG> to Listener<MSG>
    - Rename generics in Callback<EVENT,MSG> to Callback<IN,OUT> for a more generic application
    - Wire the Callback into the parent Component Msg
    - Remove unused function Cmd::from_msg
    - Add method in Cmd to accept Msg and Vec<Msg> intended to be used for deriving an Application Cmd from Component Msg
    - Add implementation to convert Cmd from Effects, rename Cmd::map_effects to Cmd::map_follow_ups
    - Add batch_map function to Cmd to simultaneously map Msg list and create a Cmd
- Rename Effects::map_follow_ups to Effects::map_msg and Cmd::map_follow_ups to Cmd::map_msg, we already know that it is mapping the follow_up and the effects is already in the same type as the component that is containing it
    - Use effects in window-tab-rows example
    - Use effects in custom_widget example
    - Move effects out into its own module, Add functions for mapping effects easily
    - Add Effects struct to contain follow_up msg and effect Msg intended for the parent component
    - Component is just a widget which returns Effects with follow_ups, but no effects
- Improve the `Component` system, Add 2 new types of `Components`: `Container`, and `View`
    - Component and Widget are merged using Component as the final name, Component can now return both effects and followups
    - map follow_up updates from components inside a component
- **breaking** Modify and expose `request_animation_frame` to execute a rust closure,
    and create the `request_animation_frame_for_closure` which accepts `wasm_bindgen::Closure` used internally in `Program`
    - **breaking** Remove unused function `execute_in_request_in_animation_frame`
- Modify `Application::init` such that program is not included in the arg, to simplfiy and make it cleaner
- Remove `node_idx` in render, it was for debugging only
- Add a way to convert mouse_event into Event
- Remove outdated comment
- Simplify the batching the follow up Msg

## 0.41.1
- Add utility functions in html attributes which manipulates classes with namespace

## 0.41.0
- **breaking** Move out `jss` module into a new crate and [repository](https://github.com/ivanceras/jss), as it has gotten bigger.
    - This also make sauron stays slim to prevent it from collapsing from its own weight.
- **breaking** Move `sauron-markdown` out to a new [repository](https://github.com/ivanceras/sauron-markdown), as it has gotten bigger.
- Move `futuristi-ui` out of the examples and into a new [repository](https://github.com/ivanceras/futuristic-ui).
- Add `init_with_program` function in `Component` for allowing components to have a way access to it's own executor.
    - This is for enabling components have access to the program
- Remove dependency `lazy_static` in favor of `once_cell`
- **breaking** Move `sauron-parse` out of the repository to keep sauron being lean to prevent it from collapsing on its own weight.
- **breaking** Rename `Component` to `Application`, use `Component` for actual components that can use subsequent updates.
- Remove internal `style` macro in favor of `jss` crate
- Modify the `Callback` such that there is lesser chance that node with different event will be recycled
    - type_id of input arguments and output arguments of function is stored for future comparison on the diffing algorithm.
    - Add a test to show node-recylcing bug
- Move out `futuristic-ui` out of the examples, since it is a lot bigger to fit as an example. New [repo](https://github.com/ivanceras/futuristic-ui)
- **breaking** `init` in `Application` not uses `&mut self` and has access to `Program`.
    This allows the application to update its state upon initialization.

## 0.40.0
 - Improve `sauron-node-macro` performance by resolving the values of `namespace` and `self_closing` tags at compile time, rather than at runtime.
 - Add plugin capability of `sauron-markdown` to easily hook user define functions such as code-highlighting.
 - Modify `Window::scroll_to_top` to return it as wrapped in `Cmd` to be used after an `update` call in a `Component`
 - Add `popstate` to `events` module.
 - Make `sauron::jss!` macro to create the css without the use of indents and new lines by default.
    - This makes it easier to setup test as we don't have to deal with whitespace anymore.
    - Added `sauron::jss_pretty!` variant to use nice indents and space on the generated css.
 - **breaking** Improve the ergonomic to `Http` api. It is now easier to receive and deserialize text response.
 - Add a code fence processor plugin for `sauron-markdown`. Developers can hook code to create a custom element out of code blocks in markdown parser.
 - Rename `Program::new_replace_mount` to `Program::replace_mount`.
    - Rename `Program::new_append_to_mount` to `Program::append_to_mount`.
    - `Program` is not optional anymore when passed to functions in modules such as `apply_patches`, `dom_updater` and `created_node`
 - Added `safe_html` to `Text` node, this indicated whether to render as text node or as `innerHTML` of its parent element.
     - `ammonia` crate is used to sanitize the html text.
 - **breaking** Program agument is not optional anymore in module `apply_patches`, `dom_updater` and `created_node`.
 - Improve rustdoc on prominent functions and modules.
 - Add measurements function to Component for letting components know how much time is spent in each of major steps in dispatching and updating the DOM.
    - Add a field `log_measurement` to `Cmd` which tells the `Program` wheter to log and call measurements.
 - Add performance optimization for `sauron-parse` crate lookup on `tag_namespace` and `self_closing_tags` by putting it in a once_cell Lazy HashSet
 - **breaking** Rename `html_element_sc` to `html_element_self_closing`.
 - **breaking** Remove the use of underscore_ to html/svg tags such as `type_`,etc and attributes which are also rust identifier to use the raw r#ident.
    - This includes `type` `for` `async`.

## 0.39.0
- Modify `apply_patches` to use only `TreePath` where `PatchPath` is already removed in `mt-dom`

## 0.38.0
- Modify `apply_patches` to make use of `PatchPatch` and `TreePath` from `mt-dom` to efficiently
    traverse and find the target element to be patched.

## 0.37.0
- Move Callback from `mt-dom` crate to here, since it is more relevant to `sauron` than `mt-dom`.
    `map_callback` and `map_msg` is done using `trait` implementation to `mt-dom's` `Node`, `Element`, `Attribute`, and `AttValue`.
- No longer `lib.rs` will be used to generate the readme. Make a separate document for each

## 0.36.0
- simplify the construction of attribute in node-macro by using `attr`
- Revamp `Cmd` to contain flag `should_update_view` which instruct the `Program` to skip
    updating the view.

## 0.35.0
- Fix svg not appearing in the client when used in node macro syntax
- Expose `sauron-parse::parser` module in `sauron::parser`

## 0.34.0
- **breaking** unify the fail and success decoder into just text_reponse_decoder, it receives the status code together with the text body and header

## 0.33.0
- **breaking** overhaul on the `Http` api to allow users to manipulate response headers, and assign callback functions to errored request.
- fix too aggressive assertion in `apply_patches` checking for matching tags.
- implement `set_location_hash` to `Browser` api.
- **breaking** dissolve `Browser` api functionalities into  `Window` api.
- Fix behavior of checkboxes, it must be explicitly set_checked to false when checked attribute is removed

## 0.32.6
 - implement `on_mount` event where virtual node (sauron::Node) can listen to when the element is materialized into an actual dom element.
    The `MountEvent` that is passed on the argument function contains the `target_node` which is the equivalent created DOM node.
 - restructure the interactive examples
## 0.32.5
 - improve implementation of markdown-parser
    - Use `<p>` to wrap multiple top-level elements instead of `<div>` in sauron-markdown.
    - use the semantic equivalent html tag, such as `<em>` and `<strong>`, instead of just `<span>` with a `class` attribute
 - fix parsing of self-closing tag for in `sauron-parse` crate.
 - Add special handling of `on_enter` event, input elements can now use `on_enter` event to listen to when the user presses the `Enter` key

## 0.32.4
 - minor: put behind a feature-flag some trace/debugging code.

## 0.32.3
- put behind a feature flag functionalities which deals with `node_idx_lookup`

## 0.32.2
- Add `on_copy` and `on_paste` event.
- modify `execute_in_request_animation_frame` to return the handle ( can be used for cancelling the execution )
- Add special attributes `focus`, `skip` and `replace`
    - If an element has a `focus(true)` attribute, it will always have a focus on each update.
    - If an element has a `skip(true)` attribute, it will skip diffing of that node and assume no changes
    - If an element has a `replace(true)` attribute, it will return a `ReplaceNode` patch replacing the matching old node skipping the diffing.
        This will be useful, for parts in the view code where it always changes at every update, for diffing would just become an uncessary overhead.

## 0.32.1
- Fix `apply_text_patch` to include RemoveNode

## 0.32.0
- Added improvements on `node!` macro, now allowing you to use the exact attribute names for rust keywords.
    before:
    ```rust
    node!(
        <main>
            <label for_="input1">Input:</label>
            <input id="input1" type_="text" />
        </main>
    )
    ```
    now:
    ```rust
    node!(
        <main>
            <label for="input1">Input:</label>
            <input id="input1" type="text" />
        </main>
    )
    ```
- simplify the algorithmn for `find_node_recursive`
- Fix server side rendering of self closing tags html, such as `input`, `br`, `hr`
- unify attribute manipulation in render and in dom_updater, the common code is in html::attributes module
- Add node_idx attribute in render if `with-nodeidx` feature is enabled
- Reexport `serde_json` in order for `jss` macro be able to use reexported `serde_json` from sauron crate
- fix example: remove the use of workaround on attributes that are keywords in rust
- Add `#[allow(unused_braces)]` inside node macro to prevent the compiler from making expressions in braces in html as unncessary

## 0.31.2
- Fix the `render` function where attributes of the same name not merged.
- use the exported Style struct from html::attributes, to avoid namespace collision with 'style' attribute
- Add Minimal SSR example
- expose a `lite-markdown` feature from `sauron` which exclude the `sauron-parse` crate to minimize binary size
- expose `empty_attr` in `html::attributes`
    - this allows you to add a conditional attribute in building the view.
    Example:
      ```rust
            img(vec![
                    src("img/image.jpg"),
                    if let Some(img_title) = self.img_title {
                       title(img_title.to_string())
                    } else {
                       empty_attr()
                    }
                ],
                vec![]
            )
      ```
- add `jss` module and utility functions which utilizes json to preprocess css styles.
    - `jss_ns` allows you to add style to your components while prefixing the classnames with a namespace to prevent clashing
        with classes from other components.
        Check out the `futuristic-ui` [example](https://github.com/ivanceras/sauron/tree/master/examples/futuristic-ui/) for complete usage.

## 0.31.0
- (**breaking**) Improved `style!` macro by using json as the syntax
- Added `jss!` macro which parses `json` and convert them to `css`
- Invocation  the `init` method of `Component` will be done
    after the `Component` has been mounted into the DOM.
- Unified base code of `append_to_mount` and `replace_mount` in `DomUpdater`
- Added `map_msg` function for mapping `DomUpdater` to a different `MSG`
- Fix all the example project's start scripts and `index.html` load script to get rid of errors from webkit based browser
    -  Modified scripts to use `--target web` and import module to fix problems in safari
- Get rid of test_fixtures and move it to the test directory
- use `span(vec![],vec![])` instead of `text("")` in `view_if(bool,..)` when the
    flag evaluates to false
- Add support for `transitionend` and `animationend` event
- Add initial support for `style` in `Component`
- Remove the deprecated svg tag macro that should have been removed together with the html tag macro

## 0.30.0
- refactor `Cmd` to not use the `Callback`.
    - remove the need for `MSG` and `PhantomData` in Cmd.
- change the `no_request_animation_frame`feature flag to `with-request-animation-frame` to be additive and is enabled by default
- change the `measure` feature flag to `with-measure`.
- Introduce a `node!` macro syntax which allows users to write view code which resembles html syntax.
- Restructure the project to have the code code of sauron in a crate `sauron-core`.
    This paves a way to introduce modules which depends on the core functionality of sauron and then will be re-exported as part of the sauron package.
    Example: `sauron-markdown`
- Reexport `sauron_markdown` in `sauron`. can be used as `sauron::markdown`
- dom internal: Remove only the event listener which match the event_name from the ActiveClosure

## 0.29.0
- Fix the todomvc with a new rewrite and storage support
- Update the todomvc app using keyed elements
- Fix set_checked when setting attributes for checkboxes, radio buttons
- Implement code for InsertChildren patches since Patch is changed in `mt-dom`
- Use the keyed-elements diffing in mt-dom, this will ensure that the elements with the callback will be matched with the same callback
- TruncateChildren is replaced with RemoveChildren in mt-dom, this provide a more flexible patches
- Add an example of server-side rendering using warp

## 0.28.2
- merge attributes of the same name first, before applying the attributes to the DOM element

## 0.28.1
- Adjusted for changes in mt-dom with performance improvements
- remove merge_attribute functionality in Element.
- MSG doesn't need to be clone, just use get_attributes instead of merge_attributes
- A lot of performance improvement due to changes in mt-dom crate

## 0.28.0
- adjust sauron for changes in mt-dom where there are multiple values in one attribute

## 0.27.0
- change syntax of attribute events which uses `_` after the `on` ie: `on_click` instead of `onclick`
- Make a special case for style than just a plain attribute
- Use `mt-dom` crate as replacement to sauron_vdom. `mt-dom` is a very generic virtual-dom crate which doesn't put too much constraints
     on the types of the member fields

## 0.26.0
- Change `to_pretty_string` to `render` with focus on writing to the common buffer.
    - This is a performance improvement

## 0.25.0
- Add a function `Node.text` to return the string of the text element
- Add a function `Node.eldest_child_text` to return the string of the only child of this element
- Add a function `Node.get_children` to return children of a node
- Add a utility function `mousemove` and `release` for MouseEvent
- Remove the call to `stop_propagation` in the add_event_listener processor, since this is handled by the event_mapper.

## 0.24.0
- **Breaking** Add the tag in patches, to give clue to backend on how to handle special tags
	- This is used in gtk, since widgets in gtk doesn't follow a proper tree nesting as with html
- **Breaking** Detach `markdown` module into a high-level crate (sauron-md)
    - Detach `to_syntax` module into a high-level crate (sauron-syntax)
    - Create `sauron-parse` crate which `sauron-md` and `sauron-syntax` depends on.
- Add functionality for inner_html which optimizes the performance of client side apps
- Add a functionality to hook url hashchange event in the browser

## 0.23.0
- move Cmd and Dispatch from sauron_vdom to sauron
- Simplify the use Dispatch without the need for Rc


## 0.22.2
- Export `sauron_vdom::diff::diff_with_key`
- Add function `take_callback` to consume attribute and get the callback

## 0.22.1
- Only expose html::events in prelude when 'with-dom' feature is enabled

## 0.22.0
- Make use of prelude to simpilfy imports in sauron
- Add feature to parse html and convert it into sauron view syntax code.
- Add link to [html2sauron](https://ivanceras.github.io/html2sauron/) tool in the docs
- Refactor Attribute key to use generic type, Attribute key was previously using `&'static str`, It got changed to a generic type, which allows us to create attribute with key other than `&'static str` such as `String` or strongly typed `enums`.
- Simplify the indent utility function
- Improve the svg_clock example to make the subsecond update to the subsecond by ticking at every 20ms
- Add cargo deny configuration

## 0.21.1
 - Add a help function classes which joins an array of string into a space class
 - Use criterion in benchmarks
 - Add data-viewer as an example

## 0.21.0
 - add Window as a global object to let components easily attached events to the window
 - add style! macro for a nicer html style attribute syntax
 - **Breaking Change** remove tag style from the macro export, as it conflicts with the style attribute macro, which is more common
 - include mousemove in the supported event type
 - implement creating an attribute that has namespace, such as xlink:href in embededd svg image
 - fix error in svg_graph example

## 0.20.3
 - expose `onclick_with`, `onclick_stop_propagation`, `onclick_prevent_default`, `onclick_prevent_all` which allows developers
   control on the behavior of the event of a DOM element.

## 0.20.2
 - Enable doubleclick event
 - improve and modularize shell scripts
 - Fix errors in the todomvc benchmark
 - Explicitly set the value of element by calling the set_value function since just setting the attribute value is not enough
 - Enable calling to event.prevent_default() to allow both oninput and keypress event play nicely together, as used in the todomvc example
 - Add svg_graph example

## 0.20.1
 - bumped up to see logo in docs.rs

## 0.20.0
 - Add macro based syntax to provide a cleaner syntax in writing the view:
    ## Old syntax:
    ```rust
    fn view(&self) -> Node<Msg> {
        div(
            vec![class("some-class"), id("some-id"), attr("data-id", 1)],
            vec![
                input(
                    vec![
                        class("client"),
                        r#type("button"),
                        value("Click me!"),
                        onclick(|_| {
                            trace!("Button is clicked");
                            Msg::Click
                        }),
                    ],
                    vec![],
                ),
                text(format!("Clicked: {}", self.click_count)),
            ],
        )
    }
    ```

    ## New syntax:
    ```rust
    fn view(&self) -> Node<Msg> {
        div!(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                input!(
                    [
                        class("client"),
                        type_("button"),
                        value("Click me!"),
                        onclick(|_| {
                            trace!("Button is clicked");
                            Msg::Click
                        }),
                    ],
                    [],
                ),
                text!("Clicked: {}", self.click_count),
            ],
        )
    }
    ```

 - Move DomEvent in dom module
 - nicer name for `dumb_patch` -> `apply_dumb_patch`
 - Refactor `dom_updater` and `created_node` out of the dom module
 - Add macro syntax, which provides a cleaner code by eliminating the `vec![]` syntax on the components view functions
 - Enable github actions
 - Reorganize dom specific module to get rid of multiple cfg feature code in the library
 - Reorganize `html::tags` and `svg::tags`
 - Remove the html_array syntax
 - Fix unused warning errors when no feature is enabled
 - Use the proper logging by using `log` and `console_log crate`
 - Completely remove the with-serde feature
 - Add feature gate 'with-dom' for browser specific functionality, such that sauron can be efficiently used for server-side rendering
 - Constraint the generic Type: `F` to be 'static in Callback, instead of the return generic type
 - Fix attributes helper functions: (`styles`, `styles_flag`, `classes`, `classes_flag`, `attrs_flag`) should not require MSG to be clone

## 0.11.1
 - attributes helper functions such as (styles, classes, etc) should not require MSG to be Clone.

## 0.11.0
 - Add underscores on html tags and attribtues(`type`,`for`, `async`, `loop`) that are also special keywords in rust.
    Now, you can use `type_("text")` as an alternative to `r#type("text")`
 - rename as_element -> as_element_mut,  children -> add_children
 - Add `dumb_patch` for patching the dom without involving the callbacks.
 - Expose to `html::tag` module for the uncommon html tags which conflicts with common html attributes such as `style`, `title`.

## 0.10.1
 - implemented removing the associated closures of elements that has been removed from the DOM including the removed element descendants.


## 0.10.0
 - performance improvement on node tree building
 - using vec![] as the argumemts for attributes and children, this changes the syntax a lot
    - The original array based syntax is still preserved by using the `html_array` module. This however has performance penalty
 - events and attributes are now unified in one field: `attrs`
 - `map` function mapping Msg from in between component is now `map_msg` to avoid confusion with the rust std common maps such `Iterator.map`
 - add units utility functions
 - Remove requirement for Msg to have any Clone,Debug,PartialEq

## 0.7.1
 - Add initial implementation for markdown handling
 - Add history function get history object
 - events now prevents defaults and stop propagation

## 0.7.0
- Added an initial implementation for Http for fetching data which returns a Cmd
- Added Examples usage of Http fetch
- Added Browser for listening to browser resize event which returns a Cmd
- Added Cmd module for abstracting calls such as Http requests
- Added an optional `init` function in Component which allows apps execute Cmd Task such as fetching data at the start of the app
- Change the update method in Component to return Cmd<Self,Msg> in update method


## 0.6.0
- Refactor sauron_vdom::Event to cater general usecase for mouse, keyboard and input event
- Events such as onclick, onkeypress, and oninput are now supplied with: MouseEvent, KeyEvent, and InputEvent
    accordingly, therefore no additional matching/unwrapping code is neccessary on the users code.
    Before:
    ```rust
       onclick(|event: Event| {
            if let Event::MouseEvent(mouse) = event{
                sauron::log!("clicked at ({},{})", mouse.x(), mouse.y())
            }else{
                panic!("This should not happen")
            }
       })
    ```
    Now:
    ```rust
        onclick(|mouse: MouseEvent| {
            sauron::log!("clicked at ({},{})", mouse.x(), mouse.y())
        })
    ```
 - Move to svg_extra the following tags and attributes: style, width, height, id, font_size, font_family,
     since these conflicts with the commonly used tags and attributes in html. Attributes that are defined in html attributes
     could also be used in svg attributes. What's not possible is using tags declared in html module in svg elements,
     since svg elements needs to be created with svg namespace in the DOM.


## 0.5.0
- Use &'static str type for Node's attribute name, event name and namespace.
- Add helper function `styles` which allows users to write style properties easily.
- Add helper function `styles_flag` which allows users to write even more granular style properties.
- Elements attributes are now appended to the existing attributes ones,
    this is needed when there is multiple calls assigning on the same attributes on the same element
- Put back `Callback<Event,MSG>` as the value of node.events.
- Add `map` functionality which lets user embed subcomponents view into the parent component by mapping the callbacks
    with a wrapped MSG variant from the parent.

## 0.4.0
- Added the complete list of svg/html attributes.
- Separate the uncommon html tags into html_extract module. These includes `style`, which conflicts with the
commonly used `style` attributes.
- Separate the uncommon attributes such as `span`, `label` which conflicts with the
commonly used `span` and `label` html tags.
- Use snake_case for non-ident tags and attributes.
