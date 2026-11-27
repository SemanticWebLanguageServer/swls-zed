(prefix_declaration
  "PREFIX" @context
  (namespace) @name) @item

(select_query
  (select_clause) @name) @item

(construct_query
  "CONSTRUCT" @name) @item

(ask_query
  "ASK" @name) @item

(describe_query
  "DESCRIBE" @name) @item
