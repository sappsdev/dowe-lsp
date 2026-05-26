(import_statement "import" @keyword "from" @keyword)
(import_statement (identifier) @variable)
(import_path) @string
(type_declaration (type_keyword) @keyword (type_name) @type)
(node_line name: (line_name (root_keyword) @keyword))
(node_line name: (line_name (block_keyword) @keyword))
(node_line name: (line_name (control_keyword) @keyword))
(node_line name: (line_name (action_keyword) @function))
(type_prop (type_keyword) @keyword (type_reference) @type)
(body_type_binding (body_keyword) @variable (type_reference) @type)
(type_field
  (type_field_key) @property
  (type_reference) @type)
(array_type_reference) @type
(host_function) @function
(node_line name: (line_name (component_name) @constructor))
(prop (property_name) @property)
(object_entry (property_name) @property)
(value (identifier) @variable)
(value (property_name) @variable)
(text_line) @string
(text_line (text_token (reference) @variable))
(text_fragment) @string
(string) @string
(number) @number
(boolean) @boolean
(null) @constant.builtin
(reference) @variable
(method_name) @constant
(path_literal) @string.special
(punctuation) @punctuation.delimiter
