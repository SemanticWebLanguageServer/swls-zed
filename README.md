# Semantic Web – Zed Extension

[Zed](https://zed.dev) extension for **Turtle**, **TriG**, **SPARQL**, and **JSON-LD**, powered by the [Semantic Web Language Server (SWLS)](https://github.com/SemanticWebLanguageServer/swls).

## Features

- Syntax highlighting, bracket matching, indentation and outline (via Tree-sitter grammars)
- Autocompletion (prefixes, properties, classes, ontologies)
- Diagnostics
- Formatting
- Semantic highlighting
- Go to definition
- Hover information

## Installation

Search for **Semantic Web** in Zed's extension marketplace (`zed: extensions`).

The extension looks for the language server in this order:

1. The path configured in `lsp.swls.binary.path` (see below).
2. A `swls` binary on your `PATH`.
3. Otherwise it downloads the latest SWLS release for your platform from
   [GitHub](https://github.com/SemanticWebLanguageServer/swls/releases).

Supported file types: `.ttl`, `.turtle` (Turtle), `.trig` (TriG), `.rq`, `.sparql`, `.sq`, `.ru` (SPARQL) and `.jsonld` (JSON-LD).

> **Note:** If you also have the separate `turtle` extension installed, both extensions
> register a "Turtle" language. Uninstall one of them to avoid conflicts.

## Grammars

| Language | Tree-sitter grammar |
| --- | --- |
| Turtle, TriG | [tree-sitter-turtle](https://github.com/GordianDziwis/tree-sitter-turtle) |
| SPARQL | [tree-sitter-sparql](https://github.com/GordianDziwis/tree-sitter-sparql) |
| JSON-LD | [tree-sitter-json](https://github.com/tree-sitter/tree-sitter-json) |

## Configuration

Add to your Zed `settings.json` to customize initialization options:

```json
{
  "lsp": {
    "swls": {
      "initialization_options": {
        "log": "info",
        "turtle": true,
        "trig": true,
        "jsonld": true,
        "sparql": false,
        "ontologies": [],
        "shapes": [],
        "completion": "none"
      }
    }
  }
}
```

To use a custom binary:

```json
{
  "lsp": {
    "swls": {
      "binary": {
        "path": "/path/to/swls"
      }
    }
  }
}
```

## License

MIT
