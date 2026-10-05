# Habanero Roadmap

## Version 0.1

### Todo
- [ ] Connection
: Create a `Connection` trait, and `Http1` which implements it under the `server/connection` module. Additionally, create a `Connection` and a `Http1` which implements it under the `client/connection` module. The connection types wrap a `Transport` type, which will allow for longer lasting and multiplexed connections at a future stage.
- [ ] Server
: Create a `Server` struct which wraps incoming requests into the `transport/Tcp` type. This should use a `codec/Http1` to obtain a `common/Request` which gets passed to a `Router` type which will need to be created and handle the business logic, returning a `common/Response`. The `common/Response` gets serialised by the `codec/Http1` ready to be sent back via the `Server`.
- [ ] Client
: Create a `Client` struct which creates a `transport/Tcp`, uses the `codec/Http1` to send a `common/Request` and subsequently uses the `codec/Http1` to form a `common/Response` from the received data.
- [ ] Binary
: Create a binary server application which serves html pages in the _`public/`_ directory and it's children.

### Doing
- [/] Codec
: Create a `Http1` codec under the `codec` module. This is in charge of serialising and parsing both `common/Request` and `common/Response` from and into byte slices.

### Done
- [x] Transport
: Create the transport layer, including a `Transport` trait and `Tcp` struct implementation under the `transport` module.
- [x] Common
: Create the shared types used by the server and client side of the library. This includes a `Version` enum, `Method` enum, `Headers` struct, `Url` struct, `Request` struct and `Response` struct under the `common` module.
