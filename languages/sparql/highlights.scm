(comment) @comment

[
  "ADD" "ALL" "AS" "ASC" "ASK" "BASE" "BIND" "BY" "CLEAR" "CONSTRUCT" "COPY"
  "CREATE" "DEFAULT" "DELETE" "DELETE DATA" "DELETE WHERE" "DESC" "DESCRIBE"
  "DISTINCT" "DROP" "EXISTS" "FILTER" "FROM" "GRAPH" "GROUP" "HAVING" "IN"
  "INSERT" "INSERT DATA" "INTO" "LIMIT" "LOAD" "MINUS" "MOVE" "NAMED" "NOT"
  "OFFSET" "OPTIONAL" "ORDER" "PREFIX" "REDUCED" "SELECT" "SEPARATOR" "SERVICE"
  "SILENT" "TO" "UNDEF" "UNION" "USING" "VALUES" "WHERE" "WITH"
] @keyword

"a" @keyword

[
  "ABS" "AVG" "BNODE" "BOUND" "CEIL" "COALESCE" "CONCAT" "CONTAINS" "COUNT"
  "DATATYPE" "DAY" "ENCODE_FOR_URI" "FLOOR" "GROUP_CONCAT" "HOURS" "IF" "IRI"
  "LANG" "LANGMATCHES" "LCASE" "MAX" "MD5" "MIN" "MINUTES" "MONTH" "NOW" "RAND"
  "REGEX" "REPLACE" "ROUND" "SAMPLE" "SECONDS" "SHA1" "SHA256" "SHA384" "SHA512"
  "STR" "STRAFTER" "STRBEFORE" "STRDT" "STRENDS" "STRLANG" "STRLEN" "STRSTARTS"
  "STRUUID" "SUBSTR" "SUM" "TIMEZONE" "TZ" "UCASE" "URI" "UUID" "YEAR"
  "isBLANK" "isIRI" "isLITERAL" "isNUMERIC" "isURI" "sameTerm"
] @function

(function_call
  identifier: (prefixed_name (pn_local) @function))

(var) @variable

(iri_reference) @link_uri

(namespace) @type

(property (prefixed_name (pn_local) @property))
(path_element (prefixed_name (pn_local) @property))

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

(binary_expression
  [
    "=" "!=" "<" ">" "<=" ">=" "&&" "||" "+" "-" "*" "/"
  ] @operator)

(unary_expression
  [
    "!" "+" "-"
  ] @operator)

[
  "^" "^^" "|"
] @operator

(path_mod) @operator

[
  "."
  ","
  ";"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket
