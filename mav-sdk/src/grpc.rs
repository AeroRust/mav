pub use options::AsyncType;

#[path = "grpc/mavsdk.options.rs"]
mod options;

#[path = "grpc/action/mavsdk.rpc.action.rs"]
pub mod action;

#[path = "grpc/calibration/mavsdk.rpc.calibration.rs"]
pub mod calibration;

#[path = "grpc/camera/mavsdk.rpc.camera.rs"]
pub mod camera;

#[path = "grpc/core/mavsdk.rpc.core.rs"]
pub mod core;

#[path = "grpc/geofence/mavsdk.rpc.geofence.rs"]
pub mod geofence;

#[path = "grpc/gimbal/mavsdk.rpc.gimbal.rs"]
pub mod gimbal;

#[path = "grpc/info/mavsdk.rpc.info.rs"]
pub mod info;

#[path = "grpc/mission/mavsdk.rpc.mission.rs"]
pub mod mission;

#[path = "grpc/mocap/mavsdk.rpc.mocap.rs"]
pub mod mocap;

#[path = "grpc/offboard/mavsdk.rpc.offboard.rs"]
pub mod offboard;

#[path = "grpc/param/mavsdk.rpc.param.rs"]
pub mod param;

#[path = "grpc/shell/mavsdk.rpc.shell.rs"]
pub mod shell;

#[path = "grpc/telemetry/mavsdk.rpc.telemetry.rs"]
pub mod telemetry;
