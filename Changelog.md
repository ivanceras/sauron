
# 0.4.0
- Added the complete list of svg/html attributes.
- Separate the uncommon html tags into html_extract module. These includes `style`, which conflicts with the 
commonly used `style` attributes.
- Separate the uncommon attributes such as `span`, `label` which conflicts with the
commonly used `span` and `label` html tags.
- Use snake_case for non-ident tags and attributes.
