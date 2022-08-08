#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamIntRequest {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamIntResponse {
    #[prost(message, optional, tag="1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
    /// Value of the requested parameter
    #[prost(int32, tag="2")]
    pub value: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamIntRequest {
    /// Name of the parameter to provide
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(int32, tag="2")]
    pub value: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamIntResponse {
    #[prost(message, optional, tag="1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamFloatRequest {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamFloatResponse {
    #[prost(message, optional, tag="1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
    /// Value of the requested parameter
    #[prost(float, tag="2")]
    pub value: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamFloatRequest {
    /// Name of the parameter to provide
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(float, tag="2")]
    pub value: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamFloatResponse {
    #[prost(message, optional, tag="1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamCustomRequest {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveParamCustomResponse {
    #[prost(message, optional, tag="1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
    /// Value of the requested parameter
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamCustomRequest {
    /// Name of the parameter to provide
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvideParamCustomResponse {
    #[prost(message, optional, tag="1")]
    pub param_server_result: ::core::option::Option<ParamServerResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveAllParamsRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveAllParamsResponse {
    /// Collection of all parameters
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<AllParams>,
}
///
/// Type for integer parameters.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntParam {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter
    #[prost(int32, tag="2")]
    pub value: i32,
}
///
/// Type for float parameters.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatParam {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter
    #[prost(float, tag="2")]
    pub value: f32,
}
///
/// Type for float parameters.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomParam {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
///
/// Type collecting all integer, float, and custom parameters.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllParams {
    /// Collection of all parameter names and values of type int
    #[prost(message, repeated, tag="1")]
    pub int_params: ::prost::alloc::vec::Vec<IntParam>,
    /// Collection of all parameter names and values of type float
    #[prost(message, repeated, tag="2")]
    pub float_params: ::prost::alloc::vec::Vec<FloatParam>,
    /// Collection of all parameter names and values of type custom
    #[prost(message, repeated, tag="3")]
    pub custom_params: ::prost::alloc::vec::Vec<CustomParam>,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamServerResult {
    /// Result enum value
    #[prost(enumeration="param_server_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ParamServerResult`.
pub mod param_server_result {
    /// Possible results returned for param requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
        /// Parameter name too long (> 128)
        ParamValueTooLong = 6,
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
                Result::NotFound => "RESULT_NOT_FOUND",
                Result::WrongType => "RESULT_WRONG_TYPE",
                Result::ParamNameTooLong => "RESULT_PARAM_NAME_TOO_LONG",
                Result::NoSystem => "RESULT_NO_SYSTEM",
                Result::ParamValueTooLong => "RESULT_PARAM_VALUE_TOO_LONG",
            }
        }
    }
}
/// Generated client implementations.
pub mod param_server_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provide raw access to retrieve and provide server parameters.
    #[derive(Debug, Clone)]
    pub struct ParamServerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParamServerServiceClient<tonic::transport::Channel> {
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
    impl<T> ParamServerServiceClient<T>
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
        ) -> ParamServerServiceClient<InterceptedService<T, F>>
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
            ParamServerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Retrieve an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn retrieve_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveParamIntRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamIntResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamInt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Provide an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn provide_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvideParamIntRequest>,
        ) -> Result<tonic::Response<super::ProvideParamIntResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamInt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Retrieve a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn retrieve_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveParamFloatRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamFloatResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamFloat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Provide a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn provide_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvideParamFloatRequest>,
        ) -> Result<tonic::Response<super::ProvideParamFloatResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamFloat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Retrieve a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn retrieve_param_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveParamCustomRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamCustomResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamCustom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Provide a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn provide_param_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvideParamCustomRequest>,
        ) -> Result<tonic::Response<super::ProvideParamCustomResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamCustom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Retrieve all parameters.
        pub async fn retrieve_all_params(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveAllParamsRequest>,
        ) -> Result<tonic::Response<super::RetrieveAllParamsResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveAllParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod param_server_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ParamServerServiceServer.
    #[async_trait]
    pub trait ParamServerService: Send + Sync + 'static {
        ///
        /// Retrieve an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn retrieve_param_int(
            &self,
            request: tonic::Request<super::RetrieveParamIntRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamIntResponse>, tonic::Status>;
        ///
        /// Provide an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn provide_param_int(
            &self,
            request: tonic::Request<super::ProvideParamIntRequest>,
        ) -> Result<tonic::Response<super::ProvideParamIntResponse>, tonic::Status>;
        ///
        /// Retrieve a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn retrieve_param_float(
            &self,
            request: tonic::Request<super::RetrieveParamFloatRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamFloatResponse>, tonic::Status>;
        ///
        /// Provide a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn provide_param_float(
            &self,
            request: tonic::Request<super::ProvideParamFloatRequest>,
        ) -> Result<tonic::Response<super::ProvideParamFloatResponse>, tonic::Status>;
        ///
        /// Retrieve a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn retrieve_param_custom(
            &self,
            request: tonic::Request<super::RetrieveParamCustomRequest>,
        ) -> Result<tonic::Response<super::RetrieveParamCustomResponse>, tonic::Status>;
        ///
        /// Provide a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn provide_param_custom(
            &self,
            request: tonic::Request<super::ProvideParamCustomRequest>,
        ) -> Result<tonic::Response<super::ProvideParamCustomResponse>, tonic::Status>;
        ///
        /// Retrieve all parameters.
        async fn retrieve_all_params(
            &self,
            request: tonic::Request<super::RetrieveAllParamsRequest>,
        ) -> Result<tonic::Response<super::RetrieveAllParamsResponse>, tonic::Status>;
    }
    /// Provide raw access to retrieve and provide server parameters.
    #[derive(Debug)]
    pub struct ParamServerServiceServer<T: ParamServerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ParamServerService> ParamServerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ParamServerServiceServer<T>
    where
        T: ParamServerService,
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveParamIntSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::RetrieveParamIntRequest>
                    for RetrieveParamIntSvc<T> {
                        type Response = super::RetrieveParamIntResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).retrieve_param_int(request).await
                            };
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct ProvideParamIntSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::ProvideParamIntRequest>
                    for ProvideParamIntSvc<T> {
                        type Response = super::ProvideParamIntResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProvideParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).provide_param_int(request).await
                            };
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveParamFloatSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::RetrieveParamFloatRequest>
                    for RetrieveParamFloatSvc<T> {
                        type Response = super::RetrieveParamFloatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).retrieve_param_float(request).await
                            };
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct ProvideParamFloatSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::ProvideParamFloatRequest>
                    for ProvideParamFloatSvc<T> {
                        type Response = super::ProvideParamFloatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProvideParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).provide_param_float(request).await
                            };
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveParamCustom" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveParamCustomSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::RetrieveParamCustomRequest>
                    for RetrieveParamCustomSvc<T> {
                        type Response = super::RetrieveParamCustomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveParamCustomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).retrieve_param_custom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RetrieveParamCustomSvc(inner);
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
                "/mavsdk.rpc.param_server.ParamServerService/ProvideParamCustom" => {
                    #[allow(non_camel_case_types)]
                    struct ProvideParamCustomSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::ProvideParamCustomRequest>
                    for ProvideParamCustomSvc<T> {
                        type Response = super::ProvideParamCustomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProvideParamCustomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).provide_param_custom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProvideParamCustomSvc(inner);
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
                "/mavsdk.rpc.param_server.ParamServerService/RetrieveAllParams" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveAllParamsSvc<T: ParamServerService>(pub Arc<T>);
                    impl<
                        T: ParamServerService,
                    > tonic::server::UnaryService<super::RetrieveAllParamsRequest>
                    for RetrieveAllParamsSvc<T> {
                        type Response = super::RetrieveAllParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveAllParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).retrieve_all_params(request).await
                            };
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
    impl<T: ParamServerService> tonic::server::NamedService
    for ParamServerServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.param_server.ParamServerService";
    }
}
