#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeIncomingMissionRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingMissionResponse {
    #[prost(message, optional, tag="1")]
    pub mission_raw_server_result: ::core::option::Option<MissionRawServerResult>,
    /// The mission plan
    #[prost(message, optional, tag="2")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCurrentItemChangedRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentItemChangedResponse {
    #[prost(message, optional, tag="1")]
    pub mission_item: ::core::option::Option<MissionItem>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeClearAllRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearAllResponse {
    #[prost(uint32, tag="1")]
    pub clear_type: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentItemCompleteRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentItemCompleteResponse {
}
/// Mission item exactly identical to MAVLink MISSION_ITEM_INT.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionItem {
    /// Sequence (uint16_t)
    #[prost(uint32, tag="1")]
    pub seq: u32,
    /// The coordinate system of the waypoint (actually uint8_t)
    #[prost(uint32, tag="2")]
    pub frame: u32,
    /// The scheduled action for the waypoint (actually uint16_t)
    #[prost(uint32, tag="3")]
    pub command: u32,
    /// false:0, true:1 (actually uint8_t)
    #[prost(uint32, tag="4")]
    pub current: u32,
    /// Autocontinue to next waypoint (actually uint8_t)
    #[prost(uint32, tag="5")]
    pub autocontinue: u32,
    /// PARAM1, see MAV_CMD enum
    #[prost(float, tag="6")]
    pub param1: f32,
    /// PARAM2, see MAV_CMD enum
    #[prost(float, tag="7")]
    pub param2: f32,
    /// PARAM3, see MAV_CMD enum
    #[prost(float, tag="8")]
    pub param3: f32,
    /// PARAM4, see MAV_CMD enum
    #[prost(float, tag="9")]
    pub param4: f32,
    /// PARAM5 / local: x position in meters * 1e4, global: latitude in degrees * 10^7
    #[prost(int32, tag="10")]
    pub x: i32,
    /// PARAM6 / y position: local: x position in meters * 1e4, global: longitude in degrees *10^7
    #[prost(int32, tag="11")]
    pub y: i32,
    /// PARAM7 / local: Z coordinate, global: altitude (relative or absolute, depending on frame)
    #[prost(float, tag="12")]
    pub z: f32,
    /// Mission type (actually uint8_t)
    #[prost(uint32, tag="13")]
    pub mission_type: u32,
}
/// Mission plan type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionPlan {
    /// The mission items
    #[prost(message, repeated, tag="1")]
    pub mission_items: ::prost::alloc::vec::Vec<MissionItem>,
}
/// Mission progress type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionProgress {
    /// Current mission item index (0-based), if equal to total, the mission is finished
    #[prost(int32, tag="1")]
    pub current: i32,
    /// Total number of mission items
    #[prost(int32, tag="2")]
    pub total: i32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionRawServerResult {
    /// Result enum value
    #[prost(enumeration="mission_raw_server_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MissionRawServerResult`.
pub mod mission_raw_server_result {
    /// Possible results returned for action requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Error
        Error = 2,
        /// Too many mission items in the mission
        TooManyMissionItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
        /// Mission downloaded from the system is not supported
        Unsupported = 7,
        /// No mission available on the system
        NoMissionAvailable = 8,
        /// Unsupported mission command
        UnsupportedMissionCmd = 11,
        /// Mission transfer (upload or download) has been cancelled
        TransferCancelled = 12,
        /// No system connected
        NoSystem = 13,
        /// Intermediate message showing progress or instructions on the next steps
        Next = 14,
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
                Result::Error => "RESULT_ERROR",
                Result::TooManyMissionItems => "RESULT_TOO_MANY_MISSION_ITEMS",
                Result::Busy => "RESULT_BUSY",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::InvalidArgument => "RESULT_INVALID_ARGUMENT",
                Result::Unsupported => "RESULT_UNSUPPORTED",
                Result::NoMissionAvailable => "RESULT_NO_MISSION_AVAILABLE",
                Result::UnsupportedMissionCmd => "RESULT_UNSUPPORTED_MISSION_CMD",
                Result::TransferCancelled => "RESULT_TRANSFER_CANCELLED",
                Result::NoSystem => "RESULT_NO_SYSTEM",
                Result::Next => "RESULT_NEXT",
            }
        }
    }
}
/// Generated client implementations.
pub mod mission_raw_server_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Acts as a vehicle and receives incoming missions from GCS (in raw MAVLINK format).
    /// Provides current mission item state, so the server can progress through missions.
    #[derive(Debug, Clone)]
    pub struct MissionRawServerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MissionRawServerServiceClient<tonic::transport::Channel> {
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
    impl<T> MissionRawServerServiceClient<T>
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
        ) -> MissionRawServerServiceClient<InterceptedService<T, F>>
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
            MissionRawServerServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        ///
        /// Subscribe to when a new mission is uploaded (asynchronous).
        pub async fn subscribe_incoming_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeIncomingMissionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::IncomingMissionResponse>>,
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SubscribeIncomingMission",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///
        /// Subscribe to when a new current item is set
        pub async fn subscribe_current_item_changed(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCurrentItemChangedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CurrentItemChangedResponse>>,
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SubscribeCurrentItemChanged",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///
        ///  Set Current item as completed
        pub async fn set_current_item_complete(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCurrentItemCompleteRequest>,
        ) -> Result<
            tonic::Response<super::SetCurrentItemCompleteResponse>,
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SetCurrentItemComplete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///  Subscribe when a MISSION_CLEAR_ALL is received
        pub async fn subscribe_clear_all(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeClearAllRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ClearAllResponse>>,
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SubscribeClearAll",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod mission_raw_server_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MissionRawServerServiceServer.
    #[async_trait]
    pub trait MissionRawServerService: Send + Sync + 'static {
        ///Server streaming response type for the SubscribeIncomingMission method.
        type SubscribeIncomingMissionStream: futures_core::Stream<
                Item = Result<super::IncomingMissionResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// Subscribe to when a new mission is uploaded (asynchronous).
        async fn subscribe_incoming_mission(
            &self,
            request: tonic::Request<super::SubscribeIncomingMissionRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeIncomingMissionStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeCurrentItemChanged method.
        type SubscribeCurrentItemChangedStream: futures_core::Stream<
                Item = Result<super::CurrentItemChangedResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// Subscribe to when a new current item is set
        async fn subscribe_current_item_changed(
            &self,
            request: tonic::Request<super::SubscribeCurrentItemChangedRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeCurrentItemChangedStream>,
            tonic::Status,
        >;
        ///
        ///  Set Current item as completed
        async fn set_current_item_complete(
            &self,
            request: tonic::Request<super::SetCurrentItemCompleteRequest>,
        ) -> Result<
            tonic::Response<super::SetCurrentItemCompleteResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeClearAll method.
        type SubscribeClearAllStream: futures_core::Stream<
                Item = Result<super::ClearAllResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        ///  Subscribe when a MISSION_CLEAR_ALL is received
        async fn subscribe_clear_all(
            &self,
            request: tonic::Request<super::SubscribeClearAllRequest>,
        ) -> Result<tonic::Response<Self::SubscribeClearAllStream>, tonic::Status>;
    }
    /// Acts as a vehicle and receives incoming missions from GCS (in raw MAVLINK format).
    /// Provides current mission item state, so the server can progress through missions.
    #[derive(Debug)]
    pub struct MissionRawServerServiceServer<T: MissionRawServerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MissionRawServerService> MissionRawServerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for MissionRawServerServiceServer<T>
    where
        T: MissionRawServerService,
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SubscribeIncomingMission" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeIncomingMissionSvc<T: MissionRawServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MissionRawServerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeIncomingMissionRequest,
                    > for SubscribeIncomingMissionSvc<T> {
                        type Response = super::IncomingMissionResponse;
                        type ResponseStream = T::SubscribeIncomingMissionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeIncomingMissionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_incoming_mission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeIncomingMissionSvc(inner);
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SubscribeCurrentItemChanged" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCurrentItemChangedSvc<T: MissionRawServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MissionRawServerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeCurrentItemChangedRequest,
                    > for SubscribeCurrentItemChangedSvc<T> {
                        type Response = super::CurrentItemChangedResponse;
                        type ResponseStream = T::SubscribeCurrentItemChangedStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeCurrentItemChangedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_current_item_changed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCurrentItemChangedSvc(inner);
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SetCurrentItemComplete" => {
                    #[allow(non_camel_case_types)]
                    struct SetCurrentItemCompleteSvc<T: MissionRawServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MissionRawServerService,
                    > tonic::server::UnaryService<super::SetCurrentItemCompleteRequest>
                    for SetCurrentItemCompleteSvc<T> {
                        type Response = super::SetCurrentItemCompleteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetCurrentItemCompleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_current_item_complete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetCurrentItemCompleteSvc(inner);
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
                "/mavsdk.rpc.mission_raw_server.MissionRawServerService/SubscribeClearAll" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeClearAllSvc<T: MissionRawServerService>(pub Arc<T>);
                    impl<
                        T: MissionRawServerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeClearAllRequest,
                    > for SubscribeClearAllSvc<T> {
                        type Response = super::ClearAllResponse;
                        type ResponseStream = T::SubscribeClearAllStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeClearAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_clear_all(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeClearAllSvc(inner);
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
    impl<T: MissionRawServerService> Clone for MissionRawServerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MissionRawServerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MissionRawServerService> tonic::server::NamedService
    for MissionRawServerServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.mission_raw_server.MissionRawServerService";
    }
}
