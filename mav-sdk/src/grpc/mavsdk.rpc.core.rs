#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeConnectionStateRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateResponse {
    /// Connection state
    #[prost(message, optional, tag="1")]
    pub connection_state: ::core::option::Option<ConnectionState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMavlinkTimeoutRequest {
    /// Timeout in seconds
    #[prost(double, tag="1")]
    pub timeout_s: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMavlinkTimeoutResponse {
}
/// Connection state type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionState {
    /// Whether the vehicle got connected or disconnected
    #[prost(bool, tag="2")]
    pub is_connected: bool,
}
/// Generated client implementations.
pub mod core_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Access to the connection state and core configurations
    #[derive(Debug, Clone)]
    pub struct CoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CoreServiceClient<tonic::transport::Channel> {
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
    impl<T> CoreServiceClient<T>
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
        ) -> CoreServiceClient<InterceptedService<T, F>>
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
            CoreServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Subscribe to 'connection state' updates.
        pub async fn subscribe_connection_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeConnectionStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ConnectionStateResponse>>,
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
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///
        /// Set timeout of MAVLink transfers.
        ///
        /// The default timeout used is generally (0.5 seconds) seconds.
        /// If MAVSDK is used on the same host this timeout can be reduced, while
        /// if MAVSDK has to communicate over links with high latency it might
        /// need to be increased to prevent timeouts.
        pub async fn set_mavlink_timeout(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMavlinkTimeoutRequest>,
        ) -> Result<tonic::Response<super::SetMavlinkTimeoutResponse>, tonic::Status> {
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
                "/mavsdk.rpc.core.CoreService/SetMavlinkTimeout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod core_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CoreServiceServer.
    #[async_trait]
    pub trait CoreService: Send + Sync + 'static {
        ///Server streaming response type for the SubscribeConnectionState method.
        type SubscribeConnectionStateStream: futures_core::Stream<
                Item = Result<super::ConnectionStateResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// Subscribe to 'connection state' updates.
        async fn subscribe_connection_state(
            &self,
            request: tonic::Request<super::SubscribeConnectionStateRequest>,
        ) -> Result<
            tonic::Response<Self::SubscribeConnectionStateStream>,
            tonic::Status,
        >;
        ///
        /// Set timeout of MAVLink transfers.
        ///
        /// The default timeout used is generally (0.5 seconds) seconds.
        /// If MAVSDK is used on the same host this timeout can be reduced, while
        /// if MAVSDK has to communicate over links with high latency it might
        /// need to be increased to prevent timeouts.
        async fn set_mavlink_timeout(
            &self,
            request: tonic::Request<super::SetMavlinkTimeoutRequest>,
        ) -> Result<tonic::Response<super::SetMavlinkTimeoutResponse>, tonic::Status>;
    }
    /// Access to the connection state and core configurations
    #[derive(Debug)]
    pub struct CoreServiceServer<T: CoreService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CoreService> CoreServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CoreServiceServer<T>
    where
        T: CoreService,
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
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeConnectionStateSvc<T: CoreService>(pub Arc<T>);
                    impl<
                        T: CoreService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeConnectionStateRequest,
                    > for SubscribeConnectionStateSvc<T> {
                        type Response = super::ConnectionStateResponse;
                        type ResponseStream = T::SubscribeConnectionStateStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeConnectionStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_connection_state(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeConnectionStateSvc(inner);
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
                "/mavsdk.rpc.core.CoreService/SetMavlinkTimeout" => {
                    #[allow(non_camel_case_types)]
                    struct SetMavlinkTimeoutSvc<T: CoreService>(pub Arc<T>);
                    impl<
                        T: CoreService,
                    > tonic::server::UnaryService<super::SetMavlinkTimeoutRequest>
                    for SetMavlinkTimeoutSvc<T> {
                        type Response = super::SetMavlinkTimeoutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetMavlinkTimeoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_mavlink_timeout(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetMavlinkTimeoutSvc(inner);
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
    impl<T: CoreService> Clone for CoreServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CoreService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CoreService> tonic::server::NamedService for CoreServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.core.CoreService";
    }
}
