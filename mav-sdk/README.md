# mav-sdk
[![Crate](https://img.shields.io/crates/v/mav-sdk.svg)](https://crates.io/crates/mav-sdk)
[![API](https://docs.rs/mav-sdk/badge.svg)](https://docs.rs/mav-sdk)

> _MAVSDK_ is a collection of libraries for various programming languages to interface with [_MAVLink_](https://mavlink.io/en/) systems such as drones, cameras or ground systems.
> 
> The libraries provides a simple API for managing one or more vehicles, providing programmatic access to vehicle information and telemetry, and control over missions, movement and other operations.
>
> The libraries can be used onboard a drone on a companion computer or on the ground for a ground station or mobile device.

- Source: _https://mavsdk.mavlink.io_

## Compatibility

Compatible with [MAVSDK v1.0](./mavsdk-proto/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mav-sdk = "0.2.0"
```

### Features:

- `with_serde` - Enables `serde` for (de)serializations of `Response`s, `Request`s and types used inside.

## Documentation:

Documentation can be found on [docs.rs/mav-sdk](https://docs.rs/mav-sdk) [![API](https://docs.rs/mav-sdk/badge.svg)](https://docs.rs/mav-sdk)

## Examples:
Please refer to the [`examples`](examples) folder which will show you usage of the crate.

# License

Mav is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.