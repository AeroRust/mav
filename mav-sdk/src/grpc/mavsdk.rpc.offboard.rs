#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsActiveRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsActiveResponse {
    /// True if offboard is active
    #[prost(bool, tag = "1")]
    pub is_active: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeRequest {
    /// Attitude roll, pitch and yaw along with thrust
    #[prost(message, optional, tag = "1")]
    pub attitude: ::core::option::Option<Attitude>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorControlRequest {
    /// Actuator control values
    #[prost(message, optional, tag = "1")]
    pub actuator_control: ::core::option::Option<ActuatorControl>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorControlResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeRateRequest {
    /// Attitude rate roll, pitch and yaw angular rate along with thrust
    #[prost(message, optional, tag = "1")]
    pub attitude_rate: ::core::option::Option<AttitudeRate>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeRateResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPositionNedRequest {
    /// Position and yaw
    #[prost(message, optional, tag = "1")]
    pub position_ned_yaw: ::core::option::Option<PositionNedYaw>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPositionNedResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPositionGlobalRequest {
    /// Position and yaw
    #[prost(message, optional, tag = "1")]
    pub position_global_yaw: ::core::option::Option<PositionGlobalYaw>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPositionGlobalResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityBodyRequest {
    /// Velocity and yaw angular rate
    #[prost(message, optional, tag = "1")]
    pub velocity_body_yawspeed: ::core::option::Option<VelocityBodyYawspeed>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityBodyResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityNedRequest {
    /// Velocity and yaw
    #[prost(message, optional, tag = "1")]
    pub velocity_ned_yaw: ::core::option::Option<VelocityNedYaw>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityNedResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPositionVelocityNedRequest {
    /// Position and yaw
    #[prost(message, optional, tag = "1")]
    pub position_ned_yaw: ::core::option::Option<PositionNedYaw>,
    /// Velocity and yaw
    #[prost(message, optional, tag = "2")]
    pub velocity_ned_yaw: ::core::option::Option<VelocityNedYaw>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPositionVelocityNedResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAccelerationNedRequest {
    /// Acceleration
    #[prost(message, optional, tag = "1")]
    pub acceleration_ned: ::core::option::Option<AccelerationNed>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAccelerationNedResponse {
    #[prost(message, optional, tag = "1")]
    pub offboard_result: ::core::option::Option<OffboardResult>,
}
/// Type for attitude body angles in NED reference frame (roll, pitch, yaw and thrust)
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Attitude {
    /// Roll angle (in degrees, positive is right side down)
    #[prost(float, tag = "1")]
    pub roll_deg: f32,
    /// Pitch angle (in degrees, positive is nose up)
    #[prost(float, tag = "2")]
    pub pitch_deg: f32,
    /// Yaw angle (in degrees, positive is move nose to the right)
    #[prost(float, tag = "3")]
    pub yaw_deg: f32,
    /// Thrust (range: 0 to 1)
    #[prost(float, tag = "4")]
    pub thrust_value: f32,
}
///
/// Eight controls that will be given to the group. Each control is a normalized
/// (-1..+1) command value, which will be mapped and scaled through the mixer.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlGroup {
    /// Controls in the group
    #[prost(float, repeated, tag = "1")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
}
///
/// Type for actuator control.
///
/// Control members should be normed to -1..+1 where 0 is neutral position.
/// Throttle for single rotation direction motors is 0..1, negative range for reverse direction.
///
/// One group support eight controls.
///
/// Up to 16 actuator controls can be set. To ignore an output group, set all it conrols to NaN.
/// If one or more controls in group is not NaN, then all NaN controls will sent as zero.
/// The first 8 actuator controls internally map to control group 0, the latter 8 actuator
/// controls map to control group 1. Depending on what controls are set (instead of NaN) 1 or 2
/// MAVLink messages are actually sent.
///
/// In PX4 v1.9.0 Only first four Control Groups are supported
/// (<https://github.com/PX4/Firmware/blob/v1.9.0/src/modules/mavlink/mavlink_receiver.cpp#L980>).
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControl {
    /// Control groups.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<ActuatorControlGroup>,
}
/// Type for attitude rate commands in body coordinates (roll, pitch, yaw angular rate and thrust)
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AttitudeRate {
    /// Roll angular rate (in degrees/second, positive for clock-wise looking from front)
    #[prost(float, tag = "1")]
    pub roll_deg_s: f32,
    /// Pitch angular rate (in degrees/second, positive for head/front moving up)
    #[prost(float, tag = "2")]
    pub pitch_deg_s: f32,
    /// Yaw angular rate (in degrees/second, positive for clock-wise looking from above)
    #[prost(float, tag = "3")]
    pub yaw_deg_s: f32,
    /// Thrust (range: 0 to 1)
    #[prost(float, tag = "4")]
    pub thrust_value: f32,
}
/// Type for position commands in NED (North East Down) coordinates and yaw.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PositionNedYaw {
    /// Position North (in metres)
    #[prost(float, tag = "1")]
    pub north_m: f32,
    /// Position East (in metres)
    #[prost(float, tag = "2")]
    pub east_m: f32,
    /// Position Down (in metres)
    #[prost(float, tag = "3")]
    pub down_m: f32,
    /// Yaw in degrees (0 North, positive is clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yaw_deg: f32,
}
/// Type for position commands in Global (Latitude, Longitude, Altitude) coordinates and yaw.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionGlobalYaw {
    /// Latitude (in degrees)
    #[prost(double, tag = "1")]
    pub lat_deg: f64,
    /// Longitude (in degrees)
    #[prost(double, tag = "2")]
    pub lon_deg: f64,
    /// altitude (in metres)
    #[prost(float, tag = "3")]
    pub alt_m: f32,
    /// Yaw in degrees (0 North, positive is clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yaw_deg: f32,
    /// altitude type for this position
    #[prost(enumeration = "position_global_yaw::AltitudeType", tag = "5")]
    pub altitude_type: i32,
}
/// Nested message and enum types in `PositionGlobalYaw`.
pub mod position_global_yaw {
    /// Possible altitude options
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AltitudeType {
        /// Altitude relative to the Home position
        RelHome = 0,
        /// Altitude above mean sea level (AMSL)
        Amsl = 1,
        /// Altitude above ground level (AGL)
        Agl = 2,
    }
}
/// Type for velocity commands in body coordinates.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VelocityBodyYawspeed {
    /// Velocity forward (in metres/second)
    #[prost(float, tag = "1")]
    pub forward_m_s: f32,
    /// Velocity right (in metres/second)
    #[prost(float, tag = "2")]
    pub right_m_s: f32,
    /// Velocity down (in metres/second)
    #[prost(float, tag = "3")]
    pub down_m_s: f32,
    /// Yaw angular rate (in degrees/second, positive for clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yawspeed_deg_s: f32,
}
/// Type for velocity commands in NED (North East Down) coordinates and yaw.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VelocityNedYaw {
    /// Velocity North (in metres/second)
    #[prost(float, tag = "1")]
    pub north_m_s: f32,
    /// Velocity East (in metres/second)
    #[prost(float, tag = "2")]
    pub east_m_s: f32,
    /// Velocity Down (in metres/second)
    #[prost(float, tag = "3")]
    pub down_m_s: f32,
    /// Yaw in degrees (0 North, positive is clock-wise looking from above)
    #[prost(float, tag = "4")]
    pub yaw_deg: f32,
}
/// Type for acceleration commands in NED (North East Down) coordinates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccelerationNed {
    /// Acceleration North (in metres/second^2)
    #[prost(float, tag = "1")]
    pub north_m_s2: f32,
    /// Acceleration East (in metres/second^2)
    #[prost(float, tag = "2")]
    pub east_m_s2: f32,
    /// Acceleration Down (in metres/second^2)
    #[prost(float, tag = "3")]
    pub down_m_s2: f32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OffboardResult {
    /// Result enum value
    #[prost(enumeration = "offboard_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OffboardResult`.
pub mod offboard_result {
    /// Possible results returned for offboard requests
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
        /// Request succeeded
        Success = 1,
        /// No system is connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command denied
        CommandDenied = 5,
        /// Request timed out
        Timeout = 6,
        /// Cannot start without setpoint set
        NoSetpointSet = 7,
    }
}
#[doc = r" Generated client implementations."]
pub mod offboard_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Control a drone with position, velocity, attitude or motor commands."]
    #[doc = ""]
    #[doc = " The module is called offboard because the commands can be sent from external sources"]
    #[doc = " as opposed to onboard control right inside the autopilot \"board\"."]
    #[doc = ""]
    #[doc = " Client code must specify a setpoint before starting offboard mode."]
    #[doc = " Mavsdk automatically sends setpoints at 20Hz (PX4 Offboard mode requires that setpoints"]
    #[doc = " are minimally sent at 2Hz)."]
    #[derive(Debug, Clone)]
    pub struct OffboardServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OffboardServiceClient<tonic::transport::Channel> {
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
    impl<T> OffboardServiceClient<T>
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
        ) -> OffboardServiceClient<InterceptedService<T, F>>
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
            OffboardServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = ""]
        #[doc = " Start offboard control."]
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRequest>,
        ) -> Result<tonic::Response<super::StartResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.offboard.OffboardService/Start");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop offboard control."]
        #[doc = ""]
        #[doc = " The vehicle will be put into Hold mode: https://docs.px4.io/en/flight_modes/hold.html"]
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.offboard.OffboardService/Stop");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Check if offboard control is active."]
        #[doc = ""]
        #[doc = " True means that the vehicle is in offboard mode and we are actively sending"]
        #[doc = " setpoints."]
        pub async fn is_active(
            &mut self,
            request: impl tonic::IntoRequest<super::IsActiveRequest>,
        ) -> Result<tonic::Response<super::IsActiveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/IsActive",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the attitude in terms of roll, pitch and yaw in degrees with thrust."]
        pub async fn set_attitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttitudeRequest>,
        ) -> Result<tonic::Response<super::SetAttitudeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetAttitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set direct actuator control values to groups #0 and #1."]
        #[doc = ""]
        #[doc = " First 8 controls will go to control group 0, the following 8 controls to control group 1 (if"]
        #[doc = " actuator_control.num_controls more than 8)."]
        pub async fn set_actuator_control(
            &mut self,
            request: impl tonic::IntoRequest<super::SetActuatorControlRequest>,
        ) -> Result<tonic::Response<super::SetActuatorControlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetActuatorControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the attitude rate in terms of pitch, roll and yaw angular rate along with thrust."]
        pub async fn set_attitude_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttitudeRateRequest>,
        ) -> Result<tonic::Response<super::SetAttitudeRateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetAttitudeRate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the position in NED coordinates and yaw."]
        pub async fn set_position_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPositionNedRequest>,
        ) -> Result<tonic::Response<super::SetPositionNedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetPositionNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the position in Global coordinates (latitude, longitude, altitude) and yaw"]
        pub async fn set_position_global(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPositionGlobalRequest>,
        ) -> Result<tonic::Response<super::SetPositionGlobalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetPositionGlobal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the velocity in body coordinates and yaw angular rate. Not available for fixed-wing aircraft."]
        pub async fn set_velocity_body(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVelocityBodyRequest>,
        ) -> Result<tonic::Response<super::SetVelocityBodyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetVelocityBody",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the velocity in NED coordinates and yaw. Not available for fixed-wing aircraft."]
        pub async fn set_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetVelocityNedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetVelocityNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the position in NED coordinates, with the velocity to be used as feed-forward."]
        pub async fn set_position_velocity_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPositionVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetPositionVelocityNedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetPositionVelocityNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the acceleration in NED coordinates."]
        pub async fn set_acceleration_ned(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAccelerationNedRequest>,
        ) -> Result<tonic::Response<super::SetAccelerationNedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.offboard.OffboardService/SetAccelerationNed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod offboard_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with OffboardServiceServer."]
    #[async_trait]
    pub trait OffboardService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Start offboard control."]
        async fn start(
            &self,
            request: tonic::Request<super::StartRequest>,
        ) -> Result<tonic::Response<super::StartResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Stop offboard control."]
        #[doc = ""]
        #[doc = " The vehicle will be put into Hold mode: https://docs.px4.io/en/flight_modes/hold.html"]
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Check if offboard control is active."]
        #[doc = ""]
        #[doc = " True means that the vehicle is in offboard mode and we are actively sending"]
        #[doc = " setpoints."]
        async fn is_active(
            &self,
            request: tonic::Request<super::IsActiveRequest>,
        ) -> Result<tonic::Response<super::IsActiveResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the attitude in terms of roll, pitch and yaw in degrees with thrust."]
        async fn set_attitude(
            &self,
            request: tonic::Request<super::SetAttitudeRequest>,
        ) -> Result<tonic::Response<super::SetAttitudeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set direct actuator control values to groups #0 and #1."]
        #[doc = ""]
        #[doc = " First 8 controls will go to control group 0, the following 8 controls to control group 1 (if"]
        #[doc = " actuator_control.num_controls more than 8)."]
        async fn set_actuator_control(
            &self,
            request: tonic::Request<super::SetActuatorControlRequest>,
        ) -> Result<tonic::Response<super::SetActuatorControlResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the attitude rate in terms of pitch, roll and yaw angular rate along with thrust."]
        async fn set_attitude_rate(
            &self,
            request: tonic::Request<super::SetAttitudeRateRequest>,
        ) -> Result<tonic::Response<super::SetAttitudeRateResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the position in NED coordinates and yaw."]
        async fn set_position_ned(
            &self,
            request: tonic::Request<super::SetPositionNedRequest>,
        ) -> Result<tonic::Response<super::SetPositionNedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the position in Global coordinates (latitude, longitude, altitude) and yaw"]
        async fn set_position_global(
            &self,
            request: tonic::Request<super::SetPositionGlobalRequest>,
        ) -> Result<tonic::Response<super::SetPositionGlobalResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the velocity in body coordinates and yaw angular rate. Not available for fixed-wing aircraft."]
        async fn set_velocity_body(
            &self,
            request: tonic::Request<super::SetVelocityBodyRequest>,
        ) -> Result<tonic::Response<super::SetVelocityBodyResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the velocity in NED coordinates and yaw. Not available for fixed-wing aircraft."]
        async fn set_velocity_ned(
            &self,
            request: tonic::Request<super::SetVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetVelocityNedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the position in NED coordinates, with the velocity to be used as feed-forward."]
        async fn set_position_velocity_ned(
            &self,
            request: tonic::Request<super::SetPositionVelocityNedRequest>,
        ) -> Result<tonic::Response<super::SetPositionVelocityNedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the acceleration in NED coordinates."]
        async fn set_acceleration_ned(
            &self,
            request: tonic::Request<super::SetAccelerationNedRequest>,
        ) -> Result<tonic::Response<super::SetAccelerationNedResponse>, tonic::Status>;
    }
    #[doc = "*"]
    #[doc = " Control a drone with position, velocity, attitude or motor commands."]
    #[doc = ""]
    #[doc = " The module is called offboard because the commands can be sent from external sources"]
    #[doc = " as opposed to onboard control right inside the autopilot \"board\"."]
    #[doc = ""]
    #[doc = " Client code must specify a setpoint before starting offboard mode."]
    #[doc = " Mavsdk automatically sends setpoints at 20Hz (PX4 Offboard mode requires that setpoints"]
    #[doc = " are minimally sent at 2Hz)."]
    #[derive(Debug)]
    pub struct OffboardServiceServer<T: OffboardService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OffboardService> OffboardServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OffboardServiceServer<T>
    where
        T: OffboardService,
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
            match req.uri().path() {
                "/mavsdk.rpc.offboard.OffboardService/Start" => {
                    #[allow(non_camel_case_types)]
                    struct StartSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService> tonic::server::UnaryService<super::StartRequest> for StartSvc<T> {
                        type Response = super::StartResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService> tonic::server::UnaryService<super::StopRequest> for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/IsActive" => {
                    #[allow(non_camel_case_types)]
                    struct IsActiveSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService> tonic::server::UnaryService<super::IsActiveRequest> for IsActiveSvc<T> {
                        type Response = super::IsActiveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsActiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_active(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsActiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetAttitude" => {
                    #[allow(non_camel_case_types)]
                    struct SetAttitudeSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService> tonic::server::UnaryService<super::SetAttitudeRequest>
                        for SetAttitudeSvc<T>
                    {
                        type Response = super::SetAttitudeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAttitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_attitude(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAttitudeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetActuatorControl" => {
                    #[allow(non_camel_case_types)]
                    struct SetActuatorControlSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetActuatorControlRequest>
                        for SetActuatorControlSvc<T>
                    {
                        type Response = super::SetActuatorControlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetActuatorControlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_actuator_control(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetActuatorControlSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetAttitudeRate" => {
                    #[allow(non_camel_case_types)]
                    struct SetAttitudeRateSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetAttitudeRateRequest>
                        for SetAttitudeRateSvc<T>
                    {
                        type Response = super::SetAttitudeRateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAttitudeRateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_attitude_rate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAttitudeRateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetPositionNed" => {
                    #[allow(non_camel_case_types)]
                    struct SetPositionNedSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetPositionNedRequest>
                        for SetPositionNedSvc<T>
                    {
                        type Response = super::SetPositionNedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPositionNedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_position_ned(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPositionNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetPositionGlobal" => {
                    #[allow(non_camel_case_types)]
                    struct SetPositionGlobalSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetPositionGlobalRequest>
                        for SetPositionGlobalSvc<T>
                    {
                        type Response = super::SetPositionGlobalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPositionGlobalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_position_global(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPositionGlobalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetVelocityBody" => {
                    #[allow(non_camel_case_types)]
                    struct SetVelocityBodySvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetVelocityBodyRequest>
                        for SetVelocityBodySvc<T>
                    {
                        type Response = super::SetVelocityBodyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetVelocityBodyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_velocity_body(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetVelocityBodySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetVelocityNed" => {
                    #[allow(non_camel_case_types)]
                    struct SetVelocityNedSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetVelocityNedRequest>
                        for SetVelocityNedSvc<T>
                    {
                        type Response = super::SetVelocityNedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetVelocityNedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_velocity_ned(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetVelocityNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetPositionVelocityNed" => {
                    #[allow(non_camel_case_types)]
                    struct SetPositionVelocityNedSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetPositionVelocityNedRequest>
                        for SetPositionVelocityNedSvc<T>
                    {
                        type Response = super::SetPositionVelocityNedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPositionVelocityNedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_position_velocity_ned(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPositionVelocityNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.offboard.OffboardService/SetAccelerationNed" => {
                    #[allow(non_camel_case_types)]
                    struct SetAccelerationNedSvc<T: OffboardService>(pub Arc<T>);
                    impl<T: OffboardService>
                        tonic::server::UnaryService<super::SetAccelerationNedRequest>
                        for SetAccelerationNedSvc<T>
                    {
                        type Response = super::SetAccelerationNedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAccelerationNedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_acceleration_ned(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAccelerationNedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: OffboardService> Clone for OffboardServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: OffboardService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OffboardService> tonic::transport::NamedService for OffboardServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.offboard.OffboardService";
    }
}
