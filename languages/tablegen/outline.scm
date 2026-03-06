(comment) @annotation
(multiline_comment) @annotation

(class
  "class" @context
  name: (identifier) @name) @item

(multiclass
  "multiclass" @context
  (identifier) @name) @item

(deftype
  "deftype" @context
  (identifier) @name) @item

(defset
  "defset" @context
  (identifier) @name) @item

(defvar
  "defvar" @context
  (identifier) @name) @item

(include
  "include" @context
  (string_string) @name) @item

(def
  "def" @context
  (value
    (identifier) @name)) @item

(defm
  "defm" @context
  (value
    (identifier) @name)) @item
