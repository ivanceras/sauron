# 0.5.0
- Add helper function `styles` which allows users to write style attributes easily.
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
