#![allow(rustdoc::bare_urls)]

pub use options::AsyncType;

#[path = "grpc/mavsdk.options.rs"]
mod options;

#[path = "grpc/mavsdk.rpc.action.rs"]
mod mavsdk_action;

pub mod action {
    pub use super::mavsdk_action::*;

    pub use action_service_client::ActionServiceClient;
    pub use action_service_server::{ActionService, ActionServiceServer};
}

#[path = "grpc/mavsdk.rpc.calibration.rs"]
mod mavsdk_calibration;

pub mod calibration {
    pub use super::mavsdk_calibration::*;

    pub use calibration_service_client::CalibrationServiceClient;
    pub use calibration_service_server::{CalibrationService, CalibrationServiceServer};
}

#[path = "grpc/mavsdk.rpc.camera.rs"]
mod mavsdk_camera;

pub mod camera {
    pub use super::mavsdk_camera::*;

    pub use camera_service_client::CameraServiceClient;
    pub use camera_service_server::{CameraService, CameraServiceServer};
}

#[path = "grpc/mavsdk.rpc.core.rs"]
mod mavsdk_core;

pub mod core {
    pub use super::mavsdk_core::*;

    pub use core_service_client::CoreServiceClient;
    pub use core_service_server::{CoreService, CoreServiceServer};
}

#[path = "grpc/mavsdk.rpc.geofence.rs"]
mod mavsdk_geofence;

pub mod geofence {
    pub use super::mavsdk_geofence::*;

    pub use geofence_service_client::GeofenceServiceClient;
    pub use geofence_service_server::{GeofenceService, GeofenceServiceServer};
}

#[path = "grpc/mavsdk.rpc.gimbal.rs"]
mod mavsdk_gimbal;

pub mod gimbal {
    pub use super::mavsdk_gimbal::*;

    pub use gimbal_service_client::GimbalServiceClient;
    pub use gimbal_service_server::{GimbalService, GimbalServiceServer};
}

#[path = "grpc/mavsdk.rpc.info.rs"]
mod mavsdk_info;

pub mod info {
    pub use super::mavsdk_info::*;

    pub use info_service_client::InfoServiceClient;
    pub use info_service_server::{InfoService, InfoServiceServer};
}

#[path = "grpc/mavsdk.rpc.mission.rs"]
mod mavsdk_mission;

pub mod mission {
    pub use super::mavsdk_mission::*;

    pub use mission_service_client::MissionServiceClient;
    pub use mission_service_server::{MissionService, MissionServiceServer};
}

#[path = "grpc/mavsdk.rpc.mocap.rs"]
mod mavsdk_mocap;

pub mod mocap {
    pub use super::mavsdk_mocap::*;

    pub use mocap_service_client::MocapServiceClient;
    pub use mocap_service_server::{MocapService, MocapServiceServer};
}

#[path = "grpc/mavsdk.rpc.offboard.rs"]
mod mavsdk_offboard;

pub mod offboard {
    pub use super::mavsdk_offboard::*;

    pub use offboard_service_client::OffboardServiceClient;
    pub use offboard_service_server::{OffboardService, OffboardServiceServer};
}

#[path = "grpc/mavsdk.rpc.param.rs"]
mod mavsdk_param;

pub mod param {
    pub use super::mavsdk_param::*;

    pub use param_service_client::ParamServiceClient;
    pub use param_service_server::{ParamService, ParamServiceServer};
}

#[path = "grpc/mavsdk.rpc.shell.rs"]
mod mavsdk_shell;

pub mod shell {
    pub use super::mavsdk_shell::*;

    pub use shell_service_client::ShellServiceClient;
    pub use shell_service_server::{ShellService, ShellServiceServer};
}

#[path = "grpc/mavsdk.rpc.telemetry.rs"]
mod mavsdk_telemetry;

pub mod telemetry {
    pub use super::mavsdk_telemetry::*;

    pub use telemetry_service_client::TelemetryServiceClient;
    pub use telemetry_service_server::{TelemetryService, TelemetryServiceServer};
}
