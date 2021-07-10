pub use options::AsyncType;

#[path = "grpc/mavsdk.options.rs"]
mod options;

#[path = "grpc/mavsdk.rpc.action.rs"]
pub mod action;

#[path = "grpc/mavsdk.rpc.calibration.rs"]
pub mod calibration;

#[path = "grpc/mavsdk.rpc.camera.rs"]
pub mod camera;

#[path = "grpc/mavsdk.rpc.core.rs"]
pub mod core;

#[path = "grpc/mavsdk.rpc.geofence.rs"]
pub mod geofence;

#[path = "grpc/mavsdk.rpc.gimbal.rs"]
pub mod gimbal;

#[path = "grpc/mavsdk.rpc.info.rs"]
pub mod info;

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
pub mod telemetry;
