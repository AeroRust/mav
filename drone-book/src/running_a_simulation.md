# Running a simulation

This is the easiest way to get started without the need of additional hardware,
apart from you laptop.

## Software requirements

Operating system: Linux / MacOS(?)

(?) - Currently being tested

- Docker and Docker-compose for simulation (Install Docker & docker-compose)[]
- Rust for `cargo run` (Install Rust)[]
- Git
- QGroundControl (optional)

1. `git clone https://github.com/AeroRust/mav && cd mav`
0.1. `git submodule init`


1. Run PX4, Gazebo and MAVSDK Server with docker-compose


```
docker-compose up --detach
```

2. Take it to the skies

```
cargo run -p mav_sdk --example takeoff
```