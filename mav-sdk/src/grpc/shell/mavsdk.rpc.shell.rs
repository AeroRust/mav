#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendRequest {
    /// The command line to send
    #[prost(string, tag = "1")]
    pub command: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(message, optional, tag = "1")]
    pub shell_result: ::std::option::Option<ShellResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeReceiveRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveResponse {
    /// Received data.
    #[prost(string, tag = "1")]
    pub data: std::string::String,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellResult {
    /// Result enum value
    #[prost(enumeration = "shell_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "*"]
    #[doc = " Allow to communicate with the vehicle's system shell."]
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
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
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
    impl<T: Clone> Clone for ShellServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ShellServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ShellServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod shell_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
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
        type SubscribeReceiveStream: Stream<Item = Result<super::ReceiveResponse, tonic::Status>>
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
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ShellService> ShellServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ShellServiceServer<T>
    where
        T: ShellService,
        B: HttpBody + Send + Sync + 'static,
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = SubscribeReceiveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ShellService> Clone for ShellServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ShellService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
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
