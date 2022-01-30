#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArmRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArmResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DisarmRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DisarmResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TakeoffRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TakeoffResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LandRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LandResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RebootRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RebootResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ShutdownRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ShutdownResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerminateRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerminateResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct KillRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct KillResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ReturnToLaunchRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ReturnToLaunchResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GotoLocationRequest {
    /// Latitude (in degrees)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude (in degrees)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (in meters)
    #[prost(float, tag = "3")]
    pub absolute_altitude_m: f32,
    /// Yaw angle (in degrees, frame is NED, 0 is North, positive is clockwise)
    #[prost(float, tag = "4")]
    pub yaw_deg: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GotoLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoOrbitRequest {
    /// Radius of circle (in meters)
    #[prost(float, tag = "1")]
    pub radius_m: f32,
    /// Tangential velocity (in m/s)
    #[prost(float, tag = "2")]
    pub velocity_ms: f32,
    /// Yaw behavior of vehicle (ORBIT_YAW_BEHAVIOUR)
    #[prost(enumeration = "OrbitYawBehavior", tag = "3")]
    pub yaw_behavior: i32,
    /// Center point latitude in degrees. NAN: use current latitude for center
    #[prost(double, tag = "5")]
    pub latitude_deg: f64,
    /// Center point longitude in degrees. NAN: use current longitude for center
    #[prost(double, tag = "6")]
    pub longitude_deg: f64,
    /// Center point altitude in meters. NAN: use current altitude for center
    #[prost(double, tag = "7")]
    pub absolute_altitude_m: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoOrbitResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorRequest {
    /// Index of actuator (starting with 1)
    #[prost(int32, tag = "1")]
    pub index: i32,
    /// Value to set the actuator to (normalized from \[-1..1\])
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransitionToFixedwingRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransitionToFixedwingResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransitionToMulticopterRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TransitionToMulticopterResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetTakeoffAltitudeRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetTakeoffAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
    /// Takeoff altitude relative to ground/takeoff location (in meters)
    #[prost(float, tag = "2")]
    pub altitude: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetTakeoffAltitudeRequest {
    /// Takeoff altitude relative to ground/takeoff location (in meters)
    #[prost(float, tag = "1")]
    pub altitude: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetTakeoffAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetMaximumSpeedRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetMaximumSpeedResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
    /// Maximum speed (in metres/second)
    #[prost(float, tag = "2")]
    pub speed: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetMaximumSpeedRequest {
    /// Maximum speed (in metres/second)
    #[prost(float, tag = "1")]
    pub speed: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetMaximumSpeedResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAltitudeRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
    /// Return altitude relative to takeoff location (in meters)
    #[prost(float, tag = "2")]
    pub relative_altitude_m: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAltitudeRequest {
    /// Return altitude relative to takeoff location (in meters)
    #[prost(float, tag = "1")]
    pub relative_altitude_m: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAltitudeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_result: ::core::option::Option<ActionResult>,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActionResult {
    /// Result enum value
    #[prost(enumeration = "action_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ActionResult`.
pub mod action_result {
    /// Possible results returned for action requests.
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
        /// Request was successful
        Success = 1,
        /// No system is connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command refused by vehicle
        CommandDenied = 5,
        /// Command refused because landed state is unknown
        CommandDeniedLandedStateUnknown = 6,
        /// Command refused because vehicle not landed
        CommandDeniedNotLanded = 7,
        /// Request timed out
        Timeout = 8,
        /// Hybrid/VTOL transition support is unknown
        VtolTransitionSupportUnknown = 9,
        /// Vehicle does not support hybrid/VTOL transitions
        NoVtolTransitionSupport = 10,
        /// Error getting or setting parameter
        ParameterError = 11,
    }
}
/// Yaw behaviour during orbit flight.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrbitYawBehavior {
    /// Vehicle front points to the center (default)
    HoldFrontToCircleCenter = 0,
    /// Vehicle front holds heading when message received
    HoldInitialHeading = 1,
    /// Yaw uncontrolled
    Uncontrolled = 2,
    /// Vehicle front follows flight path (tangential to circle)
    HoldFrontTangentToCircle = 3,
    /// Yaw controlled by RC input
    RcControlled = 4,
}
#[doc = r" Generated client implementations."]
pub mod action_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable simple actions such as arming, taking off, and landing."]
    #[derive(Debug, Clone)]
    pub struct ActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActionServiceClient<tonic::transport::Channel> {
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
    impl<T> ActionServiceClient<T>
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
        ) -> ActionServiceClient<InterceptedService<T, F>>
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
            ActionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Send command to arm the drone."]
        #[doc = ""]
        #[doc = " Arming a drone normally causes motors to spin at idle."]
        #[doc = " Before arming take all safety precautions and stand clear of the drone!"]
        pub async fn arm(
            &mut self,
            request: impl tonic::IntoRequest<super::ArmRequest>,
        ) -> Result<tonic::Response<super::ArmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Arm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to disarm the drone."]
        #[doc = ""]
        #[doc = " This will disarm a drone that considers itself landed. If flying, the drone should"]
        #[doc = " reject the disarm command. Disarming means that all motors will stop."]
        pub async fn disarm(
            &mut self,
            request: impl tonic::IntoRequest<super::DisarmRequest>,
        ) -> Result<tonic::Response<super::DisarmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Disarm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to take off and hover."]
        #[doc = ""]
        #[doc = " This switches the drone into position control mode and commands"]
        #[doc = " it to take off and hover at the takeoff altitude."]
        #[doc = ""]
        #[doc = " Note that the vehicle must be armed before it can take off."]
        pub async fn takeoff(
            &mut self,
            request: impl tonic::IntoRequest<super::TakeoffRequest>,
        ) -> Result<tonic::Response<super::TakeoffResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Takeoff");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to land at the current position."]
        #[doc = ""]
        #[doc = " This switches the drone to 'Land' flight mode."]
        pub async fn land(
            &mut self,
            request: impl tonic::IntoRequest<super::LandRequest>,
        ) -> Result<tonic::Response<super::LandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Land");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to reboot the drone components."]
        #[doc = ""]
        #[doc = " This will reboot the autopilot, companion computer, camera and gimbal."]
        pub async fn reboot(
            &mut self,
            request: impl tonic::IntoRequest<super::RebootRequest>,
        ) -> Result<tonic::Response<super::RebootResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Reboot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to shut down the drone components."]
        #[doc = ""]
        #[doc = " This will shut down the autopilot, onboard computer, camera and gimbal."]
        #[doc = " This command should only be used when the autopilot is disarmed and autopilots commonly"]
        #[doc = " reject it if they are not already ready to shut down."]
        pub async fn shutdown(
            &mut self,
            request: impl tonic::IntoRequest<super::ShutdownRequest>,
        ) -> Result<tonic::Response<super::ShutdownResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Shutdown");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to terminate the drone."]
        #[doc = ""]
        #[doc = " This will run the terminate routine as configured on the drone (e.g. disarm and open the parachute)."]
        pub async fn terminate(
            &mut self,
            request: impl tonic::IntoRequest<super::TerminateRequest>,
        ) -> Result<tonic::Response<super::TerminateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Terminate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to kill the drone."]
        #[doc = ""]
        #[doc = " This will disarm a drone irrespective of whether it is landed or flying."]
        #[doc = " Note that the drone will fall out of the sky if this command is used while flying."]
        pub async fn kill(
            &mut self,
            request: impl tonic::IntoRequest<super::KillRequest>,
        ) -> Result<tonic::Response<super::KillResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Kill");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to return to the launch (takeoff) position and land."]
        #[doc = ""]
        #[doc = " This switches the drone into [Return mode](https://docs.px4.io/master/en/flight_modes/return.html) which"]
        #[doc = " generally means it will rise up to a certain altitude to clear any obstacles before heading"]
        #[doc = " back to the launch (takeoff) position and land there."]
        pub async fn return_to_launch(
            &mut self,
            request: impl tonic::IntoRequest<super::ReturnToLaunchRequest>,
        ) -> Result<tonic::Response<super::ReturnToLaunchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/ReturnToLaunch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to move the vehicle to a specific global position."]
        #[doc = ""]
        #[doc = " The latitude and longitude are given in degrees (WGS84 frame) and the altitude"]
        #[doc = " in meters AMSL (above mean sea level)."]
        #[doc = ""]
        #[doc = " The yaw angle is in degrees (frame is NED, 0 is North, positive is clockwise)."]
        pub async fn goto_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GotoLocationRequest>,
        ) -> Result<tonic::Response<super::GotoLocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GotoLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command do orbit to the drone."]
        #[doc = ""]
        #[doc = " This will run the orbit routine with the given parameters."]
        pub async fn do_orbit(
            &mut self,
            request: impl tonic::IntoRequest<super::DoOrbitRequest>,
        ) -> Result<tonic::Response<super::DoOrbitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/DoOrbit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to hold position (a.k.a. \"Loiter\")."]
        #[doc = ""]
        #[doc = " Sends a command to drone to change to Hold flight mode, causing the"]
        #[doc = " vehicle to stop and maintain its current GPS position and altitude."]
        #[doc = ""]
        #[doc = " Note: this command is specific to the PX4 Autopilot flight stack as"]
        #[doc = " it implies a change to a PX4-specific mode."]
        pub async fn hold(
            &mut self,
            request: impl tonic::IntoRequest<super::HoldRequest>,
        ) -> Result<tonic::Response<super::HoldResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.action.ActionService/Hold");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to set the value of an actuator."]
        pub async fn set_actuator(
            &mut self,
            request: impl tonic::IntoRequest<super::SetActuatorRequest>,
        ) -> Result<tonic::Response<super::SetActuatorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetActuator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to transition the drone to fixedwing."]
        #[doc = ""]
        #[doc = " The associated action will only be executed for VTOL vehicles (on other vehicle types the"]
        #[doc = " command will fail). The command will succeed if called when the vehicle"]
        #[doc = " is already in fixedwing mode."]
        pub async fn transition_to_fixedwing(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionToFixedwingRequest>,
        ) -> Result<tonic::Response<super::TransitionToFixedwingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/TransitionToFixedwing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Send command to transition the drone to multicopter."]
        #[doc = ""]
        #[doc = " The associated action will only be executed for VTOL vehicles (on other vehicle types the"]
        #[doc = " command will fail). The command will succeed if called when the vehicle"]
        #[doc = " is already in multicopter mode."]
        pub async fn transition_to_multicopter(
            &mut self,
            request: impl tonic::IntoRequest<super::TransitionToMulticopterRequest>,
        ) -> Result<tonic::Response<super::TransitionToMulticopterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/TransitionToMulticopter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get the takeoff altitude (in meters above ground)."]
        pub async fn get_takeoff_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTakeoffAltitudeRequest>,
        ) -> Result<tonic::Response<super::GetTakeoffAltitudeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GetTakeoffAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set takeoff altitude (in meters above ground)."]
        pub async fn set_takeoff_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTakeoffAltitudeRequest>,
        ) -> Result<tonic::Response<super::SetTakeoffAltitudeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetTakeoffAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get the vehicle maximum speed (in metres/second)."]
        pub async fn get_maximum_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMaximumSpeedRequest>,
        ) -> Result<tonic::Response<super::GetMaximumSpeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GetMaximumSpeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set vehicle maximum speed (in metres/second)."]
        pub async fn set_maximum_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMaximumSpeedRequest>,
        ) -> Result<tonic::Response<super::SetMaximumSpeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetMaximumSpeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get the return to launch minimum return altitude (in meters)."]
        pub async fn get_return_to_launch_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReturnToLaunchAltitudeRequest>,
        ) -> Result<tonic::Response<super::GetReturnToLaunchAltitudeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/GetReturnToLaunchAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set the return to launch minimum return altitude (in meters)."]
        pub async fn set_return_to_launch_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReturnToLaunchAltitudeRequest>,
        ) -> Result<tonic::Response<super::SetReturnToLaunchAltitudeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action.ActionService/SetReturnToLaunchAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod action_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ActionServiceServer."]
    #[async_trait]
    pub trait ActionService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Send command to arm the drone."]
        #[doc = ""]
        #[doc = " Arming a drone normally causes motors to spin at idle."]
        #[doc = " Before arming take all safety precautions and stand clear of the drone!"]
        async fn arm(
            &self,
            request: tonic::Request<super::ArmRequest>,
        ) -> Result<tonic::Response<super::ArmResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to disarm the drone."]
        #[doc = ""]
        #[doc = " This will disarm a drone that considers itself landed. If flying, the drone should"]
        #[doc = " reject the disarm command. Disarming means that all motors will stop."]
        async fn disarm(
            &self,
            request: tonic::Request<super::DisarmRequest>,
        ) -> Result<tonic::Response<super::DisarmResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to take off and hover."]
        #[doc = ""]
        #[doc = " This switches the drone into position control mode and commands"]
        #[doc = " it to take off and hover at the takeoff altitude."]
        #[doc = ""]
        #[doc = " Note that the vehicle must be armed before it can take off."]
        async fn takeoff(
            &self,
            request: tonic::Request<super::TakeoffRequest>,
        ) -> Result<tonic::Response<super::TakeoffResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to land at the current position."]
        #[doc = ""]
        #[doc = " This switches the drone to 'Land' flight mode."]
        async fn land(
            &self,
            request: tonic::Request<super::LandRequest>,
        ) -> Result<tonic::Response<super::LandResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to reboot the drone components."]
        #[doc = ""]
        #[doc = " This will reboot the autopilot, companion computer, camera and gimbal."]
        async fn reboot(
            &self,
            request: tonic::Request<super::RebootRequest>,
        ) -> Result<tonic::Response<super::RebootResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to shut down the drone components."]
        #[doc = ""]
        #[doc = " This will shut down the autopilot, onboard computer, camera and gimbal."]
        #[doc = " This command should only be used when the autopilot is disarmed and autopilots commonly"]
        #[doc = " reject it if they are not already ready to shut down."]
        async fn shutdown(
            &self,
            request: tonic::Request<super::ShutdownRequest>,
        ) -> Result<tonic::Response<super::ShutdownResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to terminate the drone."]
        #[doc = ""]
        #[doc = " This will run the terminate routine as configured on the drone (e.g. disarm and open the parachute)."]
        async fn terminate(
            &self,
            request: tonic::Request<super::TerminateRequest>,
        ) -> Result<tonic::Response<super::TerminateResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to kill the drone."]
        #[doc = ""]
        #[doc = " This will disarm a drone irrespective of whether it is landed or flying."]
        #[doc = " Note that the drone will fall out of the sky if this command is used while flying."]
        async fn kill(
            &self,
            request: tonic::Request<super::KillRequest>,
        ) -> Result<tonic::Response<super::KillResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to return to the launch (takeoff) position and land."]
        #[doc = ""]
        #[doc = " This switches the drone into [Return mode](https://docs.px4.io/master/en/flight_modes/return.html) which"]
        #[doc = " generally means it will rise up to a certain altitude to clear any obstacles before heading"]
        #[doc = " back to the launch (takeoff) position and land there."]
        async fn return_to_launch(
            &self,
            request: tonic::Request<super::ReturnToLaunchRequest>,
        ) -> Result<tonic::Response<super::ReturnToLaunchResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to move the vehicle to a specific global position."]
        #[doc = ""]
        #[doc = " The latitude and longitude are given in degrees (WGS84 frame) and the altitude"]
        #[doc = " in meters AMSL (above mean sea level)."]
        #[doc = ""]
        #[doc = " The yaw angle is in degrees (frame is NED, 0 is North, positive is clockwise)."]
        async fn goto_location(
            &self,
            request: tonic::Request<super::GotoLocationRequest>,
        ) -> Result<tonic::Response<super::GotoLocationResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command do orbit to the drone."]
        #[doc = ""]
        #[doc = " This will run the orbit routine with the given parameters."]
        async fn do_orbit(
            &self,
            request: tonic::Request<super::DoOrbitRequest>,
        ) -> Result<tonic::Response<super::DoOrbitResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to hold position (a.k.a. \"Loiter\")."]
        #[doc = ""]
        #[doc = " Sends a command to drone to change to Hold flight mode, causing the"]
        #[doc = " vehicle to stop and maintain its current GPS position and altitude."]
        #[doc = ""]
        #[doc = " Note: this command is specific to the PX4 Autopilot flight stack as"]
        #[doc = " it implies a change to a PX4-specific mode."]
        async fn hold(
            &self,
            request: tonic::Request<super::HoldRequest>,
        ) -> Result<tonic::Response<super::HoldResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to set the value of an actuator."]
        async fn set_actuator(
            &self,
            request: tonic::Request<super::SetActuatorRequest>,
        ) -> Result<tonic::Response<super::SetActuatorResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to transition the drone to fixedwing."]
        #[doc = ""]
        #[doc = " The associated action will only be executed for VTOL vehicles (on other vehicle types the"]
        #[doc = " command will fail). The command will succeed if called when the vehicle"]
        #[doc = " is already in fixedwing mode."]
        async fn transition_to_fixedwing(
            &self,
            request: tonic::Request<super::TransitionToFixedwingRequest>,
        ) -> Result<tonic::Response<super::TransitionToFixedwingResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Send command to transition the drone to multicopter."]
        #[doc = ""]
        #[doc = " The associated action will only be executed for VTOL vehicles (on other vehicle types the"]
        #[doc = " command will fail). The command will succeed if called when the vehicle"]
        #[doc = " is already in multicopter mode."]
        async fn transition_to_multicopter(
            &self,
            request: tonic::Request<super::TransitionToMulticopterRequest>,
        ) -> Result<tonic::Response<super::TransitionToMulticopterResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get the takeoff altitude (in meters above ground)."]
        async fn get_takeoff_altitude(
            &self,
            request: tonic::Request<super::GetTakeoffAltitudeRequest>,
        ) -> Result<tonic::Response<super::GetTakeoffAltitudeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set takeoff altitude (in meters above ground)."]
        async fn set_takeoff_altitude(
            &self,
            request: tonic::Request<super::SetTakeoffAltitudeRequest>,
        ) -> Result<tonic::Response<super::SetTakeoffAltitudeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get the vehicle maximum speed (in metres/second)."]
        async fn get_maximum_speed(
            &self,
            request: tonic::Request<super::GetMaximumSpeedRequest>,
        ) -> Result<tonic::Response<super::GetMaximumSpeedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set vehicle maximum speed (in metres/second)."]
        async fn set_maximum_speed(
            &self,
            request: tonic::Request<super::SetMaximumSpeedRequest>,
        ) -> Result<tonic::Response<super::SetMaximumSpeedResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get the return to launch minimum return altitude (in meters)."]
        async fn get_return_to_launch_altitude(
            &self,
            request: tonic::Request<super::GetReturnToLaunchAltitudeRequest>,
        ) -> Result<tonic::Response<super::GetReturnToLaunchAltitudeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set the return to launch minimum return altitude (in meters)."]
        async fn set_return_to_launch_altitude(
            &self,
            request: tonic::Request<super::SetReturnToLaunchAltitudeRequest>,
        ) -> Result<tonic::Response<super::SetReturnToLaunchAltitudeResponse>, tonic::Status>;
    }
    #[doc = " Enable simple actions such as arming, taking off, and landing."]
    #[derive(Debug)]
    pub struct ActionServiceServer<T: ActionService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ActionService> ActionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ActionServiceServer<T>
    where
        T: ActionService,
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
                "/mavsdk.rpc.action.ActionService/Arm" => {
                    #[allow(non_camel_case_types)]
                    struct ArmSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::ArmRequest> for ArmSvc<T> {
                        type Response = super::ArmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).arm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ArmSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Disarm" => {
                    #[allow(non_camel_case_types)]
                    struct DisarmSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::DisarmRequest> for DisarmSvc<T> {
                        type Response = super::DisarmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisarmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).disarm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisarmSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Takeoff" => {
                    #[allow(non_camel_case_types)]
                    struct TakeoffSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::TakeoffRequest> for TakeoffSvc<T> {
                        type Response = super::TakeoffResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TakeoffRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).takeoff(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TakeoffSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Land" => {
                    #[allow(non_camel_case_types)]
                    struct LandSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::LandRequest> for LandSvc<T> {
                        type Response = super::LandResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LandRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).land(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LandSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Reboot" => {
                    #[allow(non_camel_case_types)]
                    struct RebootSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::RebootRequest> for RebootSvc<T> {
                        type Response = super::RebootResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RebootRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reboot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RebootSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Shutdown" => {
                    #[allow(non_camel_case_types)]
                    struct ShutdownSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::ShutdownRequest> for ShutdownSvc<T> {
                        type Response = super::ShutdownResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShutdownRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).shutdown(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShutdownSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Terminate" => {
                    #[allow(non_camel_case_types)]
                    struct TerminateSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::TerminateRequest> for TerminateSvc<T> {
                        type Response = super::TerminateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TerminateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).terminate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TerminateSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Kill" => {
                    #[allow(non_camel_case_types)]
                    struct KillSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::KillRequest> for KillSvc<T> {
                        type Response = super::KillResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KillRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).kill(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KillSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/ReturnToLaunch" => {
                    #[allow(non_camel_case_types)]
                    struct ReturnToLaunchSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::ReturnToLaunchRequest>
                        for ReturnToLaunchSvc<T>
                    {
                        type Response = super::ReturnToLaunchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReturnToLaunchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).return_to_launch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReturnToLaunchSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/GotoLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GotoLocationSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::GotoLocationRequest>
                        for GotoLocationSvc<T>
                    {
                        type Response = super::GotoLocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GotoLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).goto_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GotoLocationSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/DoOrbit" => {
                    #[allow(non_camel_case_types)]
                    struct DoOrbitSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::DoOrbitRequest> for DoOrbitSvc<T> {
                        type Response = super::DoOrbitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DoOrbitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).do_orbit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DoOrbitSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/Hold" => {
                    #[allow(non_camel_case_types)]
                    struct HoldSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::HoldRequest> for HoldSvc<T> {
                        type Response = super::HoldResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HoldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).hold(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HoldSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/SetActuator" => {
                    #[allow(non_camel_case_types)]
                    struct SetActuatorSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService> tonic::server::UnaryService<super::SetActuatorRequest>
                        for SetActuatorSvc<T>
                    {
                        type Response = super::SetActuatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetActuatorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_actuator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetActuatorSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/TransitionToFixedwing" => {
                    #[allow(non_camel_case_types)]
                    struct TransitionToFixedwingSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::TransitionToFixedwingRequest>
                        for TransitionToFixedwingSvc<T>
                    {
                        type Response = super::TransitionToFixedwingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionToFixedwingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).transition_to_fixedwing(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransitionToFixedwingSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/TransitionToMulticopter" => {
                    #[allow(non_camel_case_types)]
                    struct TransitionToMulticopterSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::TransitionToMulticopterRequest>
                        for TransitionToMulticopterSvc<T>
                    {
                        type Response = super::TransitionToMulticopterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransitionToMulticopterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).transition_to_multicopter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransitionToMulticopterSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/GetTakeoffAltitude" => {
                    #[allow(non_camel_case_types)]
                    struct GetTakeoffAltitudeSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::GetTakeoffAltitudeRequest>
                        for GetTakeoffAltitudeSvc<T>
                    {
                        type Response = super::GetTakeoffAltitudeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTakeoffAltitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_takeoff_altitude(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTakeoffAltitudeSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/SetTakeoffAltitude" => {
                    #[allow(non_camel_case_types)]
                    struct SetTakeoffAltitudeSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::SetTakeoffAltitudeRequest>
                        for SetTakeoffAltitudeSvc<T>
                    {
                        type Response = super::SetTakeoffAltitudeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetTakeoffAltitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_takeoff_altitude(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetTakeoffAltitudeSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/GetMaximumSpeed" => {
                    #[allow(non_camel_case_types)]
                    struct GetMaximumSpeedSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::GetMaximumSpeedRequest>
                        for GetMaximumSpeedSvc<T>
                    {
                        type Response = super::GetMaximumSpeedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMaximumSpeedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_maximum_speed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMaximumSpeedSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/SetMaximumSpeed" => {
                    #[allow(non_camel_case_types)]
                    struct SetMaximumSpeedSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::SetMaximumSpeedRequest>
                        for SetMaximumSpeedSvc<T>
                    {
                        type Response = super::SetMaximumSpeedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetMaximumSpeedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_maximum_speed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetMaximumSpeedSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/GetReturnToLaunchAltitude" => {
                    #[allow(non_camel_case_types)]
                    struct GetReturnToLaunchAltitudeSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::GetReturnToLaunchAltitudeRequest>
                        for GetReturnToLaunchAltitudeSvc<T>
                    {
                        type Response = super::GetReturnToLaunchAltitudeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReturnToLaunchAltitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_return_to_launch_altitude(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReturnToLaunchAltitudeSvc(inner);
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
                "/mavsdk.rpc.action.ActionService/SetReturnToLaunchAltitude" => {
                    #[allow(non_camel_case_types)]
                    struct SetReturnToLaunchAltitudeSvc<T: ActionService>(pub Arc<T>);
                    impl<T: ActionService>
                        tonic::server::UnaryService<super::SetReturnToLaunchAltitudeRequest>
                        for SetReturnToLaunchAltitudeSvc<T>
                    {
                        type Response = super::SetReturnToLaunchAltitudeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetReturnToLaunchAltitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_return_to_launch_altitude(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetReturnToLaunchAltitudeSvc(inner);
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
    impl<T: ActionService> Clone for ActionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ActionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ActionService> tonic::transport::NamedService for ActionServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.action.ActionService";
    }
}
