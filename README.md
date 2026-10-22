# Habanero
Habanero is a rust HTTP crate which exposes both a client and server model for sending and receiving HTTP requests.

[![Crates.io Version](https://img.shields.io/crates/v/foxpaw_habanero)](https://crates.io)
[![docs.rs](https://img.shields.io/docsrs/foxpaw_habanero)](https://docs.rs)
[![GitHub License](https://img.shields.io/github/license/foxpaw-rs/habanero)](LICENSE.md)

## Features
This crate supports being used as a crate to further develop your own application layer on top of, or as a standalone application to serve static web pages.

The architecture utilises clean threads, removing any dependency on an async runtime.

Currently, this crate only supports raw http requests. It is recommended to utilise a reverse proxy to support https in the interim.

## Quick Start
### Library
Add this crate as a dependency to your `Cargo.toml`:
```toml
[dependencies]
foxpaw_habanero = "0.1.0"
```

### Application
To compile the repository, please first install the [rust toolchain](https://rust-lang.org/tools/install/). Clone or download the repository, navigate to it in a terminal and run `cargo run` to spin up the server.

The server will now be able to serve web pages from the _`public/`_ directory, relative to your current working directory when the application is run. See the [documentation](#documentation) section for more details.

## Examples
### Start a Server
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::{HttpError, ServerError};

fn main() -> Result<(), ServerError> {
    let server = Server::bind("127.0.0.1:7878")?;
    server.run(|req: Request| {
        if req.uri == "/" {
            Ok(Response::new(Version::Http11, 200)
                .with_html("<html><body><h1>Hello World!</h1></body></html>"))
        } else {
            Err(HttpError::NotFound)
        }
    });
    Ok(())
}
```

### Send a Request to a Remote Host
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::ClientError;

fn main() -> Result<(), ClientError> {
    let client = Client::new();
    let respone = client.get(&"127.0.0.1:7878".parse()?)?;
    Ok(())
}
```

## Documentation
Documentation can be found inside the [_`docs/`_](docs/) directory of the repository.

The full API guide is available on [docs.rs](https://docs.rs), or alternatively `cargo doc` can be run inside a clone of the repository for a local version.

For a comprehensive overview of the features per version and development over time, see the project [roadmap](ROADMAP.md).

## License
Licensed under the MIT License. See the [license](LICENSE.md) for further details.