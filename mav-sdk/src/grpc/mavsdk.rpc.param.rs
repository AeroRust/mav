#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamIntRequest {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamIntResponse {
    #[prost(message, optional, tag="1")]
    pub param_result: ::core::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(int32, tag="2")]
    pub value: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamIntRequest {
    /// Name of the parameter to set
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(int32, tag="2")]
    pub value: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamIntResponse {
    #[prost(message, optional, tag="1")]
    pub param_result: ::core::option::Option<ParamResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamFloatRequest {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamFloatResponse {
    #[prost(message, optional, tag="1")]
    pub param_result: ::core::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(float, tag="2")]
    pub value: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamFloatRequest {
    /// Name of the parameter to set
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(float, tag="2")]
    pub value: f32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamFloatResponse {
    #[prost(message, optional, tag="1")]
    pub param_result: ::core::option::Option<ParamResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamCustomRequest {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamCustomResponse {
    #[prost(message, optional, tag="1")]
    pub param_result: ::core::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamCustomRequest {
    /// Name of the parameter to set
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value the parameter should be set to
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamCustomResponse {
    #[prost(message, optional, tag="1")]
    pub param_result: ::core::option::Option<ParamResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllParamsRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllParamsResponse {
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
/// Type for custom parameters
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomParam {
    /// Name of the parameter
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Value of the parameter (max len 128 bytes)
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
pub struct ParamResult {
    /// Result enum value
    #[prost(enumeration="param_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ParamResult`.
pub mod param_result {
    /// Possible results returned for param requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Request timed out
        Timeout = 2,
        /// Connection error
        ConnectionError = 3,
        /// Wrong type
        WrongType = 4,
        /// Parameter name too long (> 16)
        ParamNameTooLong = 5,
        /// No system connected
        NoSystem = 6,
        /// Param value too long (> 128)
        ParamValueTooLong = 7,
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
                Result::Timeout => "RESULT_TIMEOUT",
                Result::ConnectionError => "RESULT_CONNECTION_ERROR",
                Result::WrongType => "RESULT_WRONG_TYPE",
                Result::ParamNameTooLong => "RESULT_PARAM_NAME_TOO_LONG",
                Result::NoSystem => "RESULT_NO_SYSTEM",
                Result::ParamValueTooLong => "RESULT_PARAM_VALUE_TOO_LONG",
            }
        }
    }
}
/// Generated client implementations.
pub mod param_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provide raw access to get and set parameters.
    #[derive(Debug, Clone)]
    pub struct ParamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParamServiceClient<tonic::transport::Channel> {
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
    impl<T> ParamServiceClient<T>
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
        ) -> ParamServiceClient<InterceptedService<T, F>>
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
            ParamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Get an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn get_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParamIntRequest>,
        ) -> Result<tonic::Response<super::GetParamIntResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/GetParamInt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Set an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn set_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::SetParamIntRequest>,
        ) -> Result<tonic::Response<super::SetParamIntResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/SetParamInt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Get a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn get_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParamFloatRequest>,
        ) -> Result<tonic::Response<super::GetParamFloatResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/GetParamFloat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Set a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn set_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::SetParamFloatRequest>,
        ) -> Result<tonic::Response<super::SetParamFloatResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/SetParamFloat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Get a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn get_param_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParamCustomRequest>,
        ) -> Result<tonic::Response<super::GetParamCustomResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/GetParamCustom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Set a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        pub async fn set_param_custom(
            &mut self,
            request: impl tonic::IntoRequest<super::SetParamCustomRequest>,
        ) -> Result<tonic::Response<super::SetParamCustomResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/SetParamCustom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Get all parameters.
        pub async fn get_all_params(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllParamsRequest>,
        ) -> Result<tonic::Response<super::GetAllParamsResponse>, tonic::Status> {
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
                "/mavsdk.rpc.param.ParamService/GetAllParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod param_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ParamServiceServer.
    #[async_trait]
    pub trait ParamService: Send + Sync + 'static {
        ///
        /// Get an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn get_param_int(
            &self,
            request: tonic::Request<super::GetParamIntRequest>,
        ) -> Result<tonic::Response<super::GetParamIntResponse>, tonic::Status>;
        ///
        /// Set an int parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn set_param_int(
            &self,
            request: tonic::Request<super::SetParamIntRequest>,
        ) -> Result<tonic::Response<super::SetParamIntResponse>, tonic::Status>;
        ///
        /// Get a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn get_param_float(
            &self,
            request: tonic::Request<super::GetParamFloatRequest>,
        ) -> Result<tonic::Response<super::GetParamFloatResponse>, tonic::Status>;
        ///
        /// Set a float parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn set_param_float(
            &self,
            request: tonic::Request<super::SetParamFloatRequest>,
        ) -> Result<tonic::Response<super::SetParamFloatResponse>, tonic::Status>;
        ///
        /// Get a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn get_param_custom(
            &self,
            request: tonic::Request<super::GetParamCustomRequest>,
        ) -> Result<tonic::Response<super::GetParamCustomResponse>, tonic::Status>;
        ///
        /// Set a custom parameter.
        ///
        /// If the type is wrong, the result will be `WRONG_TYPE`.
        async fn set_param_custom(
            &self,
            request: tonic::Request<super::SetParamCustomRequest>,
        ) -> Result<tonic::Response<super::SetParamCustomResponse>, tonic::Status>;
        ///
        /// Get all parameters.
        async fn get_all_params(
            &self,
            request: tonic::Request<super::GetAllParamsRequest>,
        ) -> Result<tonic::Response<super::GetAllParamsResponse>, tonic::Status>;
    }
    /// Provide raw access to get and set parameters.
    #[derive(Debug)]
    pub struct ParamServiceServer<T: ParamService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ParamService> ParamServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ParamServiceServer<T>
    where
        T: ParamService,
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
                "/mavsdk.rpc.param.ParamService/GetParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct GetParamIntSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::GetParamIntRequest>
                    for GetParamIntSvc<T> {
                        type Response = super::GetParamIntResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_param_int(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetParamIntSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/SetParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct SetParamIntSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::SetParamIntRequest>
                    for SetParamIntSvc<T> {
                        type Response = super::SetParamIntResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_param_int(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetParamIntSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/GetParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct GetParamFloatSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::GetParamFloatRequest>
                    for GetParamFloatSvc<T> {
                        type Response = super::GetParamFloatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_param_float(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetParamFloatSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/SetParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct SetParamFloatSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::SetParamFloatRequest>
                    for SetParamFloatSvc<T> {
                        type Response = super::SetParamFloatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_param_float(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetParamFloatSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/GetParamCustom" => {
                    #[allow(non_camel_case_types)]
                    struct GetParamCustomSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::GetParamCustomRequest>
                    for GetParamCustomSvc<T> {
                        type Response = super::GetParamCustomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParamCustomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_param_custom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetParamCustomSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/SetParamCustom" => {
                    #[allow(non_camel_case_types)]
                    struct SetParamCustomSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::SetParamCustomRequest>
                    for SetParamCustomSvc<T> {
                        type Response = super::SetParamCustomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetParamCustomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_param_custom(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetParamCustomSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/GetAllParams" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllParamsSvc<T: ParamService>(pub Arc<T>);
                    impl<
                        T: ParamService,
                    > tonic::server::UnaryService<super::GetAllParamsRequest>
                    for GetAllParamsSvc<T> {
                        type Response = super::GetAllParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_all_params(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllParamsSvc(inner);
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
    impl<T: ParamService> Clone for ParamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ParamService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ParamService> tonic::server::NamedService for ParamServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.param.ParamService";
    }
}
