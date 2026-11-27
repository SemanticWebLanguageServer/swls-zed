(comment) @comment

(string) @string
(escape_sequence) @string.escape

(pair
  key: (string) @property)

; JSON-LD keywords such as "@context", "@id", "@type"
(pair
  key: (string (string_content) @keyword
    (#match? @keyword "^@")))

((string (string_content) @keyword)
  (#match? @keyword "^@[a-z]+$"))

(number) @number

[
  (true)
  (false)
] @boolean

(null) @constant

[
  ","
  ":"
] @punctuation.delimiter

[
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket
