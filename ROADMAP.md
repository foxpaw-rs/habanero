# Habanero Roadmap

## Version 0.2
### Todo
- [ ] Cryptography
: Implement cryptographic primitives to begin to support HTTPS. Initially, support ChaCha20-Poly1305 as a cipher, SHA256 as a hash function and ECDHE for secure key generation and signature verification.
- [ ] Certificate
: Implement a certificate parser to verify the ASN.1 SSL certificate and verify it against a certificate authority.
- [ ] TLS Handshake
: Implement the TLS handshake protocol to initiate a secure transmission.
- [ ] TLS Records
: Implement TLS record chunking, and additionally, create a `transport/Tls` to use secure, encyrpted communication for the `Client` and `Server`
- [ ] Enhanced Cryptography
: Additionally support AES-GCM as a cipher and SHA512 as a hash function and support the selection during the handshake process.

### Doing

### Done


## Version 0.1
### Done
- [x] Binary
: Create a binary server application which serves html pages in the _`public/`_ directory and it's children.
- [x] Client
: Create a `Client` struct which creates a `transport/Tcp`, uses the `codec/Http1` to send a `common/Request` and subsequently uses the `codec/Http1` to form a `common/Response` from the received data.
- [x] Server
: Create a `Server` struct which wraps incoming requests into the `transport/Tcp` type. This should use a `codec/Http1` to obtain a `common/Request` which gets passed to a `Router` type which will need to be created and handle the business logic, returning a `common/Response`. The `common/Response` gets serialised by the `codec/Http1` ready to be sent back via the `Server`.
- [x] Connection
: Create a `Client` trait, and `Http1Client` which implements it under the `connection` module. Additionally, create a `Server` and a `Http1Server` which implements it under the `connection` module. The connection types wrap a `Transport` type, which will allow for longer lasting and multiplexed connections at a future stage.
- [x] Codec
: Create a `Http1` codec under the `codec` module. This is in charge of serialising and parsing both `common/Request` and `common/Response` from and into byte slices.
- [x] Transport
: Create the transport layer, including a `Transport` trait and `Tcp` struct implementation under the `transport` module.
- [x] Common
: Create the shared types used by the server and client side of the library. This includes a `Version` enum, `Method` enum, `Headers` struct, `Url` struct, `Request` struct and `Response` struct under the `common` module.
