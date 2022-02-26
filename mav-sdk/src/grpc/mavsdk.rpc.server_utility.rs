#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SendStatusTextRequest {
    /// The text to send
    #[prost(enumeration = "StatusTextType", tag = "1")]
    pub r#type: i32,
    /// Text message
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SendStatusTextResponse {
    #[prost(message, optional, tag = "1")]
    pub server_utility_result: ::core::option::Option<ServerUtilityResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ServerUtilityResult {
    /// Result enum value
    #[prost(enumeration = "server_utility_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ServerUtilityResult`.
pub mod server_utility_result {
    /// Possible results returned for server utility requests.
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
        /// Invalid argument
        InvalidArgument = 4,
    }
}
/// Status types.
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
#[doc = r" Generated client implementations."]
pub mod server_utility_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Utility for onboard MAVSDK instances for common \"server\" tasks."]
    #[derive(Debug, Clone)]
    pub struct ServerUtilityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServerUtilityServiceClient<tonic::transport::Channel> {
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
    impl<T> ServerUtilityServiceClient<T>
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
        ) -> ServerUtilityServiceClient<InterceptedService<T, F>>
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
            ServerUtilityServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Sends a statustext."]
        pub async fn send_status_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SendStatusTextRequest>,
        ) -> Result<tonic::Response<super::SendStatusTextResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.server_utility.ServerUtilityService/SendStatusText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod server_utility_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ServerUtilityServiceServer."]
    #[async_trait]
    pub trait ServerUtilityService: Send + Sync + 'static {
        #[doc = " Sends a statustext."]
        async fn send_status_text(
            &self,
            request: tonic::Request<super::SendStatusTextRequest>,
        ) -> Result<tonic::Response<super::SendStatusTextResponse>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = " Utility for onboard MAVSDK instances for common \"server\" tasks."]
    #[derive(Debug)]
    pub struct ServerUtilityServiceServer<T: ServerUtilityService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServerUtilityService> ServerUtilityServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServerUtilityServiceServer<T>
    where
        T: ServerUtilityService,
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
                "/mavsdk.rpc.server_utility.ServerUtilityService/SendStatusText" => {
                    #[allow(non_camel_case_types)]
                    struct SendStatusTextSvc<T: ServerUtilityService>(pub Arc<T>);
                    impl<T: ServerUtilityService>
                        tonic::server::UnaryService<super::SendStatusTextRequest>
                        for SendStatusTextSvc<T>
                    {
                        type Response = super::SendStatusTextResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendStatusTextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_status_text(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendStatusTextSvc(inner);
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
    impl<T: ServerUtilityService> Clone for ServerUtilityServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ServerUtilityService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServerUtilityService> tonic::transport::NamedService for ServerUtilityServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.server_utility.ServerUtilityService";
    }
}
