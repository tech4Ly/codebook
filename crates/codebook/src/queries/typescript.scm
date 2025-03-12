(comment) @comment
(string_fragment) @string
(variable_declarator
    name: (identifier) @identifier)
(object
    (pair
    key: (property_identifier) @property_name))
(interface_declaration
    name: (type_identifier) @identifier)
(interface_body
    (property_signature
        name: (property_identifier) @property_name))
(catch_clause
    parameter: (identifier) @identifier)
(jsx_text) @string
(shorthand_property_identifier) @identifier
(function_declaration
    name: (identifier) @identifier)
(formal_parameters
    (required_parameter
    pattern: (identifier) @identifier))
(formal_parameters
    (optional_parameter
    pattern: (identifier) @identifier))
(method_definition
    name: (property_identifier) @identifier)
(class_declaration
    name: (type_identifier) @identifier)
(public_field_definition
    name: (property_identifier) @identifier)
