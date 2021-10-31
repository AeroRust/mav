use std::path::PathBuf;

const PROTO_GIT_SUBMODULE: &str = "mavsdk-proto";
const MAVSDK_OPTIONS: &str = "mavsdk_options";

const TELEMETRY: [&str; 126] = [
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
];

const MOCAP: [&str; 16] = [
    "AngleBody",
    "AngularVelocityBody",
    "AttitudePositionMocap",
    "Covariance",
    "MocapResult",
    "Odometry",
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


fn main() -> Result<(), std::io::Error> {
    let plugins = [
        "action",
        "calibration",
        "camera",
        "core",
        "geofence",
        "gimbal",
        "info",
        "mission",
        "mocap",
        "offboard",
        "param",
        "shell",
        "telemetry",
    ];

    let mavsdk_options_include = format!("{submodule}/protos", submodule = PROTO_GIT_SUBMODULE);

    // tonic_build(&plugins, mavsdk_options_include.into())

    tonic_build_single(&plugins, mavsdk_options_include.into())
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
    plugins: &[&str],
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

    // let mut attributes = Attributes::default();
    // attributes.push_struct("AttitudeQuaternionResponse", "#[derive(Serialize, Deserialize)]");

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

    // Telemetry
    for telemetry in TELEMETRY {
        builder = builder.type_attribute(telemetry, derive_serde);
    }

    for mocap in MOCAP {
        builder = builder.type_attribute(mocap, derive_serde);
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
