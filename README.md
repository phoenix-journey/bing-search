# bing-search
Rust program to display top Bing search engine results

## Basic usage

either invoke `cargo build --release` and run from target 
directory

or

`cargo install` and use `bing-search`

### Arguments

- **query** the thing you usually put in address bar, like Google
    but not _Evil_
- **max_result** reasonably short/long output.
- **cvid** most often not needed, but when the error is strange
    you might try to extract this value from real search query
    parameters (in address bar of web browser)