# libcontract-sys

This crate contains hand-crafted native bindings for
[libcontract(3LIB)](https://illumos.org/man/3LIB/libcontract) that do not
require bindgen, depending only on
[Committed](https://illumos.org/man/7/attributes) parts of the library API and
ABI.  This library allows programmatic access to the **contract** subsystem on
illumos systems, as described in
[contract(5)](https://illumos.org/man/5/contract),
[libcontract(3LIB)](https://illumos.org/man/3LIB/libcontract), and
[process(5)](https://illumos.org/man/5/process).

## License

Unless otherwise noted, all components are licensed under the Mozilla Public
License Version 2.0.
