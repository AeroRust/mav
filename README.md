# MAV

## MAV-SDK

A [`MAVSDK`](https://github.com/mavlink/MAVSDK) gRPC client ( HTTP/2 ) for communicating with a drone, built from `proto` files using `tonic`.

You need to be running `MAVSDK` Server (see https://github.com/mavlink/MAVSDK/releases).

### Run PX4, Gazebo and MAVSDK Server with docker-compose

```bash
docker-compose up --detach
```

### Take it to the skies

```
cargo run -p mav_sdk --example takeoff
```
