# MAV

## The Rust book for Drones

You can run the Rust book for drones by using `mdbook`:

`mdbook serve`

## MAV-SDK

A [`MAVSDK`](https://github.com/mavlink/MAVSDK) gRPC client ( HTTP/2 ) for communicating with a drone, built from `proto` files using `tonic`.

You need to be running the `MAVSDK` Server (see https://github.com/mavlink/MAVSDK/releases) in order to use this crate.

### Run PX4, Gazebo and MAVSDK Server with docker-compose

```bash
docker-compose up --detach
```

### Take it to the skies

```
cargo run -p mav-sdk --example takeoff
```

### Development
Prerequisite:

1. Make sure you have SSH key set for your Github account.

  See Github articles:
 - [Generating a new SSH key and adding it to the ssh-agent](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent)
 - [Adding a new SSH key to your GitHub account](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)


Building the project:

1. Clone the repository and change dir:

  `git clone git@github.com:AeroRust/mav.git && cd mav`

2. Install `protoc`:

  * For [Linux & mac](https://grpc.io/docs/protoc-installation/)
  * For [Windows](https://www.geeksforgeeks.org/how-to-install-protocol-buffers-on-windows/)

3. Initialize the `mavsdk-proto` submodule:

  `git submodule init && git submodule update`

4. Build the project

  `cargo build`


## Simulation
### Gazebo with PX4


Useful documentation regarding the Gazebo simulation and the PX4 flight software:

- PX4 with Gazebo docs: https://dev.px4.io/master/en/simulation/gazebo.html
  - Setting up Wind speed: https://dev.px4.io/master/en/simulation/gazebo.html#change-wind-speed
  - GPS noise: https://dev.px4.io/master/en/simulation/gazebo.html#simulating-gps-noise
  - World location: https://dev.px4.io/master/en/simulation/gazebo.html#set-world-location
  - Survey camera (simulated [MAVLINK camera](https://mavlink.io/en/services/camera.html)) + geotagging: https://dev.px4.io/master/en/simulation/gazebo.html#simulated-survey-camera 
  - Video streaming: https://dev.px4.io/master/en/simulation/gazebo.html#video-streaming
    Supported only on the [`Typhoon H480`](https://dev.px4.io/master/en/simulation/gazebo_vehicles.html#typhoon-h480-hexrotor) frame and requires `Gstreamer 1.0`
- Creating a world with (SD Format is used by Gazebo): http://sdformat.org/tutorials?tut=spec_world&cat=specification
- Physics and other setup like wind:
  - SD Format - http://sdformat.org/spec?ver=1.4&elem=physics#physics_gravity
  - "Windy world" file from PX4 - Gazebo with (SITL): https://github.com/PX4/PX4-SITL_gazebo/blob/master/worlds/windy.world#L15-L26

### ArduPilot


- Advanced configuration - Complete Parameter List: https://ardupilot.org/copter/docs/parameters.html#parameters

#### SITL (Software in the loop)

- Vehicle frame types to choose from: https://ardupilot.org/dev/docs/using-sitl-for-ardupilot-testing.html#frame-types
- Starting SITL without MAVProxy (`--no-mavproxy`): https://ardupilot.org/dev/docs/using-sitl-for-ardupilot-testing.html#connecting-other-additional-ground-stations
- Using STIL: https://ardupilot.org/dev/docs/using-sitl-for-ardupilot-testing.html
- SITL Serial mapping: https://ardupilot.org/dev/docs/sitl-serial-mapping.html
- SITL Parameters: https://ardupilot.org/dev/docs/sitl-parameters.html

Docker images:
- https://github.com/radarku/ardupilot-sitl-docker
- https://github.com/gmyoungblood-parc/docker-alpine-ardupilot

#### Gazebo

Gazebo doesn't yet have built-in support for ArduPilot and it requires a plugin to be installed.

- https://ardupilot.org/dev/docs/using-gazebo-simulator-with-sitl.html

#### MAVLink

- MAVLink basics: https://ardupilot.org/dev/docs/mavlink-basics.html#mavlink-basics
- MAVLink Routing in ArduPilot: https://ardupilot.org/dev/docs/mavlink-routing-in-ardupilot.html

