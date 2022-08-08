#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePositionRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionResponse {
    /// The next position
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHomeRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeResponse {
    /// The next home position
    #[prost(message, optional, tag="1")]
    pub home: ::core::option::Option<Position>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInAirRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InAirResponse {
    /// The next 'in-air' state
    #[prost(bool, tag="1")]
    pub is_in_air: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLandedStateRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LandedStateResponse {
    /// The next 'landed' state
    #[prost(enumeration="LandedState", tag="1")]
    pub landed_state: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeArmedRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArmedResponse {
    /// The next 'armed' state
    #[prost(bool, tag="1")]
    pub is_armed: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeVtolStateRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VtolStateResponse {
    /// The next 'vtol' state
    #[prost(enumeration="VtolState", tag="1")]
    pub vtol_state: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeAttitudeQuaternionRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeQuaternionResponse {
    /// The next attitude (quaternion)
    #[prost(message, optional, tag="1")]
    pub attitude_quaternion: ::core::option::Option<Quaternion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeAttitudeEulerRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeEulerResponse {
    /// The next attitude (Euler)
    #[prost(message, optional, tag="1")]
    pub attitude_euler: ::core::option::Option<EulerAngle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeAttitudeAngularVelocityBodyRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudeAngularVelocityBodyResponse {
    /// The next angular velocity (rad/s)
    #[prost(message, optional, tag="1")]
    pub attitude_angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCameraAttitudeQuaternionRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraAttitudeQuaternionResponse {
    /// The next camera attitude (quaternion)
    #[prost(message, optional, tag="1")]
    pub attitude_quaternion: ::core::option::Option<Quaternion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCameraAttitudeEulerRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraAttitudeEulerResponse {
    /// The next camera attitude (Euler)
    #[prost(message, optional, tag="1")]
    pub attitude_euler: ::core::option::Option<EulerAngle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeVelocityNedRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VelocityNedResponse {
    /// The next velocity (NED)
    #[prost(message, optional, tag="1")]
    pub velocity_ned: ::core::option::Option<VelocityNed>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeGpsInfoRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsInfoResponse {
    /// The next 'GPS info' state
    #[prost(message, optional, tag="1")]
    pub gps_info: ::core::option::Option<GpsInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRawGpsRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGpsResponse {
    /// The next 'Raw GPS' state. Warning: this is an advanced feature, use `Position` updates to get the location of the drone!
    #[prost(message, optional, tag="1")]
    pub raw_gps: ::core::option::Option<RawGps>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeBatteryRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatteryResponse {
    /// The next 'battery' state
    #[prost(message, optional, tag="1")]
    pub battery: ::core::option::Option<Battery>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeFlightModeRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightModeResponse {
    /// The next flight mode
    #[prost(enumeration="FlightMode", tag="1")]
    pub flight_mode: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHealthRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthResponse {
    /// The next 'health' state
    #[prost(message, optional, tag="1")]
    pub health: ::core::option::Option<Health>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRcStatusRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcStatusResponse {
    /// The next RC status
    #[prost(message, optional, tag="1")]
    pub rc_status: ::core::option::Option<RcStatus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeStatusTextRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusTextResponse {
    /// The next 'status text'
    #[prost(message, optional, tag="1")]
    pub status_text: ::core::option::Option<StatusText>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeActuatorControlTargetRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlTargetResponse {
    /// The next actuator control target
    #[prost(message, optional, tag="1")]
    pub actuator_control_target: ::core::option::Option<ActuatorControlTarget>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeActuatorOutputStatusRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorOutputStatusResponse {
    /// The next actuator output status
    #[prost(message, optional, tag="1")]
    pub actuator_output_status: ::core::option::Option<ActuatorOutputStatus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOdometryRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OdometryResponse {
    /// The next odometry status
    #[prost(message, optional, tag="1")]
    pub odometry: ::core::option::Option<Odometry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePositionVelocityNedRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionVelocityNedResponse {
    /// The next position and velocity status
    #[prost(message, optional, tag="1")]
    pub position_velocity_ned: ::core::option::Option<PositionVelocityNed>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeGroundTruthRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroundTruthResponse {
    /// Ground truth position information available in simulation
    #[prost(message, optional, tag="1")]
    pub ground_truth: ::core::option::Option<GroundTruth>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeFixedwingMetricsRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedwingMetricsResponse {
    /// The next fixedwing metrics
    #[prost(message, optional, tag="1")]
    pub fixedwing_metrics: ::core::option::Option<FixedwingMetrics>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeImuRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImuResponse {
    /// The next IMU status
    #[prost(message, optional, tag="1")]
    pub imu: ::core::option::Option<Imu>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeScaledImuRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaledImuResponse {
    /// The next scaled IMU status
    #[prost(message, optional, tag="1")]
    pub imu: ::core::option::Option<Imu>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRawImuRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawImuResponse {
    /// The next raw IMU status
    #[prost(message, optional, tag="1")]
    pub imu: ::core::option::Option<Imu>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHealthAllOkRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthAllOkResponse {
    /// The next 'health all ok' status
    #[prost(bool, tag="1")]
    pub is_health_all_ok: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeUnixEpochTimeRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnixEpochTimeResponse {
    /// The next 'unix epoch time' status
    #[prost(uint64, tag="1")]
    pub time_us: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDistanceSensorRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistanceSensorResponse {
    /// The next Distance Sensor status
    #[prost(message, optional, tag="1")]
    pub distance_sensor: ::core::option::Option<DistanceSensor>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeScaledPressureRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaledPressureResponse {
    /// The next Scaled Pressure status
    #[prost(message, optional, tag="1")]
    pub scaled_pressure: ::core::option::Option<ScaledPressure>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHeadingRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeadingResponse {
    /// The next heading (yaw) in degrees
    #[prost(message, optional, tag="1")]
    pub heading_deg: ::core::option::Option<Heading>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRatePositionRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRatePositionResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateHomeRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateHomeResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateInAirRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateInAirResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateLandedStateRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateLandedStateResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateVtolStateRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateVtolStateResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateAttitudeRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateAttitudeResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateAttitudeAngularVelocityBodyRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateAttitudeAngularVelocityBodyResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateCameraAttitudeQuaternionRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateCameraAttitudeQuaternionResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateCameraAttitudeRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateCameraAttitudeResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateVelocityNedRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateVelocityNedResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateGpsInfoRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateGpsInfoResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateRawGpsRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateBatteryRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateBatteryResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateRcStatusRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateRcStatusResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateActuatorControlTargetRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateActuatorControlTargetResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateActuatorOutputStatusRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateActuatorOutputStatusResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateOdometryRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateOdometryResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRatePositionVelocityNedRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRatePositionVelocityNedResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateGroundTruthRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateGroundTruthResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateFixedwingMetricsRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateFixedwingMetricsResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateImuRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateImuResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateScaledImuRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateScaledImuResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateRawImuRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateRawImuResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateUnixEpochTimeRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateUnixEpochTimeResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateDistanceSensorRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateDistanceSensorResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGpsGlobalOriginRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGpsGlobalOriginResponse {
    #[prost(message, optional, tag="1")]
    pub telemetry_result: ::core::option::Option<TelemetryResult>,
    #[prost(message, optional, tag="2")]
    pub gps_global_origin: ::core::option::Option<GpsGlobalOrigin>,
}
/// Position type in global coordinates.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag="1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag="2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag="3")]
    pub absolute_altitude_m: f32,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag="4")]
    pub relative_altitude_m: f32,
}
/// Heading type used for global position
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heading {
    /// Heading in degrees (range: 0 to +360)
    #[prost(double, tag="1")]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// Quaternion entry 0, also denoted as a
    #[prost(float, tag="1")]
    pub w: f32,
    /// Quaternion entry 1, also denoted as b
    #[prost(float, tag="2")]
    pub x: f32,
    /// Quaternion entry 2, also denoted as c
    #[prost(float, tag="3")]
    pub y: f32,
    /// Quaternion entry 3, also denoted as d
    #[prost(float, tag="4")]
    pub z: f32,
    /// Timestamp in microseconds
    #[prost(uint64, tag="5")]
    pub timestamp_us: u64,
}
///
/// Euler angle type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Euler angles follow the convention of a 3-2-1 intrinsic Tait-Bryan rotation sequence.
///
/// For more info see <https://en.wikipedia.org/wiki/Euler_angles>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EulerAngle {
    /// Roll angle in degrees, positive is banking to the right
    #[prost(float, tag="1")]
    pub roll_deg: f32,
    /// Pitch angle in degrees, positive is pitching nose up
    #[prost(float, tag="2")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees, positive is clock-wise seen from above
    #[prost(float, tag="3")]
    pub yaw_deg: f32,
    /// Timestamp in microseconds
    #[prost(uint64, tag="4")]
    pub timestamp_us: u64,
}
/// Angular velocity type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityBody {
    /// Roll angular velocity
    #[prost(float, tag="1")]
    pub roll_rad_s: f32,
    /// Pitch angular velocity
    #[prost(float, tag="2")]
    pub pitch_rad_s: f32,
    /// Yaw angular velocity
    #[prost(float, tag="3")]
    pub yaw_rad_s: f32,
}
/// GPS information type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsInfo {
    /// Number of visible satellites in use
    #[prost(int32, tag="1")]
    pub num_satellites: i32,
    /// Fix type
    #[prost(enumeration="FixType", tag="2")]
    pub fix_type: i32,
}
///
/// Raw GPS information type.
///
/// Warning: this is an advanced type! If you want the location of the drone, use
/// the position instead. This message exposes the raw values of the GNSS sensor.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGps {
    /// Timestamp in microseconds (UNIX Epoch time or time since system boot, to be inferred)
    #[prost(uint64, tag="1")]
    pub timestamp_us: u64,
    /// Latitude in degrees (WGS84, EGM96 ellipsoid)
    #[prost(double, tag="2")]
    pub latitude_deg: f64,
    /// Longitude in degrees (WGS84, EGM96 ellipsoid)
    #[prost(double, tag="3")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag="4")]
    pub absolute_altitude_m: f32,
    /// GPS HDOP horizontal dilution of position (unitless). If unknown, set to NaN
    #[prost(float, tag="5")]
    pub hdop: f32,
    /// GPS VDOP vertical dilution of position (unitless). If unknown, set to NaN
    #[prost(float, tag="6")]
    pub vdop: f32,
    /// Ground velocity in metres per second
    #[prost(float, tag="7")]
    pub velocity_m_s: f32,
    /// Course over ground (NOT heading, but direction of movement) in degrees. If unknown, set to NaN
    #[prost(float, tag="8")]
    pub cog_deg: f32,
    /// Altitude in metres (above WGS84, EGM96 ellipsoid)
    #[prost(float, tag="9")]
    pub altitude_ellipsoid_m: f32,
    /// Position uncertainty in metres
    #[prost(float, tag="10")]
    pub horizontal_uncertainty_m: f32,
    /// Altitude uncertainty in metres
    #[prost(float, tag="11")]
    pub vertical_uncertainty_m: f32,
    /// Velocity uncertainty in metres per second
    #[prost(float, tag="12")]
    pub velocity_uncertainty_m_s: f32,
    /// Heading uncertainty in degrees
    #[prost(float, tag="13")]
    pub heading_uncertainty_deg: f32,
    /// Yaw in earth frame from north.
    #[prost(float, tag="14")]
    pub yaw_deg: f32,
}
/// Battery type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Battery {
    /// Battery ID, for systems with multiple batteries
    #[prost(uint32, tag="3")]
    pub id: u32,
    /// Voltage in volts
    #[prost(float, tag="1")]
    pub voltage_v: f32,
    /// Estimated battery remaining (range: 0.0 to 1.0)
    #[prost(float, tag="2")]
    pub remaining_percent: f32,
}
/// Health type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Health {
    /// True if the gyrometer is calibrated
    #[prost(bool, tag="1")]
    pub is_gyrometer_calibration_ok: bool,
    /// True if the accelerometer is calibrated
    #[prost(bool, tag="2")]
    pub is_accelerometer_calibration_ok: bool,
    /// True if the magnetometer is calibrated
    #[prost(bool, tag="3")]
    pub is_magnetometer_calibration_ok: bool,
    /// True if the local position estimate is good enough to fly in 'position control' mode
    #[prost(bool, tag="5")]
    pub is_local_position_ok: bool,
    /// True if the global position estimate is good enough to fly in 'position control' mode
    #[prost(bool, tag="6")]
    pub is_global_position_ok: bool,
    /// True if the home position has been initialized properly
    #[prost(bool, tag="7")]
    pub is_home_position_ok: bool,
    /// True if system can be armed
    #[prost(bool, tag="8")]
    pub is_armable: bool,
}
/// Remote control status type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcStatus {
    /// True if an RC signal has been available once
    #[prost(bool, tag="1")]
    pub was_available_once: bool,
    /// True if the RC signal is available now
    #[prost(bool, tag="2")]
    pub is_available: bool,
    /// Signal strength (range: 0 to 100, NaN if unknown)
    #[prost(float, tag="3")]
    pub signal_strength_percent: f32,
}
/// StatusText information type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusText {
    /// Message type
    #[prost(enumeration="StatusTextType", tag="1")]
    pub r#type: i32,
    /// MAVLink status message
    #[prost(string, tag="2")]
    pub text: ::prost::alloc::string::String,
}
/// Actuator control target type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlTarget {
    /// An actuator control group is e.g. 'attitude' for the core flight controls, or 'gimbal' for a payload.
    #[prost(int32, tag="1")]
    pub group: i32,
    /// Controls normed from -1 to 1, where 0 is neutral position.
    #[prost(float, repeated, tag="2")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
}
/// Actuator output status type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorOutputStatus {
    /// Active outputs
    #[prost(uint32, tag="1")]
    pub active: u32,
    /// Servo/motor output values
    #[prost(float, repeated, tag="2")]
    pub actuator: ::prost::alloc::vec::Vec<f32>,
}
///
/// Covariance type.
///
/// Row-major representation of a 6x6 cross-covariance matrix
/// upper right triangle.
/// Set first to NaN if unknown.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Covariance {
    /// Representation of a covariance matrix.
    #[prost(float, repeated, tag="1")]
    pub covariance_matrix: ::prost::alloc::vec::Vec<f32>,
}
/// Velocity type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VelocityBody {
    /// Velocity in X in metres/second
    #[prost(float, tag="1")]
    pub x_m_s: f32,
    /// Velocity in Y in metres/second
    #[prost(float, tag="2")]
    pub y_m_s: f32,
    /// Velocity in Z in metres/second
    #[prost(float, tag="3")]
    pub z_m_s: f32,
}
/// Position type, represented in the Body (X Y Z) frame
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionBody {
    /// X Position in metres.
    #[prost(float, tag="1")]
    pub x_m: f32,
    /// Y Position in metres.
    #[prost(float, tag="2")]
    pub y_m: f32,
    /// Z Position in metres.
    #[prost(float, tag="3")]
    pub z_m: f32,
}
/// Odometry message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    #[prost(uint64, tag="1")]
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    #[prost(enumeration="odometry::MavFrame", tag="2")]
    pub frame_id: i32,
    /// Coordinate frame of reference for the velocity in free space (twist) data.
    #[prost(enumeration="odometry::MavFrame", tag="3")]
    pub child_frame_id: i32,
    /// Position.
    #[prost(message, optional, tag="4")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).
    #[prost(message, optional, tag="5")]
    pub q: ::core::option::Option<Quaternion>,
    /// Linear velocity (m/s).
    #[prost(message, optional, tag="6")]
    pub velocity_body: ::core::option::Option<VelocityBody>,
    /// Angular velocity (rad/s).
    #[prost(message, optional, tag="7")]
    pub angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag="8")]
    pub pose_covariance: ::core::option::Option<Covariance>,
    /// Velocity cross-covariance matrix.
    #[prost(message, optional, tag="9")]
    pub velocity_covariance: ::core::option::Option<Covariance>,
}
/// Nested message and enum types in `Odometry`.
pub mod odometry {
    /// Mavlink frame id
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    impl MavFrame {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MavFrame::Undef => "MAV_FRAME_UNDEF",
                MavFrame::BodyNed => "MAV_FRAME_BODY_NED",
                MavFrame::VisionNed => "MAV_FRAME_VISION_NED",
                MavFrame::EstimNed => "MAV_FRAME_ESTIM_NED",
            }
        }
    }
}
/// DistanceSensor message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistanceSensor {
    /// Minimum distance the sensor can measure, NaN if unknown.
    #[prost(float, tag="1")]
    pub minimum_distance_m: f32,
    /// Maximum distance the sensor can measure, NaN if unknown.
    #[prost(float, tag="2")]
    pub maximum_distance_m: f32,
    /// Current distance reading, NaN if unknown.
    #[prost(float, tag="3")]
    pub current_distance_m: f32,
}
/// Scaled Pressure message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaledPressure {
    /// Timestamp (time since system boot)
    #[prost(uint64, tag="1")]
    pub timestamp_us: u64,
    /// Absolute pressure in hPa
    #[prost(float, tag="2")]
    pub absolute_pressure_hpa: f32,
    /// Differential pressure 1 in hPa
    #[prost(float, tag="3")]
    pub differential_pressure_hpa: f32,
    /// Absolute pressure temperature (in celsius)
    #[prost(float, tag="4")]
    pub temperature_deg: f32,
    /// Differential pressure temperature (in celsius, 0 if not available)
    #[prost(float, tag="5")]
    pub differential_pressure_temperature_deg: f32,
}
/// PositionNed message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionNed {
    /// Position along north direction in metres
    #[prost(float, tag="1")]
    pub north_m: f32,
    /// Position along east direction in metres
    #[prost(float, tag="2")]
    pub east_m: f32,
    /// Position along down direction in metres
    #[prost(float, tag="3")]
    pub down_m: f32,
}
/// VelocityNed message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VelocityNed {
    /// Velocity along north direction in metres per second
    #[prost(float, tag="1")]
    pub north_m_s: f32,
    /// Velocity along east direction in metres per second
    #[prost(float, tag="2")]
    pub east_m_s: f32,
    /// Velocity along down direction in metres per second
    #[prost(float, tag="3")]
    pub down_m_s: f32,
}
/// PositionVelocityNed message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionVelocityNed {
    /// Position (NED)
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<PositionNed>,
    /// Velocity (NED)
    #[prost(message, optional, tag="2")]
    pub velocity: ::core::option::Option<VelocityNed>,
}
/// GroundTruth message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroundTruth {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag="1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to 180)
    #[prost(double, tag="2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag="3")]
    pub absolute_altitude_m: f32,
}
/// FixedwingMetrics message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedwingMetrics {
    /// Current indicated airspeed (IAS) in metres per second
    #[prost(float, tag="1")]
    pub airspeed_m_s: f32,
    /// Current throttle setting (0 to 100)
    #[prost(float, tag="2")]
    pub throttle_percentage: f32,
    /// Current climb rate in metres per second
    #[prost(float, tag="3")]
    pub climb_rate_m_s: f32,
}
/// AccelerationFrd message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccelerationFrd {
    /// Acceleration in forward direction in metres per second^2
    #[prost(float, tag="1")]
    pub forward_m_s2: f32,
    /// Acceleration in right direction in metres per second^2
    #[prost(float, tag="2")]
    pub right_m_s2: f32,
    /// Acceleration in down direction in metres per second^2
    #[prost(float, tag="3")]
    pub down_m_s2: f32,
}
/// AngularVelocityFrd message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityFrd {
    /// Angular velocity in forward direction in radians per second
    #[prost(float, tag="1")]
    pub forward_rad_s: f32,
    /// Angular velocity in right direction in radians per second
    #[prost(float, tag="2")]
    pub right_rad_s: f32,
    /// Angular velocity in Down direction in radians per second
    #[prost(float, tag="3")]
    pub down_rad_s: f32,
}
/// MagneticFieldFrd message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MagneticFieldFrd {
    /// Magnetic field in forward direction measured in Gauss
    #[prost(float, tag="1")]
    pub forward_gauss: f32,
    /// Magnetic field in East direction measured in Gauss
    #[prost(float, tag="2")]
    pub right_gauss: f32,
    /// Magnetic field in Down direction measured in Gauss
    #[prost(float, tag="3")]
    pub down_gauss: f32,
}
/// Imu message type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Imu {
    /// Acceleration
    #[prost(message, optional, tag="1")]
    pub acceleration_frd: ::core::option::Option<AccelerationFrd>,
    /// Angular velocity
    #[prost(message, optional, tag="2")]
    pub angular_velocity_frd: ::core::option::Option<AngularVelocityFrd>,
    /// Magnetic field
    #[prost(message, optional, tag="3")]
    pub magnetic_field_frd: ::core::option::Option<MagneticFieldFrd>,
    /// Temperature
    #[prost(float, tag="4")]
    pub temperature_degc: f32,
    /// Timestamp in microseconds
    #[prost(uint64, tag="5")]
    pub timestamp_us: u64,
}
/// Gps global origin type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpsGlobalOrigin {
    /// Latitude of the origin
    #[prost(double, tag="1")]
    pub latitude_deg: f64,
    /// Longitude of the origin
    #[prost(double, tag="2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag="3")]
    pub altitude_m: f32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TelemetryResult {
    /// Result enum value
    #[prost(enumeration="telemetry_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TelemetryResult`.
pub mod telemetry_result {
    /// Possible results returned for telemetry requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unknown => "RESULT_UNKNOWN",
                Result::Success => "RESULT_SUCCESS",
                Result::NoSystem => "RESULT_NO_SYSTEM",
                Result::ConnectionError => "RESULT_CONNECTION_ERROR",
                Result::Busy => "RESULT_BUSY",
                Result::CommandDenied => "RESULT_COMMAND_DENIED",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::Unsupported => "RESULT_UNSUPPORTED",
            }
        }
    }
}
/// GPS fix type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl FixType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FixType::NoGps => "FIX_TYPE_NO_GPS",
            FixType::NoFix => "FIX_TYPE_NO_FIX",
            FixType::Fix2d => "FIX_TYPE_FIX_2D",
            FixType::Fix3d => "FIX_TYPE_FIX_3D",
            FixType::FixDgps => "FIX_TYPE_FIX_DGPS",
            FixType::RtkFloat => "FIX_TYPE_RTK_FLOAT",
            FixType::RtkFixed => "FIX_TYPE_RTK_FIXED",
        }
    }
}
///
/// Flight modes.
///
/// For more information about flight modes, check out
/// <https://docs.px4.io/master/en/config/flight_mode.html.>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlightMode {
    /// Mode not known
    Unknown = 0,
    /// Armed and ready to take off
    Ready = 1,
    /// Taking off
    Takeoff = 2,
    /// Holding (hovering in place (or circling for fixed-wing vehicles)
    Hold = 3,
    /// In mission
    Mission = 4,
    /// Returning to launch position (then landing)
    ReturnToLaunch = 5,
    /// Landing
    Land = 6,
    /// In 'offboard' mode
    Offboard = 7,
    /// In 'follow-me' mode
    FollowMe = 8,
    /// In 'Manual' mode
    Manual = 9,
    /// In 'Altitude Control' mode
    Altctl = 10,
    /// In 'Position Control' mode
    Posctl = 11,
    /// In 'Acro' mode
    Acro = 12,
    /// In 'Stabilize' mode
    Stabilized = 13,
    /// In 'Rattitude' mode
    Rattitude = 14,
}
impl FlightMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlightMode::Unknown => "FLIGHT_MODE_UNKNOWN",
            FlightMode::Ready => "FLIGHT_MODE_READY",
            FlightMode::Takeoff => "FLIGHT_MODE_TAKEOFF",
            FlightMode::Hold => "FLIGHT_MODE_HOLD",
            FlightMode::Mission => "FLIGHT_MODE_MISSION",
            FlightMode::ReturnToLaunch => "FLIGHT_MODE_RETURN_TO_LAUNCH",
            FlightMode::Land => "FLIGHT_MODE_LAND",
            FlightMode::Offboard => "FLIGHT_MODE_OFFBOARD",
            FlightMode::FollowMe => "FLIGHT_MODE_FOLLOW_ME",
            FlightMode::Manual => "FLIGHT_MODE_MANUAL",
            FlightMode::Altctl => "FLIGHT_MODE_ALTCTL",
            FlightMode::Posctl => "FLIGHT_MODE_POSCTL",
            FlightMode::Acro => "FLIGHT_MODE_ACRO",
            FlightMode::Stabilized => "FLIGHT_MODE_STABILIZED",
            FlightMode::Rattitude => "FLIGHT_MODE_RATTITUDE",
        }
    }
}
/// Status types.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl StatusTextType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StatusTextType::Debug => "STATUS_TEXT_TYPE_DEBUG",
            StatusTextType::Info => "STATUS_TEXT_TYPE_INFO",
            StatusTextType::Notice => "STATUS_TEXT_TYPE_NOTICE",
            StatusTextType::Warning => "STATUS_TEXT_TYPE_WARNING",
            StatusTextType::Error => "STATUS_TEXT_TYPE_ERROR",
            StatusTextType::Critical => "STATUS_TEXT_TYPE_CRITICAL",
            StatusTextType::Alert => "STATUS_TEXT_TYPE_ALERT",
            StatusTextType::Emergency => "STATUS_TEXT_TYPE_EMERGENCY",
        }
    }
}
/// Landed State enumeration.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl LandedState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LandedState::Unknown => "LANDED_STATE_UNKNOWN",
            LandedState::OnGround => "LANDED_STATE_ON_GROUND",
            LandedState::InAir => "LANDED_STATE_IN_AIR",
            LandedState::TakingOff => "LANDED_STATE_TAKING_OFF",
            LandedState::Landing => "LANDED_STATE_LANDING",
        }
    }
}
/// VTOL State enumeration
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VtolState {
    /// MAV is not configured as VTOL
    Undefined = 0,
    /// VTOL is in transition from multicopter to fixed-wing
    TransitionToFw = 1,
    /// VTOL is in transition from fixed-wing to multicopter
    TransitionToMc = 2,
    /// VTOL is in multicopter state
    Mc = 3,
    /// VTOL is in fixed-wing state
    Fw = 4,
}
impl VtolState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VtolState::Undefined => "VTOL_STATE_UNDEFINED",
            VtolState::TransitionToFw => "VTOL_STATE_TRANSITION_TO_FW",
            VtolState::TransitionToMc => "VTOL_STATE_TRANSITION_TO_MC",
            VtolState::Mc => "VTOL_STATE_MC",
            VtolState::Fw => "VTOL_STATE_FW",
        }
    }
}
/// Generated client implementations.
pub mod telemetry_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    /// Allow users to get vehicle telemetry and state information
    /// (e.g. battery, GPS, RC connection, flight mode etc.) and set telemetry update rates.
    #[derive(Debug, Clone)]
    pub struct TelemetryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TelemetryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TelemetryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TelemetryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TelemetryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Subscribe to 'position' updates.
        pub async fn subscribe_position(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePositionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PositionResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribePosition",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'home position' updates.
        pub async fn subscribe_home(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHomeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HomeResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHome",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to in-air updates.
        pub async fn subscribe_in_air(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeInAirRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::InAirResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeInAir",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to landed state updates
        pub async fn subscribe_landed_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeLandedStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::LandedStateResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeLandedState",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to armed updates.
        pub async fn subscribe_armed(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeArmedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ArmedResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeArmed",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// subscribe to vtol state Updates
        pub async fn subscribe_vtol_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeVtolStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VtolStateResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeVtolState",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'attitude' updates (quaternion).
        pub async fn subscribe_attitude_quaternion(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeAttitudeQuaternionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AttitudeQuaternionResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeQuaternion",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'attitude' updates (Euler).
        pub async fn subscribe_attitude_euler(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeAttitudeEulerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AttitudeEulerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeEuler",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'attitude' updates (angular velocity)
        pub async fn subscribe_attitude_angular_velocity_body(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeAttitudeAngularVelocityBodyRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::AttitudeAngularVelocityBodyResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeAngularVelocityBody",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'camera attitude' updates (quaternion).
        pub async fn subscribe_camera_attitude_quaternion(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeCameraAttitudeQuaternionRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::CameraAttitudeQuaternionResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeQuaternion",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'camera attitude' updates (Euler).
        pub async fn subscribe_camera_attitude_euler(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCameraAttitudeEulerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CameraAttitudeEulerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeEuler",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'ground speed' updates (NED).
        pub async fn subscribe_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeVelocityNedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VelocityNedResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeVelocityNed",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'GPS info' updates.
        pub async fn subscribe_gps_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeGpsInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GpsInfoResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGpsInfo",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'Raw GPS' updates.
        pub async fn subscribe_raw_gps(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRawGpsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RawGpsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRawGps",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'battery' updates.
        pub async fn subscribe_battery(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeBatteryRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::BatteryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeBattery",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'flight mode' updates.
        pub async fn subscribe_flight_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeFlightModeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::FlightModeResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeFlightMode",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'health' updates.
        pub async fn subscribe_health(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHealthRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HealthResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHealth",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'RC status' updates.
        pub async fn subscribe_rc_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRcStatusRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RcStatusResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRcStatus",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'status text' updates.
        pub async fn subscribe_status_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeStatusTextRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StatusTextResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeStatusText",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'actuator control target' updates.
        pub async fn subscribe_actuator_control_target(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeActuatorControlTargetRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::ActuatorControlTargetResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorControlTarget",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'actuator output status' updates.
        pub async fn subscribe_actuator_output_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeActuatorOutputStatusRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::ActuatorOutputStatusResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorOutputStatus",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'odometry' updates.
        pub async fn subscribe_odometry(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeOdometryRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::OdometryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeOdometry",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'position velocity' updates.
        pub async fn subscribe_position_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePositionVelocityNedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PositionVelocityNedResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribePositionVelocityNed",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'ground truth' updates.
        pub async fn subscribe_ground_truth(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeGroundTruthRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::GroundTruthResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGroundTruth",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'fixedwing metrics' updates.
        pub async fn subscribe_fixedwing_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeFixedwingMetricsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::FixedwingMetricsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeFixedwingMetrics",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'IMU' updates (in SI units in NED body frame).
        pub async fn subscribe_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeImuRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ImuResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeImu",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'Scaled IMU' updates.
        pub async fn subscribe_scaled_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeScaledImuRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ScaledImuResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeScaledImu",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'Raw IMU' updates.
        pub async fn subscribe_raw_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRawImuRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RawImuResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRawImu",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'HealthAllOk' updates.
        pub async fn subscribe_health_all_ok(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHealthAllOkRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HealthAllOkResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHealthAllOk",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'unix epoch time' updates.
        pub async fn subscribe_unix_epoch_time(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeUnixEpochTimeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::UnixEpochTimeResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeUnixEpochTime",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'Distance Sensor' updates.
        pub async fn subscribe_distance_sensor(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeDistanceSensorRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DistanceSensorResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeDistanceSensor",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'Scaled Pressure' updates.
        pub async fn subscribe_scaled_pressure(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeScaledPressureRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ScaledPressureResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeScaledPressure",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to 'Heading' updates.
        pub async fn subscribe_heading(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHeadingRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HeadingResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHeading",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Set rate to 'position' updates.
        pub async fn set_rate_position(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRatePositionRequest>,
        ) -> Result<tonic::Response<super::SetRatePositionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRatePosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'home position' updates.
        pub async fn set_rate_home(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateHomeRequest>,
        ) -> Result<tonic::Response<super::SetRateHomeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateHome",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to in-air updates.
        pub async fn set_rate_in_air(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateInAirRequest>,
        ) -> Result<tonic::Response<super::SetRateInAirResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateInAir",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to landed state updates
        pub async fn set_rate_landed_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateLandedStateRequest>,
        ) -> Result<tonic::Response<super::SetRateLandedStateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateLandedState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to VTOL state updates
        pub async fn set_rate_vtol_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateVtolStateRequest>,
        ) -> Result<tonic::Response<super::SetRateVtolStateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateVtolState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'attitude' updates.
        pub async fn set_rate_attitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateAttitudeRequest>,
        ) -> Result<tonic::Response<super::SetRateAttitudeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateAttitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate of camera attitude updates.
        pub async fn set_rate_camera_attitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateCameraAttitudeRequest>,
        ) -> Result<
            tonic::Response<super::SetRateCameraAttitudeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateCameraAttitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'ground speed' updates (NED).
        pub async fn set_rate_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetRateVelocityNedResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateVelocityNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'GPS info' updates.
        pub async fn set_rate_gps_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateGpsInfoRequest>,
        ) -> Result<tonic::Response<super::SetRateGpsInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateGpsInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'battery' updates.
        pub async fn set_rate_battery(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateBatteryRequest>,
        ) -> Result<tonic::Response<super::SetRateBatteryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateBattery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'RC status' updates.
        pub async fn set_rate_rc_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateRcStatusRequest>,
        ) -> Result<tonic::Response<super::SetRateRcStatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateRcStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'actuator control target' updates.
        pub async fn set_rate_actuator_control_target(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateActuatorControlTargetRequest>,
        ) -> Result<
            tonic::Response<super::SetRateActuatorControlTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateActuatorControlTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'actuator output status' updates.
        pub async fn set_rate_actuator_output_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateActuatorOutputStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetRateActuatorOutputStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateActuatorOutputStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'odometry' updates.
        pub async fn set_rate_odometry(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateOdometryRequest>,
        ) -> Result<tonic::Response<super::SetRateOdometryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateOdometry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'position velocity' updates.
        pub async fn set_rate_position_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRatePositionVelocityNedRequest>,
        ) -> Result<
            tonic::Response<super::SetRatePositionVelocityNedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRatePositionVelocityNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'ground truth' updates.
        pub async fn set_rate_ground_truth(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateGroundTruthRequest>,
        ) -> Result<tonic::Response<super::SetRateGroundTruthResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateGroundTruth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'fixedwing metrics' updates.
        pub async fn set_rate_fixedwing_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateFixedwingMetricsRequest>,
        ) -> Result<
            tonic::Response<super::SetRateFixedwingMetricsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateFixedwingMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'IMU' updates.
        pub async fn set_rate_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateImuRequest>,
        ) -> Result<tonic::Response<super::SetRateImuResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateImu",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'Scaled IMU' updates.
        pub async fn set_rate_scaled_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateScaledImuRequest>,
        ) -> Result<tonic::Response<super::SetRateScaledImuResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateScaledImu",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'Raw IMU' updates.
        pub async fn set_rate_raw_imu(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateRawImuRequest>,
        ) -> Result<tonic::Response<super::SetRateRawImuResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateRawImu",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'unix epoch time' updates.
        pub async fn set_rate_unix_epoch_time(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateUnixEpochTimeRequest>,
        ) -> Result<
            tonic::Response<super::SetRateUnixEpochTimeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateUnixEpochTime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set rate to 'Distance Sensor' updates.
        pub async fn set_rate_distance_sensor(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateDistanceSensorRequest>,
        ) -> Result<
            tonic::Response<super::SetRateDistanceSensorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateDistanceSensor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the GPS location of where the estimator has been initialized.
        pub async fn get_gps_global_origin(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGpsGlobalOriginRequest>,
        ) -> Result<tonic::Response<super::GetGpsGlobalOriginResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.telemetry.TelemetryService/GetGpsGlobalOrigin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod telemetry_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TelemetryServiceServer.
    #[async_trait]
    pub trait TelemetryService: Send + Sync + 'static {
        ///Server streaming response type for the SubscribePosition method.
        type SubscribePositionStream: futures_core::Stream<
                Item = Result<super::PositionResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'position' updates.
        async fn subscribe_position(
            &self,
            request: tonic::Request<super::SubscribePositionRequest>,
        ) -> Result<tonic::Response<Self::SubscribePositionStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeHome method.
        type SubscribeHomeStream: futures_core::Stream<
                Item = Result<super::HomeResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'home position' updates.
        async fn subscribe_home(
            &self,
            request: tonic::Request<super::SubscribeHomeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeHomeStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeInAir method.
        type SubscribeInAirStream: futures_core::Stream<
                Item = Result<super::InAirResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to in-air updates.
        async fn subscribe_in_air(
            &self,
            request: tonic::Request<super::SubscribeInAirRequest>,
        ) -> Result<tonic::Response<Self::SubscribeInAirStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeLandedState method.
        type SubscribeLandedStateStream: futures_core::Stream<
                Item = Result<super::LandedStateResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to landed state updates
        async fn subscribe_landed_state(
            &self,
            request: tonic::Request<super::SubscribeLandedStateRequest>,
        ) -> Result<tonic::Response<Self::SubscribeLandedStateStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeArmed method.
        type SubscribeArmedStream: futures_core::Stream<
                Item = Result<super::ArmedResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to armed updates.
        async fn subscribe_armed(
            &self,
            request: tonic::Request<super::SubscribeArmedRequest>,
        ) -> Result<tonic::Response<Self::SubscribeArmedStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeVtolState method.
        type SubscribeVtolStateStream: futures_core::Stream<
                Item = Result<super::VtolStateResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// subscribe to vtol state Updates
        async fn subscribe_vtol_state(
            &self,
            request: tonic::Request<super::SubscribeVtolStateRequest>,
        ) -> Result<tonic::Response<Self::SubscribeVtolStateStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeAttitudeQuaternion method.
        type SubscribeAttitudeQuaternionStream: futures_core::Stream<
                Item = Result<super::AttitudeQuaternionResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'attitude' updates (quaternion).
        async fn subscribe_attitude_quaternion(
            &self,
            request: tonic::Request<super::SubscribeAttitudeQuaternionRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeAttitudeQuaternionStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeAttitudeEuler method.
        type SubscribeAttitudeEulerStream: futures_core::Stream<
                Item = Result<super::AttitudeEulerResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'attitude' updates (Euler).
        async fn subscribe_attitude_euler(
            &self,
            request: tonic::Request<super::SubscribeAttitudeEulerRequest>,
        ) -> Result<tonic::Response<Self::SubscribeAttitudeEulerStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeAttitudeAngularVelocityBody method.
        type SubscribeAttitudeAngularVelocityBodyStream: futures_core::Stream<
                Item = Result<super::AttitudeAngularVelocityBodyResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'attitude' updates (angular velocity)
        async fn subscribe_attitude_angular_velocity_body(
            &self,
            request: tonic::Request<super::SubscribeAttitudeAngularVelocityBodyRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeAttitudeAngularVelocityBodyStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeCameraAttitudeQuaternion method.
        type SubscribeCameraAttitudeQuaternionStream: futures_core::Stream<
                Item = Result<super::CameraAttitudeQuaternionResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'camera attitude' updates (quaternion).
        async fn subscribe_camera_attitude_quaternion(
            &self,
            request: tonic::Request<super::SubscribeCameraAttitudeQuaternionRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeCameraAttitudeQuaternionStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeCameraAttitudeEuler method.
        type SubscribeCameraAttitudeEulerStream: futures_core::Stream<
                Item = Result<super::CameraAttitudeEulerResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'camera attitude' updates (Euler).
        async fn subscribe_camera_attitude_euler(
            &self,
            request: tonic::Request<super::SubscribeCameraAttitudeEulerRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeCameraAttitudeEulerStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeVelocityNed method.
        type SubscribeVelocityNedStream: futures_core::Stream<
                Item = Result<super::VelocityNedResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'ground speed' updates (NED).
        async fn subscribe_velocity_ned(
            &self,
            request: tonic::Request<super::SubscribeVelocityNedRequest>,
        ) -> Result<tonic::Response<Self::SubscribeVelocityNedStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeGpsInfo method.
        type SubscribeGpsInfoStream: futures_core::Stream<
                Item = Result<super::GpsInfoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'GPS info' updates.
        async fn subscribe_gps_info(
            &self,
            request: tonic::Request<super::SubscribeGpsInfoRequest>,
        ) -> Result<tonic::Response<Self::SubscribeGpsInfoStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeRawGps method.
        type SubscribeRawGpsStream: futures_core::Stream<
                Item = Result<super::RawGpsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'Raw GPS' updates.
        async fn subscribe_raw_gps(
            &self,
            request: tonic::Request<super::SubscribeRawGpsRequest>,
        ) -> Result<tonic::Response<Self::SubscribeRawGpsStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeBattery method.
        type SubscribeBatteryStream: futures_core::Stream<
                Item = Result<super::BatteryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'battery' updates.
        async fn subscribe_battery(
            &self,
            request: tonic::Request<super::SubscribeBatteryRequest>,
        ) -> Result<tonic::Response<Self::SubscribeBatteryStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeFlightMode method.
        type SubscribeFlightModeStream: futures_core::Stream<
                Item = Result<super::FlightModeResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'flight mode' updates.
        async fn subscribe_flight_mode(
            &self,
            request: tonic::Request<super::SubscribeFlightModeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeFlightModeStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeHealth method.
        type SubscribeHealthStream: futures_core::Stream<
                Item = Result<super::HealthResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'health' updates.
        async fn subscribe_health(
            &self,
            request: tonic::Request<super::SubscribeHealthRequest>,
        ) -> Result<tonic::Response<Self::SubscribeHealthStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeRcStatus method.
        type SubscribeRcStatusStream: futures_core::Stream<
                Item = Result<super::RcStatusResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'RC status' updates.
        async fn subscribe_rc_status(
            &self,
            request: tonic::Request<super::SubscribeRcStatusRequest>,
        ) -> Result<tonic::Response<Self::SubscribeRcStatusStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeStatusText method.
        type SubscribeStatusTextStream: futures_core::Stream<
                Item = Result<super::StatusTextResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'status text' updates.
        async fn subscribe_status_text(
            &self,
            request: tonic::Request<super::SubscribeStatusTextRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStatusTextStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeActuatorControlTarget method.
        type SubscribeActuatorControlTargetStream: futures_core::Stream<
                Item = Result<super::ActuatorControlTargetResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'actuator control target' updates.
        async fn subscribe_actuator_control_target(
            &self,
            request: tonic::Request<super::SubscribeActuatorControlTargetRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeActuatorControlTargetStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeActuatorOutputStatus method.
        type SubscribeActuatorOutputStatusStream: futures_core::Stream<
                Item = Result<super::ActuatorOutputStatusResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'actuator output status' updates.
        async fn subscribe_actuator_output_status(
            &self,
            request: tonic::Request<super::SubscribeActuatorOutputStatusRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeActuatorOutputStatusStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeOdometry method.
        type SubscribeOdometryStream: futures_core::Stream<
                Item = Result<super::OdometryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'odometry' updates.
        async fn subscribe_odometry(
            &self,
            request: tonic::Request<super::SubscribeOdometryRequest>,
        ) -> Result<tonic::Response<Self::SubscribeOdometryStream>, tonic::Status>;
        ///Server streaming response type for the SubscribePositionVelocityNed method.
        type SubscribePositionVelocityNedStream: futures_core::Stream<
                Item = Result<super::PositionVelocityNedResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'position velocity' updates.
        async fn subscribe_position_velocity_ned(
            &self,
            request: tonic::Request<super::SubscribePositionVelocityNedRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribePositionVelocityNedStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeGroundTruth method.
        type SubscribeGroundTruthStream: futures_core::Stream<
                Item = Result<super::GroundTruthResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'ground truth' updates.
        async fn subscribe_ground_truth(
            &self,
            request: tonic::Request<super::SubscribeGroundTruthRequest>,
        ) -> Result<tonic::Response<Self::SubscribeGroundTruthStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeFixedwingMetrics method.
        type SubscribeFixedwingMetricsStream: futures_core::Stream<
                Item = Result<super::FixedwingMetricsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'fixedwing metrics' updates.
        async fn subscribe_fixedwing_metrics(
            &self,
            request: tonic::Request<super::SubscribeFixedwingMetricsRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeFixedwingMetricsStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeImu method.
        type SubscribeImuStream: futures_core::Stream<
                Item = Result<super::ImuResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'IMU' updates (in SI units in NED body frame).
        async fn subscribe_imu(
            &self,
            request: tonic::Request<super::SubscribeImuRequest>,
        ) -> Result<tonic::Response<Self::SubscribeImuStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeScaledImu method.
        type SubscribeScaledImuStream: futures_core::Stream<
                Item = Result<super::ScaledImuResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'Scaled IMU' updates.
        async fn subscribe_scaled_imu(
            &self,
            request: tonic::Request<super::SubscribeScaledImuRequest>,
        ) -> Result<tonic::Response<Self::SubscribeScaledImuStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeRawImu method.
        type SubscribeRawImuStream: futures_core::Stream<
                Item = Result<super::RawImuResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'Raw IMU' updates.
        async fn subscribe_raw_imu(
            &self,
            request: tonic::Request<super::SubscribeRawImuRequest>,
        ) -> Result<tonic::Response<Self::SubscribeRawImuStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeHealthAllOk method.
        type SubscribeHealthAllOkStream: futures_core::Stream<
                Item = Result<super::HealthAllOkResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'HealthAllOk' updates.
        async fn subscribe_health_all_ok(
            &self,
            request: tonic::Request<super::SubscribeHealthAllOkRequest>,
        ) -> Result<tonic::Response<Self::SubscribeHealthAllOkStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeUnixEpochTime method.
        type SubscribeUnixEpochTimeStream: futures_core::Stream<
                Item = Result<super::UnixEpochTimeResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'unix epoch time' updates.
        async fn subscribe_unix_epoch_time(
            &self,
            request: tonic::Request<super::SubscribeUnixEpochTimeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeUnixEpochTimeStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeDistanceSensor method.
        type SubscribeDistanceSensorStream: futures_core::Stream<
                Item = Result<super::DistanceSensorResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'Distance Sensor' updates.
        async fn subscribe_distance_sensor(
            &self,
            request: tonic::Request<super::SubscribeDistanceSensorRequest>,
        ) -> Result<tonic::Response<Self::SubscribeDistanceSensorStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeScaledPressure method.
        type SubscribeScaledPressureStream: futures_core::Stream<
                Item = Result<super::ScaledPressureResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'Scaled Pressure' updates.
        async fn subscribe_scaled_pressure(
            &self,
            request: tonic::Request<super::SubscribeScaledPressureRequest>,
        ) -> Result<tonic::Response<Self::SubscribeScaledPressureStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeHeading method.
        type SubscribeHeadingStream: futures_core::Stream<
                Item = Result<super::HeadingResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'Heading' updates.
        async fn subscribe_heading(
            &self,
            request: tonic::Request<super::SubscribeHeadingRequest>,
        ) -> Result<tonic::Response<Self::SubscribeHeadingStream>, tonic::Status>;
        /// Set rate to 'position' updates.
        async fn set_rate_position(
            &self,
            request: tonic::Request<super::SetRatePositionRequest>,
        ) -> Result<tonic::Response<super::SetRatePositionResponse>, tonic::Status>;
        /// Set rate to 'home position' updates.
        async fn set_rate_home(
            &self,
            request: tonic::Request<super::SetRateHomeRequest>,
        ) -> Result<tonic::Response<super::SetRateHomeResponse>, tonic::Status>;
        /// Set rate to in-air updates.
        async fn set_rate_in_air(
            &self,
            request: tonic::Request<super::SetRateInAirRequest>,
        ) -> Result<tonic::Response<super::SetRateInAirResponse>, tonic::Status>;
        /// Set rate to landed state updates
        async fn set_rate_landed_state(
            &self,
            request: tonic::Request<super::SetRateLandedStateRequest>,
        ) -> Result<tonic::Response<super::SetRateLandedStateResponse>, tonic::Status>;
        /// Set rate to VTOL state updates
        async fn set_rate_vtol_state(
            &self,
            request: tonic::Request<super::SetRateVtolStateRequest>,
        ) -> Result<tonic::Response<super::SetRateVtolStateResponse>, tonic::Status>;
        /// Set rate to 'attitude' updates.
        async fn set_rate_attitude(
            &self,
            request: tonic::Request<super::SetRateAttitudeRequest>,
        ) -> Result<tonic::Response<super::SetRateAttitudeResponse>, tonic::Status>;
        /// Set rate of camera attitude updates.
        async fn set_rate_camera_attitude(
            &self,
            request: tonic::Request<super::SetRateCameraAttitudeRequest>,
        ) -> Result<
            tonic::Response<super::SetRateCameraAttitudeResponse>,
            tonic::Status,
        >;
        /// Set rate to 'ground speed' updates (NED).
        async fn set_rate_velocity_ned(
            &self,
            request: tonic::Request<super::SetRateVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetRateVelocityNedResponse>, tonic::Status>;
        /// Set rate to 'GPS info' updates.
        async fn set_rate_gps_info(
            &self,
            request: tonic::Request<super::SetRateGpsInfoRequest>,
        ) -> Result<tonic::Response<super::SetRateGpsInfoResponse>, tonic::Status>;
        /// Set rate to 'battery' updates.
        async fn set_rate_battery(
            &self,
            request: tonic::Request<super::SetRateBatteryRequest>,
        ) -> Result<tonic::Response<super::SetRateBatteryResponse>, tonic::Status>;
        /// Set rate to 'RC status' updates.
        async fn set_rate_rc_status(
            &self,
            request: tonic::Request<super::SetRateRcStatusRequest>,
        ) -> Result<tonic::Response<super::SetRateRcStatusResponse>, tonic::Status>;
        /// Set rate to 'actuator control target' updates.
        async fn set_rate_actuator_control_target(
            &self,
            request: tonic::Request<super::SetRateActuatorControlTargetRequest>,
        ) -> Result<
            tonic::Response<super::SetRateActuatorControlTargetResponse>,
            tonic::Status,
        >;
        /// Set rate to 'actuator output status' updates.
        async fn set_rate_actuator_output_status(
            &self,
            request: tonic::Request<super::SetRateActuatorOutputStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetRateActuatorOutputStatusResponse>,
            tonic::Status,
        >;
        /// Set rate to 'odometry' updates.
        async fn set_rate_odometry(
            &self,
            request: tonic::Request<super::SetRateOdometryRequest>,
        ) -> Result<tonic::Response<super::SetRateOdometryResponse>, tonic::Status>;
        /// Set rate to 'position velocity' updates.
        async fn set_rate_position_velocity_ned(
            &self,
            request: tonic::Request<super::SetRatePositionVelocityNedRequest>,
        ) -> Result<
            tonic::Response<super::SetRatePositionVelocityNedResponse>,
            tonic::Status,
        >;
        /// Set rate to 'ground truth' updates.
        async fn set_rate_ground_truth(
            &self,
            request: tonic::Request<super::SetRateGroundTruthRequest>,
        ) -> Result<tonic::Response<super::SetRateGroundTruthResponse>, tonic::Status>;
        /// Set rate to 'fixedwing metrics' updates.
        async fn set_rate_fixedwing_metrics(
            &self,
            request: tonic::Request<super::SetRateFixedwingMetricsRequest>,
        ) -> Result<
            tonic::Response<super::SetRateFixedwingMetricsResponse>,
            tonic::Status,
        >;
        /// Set rate to 'IMU' updates.
        async fn set_rate_imu(
            &self,
            request: tonic::Request<super::SetRateImuRequest>,
        ) -> Result<tonic::Response<super::SetRateImuResponse>, tonic::Status>;
        /// Set rate to 'Scaled IMU' updates.
        async fn set_rate_scaled_imu(
            &self,
            request: tonic::Request<super::SetRateScaledImuRequest>,
        ) -> Result<tonic::Response<super::SetRateScaledImuResponse>, tonic::Status>;
        /// Set rate to 'Raw IMU' updates.
        async fn set_rate_raw_imu(
            &self,
            request: tonic::Request<super::SetRateRawImuRequest>,
        ) -> Result<tonic::Response<super::SetRateRawImuResponse>, tonic::Status>;
        /// Set rate to 'unix epoch time' updates.
        async fn set_rate_unix_epoch_time(
            &self,
            request: tonic::Request<super::SetRateUnixEpochTimeRequest>,
        ) -> Result<tonic::Response<super::SetRateUnixEpochTimeResponse>, tonic::Status>;
        /// Set rate to 'Distance Sensor' updates.
        async fn set_rate_distance_sensor(
            &self,
            request: tonic::Request<super::SetRateDistanceSensorRequest>,
        ) -> Result<
            tonic::Response<super::SetRateDistanceSensorResponse>,
            tonic::Status,
        >;
        /// Get the GPS location of where the estimator has been initialized.
        async fn get_gps_global_origin(
            &self,
            request: tonic::Request<super::GetGpsGlobalOriginRequest>,
        ) -> Result<tonic::Response<super::GetGpsGlobalOriginResponse>, tonic::Status>;
    }
    ///
    /// Allow users to get vehicle telemetry and state information
    /// (e.g. battery, GPS, RC connection, flight mode etc.) and set telemetry update rates.
    #[derive(Debug)]
    pub struct TelemetryServiceServer<T: TelemetryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TelemetryService> TelemetryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TelemetryServiceServer<T>
    where
        T: TelemetryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribePosition" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribePositionSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribePositionRequest,
                    > for SubscribePositionSvc<T> {
                        type Response = super::PositionResponse;
                        type ResponseStream = T::SubscribePositionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribePositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_position(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribePositionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHome" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeHomeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<super::SubscribeHomeRequest>
                    for SubscribeHomeSvc<T> {
                        type Response = super::HomeResponse;
                        type ResponseStream = T::SubscribeHomeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeHomeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_home(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeHomeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeInAir" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeInAirSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<super::SubscribeInAirRequest>
                    for SubscribeInAirSvc<T> {
                        type Response = super::InAirResponse;
                        type ResponseStream = T::SubscribeInAirStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeInAirRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_in_air(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeInAirSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeLandedState" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeLandedStateSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeLandedStateRequest,
                    > for SubscribeLandedStateSvc<T> {
                        type Response = super::LandedStateResponse;
                        type ResponseStream = T::SubscribeLandedStateStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeLandedStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_landed_state(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeLandedStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeArmed" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeArmedSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<super::SubscribeArmedRequest>
                    for SubscribeArmedSvc<T> {
                        type Response = super::ArmedResponse;
                        type ResponseStream = T::SubscribeArmedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeArmedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_armed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeArmedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeVtolState" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeVtolStateSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeVtolStateRequest,
                    > for SubscribeVtolStateSvc<T> {
                        type Response = super::VtolStateResponse;
                        type ResponseStream = T::SubscribeVtolStateStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeVtolStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_vtol_state(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeVtolStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeQuaternion" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeAttitudeQuaternionSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeAttitudeQuaternionRequest,
                    > for SubscribeAttitudeQuaternionSvc<T> {
                        type Response = super::AttitudeQuaternionResponse;
                        type ResponseStream = T::SubscribeAttitudeQuaternionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeAttitudeQuaternionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_attitude_quaternion(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeAttitudeQuaternionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeEuler" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeAttitudeEulerSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeAttitudeEulerRequest,
                    > for SubscribeAttitudeEulerSvc<T> {
                        type Response = super::AttitudeEulerResponse;
                        type ResponseStream = T::SubscribeAttitudeEulerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeAttitudeEulerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_attitude_euler(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeAttitudeEulerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeAttitudeAngularVelocityBody" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeAttitudeAngularVelocityBodySvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeAttitudeAngularVelocityBodyRequest,
                    > for SubscribeAttitudeAngularVelocityBodySvc<T> {
                        type Response = super::AttitudeAngularVelocityBodyResponse;
                        type ResponseStream = T::SubscribeAttitudeAngularVelocityBodyStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeAttitudeAngularVelocityBodyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .subscribe_attitude_angular_velocity_body(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeAttitudeAngularVelocityBodySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeQuaternion" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCameraAttitudeQuaternionSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeCameraAttitudeQuaternionRequest,
                    > for SubscribeCameraAttitudeQuaternionSvc<T> {
                        type Response = super::CameraAttitudeQuaternionResponse;
                        type ResponseStream = T::SubscribeCameraAttitudeQuaternionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeCameraAttitudeQuaternionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_camera_attitude_quaternion(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCameraAttitudeQuaternionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeCameraAttitudeEuler" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCameraAttitudeEulerSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeCameraAttitudeEulerRequest,
                    > for SubscribeCameraAttitudeEulerSvc<T> {
                        type Response = super::CameraAttitudeEulerResponse;
                        type ResponseStream = T::SubscribeCameraAttitudeEulerStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeCameraAttitudeEulerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_camera_attitude_euler(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCameraAttitudeEulerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeVelocityNed" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeVelocityNedSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeVelocityNedRequest,
                    > for SubscribeVelocityNedSvc<T> {
                        type Response = super::VelocityNedResponse;
                        type ResponseStream = T::SubscribeVelocityNedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeVelocityNedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_velocity_ned(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeVelocityNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGpsInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeGpsInfoSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeGpsInfoRequest,
                    > for SubscribeGpsInfoSvc<T> {
                        type Response = super::GpsInfoResponse;
                        type ResponseStream = T::SubscribeGpsInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeGpsInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_gps_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeGpsInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRawGps" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeRawGpsSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeRawGpsRequest,
                    > for SubscribeRawGpsSvc<T> {
                        type Response = super::RawGpsResponse;
                        type ResponseStream = T::SubscribeRawGpsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRawGpsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_raw_gps(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeRawGpsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeBattery" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeBatterySvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeBatteryRequest,
                    > for SubscribeBatterySvc<T> {
                        type Response = super::BatteryResponse;
                        type ResponseStream = T::SubscribeBatteryStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeBatteryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_battery(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeBatterySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeFlightMode" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeFlightModeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeFlightModeRequest,
                    > for SubscribeFlightModeSvc<T> {
                        type Response = super::FlightModeResponse;
                        type ResponseStream = T::SubscribeFlightModeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeFlightModeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_flight_mode(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeFlightModeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHealth" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeHealthSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeHealthRequest,
                    > for SubscribeHealthSvc<T> {
                        type Response = super::HealthResponse;
                        type ResponseStream = T::SubscribeHealthStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeHealthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_health(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeHealthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRcStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeRcStatusSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeRcStatusRequest,
                    > for SubscribeRcStatusSvc<T> {
                        type Response = super::RcStatusResponse;
                        type ResponseStream = T::SubscribeRcStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRcStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_rc_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeRcStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeStatusText" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeStatusTextSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeStatusTextRequest,
                    > for SubscribeStatusTextSvc<T> {
                        type Response = super::StatusTextResponse;
                        type ResponseStream = T::SubscribeStatusTextStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeStatusTextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_status_text(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeStatusTextSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorControlTarget" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeActuatorControlTargetSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeActuatorControlTargetRequest,
                    > for SubscribeActuatorControlTargetSvc<T> {
                        type Response = super::ActuatorControlTargetResponse;
                        type ResponseStream = T::SubscribeActuatorControlTargetStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeActuatorControlTargetRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_actuator_control_target(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeActuatorControlTargetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeActuatorOutputStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeActuatorOutputStatusSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeActuatorOutputStatusRequest,
                    > for SubscribeActuatorOutputStatusSvc<T> {
                        type Response = super::ActuatorOutputStatusResponse;
                        type ResponseStream = T::SubscribeActuatorOutputStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeActuatorOutputStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_actuator_output_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeActuatorOutputStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeOdometry" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeOdometrySvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeOdometryRequest,
                    > for SubscribeOdometrySvc<T> {
                        type Response = super::OdometryResponse;
                        type ResponseStream = T::SubscribeOdometryStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeOdometryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_odometry(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeOdometrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribePositionVelocityNed" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribePositionVelocityNedSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribePositionVelocityNedRequest,
                    > for SubscribePositionVelocityNedSvc<T> {
                        type Response = super::PositionVelocityNedResponse;
                        type ResponseStream = T::SubscribePositionVelocityNedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribePositionVelocityNedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_position_velocity_ned(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribePositionVelocityNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeGroundTruth" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeGroundTruthSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeGroundTruthRequest,
                    > for SubscribeGroundTruthSvc<T> {
                        type Response = super::GroundTruthResponse;
                        type ResponseStream = T::SubscribeGroundTruthStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeGroundTruthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_ground_truth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeGroundTruthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeFixedwingMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeFixedwingMetricsSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeFixedwingMetricsRequest,
                    > for SubscribeFixedwingMetricsSvc<T> {
                        type Response = super::FixedwingMetricsResponse;
                        type ResponseStream = T::SubscribeFixedwingMetricsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeFixedwingMetricsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_fixedwing_metrics(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeFixedwingMetricsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeImu" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeImuSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<super::SubscribeImuRequest>
                    for SubscribeImuSvc<T> {
                        type Response = super::ImuResponse;
                        type ResponseStream = T::SubscribeImuStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeImuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_imu(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeImuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeScaledImu" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeScaledImuSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeScaledImuRequest,
                    > for SubscribeScaledImuSvc<T> {
                        type Response = super::ScaledImuResponse;
                        type ResponseStream = T::SubscribeScaledImuStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeScaledImuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_scaled_imu(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeScaledImuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeRawImu" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeRawImuSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeRawImuRequest,
                    > for SubscribeRawImuSvc<T> {
                        type Response = super::RawImuResponse;
                        type ResponseStream = T::SubscribeRawImuStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRawImuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_raw_imu(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeRawImuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHealthAllOk" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeHealthAllOkSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeHealthAllOkRequest,
                    > for SubscribeHealthAllOkSvc<T> {
                        type Response = super::HealthAllOkResponse;
                        type ResponseStream = T::SubscribeHealthAllOkStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeHealthAllOkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_health_all_ok(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeHealthAllOkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeUnixEpochTime" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeUnixEpochTimeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeUnixEpochTimeRequest,
                    > for SubscribeUnixEpochTimeSvc<T> {
                        type Response = super::UnixEpochTimeResponse;
                        type ResponseStream = T::SubscribeUnixEpochTimeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeUnixEpochTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_unix_epoch_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeUnixEpochTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeDistanceSensor" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeDistanceSensorSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeDistanceSensorRequest,
                    > for SubscribeDistanceSensorSvc<T> {
                        type Response = super::DistanceSensorResponse;
                        type ResponseStream = T::SubscribeDistanceSensorStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeDistanceSensorRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_distance_sensor(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeDistanceSensorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeScaledPressure" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeScaledPressureSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeScaledPressureRequest,
                    > for SubscribeScaledPressureSvc<T> {
                        type Response = super::ScaledPressureResponse;
                        type ResponseStream = T::SubscribeScaledPressureStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeScaledPressureRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_scaled_pressure(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeScaledPressureSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SubscribeHeading" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeHeadingSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeHeadingRequest,
                    > for SubscribeHeadingSvc<T> {
                        type Response = super::HeadingResponse;
                        type ResponseStream = T::SubscribeHeadingStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeHeadingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_heading(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeHeadingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRatePosition" => {
                    #[allow(non_camel_case_types)]
                    struct SetRatePositionSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRatePositionRequest>
                    for SetRatePositionSvc<T> {
                        type Response = super::SetRatePositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRatePositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_position(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRatePositionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateHome" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateHomeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateHomeRequest>
                    for SetRateHomeSvc<T> {
                        type Response = super::SetRateHomeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateHomeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_home(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateHomeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateInAir" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateInAirSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateInAirRequest>
                    for SetRateInAirSvc<T> {
                        type Response = super::SetRateInAirResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateInAirRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_in_air(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateInAirSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateLandedState" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateLandedStateSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateLandedStateRequest>
                    for SetRateLandedStateSvc<T> {
                        type Response = super::SetRateLandedStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateLandedStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_landed_state(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateLandedStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateVtolState" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateVtolStateSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateVtolStateRequest>
                    for SetRateVtolStateSvc<T> {
                        type Response = super::SetRateVtolStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateVtolStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_vtol_state(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateVtolStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateAttitude" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateAttitudeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateAttitudeRequest>
                    for SetRateAttitudeSvc<T> {
                        type Response = super::SetRateAttitudeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateAttitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_attitude(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateAttitudeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateCameraAttitude" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateCameraAttitudeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateCameraAttitudeRequest>
                    for SetRateCameraAttitudeSvc<T> {
                        type Response = super::SetRateCameraAttitudeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateCameraAttitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_camera_attitude(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateCameraAttitudeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateVelocityNed" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateVelocityNedSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateVelocityNedRequest>
                    for SetRateVelocityNedSvc<T> {
                        type Response = super::SetRateVelocityNedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateVelocityNedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_velocity_ned(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateVelocityNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateGpsInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateGpsInfoSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateGpsInfoRequest>
                    for SetRateGpsInfoSvc<T> {
                        type Response = super::SetRateGpsInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateGpsInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_gps_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateGpsInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateBattery" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateBatterySvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateBatteryRequest>
                    for SetRateBatterySvc<T> {
                        type Response = super::SetRateBatteryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateBatteryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_battery(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateBatterySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateRcStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateRcStatusSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateRcStatusRequest>
                    for SetRateRcStatusSvc<T> {
                        type Response = super::SetRateRcStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateRcStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_rc_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateRcStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateActuatorControlTarget" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateActuatorControlTargetSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<
                        super::SetRateActuatorControlTargetRequest,
                    > for SetRateActuatorControlTargetSvc<T> {
                        type Response = super::SetRateActuatorControlTargetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetRateActuatorControlTargetRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_actuator_control_target(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateActuatorControlTargetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateActuatorOutputStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateActuatorOutputStatusSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<
                        super::SetRateActuatorOutputStatusRequest,
                    > for SetRateActuatorOutputStatusSvc<T> {
                        type Response = super::SetRateActuatorOutputStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetRateActuatorOutputStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_actuator_output_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateActuatorOutputStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateOdometry" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateOdometrySvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateOdometryRequest>
                    for SetRateOdometrySvc<T> {
                        type Response = super::SetRateOdometryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateOdometryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_odometry(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateOdometrySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRatePositionVelocityNed" => {
                    #[allow(non_camel_case_types)]
                    struct SetRatePositionVelocityNedSvc<T: TelemetryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<
                        super::SetRatePositionVelocityNedRequest,
                    > for SetRatePositionVelocityNedSvc<T> {
                        type Response = super::SetRatePositionVelocityNedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetRatePositionVelocityNedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_position_velocity_ned(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRatePositionVelocityNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateGroundTruth" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateGroundTruthSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateGroundTruthRequest>
                    for SetRateGroundTruthSvc<T> {
                        type Response = super::SetRateGroundTruthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateGroundTruthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_ground_truth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateGroundTruthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateFixedwingMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateFixedwingMetricsSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateFixedwingMetricsRequest>
                    for SetRateFixedwingMetricsSvc<T> {
                        type Response = super::SetRateFixedwingMetricsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetRateFixedwingMetricsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_fixedwing_metrics(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateFixedwingMetricsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateImu" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateImuSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateImuRequest>
                    for SetRateImuSvc<T> {
                        type Response = super::SetRateImuResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateImuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_imu(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateImuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateScaledImu" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateScaledImuSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateScaledImuRequest>
                    for SetRateScaledImuSvc<T> {
                        type Response = super::SetRateScaledImuResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateScaledImuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_scaled_imu(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateScaledImuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateRawImu" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateRawImuSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateRawImuRequest>
                    for SetRateRawImuSvc<T> {
                        type Response = super::SetRateRawImuResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateRawImuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_raw_imu(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateRawImuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateUnixEpochTime" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateUnixEpochTimeSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateUnixEpochTimeRequest>
                    for SetRateUnixEpochTimeSvc<T> {
                        type Response = super::SetRateUnixEpochTimeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateUnixEpochTimeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_unix_epoch_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateUnixEpochTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/SetRateDistanceSensor" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateDistanceSensorSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::SetRateDistanceSensorRequest>
                    for SetRateDistanceSensorSvc<T> {
                        type Response = super::SetRateDistanceSensorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateDistanceSensorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_distance_sensor(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateDistanceSensorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.telemetry.TelemetryService/GetGpsGlobalOrigin" => {
                    #[allow(non_camel_case_types)]
                    struct GetGpsGlobalOriginSvc<T: TelemetryService>(pub Arc<T>);
                    impl<
                        T: TelemetryService,
                    > tonic::server::UnaryService<super::GetGpsGlobalOriginRequest>
                    for GetGpsGlobalOriginSvc<T> {
                        type Response = super::GetGpsGlobalOriginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGpsGlobalOriginRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_gps_global_origin(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGpsGlobalOriginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TelemetryService> Clone for TelemetryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TelemetryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TelemetryService> tonic::server::NamedService for TelemetryServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.telemetry.TelemetryService";
    }
}
