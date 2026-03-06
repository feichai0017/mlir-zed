(comment) @annotation

(type_alias_def
  "!" @context
  (bare_id) @name) @item

(attribute_alias_def
  "#" @context
  (bare_id) @name) @item

(block
  (block_label
    (caret_id) @name)) @item

(operation
  rhs: (custom_operation
    (func_dialect
      "func.func" @context
      name: (symbol_ref_id) @name))) @item

(operation
  rhs: (custom_operation
    (llvm_dialect
      "llvm.func" @context
      name: (symbol_ref_id) @name))) @item

(operation
  lhs: (op_result
    (value_use) @name)
  rhs: (custom_operation) @context) @item

(operation
  lhs: (op_result
    (value_use) @name)
  rhs: (generic_operation
    (string_literal) @context)) @item

(operation
  rhs: (generic_operation
    (string_literal) @name)) @item
