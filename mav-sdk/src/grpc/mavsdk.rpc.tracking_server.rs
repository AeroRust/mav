#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTrackingPointStatusRequest {
    /// The tracked point
    #[prost(message, optional, tag="1")]
    pub tracked_point: ::core::option::Option<TrackPoint>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTrackingPointStatusResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTrackingRectangleStatusRequest {
    /// The tracked rectangle
    #[prost(message, optional, tag="1")]
    pub tracked_rectangle: ::core::option::Option<TrackRectangle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTrackingRectangleStatusResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTrackingOffStatusRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTrackingOffStatusResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTrackingPointCommandRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingPointCommandResponse {
    /// The point to track if a point is to be tracked
    #[prost(message, optional, tag="1")]
    pub track_point: ::core::option::Option<TrackPoint>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTrackingRectangleCommandRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingRectangleCommandResponse {
    /// The point to track if a point is to be tracked
    #[prost(message, optional, tag="1")]
    pub track_rectangle: ::core::option::Option<TrackRectangle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTrackingOffCommandRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingOffCommandResponse {
    /// Unused
    #[prost(int32, tag="1")]
    pub dummy: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondTrackingPointCommandRequest {
    /// The ack to answer to the incoming command
    #[prost(enumeration="CommandAnswer", tag="1")]
    pub command_answer: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondTrackingPointCommandResponse {
    /// The result of sending the response.
    #[prost(message, optional, tag="1")]
    pub tracking_server_result: ::core::option::Option<TrackingServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondTrackingRectangleCommandRequest {
    /// The ack to answer to the incoming command
    #[prost(enumeration="CommandAnswer", tag="1")]
    pub command_answer: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondTrackingRectangleCommandResponse {
    /// The result of sending the response.
    #[prost(message, optional, tag="1")]
    pub tracking_server_result: ::core::option::Option<TrackingServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondTrackingOffCommandRequest {
    /// The ack to answer to the incoming command
    #[prost(enumeration="CommandAnswer", tag="1")]
    pub command_answer: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespondTrackingOffCommandResponse {
    /// The result of sending the response.
    #[prost(message, optional, tag="1")]
    pub tracking_server_result: ::core::option::Option<TrackingServerResult>,
}
/// Point description type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPoint {
    /// Point to track x value (normalized 0..1, 0 is left, 1 is right).
    #[prost(float, tag="1")]
    pub point_x: f32,
    /// Point to track y value (normalized 0..1, 0 is top, 1 is bottom).
    #[prost(float, tag="2")]
    pub point_y: f32,
    /// Point to track y value (normalized 0..1, 0 is top, 1 is bottom).
    #[prost(float, tag="3")]
    pub radius: f32,
}
/// Rectangle description type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackRectangle {
    /// Top left corner of rectangle x value (normalized 0..1, 0 is left, 1 is right).
    #[prost(float, tag="1")]
    pub top_left_corner_x: f32,
    /// Top left corner of rectangle y value (normalized 0..1, 0 is top, 1 is bottom).
    #[prost(float, tag="2")]
    pub top_left_corner_y: f32,
    /// Bottom right corner of rectangle x value (normalized 0..1, 0 is left, 1 is right).
    #[prost(float, tag="3")]
    pub bottom_right_corner_x: f32,
    /// Bottom right corner of rectangle y value (normalized 0..1, 0 is top, 1 is bottom).
    #[prost(float, tag="4")]
    pub bottom_right_corner_y: f32,
}
/// Result type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingServerResult {
    /// Result enum value
    #[prost(enumeration="tracking_server_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TrackingServerResult`.
pub mod tracking_server_result {
    /// Possible results returned for tracking_server requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
            }
        }
    }
}
/// Answer to respond to an incoming command
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandAnswer {
    /// Command accepted
    Accepted = 0,
    /// Command temporarily rejected
    TemporarilyRejected = 1,
    /// Command denied
    Denied = 2,
    /// Command unsupported
    Unsupported = 3,
    /// Command failed
    Failed = 4,
}
impl CommandAnswer {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandAnswer::Accepted => "COMMAND_ANSWER_ACCEPTED",
            CommandAnswer::TemporarilyRejected => "COMMAND_ANSWER_TEMPORARILY_REJECTED",
            CommandAnswer::Denied => "COMMAND_ANSWER_DENIED",
            CommandAnswer::Unsupported => "COMMAND_ANSWER_UNSUPPORTED",
            CommandAnswer::Failed => "COMMAND_ANSWER_FAILED",
        }
    }
}
/// Generated client implementations.
pub mod tracking_server_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// API for an onboard image tracking software.
    #[derive(Debug, Clone)]
    pub struct TrackingServerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TrackingServerServiceClient<tonic::transport::Channel> {
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
    impl<T> TrackingServerServiceClient<T>
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
        ) -> TrackingServerServiceClient<InterceptedService<T, F>>
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
            TrackingServerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Set/update the current point tracking status.
        pub async fn set_tracking_point_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTrackingPointStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetTrackingPointStatusResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SetTrackingPointStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set/update the current rectangle tracking status.
        pub async fn set_tracking_rectangle_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTrackingRectangleStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetTrackingRectangleStatusResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SetTrackingRectangleStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set the current tracking status to off.
        pub async fn set_tracking_off_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTrackingOffStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetTrackingOffStatusResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SetTrackingOffStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Subscribe to incoming tracking point command.
        pub async fn subscribe_tracking_point_command(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeTrackingPointCommandRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::TrackingPointCommandResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SubscribeTrackingPointCommand",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to incoming tracking rectangle command.
        pub async fn subscribe_tracking_rectangle_command(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeTrackingRectangleCommandRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::TrackingRectangleCommandResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SubscribeTrackingRectangleCommand",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Subscribe to incoming tracking off command.
        pub async fn subscribe_tracking_off_command(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeTrackingOffCommandRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TrackingOffCommandResponse>>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SubscribeTrackingOffCommand",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Respond to an incoming tracking point command.
        pub async fn respond_tracking_point_command(
            &mut self,
            request: impl tonic::IntoRequest<super::RespondTrackingPointCommandRequest>,
        ) -> Result<
            tonic::Response<super::RespondTrackingPointCommandResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/RespondTrackingPointCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Respond to an incoming tracking rectangle command.
        pub async fn respond_tracking_rectangle_command(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RespondTrackingRectangleCommandRequest,
            >,
        ) -> Result<
            tonic::Response<super::RespondTrackingRectangleCommandResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/RespondTrackingRectangleCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Respond to an incoming tracking off command.
        pub async fn respond_tracking_off_command(
            &mut self,
            request: impl tonic::IntoRequest<super::RespondTrackingOffCommandRequest>,
        ) -> Result<
            tonic::Response<super::RespondTrackingOffCommandResponse>,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/RespondTrackingOffCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod tracking_server_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TrackingServerServiceServer.
    #[async_trait]
    pub trait TrackingServerService: Send + Sync + 'static {
        /// Set/update the current point tracking status.
        async fn set_tracking_point_status(
            &self,
            request: tonic::Request<super::SetTrackingPointStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetTrackingPointStatusResponse>,
            tonic::Status,
        >;
        /// Set/update the current rectangle tracking status.
        async fn set_tracking_rectangle_status(
            &self,
            request: tonic::Request<super::SetTrackingRectangleStatusRequest>,
        ) -> Result<
            tonic::Response<super::SetTrackingRectangleStatusResponse>,
            tonic::Status,
        >;
        /// Set the current tracking status to off.
        async fn set_tracking_off_status(
            &self,
            request: tonic::Request<super::SetTrackingOffStatusRequest>,
        ) -> Result<tonic::Response<super::SetTrackingOffStatusResponse>, tonic::Status>;
        ///Server streaming response type for the SubscribeTrackingPointCommand method.
        type SubscribeTrackingPointCommandStream: futures_core::Stream<
                Item = Result<super::TrackingPointCommandResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to incoming tracking point command.
        async fn subscribe_tracking_point_command(
            &self,
            request: tonic::Request<super::SubscribeTrackingPointCommandRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeTrackingPointCommandStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeTrackingRectangleCommand method.
        type SubscribeTrackingRectangleCommandStream: futures_core::Stream<
                Item = Result<super::TrackingRectangleCommandResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to incoming tracking rectangle command.
        async fn subscribe_tracking_rectangle_command(
            &self,
            request: tonic::Request<super::SubscribeTrackingRectangleCommandRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeTrackingRectangleCommandStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the SubscribeTrackingOffCommand method.
        type SubscribeTrackingOffCommandStream: futures_core::Stream<
                Item = Result<super::TrackingOffCommandResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to incoming tracking off command.
        async fn subscribe_tracking_off_command(
            &self,
            request: tonic::Request<super::SubscribeTrackingOffCommandRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeTrackingOffCommandStream>,
            tonic::Status,
        >;
        /// Respond to an incoming tracking point command.
        async fn respond_tracking_point_command(
            &self,
            request: tonic::Request<super::RespondTrackingPointCommandRequest>,
        ) -> Result<
            tonic::Response<super::RespondTrackingPointCommandResponse>,
            tonic::Status,
        >;
        /// Respond to an incoming tracking rectangle command.
        async fn respond_tracking_rectangle_command(
            &self,
            request: tonic::Request<super::RespondTrackingRectangleCommandRequest>,
        ) -> Result<
            tonic::Response<super::RespondTrackingRectangleCommandResponse>,
            tonic::Status,
        >;
        /// Respond to an incoming tracking off command.
        async fn respond_tracking_off_command(
            &self,
            request: tonic::Request<super::RespondTrackingOffCommandRequest>,
        ) -> Result<
            tonic::Response<super::RespondTrackingOffCommandResponse>,
            tonic::Status,
        >;
    }
    /// API for an onboard image tracking software.
    #[derive(Debug)]
    pub struct TrackingServerServiceServer<T: TrackingServerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TrackingServerService> TrackingServerServiceServer<T> {
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
    for TrackingServerServiceServer<T>
    where
        T: TrackingServerService,
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SetTrackingPointStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetTrackingPointStatusSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::UnaryService<super::SetTrackingPointStatusRequest>
                    for SetTrackingPointStatusSvc<T> {
                        type Response = super::SetTrackingPointStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetTrackingPointStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_tracking_point_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetTrackingPointStatusSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SetTrackingRectangleStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetTrackingRectangleStatusSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::UnaryService<
                        super::SetTrackingRectangleStatusRequest,
                    > for SetTrackingRectangleStatusSvc<T> {
                        type Response = super::SetTrackingRectangleStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetTrackingRectangleStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_tracking_rectangle_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetTrackingRectangleStatusSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SetTrackingOffStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetTrackingOffStatusSvc<T: TrackingServerService>(pub Arc<T>);
                    impl<
                        T: TrackingServerService,
                    > tonic::server::UnaryService<super::SetTrackingOffStatusRequest>
                    for SetTrackingOffStatusSvc<T> {
                        type Response = super::SetTrackingOffStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetTrackingOffStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_tracking_off_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetTrackingOffStatusSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SubscribeTrackingPointCommand" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTrackingPointCommandSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeTrackingPointCommandRequest,
                    > for SubscribeTrackingPointCommandSvc<T> {
                        type Response = super::TrackingPointCommandResponse;
                        type ResponseStream = T::SubscribeTrackingPointCommandStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeTrackingPointCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_tracking_point_command(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTrackingPointCommandSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SubscribeTrackingRectangleCommand" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTrackingRectangleCommandSvc<
                        T: TrackingServerService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeTrackingRectangleCommandRequest,
                    > for SubscribeTrackingRectangleCommandSvc<T> {
                        type Response = super::TrackingRectangleCommandResponse;
                        type ResponseStream = T::SubscribeTrackingRectangleCommandStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeTrackingRectangleCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_tracking_rectangle_command(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTrackingRectangleCommandSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/SubscribeTrackingOffCommand" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTrackingOffCommandSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeTrackingOffCommandRequest,
                    > for SubscribeTrackingOffCommandSvc<T> {
                        type Response = super::TrackingOffCommandResponse;
                        type ResponseStream = T::SubscribeTrackingOffCommandStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeTrackingOffCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_tracking_off_command(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTrackingOffCommandSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/RespondTrackingPointCommand" => {
                    #[allow(non_camel_case_types)]
                    struct RespondTrackingPointCommandSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::UnaryService<
                        super::RespondTrackingPointCommandRequest,
                    > for RespondTrackingPointCommandSvc<T> {
                        type Response = super::RespondTrackingPointCommandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RespondTrackingPointCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).respond_tracking_point_command(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RespondTrackingPointCommandSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/RespondTrackingRectangleCommand" => {
                    #[allow(non_camel_case_types)]
                    struct RespondTrackingRectangleCommandSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::UnaryService<
                        super::RespondTrackingRectangleCommandRequest,
                    > for RespondTrackingRectangleCommandSvc<T> {
                        type Response = super::RespondTrackingRectangleCommandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RespondTrackingRectangleCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).respond_tracking_rectangle_command(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RespondTrackingRectangleCommandSvc(inner);
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
                "/mavsdk.rpc.tracking_server.TrackingServerService/RespondTrackingOffCommand" => {
                    #[allow(non_camel_case_types)]
                    struct RespondTrackingOffCommandSvc<T: TrackingServerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: TrackingServerService,
                    > tonic::server::UnaryService<
                        super::RespondTrackingOffCommandRequest,
                    > for RespondTrackingOffCommandSvc<T> {
                        type Response = super::RespondTrackingOffCommandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RespondTrackingOffCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).respond_tracking_off_command(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RespondTrackingOffCommandSvc(inner);
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
    impl<T: TrackingServerService> Clone for TrackingServerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TrackingServerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TrackingServerService> tonic::server::NamedService
    for TrackingServerServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.tracking_server.TrackingServerService";
    }
}
