(import_statement "import" @keyword "from" @keyword)
(import_statement (identifier) @variable)
(import_path) @string
(root_keyword) @keyword
(block_keyword) @keyword
(control_keyword) @keyword
(action_keyword) @function
(host_function) @function
(component_name) @constructor
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
