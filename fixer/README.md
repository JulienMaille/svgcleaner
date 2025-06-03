# Fixer

Fixes SVGs before they are fed into the SVG Cleaner.


## Why
The svgcleaner app is very strict and does not handle invalid syntax too well, this app is for dealing with invalid syntax and other such issues.

## Usage
You can run it by using cargo run.
-- passes args to svgcleaner

``` bash
cargo run -- <directory>
```

## Build
You can build using;

``` bash
cargo build
```

## Test
You can test using;

``` bash
cargo test
```

## Install system-wide
You can then install it (optional) using;

``` bash
cargo install --path .
```
