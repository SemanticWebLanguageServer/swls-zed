(comment) @comment

[
  "@prefix"
  "@base"
  "PREFIX"
  "BASE"
  "GRAPH"
] @keyword

"a" @keyword

(iri_reference) @link_uri

(namespace) @type

(subject (prefixed_name (pn_local) @variable))
(predicate (prefixed_name (pn_local) @property))
(object_list (prefixed_name (pn_local) @constant))
(object_collection (prefixed_name (pn_local) @constant))
(rdf_literal datatype: (prefixed_name (pn_local) @type))

(blank_node_label) @variable.special

(string) @string
(echar) @string.escape
(lang_tag) @attribute

[
  (integer)
  (decimal)
  (double)
] @number

(boolean_literal) @boolean

"^^" @operator

[
  "."
  ","
  ";"
] @punctuation.delimiter

[
  "["
  "]"
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket
