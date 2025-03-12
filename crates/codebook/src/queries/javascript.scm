(comment) @comment
(string_fragment) @string
(variable_declarator
    name: (identifier) @identifier)
(object
    (pair
    key: (property_identifier) @property_name))
(catch_clause
    parameter: (identifier) @identifier)
(jsx_text) @string
(shorthand_property_identifier) @identifier
(function_declaration
    name: (identifier) @identifier)
(function_declaration
    parameters: (formal_parameters
    (identifier) @identifier))
(method_definition
    name: (property_identifier) @identifier)
(class_declaration
    name: (identifier) @identifier)
