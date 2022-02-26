#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamIntRequest {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamIntResponse {
    #[prost(message, optional, tag = "1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
    /// Value of the requested parameter
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamIntRequest {
    /// Name of the parameter to provide
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamIntResponse {
    #[prost(message, optional, tag = "1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamFloatRequest {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamFloatResponse {
    #[prost(message, optional, tag = "1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
    /// Value of the requested parameter
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamFloatRequest {
    /// Name of the parameter to provide
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamFloatResponse {
    #[prost(message, optional, tag = "1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RetrieveAllParamsRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RetrieveAllParamsResponse {
    /// Collection of all parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<AllParams>,
}
///
/// Type for integer parameters.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IntParam {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter
    #[prost(int32, tag = "2")]
    pub value: i32,
}
///
/// Type for float parameters.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FloatParam {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter
    #[prost(float, tag = "2")]
    pub value: f32,
}
///
/// Type collecting all integer and float parameters.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AllParams {
    /// Collection of all parameter names and values of type int
    #[prost(message, repeated, tag = "1")]
    pub int_params: ::prost::alloc::vec::Vec<IntParam>,
    /// Collection of all parameter names and values of type float
    #[prost(message, repeated, tag = "2")]
    pub float_params: ::prost::alloc::vec::Vec<FloatParam>,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamServerResult {
    /// Result enum value
    #[prost(enumeration = "param_server_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ParamServerResult`.
pub mod param_server_result {
    /// Possible results returned for param requests.
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
        /// Not Found
        NotFound = 2,
        /// Wrong type
        WrongType = 3,
        /// Parameter name too long (> 16)
        ParamNameTooLong = 4,
        /// No system available
        NoSystem = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod param_server_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provide raw access to retrieve and provide server parameters."]
    #[derive(Debug, Clone)]
    pub struct ParamServerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParamServerServiceClient<tonic::transport::Channel> {
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
    impl<T> ParamServerServiceClient<T>
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
        ) -> ParamServerServiceClient<InterceptedService<T, F>>
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
            ParamServerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Retrieve an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn retrieve_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveParamIntRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamIntResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamInt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Provide an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn provide_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvideParamIntRequest>,
        ) -> Result<tonic::Response<super::ProvideParamIntResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamInt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Retrieve a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn retrieve_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveParamFloatRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamFloatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamFloat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Provide a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn provide_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvideParamFloatRequest>,
        ) -> Result<tonic::Response<super::ProvideParamFloatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamFloat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Retrieve all parameters."]
        pub async fn retrieve_all_params(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveAllParamsRequest>,
        ) -> Result<tonic::Response<super::RetrieveAllParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveAllParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod param_server_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ParamServerServiceServer."]
    #[async_trait]
    pub trait ParamServerService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Retrieve an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn retrieve_param_int(
            &self,
            request: tonic::Request<super::RetrieveParamIntRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamIntResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Provide an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn provide_param_int(
            &self,
            request: tonic::Request<super::ProvideParamIntRequest>,
        ) -> Result<tonic::Response<super::ProvideParamIntResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Retrieve a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn retrieve_param_float(
            &self,
            request: tonic::Request<super::RetrieveParamFloatRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamFloatResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Provide a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn provide_param_float(
            &self,
            request: tonic::Request<super::ProvideParamFloatRequest>,
        ) -> Result<tonic::Response<super::ProvideParamFloatResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Retrieve all parameters."]
        async fn retrieve_all_params(
            &self,
            request: tonic::Request<super::RetrieveAllParamsRequest>,
        ) -> Result<tonic::Response<super::RetrieveAllParamsResponse>, tonic::Status>;
    }
    #[doc = " Provide raw access to retrieve and provide server parameters."]
    #[derive(Debug)]
    pub struct ParamServerServiceServer<T: ParamServerService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ParamServerService> ParamServerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ParamServerServiceServer<T>
    where
        T: ParamServerService,
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveParamIntSvc<T: ParamServerService>(pub Arc<T>);
                    impl<T: ParamServerService>
                        tonic::server::UnaryService<super::RetrieveParamIntRequest>
                        for RetrieveParamIntSvc<T>
                    {
                        type Response = super::RetrieveParamIntResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).retrieve_param_int(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RetrieveParamIntSvc(inner);
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct ProvideParamIntSvc<T: ParamServerService>(pub Arc<T>);
                    impl<T: ParamServerService>
                        tonic::server::UnaryService<super::ProvideParamIntRequest>
                        for ProvideParamIntSvc<T>
                    {
                        type Response = super::ProvideParamIntResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProvideParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).provide_param_int(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProvideParamIntSvc(inner);
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveParamFloatSvc<T: ParamServerService>(pub Arc<T>);
                    impl<T: ParamServerService>
                        tonic::server::UnaryService<super::RetrieveParamFloatRequest>
                        for RetrieveParamFloatSvc<T>
                    {
                        type Response = super::RetrieveParamFloatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).retrieve_param_float(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RetrieveParamFloatSvc(inner);
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct ProvideParamFloatSvc<T: ParamServerService>(pub Arc<T>);
                    impl<T: ParamServerService>
                        tonic::server::UnaryService<super::ProvideParamFloatRequest>
                        for ProvideParamFloatSvc<T>
                    {
                        type Response = super::ProvideParamFloatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProvideParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).provide_param_float(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProvideParamFloatSvc(inner);
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveAllParams" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveAllParamsSvc<T: ParamServerService>(pub Arc<T>);
                    impl<T: ParamServerService>
                        tonic::server::UnaryService<super::RetrieveAllParamsRequest>
                        for RetrieveAllParamsSvc<T>
                    {
                        type Response = super::RetrieveAllParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveAllParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).retrieve_all_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RetrieveAllParamsSvc(inner);
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
    impl<T: ParamServerService> Clone for ParamServerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ParamServerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ParamServerService> tonic::transport::NamedService for ParamServerServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.param_server.ParamServerService";
    }
}
