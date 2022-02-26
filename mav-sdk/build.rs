#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::{collections::HashSet, path::PathBuf};

const PROTO_GIT_SUBMODULE: &str = "mavsdk-proto";
const MAVSDK_OPTIONS: &str = "mavsdk_options";

const PLUGINS: Lazy<HashSet<&str>> = Lazy::new(|| vec![
    "action",
    // TODO: Add derive structs for serde
    "action_server",
    "calibration",
    "camera",
    "core",
    // TODO: Add derive structs for serde
    "failure",
    // TODO: Add derive structs for serde
    "follow_me",
    // TODO: Add derive structs for serde
    "ftp",
    "geofence",
    "gimbal",
    "info",
    // TODO: Add derive structs for serde
    "log_files",
    // TODO: Add derive structs for serde
    "manual_control",
    "mission",
    // TODO: Add derive structs for serde
    "mission_raw",
    // TODO: Add derive structs for serde
    "mission_raw_server",
    "mocap",
    "offboard",
    "param",
    // TODO: Add derive structs for serde
    "param_server",
    // TODO: Add derive structs for serde
    "server_utility",
    "shell",
    "telemetry",
    // TODO: Add derive structs for serde
    "telemetry_server",
    // TODO: Add derive structs for serde
    "tracking_server",
    // TODO: Add derive structs for serde
    "transponder",
    // TODO: Add derive structs for serde
    "tune",
].into_iter().collect());

const COMMON_TYPE_NAMES: [&str; 1] = ["Result"];

const ACTION: [&str; 37] = [
    "ActionResult",
    "ArmRequest",
    "ArmResponse",
    "DisarmRequest",
    "DisarmResponse",
    "GetMaximumSpeedRequest",
    "GetMaximumSpeedResponse",
    "GetReturnToLaunchAltitudeRequest",
    "GetReturnToLaunchAltitudeResponse",
    "GetTakeoffAltitudeRequest",
    "GetTakeoffAltitudeResponse",
    "GotoLocationRequest",
    "GotoLocationResponse",
    "KillRequest",
    "KillResponse",
    "LandRequest",
    "LandResponse",
    "RebootRequest",
    "RebootResponse",
    "ReturnToLaunchRequest",
    "ReturnToLaunchResponse",
    "SetMaximumSpeedRequest",
    "SetMaximumSpeedResponse",
    "SetReturnToLaunchAltitudeRequest",
    "SetReturnToLaunchAltitudeResponse",
    "SetTakeoffAltitudeRequest",
    "SetTakeoffAltitudeResponse",
    "ShutdownRequest",
    "ShutdownResponse",
    "TakeoffRequest",
    "TakeoffResponse",
    "TerminateRequest",
    "TerminateResponse",
    "TransitionToFixedwingRequest",
    "TransitionToFixedwingResponse",
    "TransitionToMulticopterRequest",
    "TransitionToMulticopterResponse",
];

// TODO: Add derive structs for serde
const ACTION_SERVER: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

const CALIBRATION: [&str; 14] = [
    "CalibrateAccelerometerResponse",
    "CalibrateGimbalAccelerometerResponse",
    "CalibrateGyroResponse",
    "CalibrateLevelHorizonResponse",
    "CalibrateMagnetometerResponse",
    "CalibrationResult",
    "CancelRequest",
    "CancelResponse",
    "ProgressData",
    "SubscribeCalibrateAccelerometerRequest",
    "SubscribeCalibrateGimbalAccelerometerRequest",
    "SubscribeCalibrateGyroRequest",
    "SubscribeCalibrateLevelHorizonRequest",
    "SubscribeCalibrateMagnetometerRequest",
];

const CAMERA: [&str; 51] = [
    "CameraResult",
    "CaptureInfo",
    "CaptureInfoResponse",
    "CurrentSettingsResponse",
    "EulerAngle",
    "FormatStorageRequest",
    "FormatStorageResponse",
    "GetSettingRequest",
    "GetSettingResponse",
    "Information",
    "InformationResponse",
    "ModeResponse",
    "Option",
    "Position",
    "PossibleSettingOptionsResponse",
    "Result",
    "Quaternion",
    "SetModeRequest",
    "SetModeResponse",
    "SetSettingRequest",
    "SetSettingResponse",
    "Setting",
    "SettingOptions",
    "StartPhotoIntervalRequest",
    "StartPhotoIntervalResponse",
    "StartVideoRequest",
    "StartVideoResponse",
    "StartVideoStreamingRequest",
    "StartVideoStreamingResponse",
    "Status",
    "StorageStatus",
    "StatusResponse",
    "StopPhotoIntervalRequest",
    "StopPhotoIntervalResponse",
    "StopVideoRequest",
    "StopVideoResponse",
    "StopVideoStreamingRequest",
    "StopVideoStreamingResponse",
    "SubscribeCaptureInfoRequest",
    "SubscribeCurrentSettingsRequest",
    "SubscribeInformationRequest",
    "SubscribeModeRequest",
    "SubscribePossibleSettingOptionsRequest",
    "SubscribeStatusRequest",
    "SubscribeVideoStreamInfoRequest",
    "TakePhotoRequest",
    "TakePhotoResponse",
    "VideoStreamInfo",
    "VideoStreamInfoResponse",
    "VideoStreamSettings",
    "Mode",
];

static CORE: [&str; 6] = [
    "ConnectionState",
    "ConnectionStateResponse",
    "ListRunningPluginsRequest",
    "ListRunningPluginsResponse",
    "PluginInfo",
    "SubscribeConnectionStateRequest",
];

// TODO: Add derive structs for serde
const FAILURE: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const FOLLOW_ME: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const FTP: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

static GEOFENCE: [&str; 6] = [
    "GeofenceResult",
    "Point",
    "Polygon",
    "FenceType",
    "UploadGeofenceRequest",
    "UploadGeofenceResponse",
];

static GIMBAL: [&str; 8] = [
    "GimbalResult",
    "SetModeRequest",
    "SetModeResponse",
    "SetPitchAndYawRequest",
    "SetPitchAndYawResponse",
    "SetRoiLocationRequest",
    "SetRoiLocationResponse",
    "GimbalMode",
];

static INFO: [&str; 15] = [
    "FlightInfo",
    "GetFlightInformationRequest",
    "GetFlightInformationResponse",
    "GetIdentificationRequest",
    "GetIdentificationResponse",
    "GetProductRequest",
    "GetProductResponse",
    "GetSpeedFactorRequest",
    "GetSpeedFactorResponse",
    "GetVersionRequest",
    "GetVersionResponse",
    "Identification",
    "InfoResult",
    "Product",
    "Version",
];

// TODO: Add derive structs for serde
const LOG_FILES: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const MANUAL_CONTROL: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

static MISSION: [&str; 31] = [
    "CancelMissionDownloadRequest",
    "CancelMissionDownloadResponse",
    "CancelMissionUploadRequest",
    "CancelMissionUploadResponse",
    "ClearMissionRequest",
    "ClearMissionResponse",
    "DownloadMissionRequest",
    "DownloadMissionResponse",
    "GetReturnToLaunchAfterMissionRequest",
    "GetReturnToLaunchAfterMissionResponse",
    "ImportQgroundcontrolMissionRequest",
    "ImportQgroundcontrolMissionResponse",
    "IsMissionFinishedRequest",
    "IsMissionFinishedResponse",
    "MissionItem",
    "CameraAction",
    "MissionPlan",
    "MissionProgress",
    "MissionProgressResponse",
    "MissionResult",
    "PauseMissionRequest",
    "PauseMissionResponse",
    "SetCurrentMissionItemRequest",
    "SetCurrentMissionItemResponse",
    "SetReturnToLaunchAfterMissionRequest",
    "SetReturnToLaunchAfterMissionResponse",
    "StartMissionRequest",
    "StartMissionResponse",
    "SubscribeMissionProgressRequest",
    "UploadMissionRequest",
    "UploadMissionResponse",
];

// TODO: Add derive structs for serde
const MISSION_RAW: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const MISSION_RAW_SERVER: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

const MOCAP: [&str; 17] = [
    "AngleBody",
    "AngularVelocityBody",
    "AttitudePositionMocap",
    "Covariance",
    "MocapResult",
    "Odometry",
    "MavFrame",
    "PositionBody",
    "Quaternion",
    "SetAttitudePositionMocapRequest",
    "SetAttitudePositionMocapResponse",
    "SetOdometryRequest",
    "SetOdometryResponse",
    "SetVisionPositionEstimateRequest",
    "SetVisionPositionEstimateResponse",
    "SpeedBody",
    "VisionPositionEstimate",
];

static OFFBOARD: [&str; 28] = [
    "ActuatorControl",
    "ActuatorControlGroup",
    "Attitude",
    "AttitudeRate",
    "IsActiveRequest",
    "IsActiveResponse",
    "OffboardResult",
    "PositionNedYaw",
    "SetActuatorControlRequest",
    "SetActuatorControlResponse",
    "SetAttitudeRateRequest",
    "SetAttitudeRateResponse",
    "SetAttitudeRequest",
    "SetAttitudeResponse",
    "SetPositionNedRequest",
    "SetPositionNedResponse",
    "SetPositionVelocityNedRequest",
    "SetPositionVelocityNedResponse",
    "SetVelocityBodyRequest",
    "SetVelocityBodyResponse",
    "SetVelocityNedRequest",
    "SetVelocityNedResponse",
    "StartRequest",
    "StartResponse",
    "StopRequest",
    "StopResponse",
    "VelocityBodyYawspeed",
    "VelocityNedYaw",
];

static PARAM: [&str; 14] = [
    "AllParams",
    "FloatParam",
    "GetAllParamsRequest",
    "GetAllParamsResponse",
    "GetParamFloatRequest",
    "GetParamFloatResponse",
    "GetParamIntRequest",
    "GetParamIntResponse",
    "IntParam",
    "ParamResult",
    "SetParamFloatRequest",
    "SetParamFloatResponse",
    "SetParamIntRequest",
    "SetParamIntResponse",
];

// TODO: Add derive structs for serde
const PARAM_SERVER: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const SERVER_UTILITY: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

static SHELL: [&str; 5] = [
    "ReceiveResponse",
    "SendRequest",
    "SendResponse",
    "ShellResult",
    "SubscribeReceiveRequest",
];

const TELEMETRY: [&str; 131] = [
    "AccelerationFrd",
    "ActuatorControlTarget",
    "ActuatorControlTargetResponse",
    "ActuatorOutputStatus",
    "ActuatorOutputStatusResponse",
    "AngularVelocityBody",
    "AngularVelocityFrd",
    "ArmedResponse",
    "AttitudeAngularVelocityBodyResponse",
    "AttitudeEulerResponse",
    "AttitudeQuaternionResponse",
    "Battery",
    "BatteryResponse",
    "CameraAttitudeEulerResponse",
    "CameraAttitudeQuaternionResponse",
    "Covariance",
    "DistanceSensor",
    "DistanceSensorResponse",
    "EulerAngle",
    "FixedwingMetrics",
    "FixedwingMetricsResponse",
    "FlightModeResponse",
    "GetGpsGlobalOriginRequest",
    "GetGpsGlobalOriginResponse",
    "GpsGlobalOrigin",
    "GpsInfo",
    "GpsInfoResponse",
    "GroundTruth",
    "GroundTruthResponse",
    "Health",
    "HealthAllOkResponse",
    "HealthResponse",
    "HomeResponse",
    "Imu",
    "ImuResponse",
    "InAirResponse",
    "LandedStateResponse",
    "MagneticFieldFrd",
    "Odometry",
    "OdometryResponse",
    "MavFrame",
    "Position",
    "PositionBody",
    "PositionNed",
    "PositionResponse",
    "PositionVelocityNed",
    "PositionVelocityNedResponse",
    "Quaternion",
    "RcStatus",
    "RcStatusResponse",
    "SetRateActuatorControlTargetRequest",
    "SetRateActuatorControlTargetResponse",
    "SetRateActuatorOutputStatusRequest",
    "SetRateActuatorOutputStatusResponse",
    "SetRateAttitudeAngularVelocityBodyRequest",
    "SetRateAttitudeAngularVelocityBodyResponse",
    "SetRateAttitudeRequest",
    "SetRateAttitudeResponse",
    "SetRateBatteryRequest",
    "SetRateBatteryResponse",
    "SetRateCameraAttitudeQuaternionRequest",
    "SetRateCameraAttitudeQuaternionResponse",
    "SetRateCameraAttitudeRequest",
    "SetRateCameraAttitudeResponse",
    "SetRateDistanceSensorRequest",
    "SetRateDistanceSensorResponse",
    "SetRateFixedwingMetricsRequest",
    "SetRateFixedwingMetricsResponse",
    "SetRateGpsInfoRequest",
    "SetRateGpsInfoResponse",
    "SetRateGroundTruthRequest",
    "SetRateGroundTruthResponse",
    "SetRateHomeRequest",
    "SetRateHomeResponse",
    "SetRateImuRequest",
    "SetRateImuResponse",
    "SetRateInAirRequest",
    "SetRateInAirResponse",
    "SetRateLandedStateRequest",
    "SetRateLandedStateResponse",
    "SetRateOdometryRequest",
    "SetRateOdometryResponse",
    "SetRatePositionRequest",
    "SetRatePositionResponse",
    "SetRatePositionVelocityNedRequest",
    "SetRatePositionVelocityNedResponse",
    "SetRateRcStatusRequest",
    "SetRateRcStatusResponse",
    "SetRateUnixEpochTimeRequest",
    "SetRateUnixEpochTimeResponse",
    "SetRateVelocityNedRequest",
    "SetRateVelocityNedResponse",
    "StatusText",
    "StatusTextResponse",
    "SubscribeActuatorControlTargetRequest",
    "SubscribeActuatorOutputStatusRequest",
    "SubscribeArmedRequest",
    "SubscribeAttitudeAngularVelocityBodyRequest",
    "SubscribeAttitudeEulerRequest",
    "SubscribeAttitudeQuaternionRequest",
    "SubscribeBatteryRequest",
    "SubscribeCameraAttitudeEulerRequest",
    "SubscribeCameraAttitudeQuaternionRequest",
    "SubscribeDistanceSensorRequest",
    "SubscribeFixedwingMetricsRequest",
    "SubscribeFlightModeRequest",
    "SubscribeGpsInfoRequest",
    "SubscribeGroundTruthRequest",
    "SubscribeHealthAllOkRequest",
    "SubscribeHealthRequest",
    "SubscribeHomeRequest",
    "SubscribeImuRequest",
    "SubscribeInAirRequest",
    "SubscribeLandedStateRequest",
    "SubscribeOdometryRequest",
    "SubscribePositionRequest",
    "SubscribePositionVelocityNedRequest",
    "SubscribeRcStatusRequest",
    "SubscribeStatusTextRequest",
    "SubscribeUnixEpochTimeRequest",
    "SubscribeVelocityNedRequest",
    "TelemetryResult",
    "UnixEpochTimeResponse",
    "VelocityBody",
    "Velocity",
    "VelocityNed",
    "VelocityNedResponse",
    "FixType",
    "FlightMode",
    "LandedState",
    "StatusTextType",
];

// TODO: Add derive structs for serde
const TELEMETRY_SERVER: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const TRACKING_SERVER: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const TRANSPONDER: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

// TODO: Add derive structs for serde
const TUNE: Lazy<HashSet<&str>> = Lazy::new(|| vec![].into_iter().collect());

static DERIVE_SERDE_FOR: Lazy<HashSet<&str>> = Lazy::new(|| {
    // will match all messages (structs and enums)
    vec!["."].into_iter().collect()
    // COMMON_TYPE_NAMES
    //     .to_vec()
    //     .into_iter()
    //     .chain(ACTION.to_vec())
    //     .chain(ACTION_SERVER.clone().into_iter())
    //     .chain(CALIBRATION.to_vec())
    //     .chain(CAMERA.to_vec())
    //     .chain(CORE.to_vec())
    //     .chain(FAILURE.clone().into_iter())
    //     .chain(FOLLOW_ME.clone().into_iter())
    //     .chain(FTP.clone().into_iter())
    //     .chain(GEOFENCE.to_vec())
    //     .chain(GIMBAL.to_vec())
    //     .chain(INFO.to_vec())
    //     .chain(LOG_FILES.clone().into_iter())
    //     .chain(MANUAL_CONTROL.clone().into_iter())
    //     .chain(MISSION.to_vec())
    //     .chain(MISSION_RAW.clone().into_iter())
    //     .chain(MISSION_RAW_SERVER.clone().into_iter())
    //     .chain(MOCAP.to_vec())
    //     .chain(OFFBOARD.to_vec())
    //     .chain(PARAM.to_vec())
    //     .chain(PARAM_SERVER.clone().into_iter())
    //     .chain(SERVER_UTILITY.clone().into_iter())
    //     .chain(SHELL.to_vec())
    //     .chain(TELEMETRY.to_vec())
    //     .chain(TELEMETRY_SERVER.clone().into_iter())
    //     .chain(TRACKING_SERVER.clone().into_iter())
    //     .chain(TRANSPONDER.clone().into_iter())
    //     .chain(TUNE.clone().into_iter())
    //     .collect()
});

fn main() -> Result<(), std::io::Error> {
    let mavsdk_options_include = format!("{submodule}/protos", submodule = PROTO_GIT_SUBMODULE);

    // tonic_build(&plugins, mavsdk_options_include.into())

    tonic_build_single(&PLUGINS, mavsdk_options_include.into())
}

fn proto_path(plugin_name: &str) -> PathBuf {
    let path_string = format!(
        "{submodule}/protos/{name}/{name}.proto",
        submodule = PROTO_GIT_SUBMODULE,
        name = plugin_name
    );

    PathBuf::from(path_string)
}

fn proto_include(plugin_name: &str) -> PathBuf {
    let path = format!(
        "{submodule}/protos/{name}",
        submodule = PROTO_GIT_SUBMODULE,
        name = plugin_name
    );

    PathBuf::from(path)
}

/// build in grpc, adding all files to the build instead generating them one by one
fn tonic_build_single(
    plugins: &HashSet<&str>,
    mavsdk_options_include: PathBuf,
) -> Result<(), std::io::Error> {
    let mavsdk_options_path = PathBuf::from(format!(
        "{submodule}/protos/{name}.proto",
        submodule = PROTO_GIT_SUBMODULE,
        name = MAVSDK_OPTIONS
    ));

    let (proto_paths, proto_includes) = plugins.iter().fold(
        (vec![mavsdk_options_path], vec![mavsdk_options_include]),
        |(mut proto_paths, mut proto_includes), plugin| {
            proto_paths.push(proto_path(plugin));
            proto_includes.push(proto_include(plugin));

            (proto_paths, proto_includes)
        },
    );

    let builder = tonic_build::configure();

    #[cfg(feature = "with_serde")]
    let builder = derive_serde(builder);

    builder
        // .build_server(false)
        .format(true)
        .out_dir("src/grpc")
        .compile(&proto_paths, &proto_includes)
}

#[cfg(feature = "with_serde")]
fn derive_serde(mut builder: tonic_build::Builder) -> tonic_build::Builder {
    let derive_serde = "#[derive(serde::Serialize, serde::Deserialize)]";

    for derive_for_type in DERIVE_SERDE_FOR.iter() {
        builder = builder.type_attribute(derive_for_type, derive_serde);
    }

    builder
}

/// build in sub-dirs each of the plugins and finally adding `MAVSDK_OPTIONS`

fn _tonic_build(plugins: &[&str], mavsdk_options_include: PathBuf) -> Result<(), std::io::Error> {
    plugins
        .iter()
        .map(|plugin| {
            // (proto_path(plugin), proto_include(plugin))
            tonic_build::configure()
                // .build_server(false)
                .format(true)
                .out_dir(format!("src/grpc/{}", plugin))
                .compile(
                    &[proto_path(plugin)],
                    &[proto_include(plugin), mavsdk_options_include.clone()],
                )
        })
        // Add the `mavsdk_options.proto`, since it's in the main `protos` directory
        .chain(std::iter::once_with(|| {
            let path = PathBuf::from(format!(
                "{submodule}/protos/{name}.proto",
                submodule = PROTO_GIT_SUBMODULE,
                name = MAVSDK_OPTIONS
            ));

            tonic_build::configure()
                .build_server(false)
                .format(true)
                .out_dir("src/grpc")
                .compile(&[path], &[mavsdk_options_include.clone()])
        }))
        .collect::<Result<_, _>>()
}
