# `did-playground`

A small library to handle subjects with decentralized identifiers (DIDs).

The library is built on top of [identity-iota][] supporting the [Iota identity framework (version 0.6)][framework-v0.6].

Currently the library supports the following operations:

* Management of secret keys through [Stronghold][]. To this end, the library
  uses two environment variables:
  * `STRONGHOLD_PASSWORD` [required]: This is the password to the local storage.
  * `STRONGHOLD_PATH` [optional]: This is the path to the local storage file. If unset defaults to `./key-manager.hodl`. 
* Creation of DIDs and the associated documents.
* Publication on the IOTA distributed ledger
* Authentication handshake between subjects through HTTP

The library is used in a cli application that illustrates all the operations above.

## Run the application

The application does the following

* Creates the DIDs and associated documents for two subjects.
* Publishes the documents on IOTA mainnet.
* Implements an authentication handshake between the two subjects through HTTP.

### Setup

* [Install Rust][install-rust]
* Minimum supported Rust version: `1.64`

### Command

```
RUST_LOG=info STRONGHOLD_PASSWORD=secret cargo run
```

## Limitations

* Currently the API communications only with IOTA `mainnet`. This can be handled by setting the `identity_iota::client::Client` while creating the subjects.
* The resolver used for retrieving DID documents is also configured to connect with the `mainnet`. This should be also dynamically set in the future, and bound to the network that the subjects are using.
* The key used for authentication through asymmetric encryption, is the key used in the default signing method configured in the DID document. In the future we could add more granular control over similar operation by providing dedicated [verification method][verification-method] for authentication.
* The web interface is only exposed through the library. In the future this should be part of the DID document, by [adding a service][adding-a-service].

[identity-iota]: https://github.com/iotaledger/identity.rs/tree/support/v0.6
[install-rust]: https://www.rust-lang.org/tools/install
[framework-v0.6]: https://wiki.iota.org/identity.rs/introduction/
[Stronghold]: https://wiki.iota.org/stronghold.rs/welcome
[verification-method]: https://wiki.iota.org/identity.rs/concepts/decentralized_identifiers/update/#adding-verification-methods
[adding-a-service]: https://wiki.iota.org/identity.rs/concepts/decentralized_identifiers/update/#adding-a-service
