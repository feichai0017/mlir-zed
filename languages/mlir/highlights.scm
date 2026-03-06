[
  (integer_literal)
  (float_literal)
  (complex_literal)
] @number

(bool_literal) @boolean
(string_literal) @string

[
  (attribute_alias)
  (attribute_alias_def)
  (fastmath_attr)
] @attribute

[
  (builtin_type)
  (dialect_type)
  (function_type)
  (memref_type)
  (ranked_tensor_type)
  (tuple_type)
  (type_alias)
  (type_alias_def)
  (unranked_memref_type)
  (unranked_tensor_type)
  (vector_type)
] @type

(function_definition
  name: (symbol_ref_id) @function)

(custom_operation
  name: (bare_id) @function)

(block_label
  (caret_id) @label)

(operation_results
  (value_id) @variable)

(value_use) @variable

(func_arg_list
  (value_use) @variable.parameter)

(function_definition
  body: (region
    (block
      (block_arg_list
        (value_use) @variable.parameter))))

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ":"
  "="
  "->"
] @punctuation.delimiter
