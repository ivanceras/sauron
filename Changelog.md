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
