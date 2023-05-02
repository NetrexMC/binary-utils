# binary-util
A panic-free binary utility crate to read/write binary streams over the wire.

BinaryUtils provides the following features:

* [`binary_util::io`](https://docs.rs/binary-util/latest/binary_util/io), to read and write to streams manually.
* [`binary_util::interfaces`](https://docs.rs/binary-util/latest/binary_util/interfaces), to allow automation of reading data structures.
* [`binary_util::BinaryIo`](https://docs.rs/binary-util-derive/latest), to automatically implement [`binary_util::interfaces::Reader`](https://docs.rs/binary-util/latest/binary_util/interfaces)
  and [`binary_util::interfaces::Writer`](https://docs.rs/binary-util/latest/binary_util/interfaces) .

# Getting Started

Binary Utils is available on [crates.io](https://crates.io/crates/binary_util), add the following to your `Cargo.toml`:

```toml
[dependencies]
binary_util = "0.3.0"
```

Optionally, if you wish to remove the `derive` feature, you can add the following to your `Cargo.toml`:

```toml
[dependencies]
binary_util = { version = "0.3.0", default-features = false }
```

To explicitly enable derive, you can use:

```toml
[dependencies]
binary_util = { version = "0.3.0", default-features = false, features = ["derive"] }
```