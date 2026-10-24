# Habanero Crate
This guide will guide you through using Habanero as a crate, either for an HTTP Client or Server.

## Installation
Add this crate as a dependency to your `Cargo.toml`:
```toml
[dependencies]
foxpaw_habanero = "0.1.0"
```

The remainder of this guide is separated depending on whether you're using the [`Client`](#client) or [`Server`](#server) stream of the crate.

## Client
This section of the document will provide guides in relation to using Habanero as an HTTP client. Note that all the errors returned from any client section will be a `foxpaw_habanero::err::ClientError`.

### Creating a Client
To use the Habanero client to connect to a remote host, the first requirement is to create one. Bring in the `prelude` module and, if you want to handle errors gracefully, the `err::ClientError` type.

Okay, first lets create a simple example to create a `Client`:
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::ClientError;

fn main() -> Result<(), ClientError> {
    let client = Client::new();
    Ok(())
}
``` 

### Sending a Simple Request
Once we have a `Client`, we can send simple requests to remote hosts. The `Client` has a number of simple request methods that match the HTTP specification. If these HTTP methods expect a body to the request, so does the method. These convenience methods automatically set the Host header, and, if a body is provided, the Content-Length header.

Note that unless there's something waiting to accept the connection in the following examples, they will fail with a connection refused, however, expanding on our earlier example:
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::ClientError;

fn main() -> Result<(), ClientError> {
    let client = Client::new();

    // GET HTTP requests do not expect a request body...
    let res = client.get(&"http://127.0.0.1".parse()?)?;

    // ...whereas POST HTTP requests do.
    let res = client.post(&"http://127.0.0.1".parse()?, b"Hello World")?;
    Ok(())
}
```

This will automatically set the Content-Length and Host headers for your request.

### Creating and Sending More Complex Requests
Often though, a simple HTTP request is not what is required. For times when more customization of our outgoing request is called for, reach for the `Request` type. For a full reference to the available helper methods refer to the API documentation, however, it is important to note here that the `Request` type is designed to be fluently built. Each of the methods supplied will consume, update and return the `Request`. This allows you to create a Request in a fluid method chain.

Once we're happy with our `Request`, the `Client::dispatch` method can be used to dispatch the request to the remote. Note that unlike the convenience methods above, this only sets the Host header.

Revisiting our first example, but this time dispatching a custom `Request`
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::ClientError;

fn main() -> Result<(), ClientError> {
    let client = Client::new();

    // Create a request
    let req = Request::new(Method::Post, "/", Version::Http11)
    	.with_header("my-custom-header", "my-custom-value")
    	.with_json(r#"{message: "Hello World"}"#);

    // Note that if we want to update the request later, we need to re-set the
    // variable, here as the request was consumed by the with_header call, we
    // can shadow it
    let req = req.with_header("another-header", "another-value");

    // And dispatch the Request to send it
    client.dispatch(&"127.0.0.1:7878".parse()?, req)?;

    Ok(())
}
```

## Server
This section of the document will provide guides in relation to using Habanero as an HTTP server. Note that all the errors returned from any server section will be a `foxpaw_habanero::err::ServerError`.

The server uses a threaded approach to a non-blocking server. This ensures that any application using Habanero as a server is not tied to an aysnc ecosystem. Additionally, it ensures the server handles any concurrency, and you as the developer of the application can focus on primarily single threaded development.

Note that currently this crate does not support TLS connections, and therefore, not HTTPS requests. It is currently advised to use a reverse proxy from an established web server to forward unencrypted HTTPS requests to your application.

### Creating a Server
To use the Habanero server to accept connections we first need to bind a server to an address to listen on. Bring in the `prelude` module and, if you want to handle errors gracefully, the `err::ServerError` type.

```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::ServerError;

fn main() -> Result<(), ServerError> {
    let server = Server::bind("127.0.0.1:7878")?;
    Ok(())
}
``` 

### Handling Requests
Once you have created a server, we now need to run it. The `Server::run` method will begin listening for incoming requests and responding to them. `Server::run` takes a `Handler` type which specifies how to handle each incoming request. There are two options, using a simple function, or, using the `Router`. In this section we'll explore the simpler callback method.

Any function of the form `fn(req: Request) -> Result<Response, HttpError>` can be used as a handler. The `HttpError` type just defines 4xx an 5xx HTTP codes as error types to quickly exit your handler. These are converted by the server to proper `Response` types to be sent back to the client trying to reach your server.

Expanding on our earlier example:
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::{HttpError, ServerError};

fn main() -> Result<(), ServerError> {
    let server = Server::bind("127.0.0.1:7878")?;
    server.run(|req: Request| {
    	if req.uri == "/" {
    		Ok(Response::new(Version::Http11, 200))
    	} else {
    		Err(HttpError::NotFound)
    	}
    });
    Ok(())
}
``` 

### Using the Router
For a more complex application, a `Router` type is provided to split incoming requests and hand them off to specific controllers. These controllers are again, callback functions, of the form `dyn Fn(Request) -> Result<Response, HttpError> + Send + Sync`. Basically, again, the callback method as above. As the route handler is shared across threads though, the callback must be `Send + Sync`. The `Router` is included in the prelude so no new imports required.

Similarly to the `Request` and `Response`, the `Router` supports method chaining. This allows for a chain of routes to be added cleanly, however, if routes are added later, the original variable isn't mutated and will have to be re-stored or shadowed.

Lets expand on our original example by creating the router and running a slightly more complex server:
```rust, no_run
use foxpaw_habanero::prelude::*;
use foxpaw_habanero::err::{HttpError, ServerError};

fn main() -> Result<(), ServerError> {
    let server = Server::bind("127.0.0.1:7878")?;
    
    // Create the Router
    let router = Router::new()
    	.with_get("/", |req: Request| { 
    		if let Some(val) = req.headers.find("Accept") && val == "application/json" {
    			Ok(Response::new(Version::Http11, 200).with_json(r#"{message: "Hello World"}"#))
    		} else { 
    			Ok(Response::new(Version::Http11, 200).with_html("<h1>Hello World!</h1>"))
    		}
    	});

    // Note that if we want to update the router later, we need to re-set the
    // variable, here as the router was consumed by the with_get call, we
    // can shadow it
    let router = router.with_get("/greet", |req: Request| Ok(Response::new(Version::Http11, 200)));

    // And finally, run the server using the router as the handler
    server.run(router);
    Ok(())
}
```