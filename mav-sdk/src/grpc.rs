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

#[path = "grpc/mavsdk.rpc.action_server.rs"]
mod mavsdk_action_server;

pub mod action_server {
    pub use super::mavsdk_action_server::*;

    pub use action_server_service_client::ActionServerServiceClient;
    pub use action_server_service_server::{ActionServerService, ActionServerServiceServer};
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

#[path = "grpc/mavsdk.rpc.failure.rs"]
mod mavsdk_failure;

pub mod failure {
    pub use super::mavsdk_failure::*;

    pub use failure_service_client::FailureServiceClient;
    pub use failure_service_server::{FailureService, FailureServiceServer};
}

#[path = "grpc/mavsdk.rpc.follow_me.rs"]
mod mavsdk_follow_me;

pub mod follow_me {
    pub use super::mavsdk_follow_me::*;

    pub use follow_me_service_client::FollowMeServiceClient;
    pub use follow_me_service_server::{FollowMeService, FollowMeServiceServer};
}

#[path = "grpc/mavsdk.rpc.ftp.rs"]
mod mavsdk_ftp;

pub mod ftp {
    pub use super::mavsdk_ftp::*;

    pub use ftp_service_client::FtpServiceClient;
    pub use ftp_service_server::{FtpService, FtpServiceServer};
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

#[path = "grpc/mavsdk.rpc.log_files.rs"]
mod mavsdk_log_files;

pub mod log_files {
    pub use super::mavsdk_log_files::*;

    pub use log_files_service_client::LogFilesServiceClient;
    pub use log_files_service_server::{LogFilesService, LogFilesServiceServer};
}

#[path = "grpc/mavsdk.rpc.manual_control.rs"]
mod mavsdk_manual_control;

pub mod manual_control {
    pub use super::mavsdk_manual_control::*;

    pub use manual_control_service_client::ManualControlServiceClient;
    pub use manual_control_service_server::{ManualControlService, ManualControlServiceServer};
}

#[path = "grpc/mavsdk.rpc.mission.rs"]
mod mavsdk_mission;

pub mod mission {
    pub use super::mavsdk_mission::*;

    pub use mission_service_client::MissionServiceClient;
    pub use mission_service_server::{MissionService, MissionServiceServer};
}

#[path = "grpc/mavsdk.rpc.mission_raw.rs"]
mod mavsdk_mission_raw;

pub mod mission_raw {
    pub use super::mavsdk_mission_raw::*;

    pub use mission_raw_service_client::MissionRawServiceClient;
    pub use mission_raw_service_server::{MissionRawService, MissionRawServiceServer};
}

#[path = "grpc/mavsdk.rpc.mission_raw_server.rs"]
mod mavsdk_mission_raw_server;

pub mod mission_raw_server {
    pub use super::mavsdk_mission_raw_server::*;

    pub use mission_raw_server_service_client::MissionRawServerServiceClient;
    pub use mission_raw_server_service_server::{MissionRawServerService, MissionRawServerServiceServer};
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

#[path = "grpc/mavsdk.rpc.param_server.rs"]
mod mavsdk_param_server;

pub mod param_server {
    pub use super::mavsdk_param_server::*;

    pub use param_server_service_client::ParamServerServiceClient;
    pub use param_server_service_server::{ParamServerService, ParamServerServiceServer};
}

#[path = "grpc/mavsdk.rpc.server_utility.rs"]
mod mavsdk_server_utility;

pub mod server_utility {
    pub use super::mavsdk_server_utility::*;

    pub use server_utility_service_client::ServerUtilityServiceClient;
    pub use server_utility_service_server::{ServerUtilityService, ServerUtilityServiceServer};
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
#[path = "grpc/mavsdk.rpc.telemetry_server.rs"]
mod mavsdk_telemetry_server;

pub mod telemetry_server {
    pub use super::mavsdk_telemetry_server::*;

    pub use telemetry_server_service_client::TelemetryServerServiceClient;
    pub use telemetry_server_service_server::{TelemetryServerService, TelemetryServerServiceServer};
}

#[path = "grpc/mavsdk.rpc.tracking_server.rs"]
mod mavsdk_tracking_server;

pub mod tracking_server {
    pub use super::mavsdk_tracking_server::*;

    pub use tracking_server_service_client::TrackingServerServiceClient;
    pub use tracking_server_service_server::{TrackingServerService, TrackingServerServiceServer};
}

#[path = "grpc/mavsdk.rpc.transponder.rs"]
mod mavsdk_transponder;

pub mod transponder {
    pub use super::mavsdk_transponder::*;

    pub use transponder_service_client::TransponderServiceClient;
    pub use transponder_service_server::{TransponderService, TransponderServiceServer};
}

#[path = "grpc/mavsdk.rpc.tune.rs"]
mod mavsdk_tune;

pub mod tune {
    pub use super::mavsdk_tune::*;

    pub use tune_service_client::TuneServiceClient;
    pub use tune_service_server::{TuneService, TuneServiceServer};
}