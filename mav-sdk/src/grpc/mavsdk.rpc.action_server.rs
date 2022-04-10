#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAllowTakeoffRequest {
    /// Is takeoff allowed?
    #[prost(bool, tag = "1")]
    pub allow_takeoff: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetArmableRequest {
    /// Is Armable now?
    #[prost(bool, tag = "1")]
    pub armable: bool,
    /// Is armable with force?
    #[prost(bool, tag = "2")]
    pub force_armable: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetDisarmableRequest {
    /// Is disarmable now?
    #[prost(bool, tag = "1")]
    pub disarmable: bool,
    /// Is disarmable with force? (Kill)
    #[prost(bool, tag = "2")]
    pub force_disarmable: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAllowableFlightModesRequest {
    #[prost(message, optional, tag = "1")]
    pub flight_modes: ::core::option::Option<AllowableFlightModes>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetAllowableFlightModesRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeArmDisarmRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeFlightModeChangeRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTakeoffRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLandRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRebootRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeShutdownRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTerminateRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArmDisarmResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(message, optional, tag = "2")]
    pub arm: ::core::option::Option<ArmDisarm>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlightModeChangeResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(enumeration = "FlightMode", tag = "2")]
    pub flight_mode: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TakeoffResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(bool, tag = "2")]
    pub takeoff: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LandResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(bool, tag = "2")]
    pub land: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RebootResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(bool, tag = "2")]
    pub reboot: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ShutdownResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(bool, tag = "2")]
    pub shutdown: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerminateResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
    #[prost(bool, tag = "2")]
    pub terminate: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetArmableResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetDisarmableResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAllowableFlightModesResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAllowTakeoffResponse {
    #[prost(message, optional, tag = "1")]
    pub action_server_result: ::core::option::Option<ActionServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetAllowableFlightModesResponse {
    #[prost(message, optional, tag = "1")]
    pub flight_modes: ::core::option::Option<AllowableFlightModes>,
}
/// State to check if the vehicle can transition to
/// respective flightmodes
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AllowableFlightModes {
    /// Auto/mission mode
    #[prost(bool, tag = "1")]
    pub can_auto_mode: bool,
    /// Guided mode
    #[prost(bool, tag = "2")]
    pub can_guided_mode: bool,
    /// Stabilize mode
    #[prost(bool, tag = "3")]
    pub can_stabilize_mode: bool,
}
/// Arming message type
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ArmDisarm {
    /// Should vehicle arm
    #[prost(bool, tag = "1")]
    pub arm: bool,
    /// Should arm override pre-flight checks
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActionServerResult {
    /// Result enum value
    #[prost(enumeration = "action_server_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ActionServerResult`.
pub mod action_server_result {
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
        /// Intermediate message showing progress or instructions on the next steps
        Next = 12,
    }
}
///
/// Flight modes.
///
/// For more information about flight modes, check out
/// <https://docs.px4.io/master/en/config/flight_mode.html.>
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
}
#[doc = r" Generated client implementations."]
pub mod action_server_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provide vehicle actions (as a server) such as arming, taking off, and landing."]
    #[derive(Debug, Clone)]
    pub struct ActionServerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActionServerServiceClient<tonic::transport::Channel> {
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
    impl<T> ActionServerServiceClient<T>
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
        ) -> ActionServerServiceClient<InterceptedService<T, F>>
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
            ActionServerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Subscribe to ARM/DISARM commands"]
        pub async fn subscribe_arm_disarm(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeArmDisarmRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ArmDisarmResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeArmDisarm",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to DO_SET_MODE"]
        pub async fn subscribe_flight_mode_change(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeFlightModeChangeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::FlightModeChangeResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeFlightModeChange",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to takeoff command"]
        pub async fn subscribe_takeoff(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeTakeoffRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::TakeoffResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeTakeoff",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to land command"]
        pub async fn subscribe_land(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeLandRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::LandResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeLand",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to reboot command"]
        pub async fn subscribe_reboot(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRebootRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::RebootResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeReboot",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to shutdown command"]
        pub async fn subscribe_shutdown(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeShutdownRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ShutdownResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeShutdown",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Subscribe to terminate command"]
        pub async fn subscribe_terminate(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeTerminateRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::TerminateResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeTerminate",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Can the vehicle takeoff"]
        pub async fn set_allow_takeoff(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAllowTakeoffRequest>,
        ) -> Result<tonic::Response<super::SetAllowTakeoffResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SetAllowTakeoff",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Can the vehicle arm when requested"]
        pub async fn set_armable(
            &mut self,
            request: impl tonic::IntoRequest<super::SetArmableRequest>,
        ) -> Result<tonic::Response<super::SetArmableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SetArmable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Can the vehicle disarm when requested"]
        pub async fn set_disarmable(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDisarmableRequest>,
        ) -> Result<tonic::Response<super::SetDisarmableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SetDisarmable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Set which modes the vehicle can transition to (Manual always allowed)"]
        pub async fn set_allowable_flight_modes(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAllowableFlightModesRequest>,
        ) -> Result<tonic::Response<super::SetAllowableFlightModesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/SetAllowableFlightModes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get which modes the vehicle can transition to (Manual always allowed)"]
        pub async fn get_allowable_flight_modes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllowableFlightModesRequest>,
        ) -> Result<tonic::Response<super::GetAllowableFlightModesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.action_server.ActionServerService/GetAllowableFlightModes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod action_server_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ActionServerServiceServer."]
    #[async_trait]
    pub trait ActionServerService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeArmDisarm method."]
        type SubscribeArmDisarmStream: futures_core::Stream<Item = Result<super::ArmDisarmResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to ARM/DISARM commands"]
        async fn subscribe_arm_disarm(
            &self,
            request: tonic::Request<super::SubscribeArmDisarmRequest>,
        ) -> Result<tonic::Response<Self::SubscribeArmDisarmStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeFlightModeChange method."]
        type SubscribeFlightModeChangeStream: futures_core::Stream<Item = Result<super::FlightModeChangeResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to DO_SET_MODE"]
        async fn subscribe_flight_mode_change(
            &self,
            request: tonic::Request<super::SubscribeFlightModeChangeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeFlightModeChangeStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeTakeoff method."]
        type SubscribeTakeoffStream: futures_core::Stream<Item = Result<super::TakeoffResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to takeoff command"]
        async fn subscribe_takeoff(
            &self,
            request: tonic::Request<super::SubscribeTakeoffRequest>,
        ) -> Result<tonic::Response<Self::SubscribeTakeoffStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeLand method."]
        type SubscribeLandStream: futures_core::Stream<Item = Result<super::LandResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to land command"]
        async fn subscribe_land(
            &self,
            request: tonic::Request<super::SubscribeLandRequest>,
        ) -> Result<tonic::Response<Self::SubscribeLandStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeReboot method."]
        type SubscribeRebootStream: futures_core::Stream<Item = Result<super::RebootResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to reboot command"]
        async fn subscribe_reboot(
            &self,
            request: tonic::Request<super::SubscribeRebootRequest>,
        ) -> Result<tonic::Response<Self::SubscribeRebootStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeShutdown method."]
        type SubscribeShutdownStream: futures_core::Stream<Item = Result<super::ShutdownResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to shutdown command"]
        async fn subscribe_shutdown(
            &self,
            request: tonic::Request<super::SubscribeShutdownRequest>,
        ) -> Result<tonic::Response<Self::SubscribeShutdownStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeTerminate method."]
        type SubscribeTerminateStream: futures_core::Stream<Item = Result<super::TerminateResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to terminate command"]
        async fn subscribe_terminate(
            &self,
            request: tonic::Request<super::SubscribeTerminateRequest>,
        ) -> Result<tonic::Response<Self::SubscribeTerminateStream>, tonic::Status>;
        #[doc = " Can the vehicle takeoff"]
        async fn set_allow_takeoff(
            &self,
            request: tonic::Request<super::SetAllowTakeoffRequest>,
        ) -> Result<tonic::Response<super::SetAllowTakeoffResponse>, tonic::Status>;
        #[doc = " Can the vehicle arm when requested"]
        async fn set_armable(
            &self,
            request: tonic::Request<super::SetArmableRequest>,
        ) -> Result<tonic::Response<super::SetArmableResponse>, tonic::Status>;
        #[doc = " Can the vehicle disarm when requested"]
        async fn set_disarmable(
            &self,
            request: tonic::Request<super::SetDisarmableRequest>,
        ) -> Result<tonic::Response<super::SetDisarmableResponse>, tonic::Status>;
        #[doc = " Set which modes the vehicle can transition to (Manual always allowed)"]
        async fn set_allowable_flight_modes(
            &self,
            request: tonic::Request<super::SetAllowableFlightModesRequest>,
        ) -> Result<tonic::Response<super::SetAllowableFlightModesResponse>, tonic::Status>;
        #[doc = " Get which modes the vehicle can transition to (Manual always allowed)"]
        async fn get_allowable_flight_modes(
            &self,
            request: tonic::Request<super::GetAllowableFlightModesRequest>,
        ) -> Result<tonic::Response<super::GetAllowableFlightModesResponse>, tonic::Status>;
    }
    #[doc = " Provide vehicle actions (as a server) such as arming, taking off, and landing."]
    #[derive(Debug)]
    pub struct ActionServerServiceServer<T: ActionServerService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ActionServerService> ActionServerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ActionServerServiceServer<T>
    where
        T: ActionServerService,
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
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeArmDisarm" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeArmDisarmSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<super::SubscribeArmDisarmRequest>
                        for SubscribeArmDisarmSvc<T>
                    {
                        type Response = super::ArmDisarmResponse;
                        type ResponseStream = T::SubscribeArmDisarmStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeArmDisarmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_arm_disarm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeArmDisarmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeFlightModeChange" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeFlightModeChangeSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeFlightModeChangeRequest,
                        > for SubscribeFlightModeChangeSvc<T>
                    {
                        type Response = super::FlightModeChangeResponse;
                        type ResponseStream = T::SubscribeFlightModeChangeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeFlightModeChangeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_flight_mode_change(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeFlightModeChangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeTakeoff" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTakeoffSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<super::SubscribeTakeoffRequest>
                        for SubscribeTakeoffSvc<T>
                    {
                        type Response = super::TakeoffResponse;
                        type ResponseStream = T::SubscribeTakeoffStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeTakeoffRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_takeoff(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTakeoffSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeLand" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeLandSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<super::SubscribeLandRequest>
                        for SubscribeLandSvc<T>
                    {
                        type Response = super::LandResponse;
                        type ResponseStream = T::SubscribeLandStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeLandRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_land(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeLandSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeReboot" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeRebootSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<super::SubscribeRebootRequest>
                        for SubscribeRebootSvc<T>
                    {
                        type Response = super::RebootResponse;
                        type ResponseStream = T::SubscribeRebootStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRebootRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_reboot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeRebootSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeShutdown" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeShutdownSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<super::SubscribeShutdownRequest>
                        for SubscribeShutdownSvc<T>
                    {
                        type Response = super::ShutdownResponse;
                        type ResponseStream = T::SubscribeShutdownStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeShutdownRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_shutdown(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeShutdownSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SubscribeTerminate" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTerminateSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::ServerStreamingService<super::SubscribeTerminateRequest>
                        for SubscribeTerminateSvc<T>
                    {
                        type Response = super::TerminateResponse;
                        type ResponseStream = T::SubscribeTerminateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeTerminateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_terminate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTerminateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.action_server.ActionServerService/SetAllowTakeoff" => {
                    #[allow(non_camel_case_types)]
                    struct SetAllowTakeoffSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::UnaryService<super::SetAllowTakeoffRequest>
                        for SetAllowTakeoffSvc<T>
                    {
                        type Response = super::SetAllowTakeoffResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAllowTakeoffRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_allow_takeoff(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAllowTakeoffSvc(inner);
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
                "/mavsdk.rpc.action_server.ActionServerService/SetArmable" => {
                    #[allow(non_camel_case_types)]
                    struct SetArmableSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::UnaryService<super::SetArmableRequest> for SetArmableSvc<T>
                    {
                        type Response = super::SetArmableResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetArmableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_armable(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetArmableSvc(inner);
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
                "/mavsdk.rpc.action_server.ActionServerService/SetDisarmable" => {
                    #[allow(non_camel_case_types)]
                    struct SetDisarmableSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::UnaryService<super::SetDisarmableRequest>
                        for SetDisarmableSvc<T>
                    {
                        type Response = super::SetDisarmableResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDisarmableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_disarmable(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDisarmableSvc(inner);
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
                "/mavsdk.rpc.action_server.ActionServerService/SetAllowableFlightModes" => {
                    #[allow(non_camel_case_types)]
                    struct SetAllowableFlightModesSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::UnaryService<super::SetAllowableFlightModesRequest>
                        for SetAllowableFlightModesSvc<T>
                    {
                        type Response = super::SetAllowableFlightModesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAllowableFlightModesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_allowable_flight_modes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAllowableFlightModesSvc(inner);
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
                "/mavsdk.rpc.action_server.ActionServerService/GetAllowableFlightModes" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllowableFlightModesSvc<T: ActionServerService>(pub Arc<T>);
                    impl<T: ActionServerService>
                        tonic::server::UnaryService<super::GetAllowableFlightModesRequest>
                        for GetAllowableFlightModesSvc<T>
                    {
                        type Response = super::GetAllowableFlightModesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllowableFlightModesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_allowable_flight_modes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllowableFlightModesSvc(inner);
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
    impl<T: ActionServerService> Clone for ActionServerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ActionServerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ActionServerService> tonic::transport::NamedService for ActionServerServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.action_server.ActionServerService";
    }
}
