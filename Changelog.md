# Changelog

# 0.22.0
- Make use of prelude to simpilfy imports in sauron
- Add feature to parse html and convert it into sauron view syntax code.
- Add link to [html2sauron](https://ivanceras.github.io/html2sauron/) tool in the docs
- Refactor Attribute key to use generic type, Attribute key was previously using `&'static str`, It got changed to a generic type, which allows us to create attribute with key other than `&'static str` such as `String` or strongly typed `enums`.
- Simplify the indent utility function
- Improve the svg_clock example to make the subsecond update to the subsecond by ticking at every 20ms
- Add cargo deny configuration

# 0.21.1
 - Add a help function classes which joins an array of string into a space class
 - Use criterion in benchmarks
 - Add data-viewer as an example

# 0.21.0
 - add Window as a global object to let components easily attached events to the window
 - add style! macro for a nicer html style attribute syntax
 - **Breaking Change** remove tag style from the macro export, as it conflicts with the style attribute macro, which is more common
 - include mousemove in the supported event type
 - implement creating an attribute that has namespace, such as xlink:href in embededd svg image
 - fix error in svg_graph example

# 0.20.3
 - expose `onclick_with`, `onclick_stop_propagation`, `onclick_prevent_default`, `onclick_prevent_all` which allows developers
   control on the behavior of the event of a DOM element.

# 0.20.2
 - Enable doubleclick event
 - improve and modularize shell scripts
 - Fix errors in the todomvc benchmark
 - Explicitly set the value of element by calling the set_value function since just setting the attribute value is not enough
 - Enable calling to event.prevent_default() to allow both oninput and keypress event play nicely together, as used in the todomvc example
 - Add svg_graph example

# 0.20.1
 - bumped up to see logo in docs.rs

# 0.20.0
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

# 0.11.1
 - attributes helper functions such as (styles, classes, etc) should not require MSG to be Clone.

# 0.11.0
 - Add underscores on html tags and attribtues(`type`,`for`, `async`, `loop`) that are also special keywords in rust.
    Now, you can use `type_("text")` as an alternative to `r#type("text")`
 - rename as_element -> as_element_mut,  children -> add_children
 - Add `dumb_patch` for patching the dom without involving the callbacks.
 - Expose to `html::tag` module for the uncommon html tags which conflicts with common html attributes such as `style`, `title`.

# 0.10.1
 - implemented removing the associated closures of elements that has been removed from the DOM including the removed element descendants.


# 0.10.0
 - performance improvement on node tree building
 - using vec![] as the argumemts for attributes and children, this changes the syntax a lot
    - The original array based syntax is still preserved by using the `html_array` module. This however has performance penalty
 - events and attributes are now unified in one field: `attrs`
 - `map` function mapping Msg from in between component is now `map_msg` to avoid confusion with the rust std common maps such `Iterator.map`
 - add units utility functions
 - Remove requirement for Msg to have any Clone,Debug,PartialEq

# 0.7.1
 - Add initial implementation for markdown handling
 - Add history function get history object
 - events now prevents defaults and stop propagation

# 0.7.0
- Added an initial implementation for Http for fetching data which returns a Cmd
- Added Examples usage of Http fetch
- Added Browser for listening to browser resize event which returns a Cmd
- Added Cmd module for abstracting calls such as Http requests
- Added an optional `init` function in Component which allows apps execute Cmd Task such as fetching data at the start of the app
- Change the update method in Component to return Cmd<Self,Msg> in update method


# 0.6.0
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


# 0.5.0
- Use &'static str type for Node's attribute name, event name and namespace.
- Add helper function `styles` which allows users to write style properties easily.
- Add helper function `styles_flag` which allows users to write even more granular style properties.
- Elements attributes are now appended to the existing attributes ones,
    this is needed when there is multiple calls assigning on the same attributes on the same element
- Put back `Callback<Event,MSG>` as the value of node.events.
- Add `map` functionality which lets user embed subcomponents view into the parent component by mapping the callbacks
    with a wrapped MSG variant from the parent.

# 0.4.0
- Added the complete list of svg/html attributes.
- Separate the uncommon html tags into html_extract module. These includes `style`, which conflicts with the
commonly used `style` attributes.
- Separate the uncommon attributes such as `span`, `label` which conflicts with the
commonly used `span` and `label` html tags.
- Use snake_case for non-ident tags and attributes.
