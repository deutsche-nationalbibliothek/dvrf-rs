# dvrf-rs

[![License](https://img.shields.io/github/license/deutsche-nationalbibliothek/dvrf-rs?color=blue)](./LICENSE)

This project provides a library for processing the [Data Validation
Report Format (DVRF)] in the Rust programming language. In addition
to the library, it also provides the `dvrf` tool, which offers useful
commands for analysis and further processing.

## Demo

```console
$ dvrf concat --pretty tests/data/errors.json
[
  {
    "message": "Unexpected end of JSON input at character 8",
    "position": {
      "char": "8",
      "line": "1"
    }
  }
]
```

## Contributing

All contributors are required to "sign-off" their commits (using
`git commit -s`) to indicate that they have agreed to the [Developer
Certificate of Origin][DCO].

This project uses a strict **no AI** / **no LLM** policy. Please do
not use large language models (LLMs) to create issues, patches, pull
requests, or comments. Although English is the preferred language, you
are welcome to communicate in your native language.

## License

This project is licensed under the [European Union Public License 1.2].


[Data Validation Report Format (DVRF)]: https://zenodo.org/records/20792191
[DCO]: https://developercertificate.org
[European Union Public License 1.2]: ./LICENSE
