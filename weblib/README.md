# Rust `weblib` Library
The Rust `weblib` Library is a small, simple library for fetching the contents of a URL and returning them as a string.
<br>
![GitHub](https://img.shields.io/github/license/mac-lawson/web)![GitHub commit activity](https://img.shields.io/github/commit-activity/y/mac-lawson/web)


### Installation:
You can add this library as a dependency to your Rust project by adding the following line to your Cargo.toml file:
```rust
[dependencies]
web = "0.1.1"
```


### Usage:
Here is an example of how to use the text() function:
```rust
let url = "https://httpbin.org/ip";
match weblib::text(url) {
    Ok(resp) => println!("{}", resp),
    Err(e) => panic!("Error: {}", e),
}
```
And here is an example of how to use the query() function:
```rust
let url = "https://httpbin.org/get";
let query_string = "key1=value1&key2=value2";
match weblib::query(url, query_string) {
    Ok(resp) => println!("{}", resp),
    Err(e) => panic!("Error: {}", e),
}
```

### Running tests:
Tests can be run with the following command:
``` rust
cargo test
```

### Author
This library was created by Mac Lawson.

### License
This library is licensed under the GPLv3. See the LICENSE file for details.