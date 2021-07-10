#![allow(rustdoc::bare_urls)]

pub use options::AsyncType;


#[path = "grpc/mavsdk.options.rs"]
mod options;

#[path = "grpc/mavsdk.rpc.action.rs"]
mod mavsdk_action;

pub mod action {
    
    pub use super::mavsdk_action::*;
    pub use action_service_client::ActionServiceClient;
    pub use action_service_server::{ActionServiceServer, ActionService};
}

#[path = "grpc/mavsdk.rpc.calibration.rs"]
mod mavsdk_calibration;

pub mod calibration {
    pub use super::mavsdk_calibration::*;

    pub use calibration_service_client::CalibrationServiceClient;
    pub use calibration_service_server::{CalibrationServiceServer, CalibrationService};
}

#[path = "grpc/mavsdk.rpc.camera.rs"]
pub mod camera;

#[path = "grpc/mavsdk.rpc.core.rs"]
pub mod core;

#[path = "grpc/mavsdk.rpc.geofence.rs"]
pub mod geofence;

#[path = "grpc/mavsdk.rpc.gimbal.rs"]
pub mod gimbal;

#[path = "grpc/mavsdk.rpc.info.rs"]
mod mavsdk_info;

pub mod info {
    pub use super::mavsdk_info::*;
    
    pub use info_service_client::InfoServiceClient;
    pub use info_service_server::{InfoService, InfoServiceServer};
}

#[path = "grpc/mavsdk.rpc.mission.rs"]
pub mod mission;

#[path = "grpc/mavsdk.rpc.mocap.rs"]
pub mod mocap;

#[path = "grpc/mavsdk.rpc.offboard.rs"]
pub mod offboard;

#[path = "grpc/mavsdk.rpc.param.rs"]
pub mod param;

#[path = "grpc/mavsdk.rpc.shell.rs"]
pub mod shell;

#[path = "grpc/mavsdk.rpc.telemetry.rs"]
mod mavsdk_telemetry;

pub mod telemetry {
    pub use super::mavsdk_telemetry::*;
    
    pub use telemetry_service_client::TelemetryServiceClient;
    pub use telemetry_service_server::{TelemetryServiceServer, TelemetryService};
}