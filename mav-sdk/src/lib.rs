#![warn(
    missing_debug_implementations,
    // missing_docs,
    rust_2018_idioms,
    // unreachable_pub
)]
#![deny(unused_must_use)]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! mav-sdk is a [gRPC] HTTP/2 client for communicating with a [MAVSDK] server.
//!
//! [MAVSDK] uses the [MAVLink] protocol to communicate with drones and other vehicles that use it.
//!
//! Well-known autopilots that support the [MAVLink] protocol are [PX4] & [ArduPilot].
//!
//!
//! ## Features:
//! - `serde` - Enables `serde` for (de)serializations of `Response`s, `Request`s and types used inside.
//!
//! [PX4]: https://px4.io
//! [ArduPilot]: https://ardupilot.org
//! [gRPC]: https://grpc.io
//! [MAVLink]: https://mavlink.io
//! [MAVSDK]: https://mavsdk.mavlink.io

/// Generated MAVSDK services for client/server implementations.
pub mod grpc;

/// A result using [`tonic::transport::Error`].
pub type Result<T> = std::result::Result<T, tonic::transport::Error>;

use grpc::{
    action::ActionServiceClient, calibration::CalibrationServiceClient,
    camera::CameraServiceClient, core::CoreServiceClient, geofence::GeofenceServiceClient,
    gimbal::GimbalServiceClient, info::InfoServiceClient,
    manual_control::manual_control_service_client::ManualControlServiceClient,
    mission::MissionServiceClient, mocap::mocap_service_client::MocapServiceClient,
    offboard::OffboardServiceClient, param::ParamServiceClient, shell::ShellServiceClient,
    telemetry::TelemetryServiceClient,
};

use tonic::transport::{Channel, Endpoint};

/// HTTP/2 server & client transport
pub mod transport {
    /// Re-export tonic's transport [`Channel`] & [`Error`].
    pub use tonic::transport::{Channel, Error};
}

pub const DEFAULT_URL: &str = "http://0.0.0.0:14540";
/// MAVLink ports
///
/// Check the [PX4 documentation](https://docs.px4.io/master/en/simulation/#default-px4-mavlink-udp-ports) for more details.
///
pub mod ports {
    /// Default Offboard port `14540`.
    ///
    /// Note: Ports from `14540` - `14549` are used for API or Offboard communication.
    pub const OFFBOARD_PORT: u16 = 14540;

    /// Port used by Ground control station
    pub const GROUND_CONTROL: u16 = 14550;

    /// Port used by [QGroundControl](http://qgroundcontrol.com)
    pub const QGROUND_CONTROL: u16 = GROUND_CONTROL;

    /// Port used by simulation tools like [`Gazebo`](https://gazebosim.org)
    pub const SIMULATOR: u16 = 14560;
}

#[derive(Debug, Clone)]
pub struct Drone {
    pub action: ActionServiceClient<Channel>,
    pub calibration: CalibrationServiceClient<Channel>,
    pub camera: CameraServiceClient<Channel>,
    pub core: CoreServiceClient<Channel>,
    pub geofence: GeofenceServiceClient<Channel>,
    pub gimbal: GimbalServiceClient<Channel>,
    pub info: InfoServiceClient<Channel>,
    pub manual_control: ManualControlServiceClient<Channel>,
    pub mission: MissionServiceClient<Channel>,
    pub mocap: MocapServiceClient<Channel>,
    pub offboard: OffboardServiceClient<Channel>,
    pub param: ParamServiceClient<Channel>,
    pub shell: ShellServiceClient<Channel>,
    pub telemetry: TelemetryServiceClient<Channel>,
}

impl Drone {
    /// Connects to a MAVSDK (gRPC) server with a valid URI
    ///
    pub async fn connect(url: &'static str) -> Result<Self> {
        // TODO: Add timeout
        // TODO: Evaluate adding concurrency limit for a request
        let channel = Endpoint::new(url)?.connect().await?;

        Ok(Self {
            action: ActionServiceClient::new(channel.clone()),
            calibration: CalibrationServiceClient::new(channel.clone()),
            camera: CameraServiceClient::new(channel.clone()),
            core: CoreServiceClient::new(channel.clone()),
            geofence: GeofenceServiceClient::new(channel.clone()),
            gimbal: GimbalServiceClient::new(channel.clone()),
            info: InfoServiceClient::new(channel.clone()),
            manual_control: ManualControlServiceClient::new(channel.clone()),
            mission: MissionServiceClient::new(channel.clone()),
            mocap: MocapServiceClient::new(channel.clone()),
            offboard: OffboardServiceClient::new(channel.clone()),
            param: ParamServiceClient::new(channel.clone()),
            shell: ShellServiceClient::new(channel.clone()),
            telemetry: TelemetryServiceClient::new(channel),
        })
    }
}
