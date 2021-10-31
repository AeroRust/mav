#![warn(
    missing_debug_implementations,
    // missing_docs,
    rust_2018_idioms,
    // unreachable_pub
)]
#![deny(unused_must_use)]
#![cfg_attr(docsrs, deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! mav-sdk is a WIP gRPC (HTTP/2 enabled) client/server for communication with the MAVLink protocol.
//! 
//! It is used to connect to drones and other vehicles that use MAVLink.
//! Well-known autopilots that support MAVLink are [PX4] & [ArduPilot].
//! 
//! 
//! ## Features:
//! - `serde` - Enables `serde` for (de)serializations of `Response`s, `Request`s and types used inside. (WIP)
//! 
//! [PX4]: https://px4.io/
//! [ArduPilot]: https://ardupilot.org

/// Generated MAVSDK services for client/server implementations.
pub mod grpc;

/// A result using [`tonic::transport::Error`].
pub type Result<T> = std::result::Result<T, tonic::transport::Error>;

use self::grpc::{
    action::ActionServiceClient, info::InfoServiceClient,
    mocap::mocap_service_client::MocapServiceClient, telemetry::TelemetryServiceClient,
};

use grpc::{
    calibration::CalibrationServiceClient, camera::CameraServiceClient, core::CoreServiceClient,
    geofence::GeofenceServiceClient, gimbal::GimbalServiceClient, mission::MissionServiceClient,
    offboard::OffboardServiceClient, param::ParamServiceClient, shell::ShellServiceClient,
};

use tonic::transport::{Channel, Endpoint};

/// HTTP/2 server & client transport
pub mod transport {
    /// Re-export Channel
    pub use tonic::transport::{Channel, Error};
}

pub const DEFAULT_URL: &str = "http://0.0.0.0:14540";
pub const OFFBOARD_PORT: u16 = 14540;

#[derive(Debug, Clone)]
pub struct Drone {
    pub action: ActionServiceClient<Channel>,
    pub calibration: CalibrationServiceClient<Channel>,
    pub camera: CameraServiceClient<Channel>,
    pub core: CoreServiceClient<Channel>,
    pub geofence: GeofenceServiceClient<Channel>,
    pub gimbal: GimbalServiceClient<Channel>,
    pub info: InfoServiceClient<Channel>,
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
            mission: MissionServiceClient::new(channel.clone()),
            mocap: MocapServiceClient::new(channel.clone()),
            offboard: OffboardServiceClient::new(channel.clone()),
            param: ParamServiceClient::new(channel.clone()),
            shell: ShellServiceClient::new(channel.clone()),
            telemetry: TelemetryServiceClient::new(channel),
        })
    }
}
