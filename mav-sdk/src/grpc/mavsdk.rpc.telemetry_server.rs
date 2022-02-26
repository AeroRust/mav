#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishPositionRequest {
    /// The next position
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<Position>,
    /// The next velocity (NED)
    #[prost(message, optional, tag = "2")]
    pub velocity_ned: ::core::option::Option<VelocityNed>,
    /// Heading (yaw) in degrees
    #[prost(message, optional, tag = "3")]
    pub heading: ::core::option::Option<Heading>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishHomeRequest {
    /// The next home position
    #[prost(message, optional, tag = "1")]
    pub home: ::core::option::Option<Position>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishSysStatusRequest {
    /// The next 'battery' state
    #[prost(message, optional, tag = "1")]
    pub battery: ::core::option::Option<Battery>,
    /// rc receiver status
    #[prost(bool, tag = "2")]
    pub rc_receiver_status: bool,
    #[prost(bool, tag = "3")]
    pub gyro_status: bool,
    #[prost(bool, tag = "4")]
    pub accel_status: bool,
    #[prost(bool, tag = "5")]
    pub mag_status: bool,
    #[prost(bool, tag = "6")]
    pub gps_status: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishExtendedSysStateRequest {
    #[prost(enumeration = "VtolState", tag = "1")]
    pub vtol_state: i32,
    #[prost(enumeration = "LandedState", tag = "2")]
    pub landed_state: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishInAirRequest {
    /// The next 'in-air' state
    #[prost(bool, tag = "1")]
    pub is_in_air: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishLandedStateRequest {
    /// The next 'landed' state
    #[prost(enumeration = "LandedState", tag = "1")]
    pub landed_state: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishRawGpsRequest {
    /// The next 'Raw GPS' state. Warning: this is an advanced feature, use `Position` updates to get the location of the drone!
    #[prost(message, optional, tag = "1")]
    pub raw_gps: ::core::option::Option<RawGps>,
    /// The next 'GPS info' state
    #[prost(message, optional, tag = "2")]
    pub gps_info: ::core::option::Option<GpsInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishBatteryRequest {
    /// The next 'battery' state
    #[prost(message, optional, tag = "1")]
    pub battery: ::core::option::Option<Battery>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishRcStatusRequest {
    /// The next RC status
    #[prost(message, optional, tag = "1")]
    pub rc_status: ::core::option::Option<RcStatus>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishStatusTextRequest {
    /// The next 'status text'
    #[prost(message, optional, tag = "1")]
    pub status_text: ::core::option::Option<StatusText>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishOdometryRequest {
    /// The next odometry status
    #[prost(message, optional, tag = "1")]
    pub odometry: ::core::option::Option<Odometry>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishPositionVelocityNedRequest {
    /// The next position and velocity status
    #[prost(message, optional, tag = "1")]
    pub position_velocity_ned: ::core::option::Option<PositionVelocityNed>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishGroundTruthRequest {
    /// Ground truth position information available in simulation
    #[prost(message, optional, tag = "1")]
    pub ground_truth: ::core::option::Option<GroundTruth>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishImuRequest {
    /// The next IMU status
    #[prost(message, optional, tag = "1")]
    pub imu: ::core::option::Option<Imu>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishScaledImuRequest {
    /// The next scaled IMU status
    #[prost(message, optional, tag = "1")]
    pub imu: ::core::option::Option<Imu>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishRawImuRequest {
    /// The next raw IMU status
    #[prost(message, optional, tag = "1")]
    pub imu: ::core::option::Option<Imu>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishUnixEpochTimeRequest {
    /// The next 'unix epoch time' status
    #[prost(uint64, tag = "1")]
    pub time_us: u64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishPositionResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishHomeResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishSysStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishExtendedSysStateResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishRawGpsResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishBatteryResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishStatusTextResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishOdometryResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishPositionVelocityNedResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishGroundTruthResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishImuResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishScaledImuResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishRawImuResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PublishUnixEpochTimeResponse {
    #[prost(message, optional, tag = "1")]
    pub telemetry_server_result: ::core::option::Option<TelemetryServerResult>,
}
/// Position type in global coordinates.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag = "3")]
    pub absolute_altitude_m: f32,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag = "4")]
    pub relative_altitude_m: f32,
}
/// Heading type used for global position
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Heading {
    /// Heading in degrees (range: 0 to +360)
    #[prost(double, tag = "1")]
    pub heading_deg: f64,
}
///
/// Quaternion type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Hamilton quaternion product definition is used.
/// A zero-rotation quaternion is represented by (1,0,0,0).
/// The quaternion could also be written as w + xi + yj + zk.
///
/// For more info see: <https://en.wikipedia.org/wiki/Quaternion>
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// Quaternion entry 0, also denoted as a
    #[prost(float, tag = "1")]
    pub w: f32,
    /// Quaternion entry 1, also denoted as b
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Quaternion entry 2, also denoted as c
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Quaternion entry 3, also denoted as d
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Timestamp in microseconds
    #[prost(uint64, tag = "5")]
    pub timestamp_us: u64,
}
///
/// Euler angle type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Euler angles follow the convention of a 3-2-1 intrinsic Tait-Bryan rotation sequence.
///
/// For more info see <https://en.wikipedia.org/wiki/Euler_angles>
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EulerAngle {
    /// Roll angle in degrees, positive is banking to the right
    #[prost(float, tag = "1")]
    pub roll_deg: f32,
    /// Pitch angle in degrees, positive is pitching nose up
    #[prost(float, tag = "2")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees, positive is clock-wise seen from above
    #[prost(float, tag = "3")]
    pub yaw_deg: f32,
    /// Timestamp in microseconds
    #[prost(uint64, tag = "4")]
    pub timestamp_us: u64,
}
/// Angular velocity type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityBody {
    /// Roll angular velocity
    #[prost(float, tag = "1")]
    pub roll_rad_s: f32,
    /// Pitch angular velocity
    #[prost(float, tag = "2")]
    pub pitch_rad_s: f32,
    /// Yaw angular velocity
    #[prost(float, tag = "3")]
    pub yaw_rad_s: f32,
}
/// GPS information type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsInfo {
    /// Number of visible satellites in use
    #[prost(int32, tag = "1")]
    pub num_satellites: i32,
    /// Fix type
    #[prost(enumeration = "FixType", tag = "2")]
    pub fix_type: i32,
}
///
/// Raw GPS information type.
///
/// Warning: this is an advanced type! If you want the location of the drone, use
/// the position instead. This message exposes the raw values of the GNSS sensor.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RawGps {
    /// Timestamp in microseconds (UNIX Epoch time or time since system boot, to be inferred)
    #[prost(uint64, tag = "1")]
    pub timestamp_us: u64,
    /// Latitude in degrees (WGS84, EGM96 ellipsoid)
    #[prost(double, tag = "2")]
    pub latitude_deg: f64,
    /// Longitude in degrees (WGS84, EGM96 ellipsoid)
    #[prost(double, tag = "3")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag = "4")]
    pub absolute_altitude_m: f32,
    /// GPS HDOP horizontal dilution of position (unitless). If unknown, set to NaN
    #[prost(float, tag = "5")]
    pub hdop: f32,
    /// GPS VDOP vertical dilution of position (unitless). If unknown, set to NaN
    #[prost(float, tag = "6")]
    pub vdop: f32,
    /// Ground velocity in metres per second
    #[prost(float, tag = "7")]
    pub velocity_m_s: f32,
    /// Course over ground (NOT heading, but direction of movement) in degrees. If unknown, set to NaN
    #[prost(float, tag = "8")]
    pub cog_deg: f32,
    /// Altitude in metres (above WGS84, EGM96 ellipsoid)
    #[prost(float, tag = "9")]
    pub altitude_ellipsoid_m: f32,
    /// Position uncertainty in metres
    #[prost(float, tag = "10")]
    pub horizontal_uncertainty_m: f32,
    /// Altitude uncertainty in metres
    #[prost(float, tag = "11")]
    pub vertical_uncertainty_m: f32,
    /// Velocity uncertainty in metres per second
    #[prost(float, tag = "12")]
    pub velocity_uncertainty_m_s: f32,
    /// Heading uncertainty in degrees
    #[prost(float, tag = "13")]
    pub heading_uncertainty_deg: f32,
    /// Yaw in earth frame from north.
    #[prost(float, tag = "14")]
    pub yaw_deg: f32,
}
/// Battery type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Battery {
    /// Voltage in volts
    #[prost(float, tag = "1")]
    pub voltage_v: f32,
    /// Estimated battery remaining (range: 0.0 to 1.0)
    #[prost(float, tag = "2")]
    pub remaining_percent: f32,
}
/// Remote control status type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RcStatus {
    /// True if an RC signal has been available once
    #[prost(bool, tag = "1")]
    pub was_available_once: bool,
    /// True if the RC signal is available now
    #[prost(bool, tag = "2")]
    pub is_available: bool,
    /// Signal strength (range: 0 to 100, NaN if unknown)
    #[prost(float, tag = "3")]
    pub signal_strength_percent: f32,
}
/// StatusText information type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StatusText {
    /// Message type
    #[prost(enumeration = "StatusTextType", tag = "1")]
    pub r#type: i32,
    /// MAVLink status message
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// Actuator control target type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlTarget {
    /// An actuator control group is e.g. 'attitude' for the core flight controls, or 'gimbal' for a payload.
    #[prost(int32, tag = "1")]
    pub group: i32,
    /// Controls normed from -1 to 1, where 0 is neutral position.
    #[prost(float, repeated, tag = "2")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
}
/// Actuator output status type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActuatorOutputStatus {
    /// Active outputs
    #[prost(uint32, tag = "1")]
    pub active: u32,
    /// Servo/motor output values
    #[prost(float, repeated, tag = "2")]
    pub actuator: ::prost::alloc::vec::Vec<f32>,
}
///
/// Covariance type.
///
/// Row-major representation of a 6x6 cross-covariance matrix
/// upper right triangle.
/// Set first to NaN if unknown.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Covariance {
    /// Representation of a covariance matrix.
    #[prost(float, repeated, tag = "1")]
    pub covariance_matrix: ::prost::alloc::vec::Vec<f32>,
}
/// Velocity type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VelocityBody {
    /// Velocity in X in metres/second
    #[prost(float, tag = "1")]
    pub x_m_s: f32,
    /// Velocity in Y in metres/second
    #[prost(float, tag = "2")]
    pub y_m_s: f32,
    /// Velocity in Z in metres/second
    #[prost(float, tag = "3")]
    pub z_m_s: f32,
}
/// Position type, represented in the Body (X Y Z) frame
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PositionBody {
    /// X Position in metres.
    #[prost(float, tag = "1")]
    pub x_m: f32,
    /// Y Position in metres.
    #[prost(float, tag = "2")]
    pub y_m: f32,
    /// Z Position in metres.
    #[prost(float, tag = "3")]
    pub z_m: f32,
}
/// Odometry message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    #[prost(enumeration = "odometry::MavFrame", tag = "2")]
    pub frame_id: i32,
    /// Coordinate frame of reference for the velocity in free space (twist) data.
    #[prost(enumeration = "odometry::MavFrame", tag = "3")]
    pub child_frame_id: i32,
    /// Position.
    #[prost(message, optional, tag = "4")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).
    #[prost(message, optional, tag = "5")]
    pub q: ::core::option::Option<Quaternion>,
    /// Linear velocity (m/s).
    #[prost(message, optional, tag = "6")]
    pub velocity_body: ::core::option::Option<VelocityBody>,
    /// Angular velocity (rad/s).
    #[prost(message, optional, tag = "7")]
    pub angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag = "8")]
    pub pose_covariance: ::core::option::Option<Covariance>,
    /// Velocity cross-covariance matrix.
    #[prost(message, optional, tag = "9")]
    pub velocity_covariance: ::core::option::Option<Covariance>,
}
/// Nested message and enum types in `Odometry`.
pub mod odometry {
    /// Mavlink frame id
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum MavFrame {
        /// Frame is undefined.
        Undef = 0,
        /// Setpoint in body NED frame. This makes sense if all position control is externalized - e.g. useful to command 2 m/s^2 acceleration to the right.
        BodyNed = 8,
        /// Odometry local coordinate frame of data given by a vision estimation system, Z-down (x: north, y: east, z: down).
        VisionNed = 16,
        /// Odometry local coordinate frame of data given by an estimator running onboard the vehicle, Z-down (x: north, y: east, z: down).
        EstimNed = 18,
    }
}
/// DistanceSensor message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DistanceSensor {
    /// Minimum distance the sensor can measure, NaN if unknown.
    #[prost(float, tag = "1")]
    pub minimum_distance_m: f32,
    /// Maximum distance the sensor can measure, NaN if unknown.
    #[prost(float, tag = "2")]
    pub maximum_distance_m: f32,
    /// Current distance reading, NaN if unknown.
    #[prost(float, tag = "3")]
    pub current_distance_m: f32,
}
/// Scaled Pressure message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledPressure {
    /// Timestamp (time since system boot)
    #[prost(uint64, tag = "1")]
    pub timestamp_us: u64,
    /// Absolute pressure in hPa
    #[prost(float, tag = "2")]
    pub absolute_pressure_hpa: f32,
    /// Differential pressure 1 in hPa
    #[prost(float, tag = "3")]
    pub differential_pressure_hpa: f32,
    /// Absolute pressure temperature (in celsius)
    #[prost(float, tag = "4")]
    pub temperature_deg: f32,
    /// Differential pressure temperature (in celsius, 0 if not available)
    #[prost(float, tag = "5")]
    pub differential_pressure_temperature_deg: f32,
}
/// PositionNed message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PositionNed {
    /// Position along north direction in metres
    #[prost(float, tag = "1")]
    pub north_m: f32,
    /// Position along east direction in metres
    #[prost(float, tag = "2")]
    pub east_m: f32,
    /// Position along down direction in metres
    #[prost(float, tag = "3")]
    pub down_m: f32,
}
/// VelocityNed message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VelocityNed {
    /// Velocity along north direction in metres per second
    #[prost(float, tag = "1")]
    pub north_m_s: f32,
    /// Velocity along east direction in metres per second
    #[prost(float, tag = "2")]
    pub east_m_s: f32,
    /// Velocity along down direction in metres per second
    #[prost(float, tag = "3")]
    pub down_m_s: f32,
}
/// PositionVelocityNed message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PositionVelocityNed {
    /// Position (NED)
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<PositionNed>,
    /// Velocity (NED)
    #[prost(message, optional, tag = "2")]
    pub velocity: ::core::option::Option<VelocityNed>,
}
/// GroundTruth message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GroundTruth {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to 180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag = "3")]
    pub absolute_altitude_m: f32,
}
/// FixedwingMetrics message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FixedwingMetrics {
    /// Current indicated airspeed (IAS) in metres per second
    #[prost(float, tag = "1")]
    pub airspeed_m_s: f32,
    /// Current throttle setting (0 to 100)
    #[prost(float, tag = "2")]
    pub throttle_percentage: f32,
    /// Current climb rate in metres per second
    #[prost(float, tag = "3")]
    pub climb_rate_m_s: f32,
}
/// AccelerationFrd message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AccelerationFrd {
    /// Acceleration in forward direction in metres per second^2
    #[prost(float, tag = "1")]
    pub forward_m_s2: f32,
    /// Acceleration in right direction in metres per second^2
    #[prost(float, tag = "2")]
    pub right_m_s2: f32,
    /// Acceleration in down direction in metres per second^2
    #[prost(float, tag = "3")]
    pub down_m_s2: f32,
}
/// AngularVelocityFrd message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityFrd {
    /// Angular velocity in forward direction in radians per second
    #[prost(float, tag = "1")]
    pub forward_rad_s: f32,
    /// Angular velocity in right direction in radians per second
    #[prost(float, tag = "2")]
    pub right_rad_s: f32,
    /// Angular velocity in Down direction in radians per second
    #[prost(float, tag = "3")]
    pub down_rad_s: f32,
}
/// MagneticFieldFrd message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MagneticFieldFrd {
    /// Magnetic field in forward direction measured in Gauss
    #[prost(float, tag = "1")]
    pub forward_gauss: f32,
    /// Magnetic field in East direction measured in Gauss
    #[prost(float, tag = "2")]
    pub right_gauss: f32,
    /// Magnetic field in Down direction measured in Gauss
    #[prost(float, tag = "3")]
    pub down_gauss: f32,
}
/// Imu message type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Imu {
    /// Acceleration
    #[prost(message, optional, tag = "1")]
    pub acceleration_frd: ::core::option::Option<AccelerationFrd>,
    /// Angular velocity
    #[prost(message, optional, tag = "2")]
    pub angular_velocity_frd: ::core::option::Option<AngularVelocityFrd>,
    /// Magnetic field
    #[prost(message, optional, tag = "3")]
    pub magnetic_field_frd: ::core::option::Option<MagneticFieldFrd>,
    /// Temperature
    #[prost(float, tag = "4")]
    pub temperature_degc: f32,
    /// Timestamp in microseconds
    #[prost(uint64, tag = "5")]
    pub timestamp_us: u64,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TelemetryServerResult {
    /// Result enum value
    #[prost(enumeration = "telemetry_server_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TelemetryServerResult`.
pub mod telemetry_server_result {
    /// Possible results returned for telemetry requests.
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Success: the telemetry command was accepted by the vehicle
        Success = 1,
        /// No system connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command refused by vehicle
        CommandDenied = 5,
        /// Request timed out
        Timeout = 6,
        /// Request not supported
        Unsupported = 7,
    }
}
/// GPS fix type.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum FixType {
    /// No GPS connected
    NoGps = 0,
    /// No position information, GPS is connected
    NoFix = 1,
    /// 2D position
    Fix2d = 2,
    /// 3D position
    Fix3d = 3,
    /// DGPS/SBAS aided 3D position
    FixDgps = 4,
    /// RTK float, 3D position
    RtkFloat = 5,
    /// RTK Fixed, 3D position
    RtkFixed = 6,
}
/// Maps to MAV_VTOL_STATE
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum VtolState {
    /// Not VTOL
    Undefined = 0,
    /// Transitioning to fixed-wing
    TransitionToFw = 1,
    /// Transitioning to multi-copter
    TransitionToMc = 2,
    /// Multi-copter
    Mc = 3,
    /// Fixed-wing
    Fw = 4,
}
/// Status types.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum StatusTextType {
    /// Debug
    Debug = 0,
    /// Information
    Info = 1,
    /// Notice
    Notice = 2,
    /// Warning
    Warning = 3,
    /// Error
    Error = 4,
    /// Critical
    Critical = 5,
    /// Alert
    Alert = 6,
    /// Emergency
    Emergency = 7,
}
/// Landed State enumeration.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum LandedState {
    /// Landed state is unknown
    Unknown = 0,
    /// The vehicle is on the ground
    OnGround = 1,
    /// The vehicle is in the air
    InAir = 2,
    /// The vehicle is taking off
    TakingOff = 3,
    /// The vehicle is landing
    Landing = 4,
}
#[doc = r" Generated client implementations."]
pub mod telemetry_server_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Allow users to provide vehicle telemetry and state information"]
    #[doc = " (e.g. battery, GPS, RC connection, flight mode etc.) and set telemetry update rates."]
    #[derive(Debug, Clone)]
    pub struct TelemetryServerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TelemetryServerServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TelemetryServerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TelemetryServerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TelemetryServerServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Publish to 'position' updates."]
        pub async fn publish_position(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishPositionRequest>,
        ) -> Result<tonic::Response<super::PublishPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'home position' updates."]
        pub async fn publish_home(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishHomeRequest>,
        ) -> Result<tonic::Response<super::PublishHomeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishHome",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish 'sys status' updates."]
        pub async fn publish_sys_status(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishSysStatusRequest>,
        ) -> Result<tonic::Response<super::PublishSysStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishSysStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish 'extended sys state' updates."]
        pub async fn publish_extended_sys_state(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishExtendedSysStateRequest>,
        ) -> Result<tonic::Response<super::PublishExtendedSysStateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishExtendedSysState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'Raw GPS' updates."]
        pub async fn publish_raw_gps(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRawGpsRequest>,
        ) -> Result<tonic::Response<super::PublishRawGpsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishRawGps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'battery' updates."]
        pub async fn publish_battery(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishBatteryRequest>,
        ) -> Result<tonic::Response<super::PublishBatteryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishBattery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'status text' updates."]
        pub async fn publish_status_text(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishStatusTextRequest>,
        ) -> Result<tonic::Response<super::PublishStatusTextResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishStatusText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'odometry' updates."]
        pub async fn publish_odometry(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishOdometryRequest>,
        ) -> Result<tonic::Response<super::PublishOdometryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishOdometry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'position velocity' updates."]
        pub async fn publish_position_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishPositionVelocityNedRequest>,
        ) -> Result<tonic::Response<super::PublishPositionVelocityNedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishPositionVelocityNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'ground truth' updates."]
        pub async fn publish_ground_truth(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishGroundTruthRequest>,
        ) -> Result<tonic::Response<super::PublishGroundTruthResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishGroundTruth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'IMU' updates (in SI units in NED body frame)."]
        pub async fn publish_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishImuRequest>,
        ) -> Result<tonic::Response<super::PublishImuResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishImu",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'Scaled IMU' updates."]
        pub async fn publish_scaled_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishScaledImuRequest>,
        ) -> Result<tonic::Response<super::PublishScaledImuResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishScaledImu",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'Raw IMU' updates."]
        pub async fn publish_raw_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRawImuRequest>,
        ) -> Result<tonic::Response<super::PublishRawImuResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishRawImu",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Publish to 'unix epoch time' updates."]
        pub async fn publish_unix_epoch_time(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishUnixEpochTimeRequest>,
        ) -> Result<tonic::Response<super::PublishUnixEpochTimeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishUnixEpochTime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod telemetry_server_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TelemetryServerServiceServer."]
    #[async_trait]
    pub trait TelemetryServerService: Send + Sync + 'static {
        #[doc = " Publish to 'position' updates."]
        async fn publish_position(
            &self,
            request: tonic::Request<super::PublishPositionRequest>,
        ) -> Result<tonic::Response<super::PublishPositionResponse>, tonic::Status>;
        #[doc = " Publish to 'home position' updates."]
        async fn publish_home(
            &self,
            request: tonic::Request<super::PublishHomeRequest>,
        ) -> Result<tonic::Response<super::PublishHomeResponse>, tonic::Status>;
        #[doc = " Publish 'sys status' updates."]
        async fn publish_sys_status(
            &self,
            request: tonic::Request<super::PublishSysStatusRequest>,
        ) -> Result<tonic::Response<super::PublishSysStatusResponse>, tonic::Status>;
        #[doc = " Publish 'extended sys state' updates."]
        async fn publish_extended_sys_state(
            &self,
            request: tonic::Request<super::PublishExtendedSysStateRequest>,
        ) -> Result<tonic::Response<super::PublishExtendedSysStateResponse>, tonic::Status>;
        #[doc = " Publish to 'Raw GPS' updates."]
        async fn publish_raw_gps(
            &self,
            request: tonic::Request<super::PublishRawGpsRequest>,
        ) -> Result<tonic::Response<super::PublishRawGpsResponse>, tonic::Status>;
        #[doc = " Publish to 'battery' updates."]
        async fn publish_battery(
            &self,
            request: tonic::Request<super::PublishBatteryRequest>,
        ) -> Result<tonic::Response<super::PublishBatteryResponse>, tonic::Status>;
        #[doc = " Publish to 'status text' updates."]
        async fn publish_status_text(
            &self,
            request: tonic::Request<super::PublishStatusTextRequest>,
        ) -> Result<tonic::Response<super::PublishStatusTextResponse>, tonic::Status>;
        #[doc = " Publish to 'odometry' updates."]
        async fn publish_odometry(
            &self,
            request: tonic::Request<super::PublishOdometryRequest>,
        ) -> Result<tonic::Response<super::PublishOdometryResponse>, tonic::Status>;
        #[doc = " Publish to 'position velocity' updates."]
        async fn publish_position_velocity_ned(
            &self,
            request: tonic::Request<super::PublishPositionVelocityNedRequest>,
        ) -> Result<tonic::Response<super::PublishPositionVelocityNedResponse>, tonic::Status>;
        #[doc = " Publish to 'ground truth' updates."]
        async fn publish_ground_truth(
            &self,
            request: tonic::Request<super::PublishGroundTruthRequest>,
        ) -> Result<tonic::Response<super::PublishGroundTruthResponse>, tonic::Status>;
        #[doc = " Publish to 'IMU' updates (in SI units in NED body frame)."]
        async fn publish_imu(
            &self,
            request: tonic::Request<super::PublishImuRequest>,
        ) -> Result<tonic::Response<super::PublishImuResponse>, tonic::Status>;
        #[doc = " Publish to 'Scaled IMU' updates."]
        async fn publish_scaled_imu(
            &self,
            request: tonic::Request<super::PublishScaledImuRequest>,
        ) -> Result<tonic::Response<super::PublishScaledImuResponse>, tonic::Status>;
        #[doc = " Publish to 'Raw IMU' updates."]
        async fn publish_raw_imu(
            &self,
            request: tonic::Request<super::PublishRawImuRequest>,
        ) -> Result<tonic::Response<super::PublishRawImuResponse>, tonic::Status>;
        #[doc = " Publish to 'unix epoch time' updates."]
        async fn publish_unix_epoch_time(
            &self,
            request: tonic::Request<super::PublishUnixEpochTimeRequest>,
        ) -> Result<tonic::Response<super::PublishUnixEpochTimeResponse>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = " Allow users to provide vehicle telemetry and state information"]
    #[doc = " (e.g. battery, GPS, RC connection, flight mode etc.) and set telemetry update rates."]
    #[derive(Debug)]
    pub struct TelemetryServerServiceServer<T: TelemetryServerService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TelemetryServerService> TelemetryServerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TelemetryServerServiceServer<T>
    where
        T: TelemetryServerService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req . uri () . path () { "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishPosition" => { # [allow (non_camel_case_types)] struct PublishPositionSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishPositionRequest > for PublishPositionSvc < T > { type Response = super :: PublishPositionResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishPositionRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_position (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishPositionSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishHome" => { # [allow (non_camel_case_types)] struct PublishHomeSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishHomeRequest > for PublishHomeSvc < T > { type Response = super :: PublishHomeResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishHomeRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_home (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishHomeSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishSysStatus" => { # [allow (non_camel_case_types)] struct PublishSysStatusSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishSysStatusRequest > for PublishSysStatusSvc < T > { type Response = super :: PublishSysStatusResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishSysStatusRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_sys_status (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishSysStatusSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishExtendedSysState" => { # [allow (non_camel_case_types)] struct PublishExtendedSysStateSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishExtendedSysStateRequest > for PublishExtendedSysStateSvc < T > { type Response = super :: PublishExtendedSysStateResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishExtendedSysStateRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_extended_sys_state (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishExtendedSysStateSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishRawGps" => { # [allow (non_camel_case_types)] struct PublishRawGpsSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishRawGpsRequest > for PublishRawGpsSvc < T > { type Response = super :: PublishRawGpsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishRawGpsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_raw_gps (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishRawGpsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishBattery" => { # [allow (non_camel_case_types)] struct PublishBatterySvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishBatteryRequest > for PublishBatterySvc < T > { type Response = super :: PublishBatteryResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishBatteryRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_battery (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishBatterySvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishStatusText" => { # [allow (non_camel_case_types)] struct PublishStatusTextSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishStatusTextRequest > for PublishStatusTextSvc < T > { type Response = super :: PublishStatusTextResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishStatusTextRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_status_text (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishStatusTextSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishOdometry" => { # [allow (non_camel_case_types)] struct PublishOdometrySvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishOdometryRequest > for PublishOdometrySvc < T > { type Response = super :: PublishOdometryResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishOdometryRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_odometry (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishOdometrySvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishPositionVelocityNed" => { # [allow (non_camel_case_types)] struct PublishPositionVelocityNedSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishPositionVelocityNedRequest > for PublishPositionVelocityNedSvc < T > { type Response = super :: PublishPositionVelocityNedResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishPositionVelocityNedRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_position_velocity_ned (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishPositionVelocityNedSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishGroundTruth" => { # [allow (non_camel_case_types)] struct PublishGroundTruthSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishGroundTruthRequest > for PublishGroundTruthSvc < T > { type Response = super :: PublishGroundTruthResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishGroundTruthRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_ground_truth (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishGroundTruthSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishImu" => { # [allow (non_camel_case_types)] struct PublishImuSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishImuRequest > for PublishImuSvc < T > { type Response = super :: PublishImuResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishImuRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_imu (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishImuSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishScaledImu" => { # [allow (non_camel_case_types)] struct PublishScaledImuSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishScaledImuRequest > for PublishScaledImuSvc < T > { type Response = super :: PublishScaledImuResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishScaledImuRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_scaled_imu (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishScaledImuSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishRawImu" => { # [allow (non_camel_case_types)] struct PublishRawImuSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishRawImuRequest > for PublishRawImuSvc < T > { type Response = super :: PublishRawImuResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishRawImuRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_raw_imu (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishRawImuSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.telemetry_server.TelemetryServerService/PublishUnixEpochTime" => { # [allow (non_camel_case_types)] struct PublishUnixEpochTimeSvc < T : TelemetryServerService > (pub Arc < T >) ; impl < T : TelemetryServerService > tonic :: server :: UnaryService < super :: PublishUnixEpochTimeRequest > for PublishUnixEpochTimeSvc < T > { type Response = super :: PublishUnixEpochTimeResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: PublishUnixEpochTimeRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . publish_unix_epoch_time (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = PublishUnixEpochTimeSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (empty_body ()) . unwrap ()) }) , }
        }
    }
    impl<T: TelemetryServerService> Clone for TelemetryServerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TelemetryServerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TelemetryServerService> tonic::transport::NamedService for TelemetryServerServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.telemetry_server.TelemetryServerService";
    }
}
