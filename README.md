# Overview

This program will query a `HTTP`/`HTTPS` endpoint using `GET` until the status return code is `200` (`OK`). 

# Build Instructions

Debug Build
```
cargo build
```

Prod Build
```
cargo build --release
```

Builds can be found in the `target/debug` and `target/release` dirs for the debug and prod builds respectively.

# Run Instructions

```
./queryendpoint <endpoint>
```
