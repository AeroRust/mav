#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeConnectionStateRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateResponse {
    /// Connection state
    #[prost(message, optional, tag = "1")]
    pub connection_state: ::core::option::Option<ConnectionState>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetMavlinkTimeoutRequest {
    /// Timeout in seconds
    #[prost(double, tag = "1")]
    pub timeout_s: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetMavlinkTimeoutResponse {}
/// Connection state type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ConnectionState {
    /// Whether the vehicle got connected or disconnected
    #[prost(bool, tag = "2")]
    pub is_connected: bool,
}
#[doc = r" Generated client implementations."]
pub mod core_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Access to the connection state and core configurations"]
    #[derive(Debug, Clone)]
    pub struct CoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CoreServiceClient<tonic::transport::Channel> {
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
    impl<T> CoreServiceClient<T>
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
        ) -> CoreServiceClient<InterceptedService<T, F>>
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
            CoreServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Subscribe to 'connection state' updates."]
        pub async fn subscribe_connection_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeConnectionStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ConnectionStateResponse>>,
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
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Set timeout of MAVLink transfers."]
        #[doc = ""]
        #[doc = " The default timeout used is generally (0.5 seconds) seconds."]
        #[doc = " If MAVSDK is used on the same host this timeout can be reduced, while"]
        #[doc = " if MAVSDK has to communicate over links with high latency it might"]
        #[doc = " need to be increased to prevent timeouts."]
        pub async fn set_mavlink_timeout(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMavlinkTimeoutRequest>,
        ) -> Result<tonic::Response<super::SetMavlinkTimeoutResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
#[doc = r" Generated server implementations."]
pub mod core_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CoreServiceServer."]
    #[async_trait]
    pub trait CoreService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeConnectionState method."]
        type SubscribeConnectionStateStream: futures_core::Stream<Item = Result<super::ConnectionStateResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to 'connection state' updates."]
        async fn subscribe_connection_state(
            &self,
            request: tonic::Request<super::SubscribeConnectionStateRequest>,
        ) -> Result<tonic::Response<Self::SubscribeConnectionStateStream>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set timeout of MAVLink transfers."]
        #[doc = ""]
        #[doc = " The default timeout used is generally (0.5 seconds) seconds."]
        #[doc = " If MAVSDK is used on the same host this timeout can be reduced, while"]
        #[doc = " if MAVSDK has to communicate over links with high latency it might"]
        #[doc = " need to be increased to prevent timeouts."]
        async fn set_mavlink_timeout(
            &self,
            request: tonic::Request<super::SetMavlinkTimeoutRequest>,
        ) -> Result<tonic::Response<super::SetMavlinkTimeoutResponse>, tonic::Status>;
    }
    #[doc = " Access to the connection state and core configurations"]
    #[derive(Debug)]
    pub struct CoreServiceServer<T: CoreService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CoreService> CoreServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CoreServiceServer<T>
    where
        T: CoreService,
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
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeConnectionStateSvc<T: CoreService>(pub Arc<T>);
                    impl<T: CoreService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeConnectionStateRequest,
                        > for SubscribeConnectionStateSvc<T>
                    {
                        type Response = super::ConnectionStateResponse;
                        type ResponseStream = T::SubscribeConnectionStateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeConnectionStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_connection_state(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
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
                    impl<T: CoreService>
                        tonic::server::UnaryService<super::SetMavlinkTimeoutRequest>
                        for SetMavlinkTimeoutSvc<T>
                    {
                        type Response = super::SetMavlinkTimeoutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetMavlinkTimeoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_mavlink_timeout(request).await };
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
    impl<T: CoreService> tonic::transport::NamedService for CoreServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.core.CoreService";
    }
}
