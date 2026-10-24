# Habanero
Habanero is a rust HTTP crate which exposes both a client and server model for sending and receiving HTTP requests.

Habanero can be utilized both as a crate to develop a web application on top of, or as a standalone application to serve a web page.

For other resources, please see:
* The [Github repository](https://github.com/foxpaw-rs/habanero/);
* The [API reference](https://docs.rs/foxpaw_habanero/);
* The [Crate reference](https://docs.rs/foxpaw_habanero/)

## Installation
Your installation method will vary depending on whether you would like the application version or the crate version of Habanero.

### Crate
Add this crate as a dependency to your `Cargo.toml`:
```toml
[dependencies]
foxpaw_habanero = "0.1.0"
```

For guides and further documentation please see the [crate documentation](crate.md) for further.

### Application
To compile the repository, please first install the [rust toolchain](https://rust-lang.org/tools/install/). Clone or download the repository, navigate to it in a terminal and run `cargo run` to spin up the server.

The server will now be able to serve web pages from the _`public/`_ directory, relative to your current working directory when the application is run. See the [application documentation](application.md) for more details.

## Guides
For more guides and documentation, please visit the relevant [crate](crate.md) or [application](application.md) document.

