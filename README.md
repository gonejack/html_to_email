# html_to_email

This command line converts .html file to .eml file.

![Build](https://github.com/gonejack/html_to_email/actions/workflows/rust.yml/badge.svg)
[![GitHub license](https://img.shields.io/github/license/gonejack/html_to_email.svg?color=blue)](LICENSE)

### Install
```shell
> cargo install --git https://github.com/gonejack/html_to_email
```

### Usage
```shell
> html_to_email *.html
```
```
html_to_email 
https://github.com/gonejack/html_to_email

USAGE:
    html_to_email [FLAGS] [OPTIONS] [html]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    verbose printing

OPTIONS:
    -f, --from <FROM>    set sender address
    -t, --to <TO>        set receiver address

ARGS:
    <html>...   
```
