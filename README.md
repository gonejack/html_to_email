# html_to_email

This command line converts .html file to .eml file.

![Build](https://github.com/gonejack/html_to_email/actions/workflows/rust.yml/badge.svg)
[![GitHub license](https://img.shields.io/github/license/gonejack/html_to_email.svg?color=blue)](LICENSE)

### Install
From crates.io
```shell
> cargo install html_to_email
```
Or the latest version
```shell
> cargo install --git https://github.com/gonejack/html_to_email
```

### Usage
```shell
> html_to_email *.html
```
```
Options:
    -f, --from FROM     Set sender address
    -t, --to TO         Set receiver address
    -v, --verbose       Verbose printing
    -h, --help          Print this help
        --about         Show about
```
