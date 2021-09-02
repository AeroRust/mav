#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendRequest {
    /// The command line to send
    #[prost(string, tag = "1")]
    pub command: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(message, optional, tag = "1")]
    pub shell_result: ::core::option::Option<ShellResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeReceiveRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveResponse {
    /// Received data.
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellResult {
    /// Result enum value
    #[prost(enumeration = "shell_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ShellResult`.
pub mod shell_result {
    /// Possible results returned for shell requests
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
        /// Response was not received
        NoResponse = 4,
        /// Shell busy (transfer in progress)
        Busy = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod shell_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Allow to communicate with the vehicle's system shell."]
    #[derive(Debug, Clone)]
    pub struct ShellServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShellServiceClient<tonic::transport::Channel> {
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
    impl<T> ShellServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> ShellServiceClient<InterceptedService<T, F>>
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
            ShellServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Send a command line."]
        pub async fn send(
            &mut self,
            request: impl tonic::IntoRequest<super::SendRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/mavsdk.rpc.shell.ShellService/Send");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Receive feedback from a sent command line."]
        #[doc = ""]
        #[doc = " This subscription needs to be made before a command line is sent, otherwise, no response will be sent."]
        pub async fn subscribe_receive(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeReceiveRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReceiveResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.shell.ShellService/SubscribeReceive",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod shell_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ShellServiceServer."]
    #[async_trait]
    pub trait ShellService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Send a command line."]
        async fn send(
            &self,
            request: tonic::Request<super::SendRequest>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeReceive method."]
        type SubscribeReceiveStream: futures_core::Stream<Item = Result<super::ReceiveResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Receive feedback from a sent command line."]
        #[doc = ""]
        #[doc = " This subscription needs to be made before a command line is sent, otherwise, no response will be sent."]
        async fn subscribe_receive(
            &self,
            request: tonic::Request<super::SubscribeReceiveRequest>,
        ) -> Result<tonic::Response<Self::SubscribeReceiveStream>, tonic::Status>;
    }
    #[doc = "*"]
    #[doc = " Allow to communicate with the vehicle's system shell."]
    #[derive(Debug)]
    pub struct ShellServiceServer<T: ShellService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ShellService> ShellServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ShellServiceServer<T>
    where
        T: ShellService,
        B: Body + Send + Sync + 'static,
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
                "/mavsdk.rpc.shell.ShellService/Send" => {
                    #[allow(non_camel_case_types)]
                    struct SendSvc<T: ShellService>(pub Arc<T>);
                    impl<T: ShellService> tonic::server::UnaryService<super::SendRequest> for SendSvc<T> {
                        type Response = super::SendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendSvc(inner);
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
                "/mavsdk.rpc.shell.ShellService/SubscribeReceive" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeReceiveSvc<T: ShellService>(pub Arc<T>);
                    impl<T: ShellService>
                        tonic::server::ServerStreamingService<super::SubscribeReceiveRequest>
                        for SubscribeReceiveSvc<T>
                    {
                        type Response = super::ReceiveResponse;
                        type ResponseStream = T::SubscribeReceiveStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeReceiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_receive(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeReceiveSvc(inner);
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
    impl<T: ShellService> Clone for ShellServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ShellService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ShellService> tonic::transport::NamedService for ShellServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.shell.ShellService";
    }
}
