# Habanero Roadmap

## Version 0.1

### Todo
- [ ] Transport
: Create the transport layer, including a `Transport` trait and `Tcp` struct implementation under the `transport` module.
- [ ] Parsers
: Create `Request` and `Response` under the `parser` module, which convert the raw request and response into `type/Request` and `type/Response` objects. Additionally, create `Request` and `Response` types under the `serialise` module, which do the inverse and serialise `type/Request` and `type/Response` objects ready for sending.
- [ ] Connection
: Create a `Connection` trait, and `Http1` which implements it under the `server/connection` module. Additionally, create a `Connection` and a `Http1` which implements it under the `client/connection` module. The connection types wrap a `Transport` type, which will allow for longer lasting and multiplexed connections at a future stage.
- [ ] Server
: Create a `Server` struct which wraps incoming requests into the `transport/Tcp` type. This should use a `parser/Request` to obtain a `type/Request` which gets passed to a `Router` type which will need to be created and handle the business logic, returning a `type/Response`. The `type/Response` gets serialised by the `serialiser/Response` ready to be sent back via the `Server`.
- [ ] Client
: Create a `Client` struct which creates a `transport/Tcp`, uses the `serialiser/Request` to send a `type/Request` and subsequently uses the `parser/Response` to form a response from the received data. Additionally, create a `Url` type (under the `type` module) which the `Client` utilises which parses common url patterns.
- [ ] Binary
: Create a binary server application which serves html pages in the _`public/`_ directory and it's children.

### Doing

### Done
- [x] Common
: Create the shared types used by the server and client side of the library. This includes a `Version` enum, `Method` enum, `Headers` struct, `Url` struct, `Request` struct and `Response` struct under the `common` module.
