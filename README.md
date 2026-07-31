# dvrf-rs

[![CI](https://github.com/deutsche-nationalbibliothek/dvrf-rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/deutsche-nationalbibliothek/dvrf-rs/actions/workflows/ci.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/dvrf)](https://crates.io/crates/dvrf)
[![docs.rs](https://img.shields.io/docsrs/dvrf)](https://docs.rs/dvrf/latest/dvrf/)
[![dependency status](https://deps.rs/crate/dvrf/latest/status.svg)](https://deps.rs/crate/dvrf/latest)
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

## License

This project is licensed under the [European Union Public License 1.2].


[Data Validation Report Format (DVRF)]: https://gbv.github.io/data-validation-report-format/
[DCO]: https://developercertificate.org
[European Union Public License 1.2]: ./LICENSE
