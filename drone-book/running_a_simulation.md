# Running a simulation

This is the easiest way to get started without the need of additional hardware,
apart from you laptop.

## Software requirements

Operating system: Linux / MacOS(?)

(?) - Currently being tested

- Docker and Docker-compose for simulation (Install [Docker][install-docker] & [docker-compose][install-docker-compose])
- Rust for `cargo run` (Install the [Rust][install-rust] programming language)
- Git
- `ssh` key set on your GitHub profile, see [Connecting to GitHub with SSH][github-ssh]
- QGroundControl (optional)

1. `git clone git@github.com:AeroRust/mav.git && cd mav`

    1.1. Install `protoc` - For [Windows](https://www.geeksforgeeks.org/how-to-install-protocol-buffers-on-windows/), [Linux & mac](https://grpc.io/docs/protoc-installation/)

    1.2. `git submodule init && git submodule update`


2. Run PX4, Gazebo and MAVSDK Server with `docker-compose`


```
docker-compose up --detach
```

**Tools:** `PX4` (autopilot), `Gazebo` (a tool for simulations) and `MAVSDK` server are all open-source tools and later we will get to know what each tool does.

For the time being, however, all you need to know is that this is how we simulate a drone using Docker containers.

3. Take it to the skies

```
cargo run -p mav-sdk --example takeoff
```


[install-rust]: https://rustup.rs/
[install-docker]: https://docs.docker.com/engine/install/
[install-docker-compose]: https://docs.docker.com/compose/install/
[github-ssh]: https://docs.github.com/en/authentication/connecting-to-github-with-ssh