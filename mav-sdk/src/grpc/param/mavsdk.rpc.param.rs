#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamIntRequest {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamIntResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::std::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamIntRequest {
    /// Name of the parameter to set
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Value the parameter should be set to
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamIntResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::std::option::Option<ParamResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamFloatRequest {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParamFloatResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::std::option::Option<ParamResult>,
    /// Value of the requested parameter
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamFloatRequest {
    /// Name of the parameter to set
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Value the parameter should be set to
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamFloatResponse {
    #[prost(message, optional, tag = "1")]
    pub param_result: ::std::option::Option<ParamResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllParamsResponse {
    /// Collection of all parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::std::option::Option<AllParams>,
}
///
/// Type for integer parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntParam {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Value of the parameter
    #[prost(int32, tag = "2")]
    pub value: i32,
}
///
/// Type for float paramters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatParam {
    /// Name of the parameter
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Value of the parameter
    #[prost(float, tag = "2")]
    pub value: f32,
}
///
/// Type collecting all integer and float parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllParams {
    /// Collection of all parameter names and values of type int
    #[prost(message, repeated, tag = "1")]
    pub int_params: ::std::vec::Vec<IntParam>,
    /// Collection of all parameter names and values of type float
    #[prost(message, repeated, tag = "2")]
    pub float_params: ::std::vec::Vec<FloatParam>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamResult {
    /// Result enum value
    #[prost(enumeration = "param_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod param_result {
    /// Possible results returned for param requests.
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
    }
}
#[doc = r" Generated client implementations."]
pub mod param_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provide raw access to get and set parameters."]
    pub struct ParamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ParamServiceClient<tonic::transport::Channel> {
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
    impl<T> ParamServiceClient<T>
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
        #[doc = " Get an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn get_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParamIntRequest>,
        ) -> Result<tonic::Response<super::GetParamIntResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.param.ParamService/GetParamInt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn set_param_int(
            &mut self,
            request: impl tonic::IntoRequest<super::SetParamIntRequest>,
        ) -> Result<tonic::Response<super::SetParamIntResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.param.ParamService/SetParamInt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn get_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParamFloatRequest>,
        ) -> Result<tonic::Response<super::GetParamFloatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = ""]
        #[doc = " Set a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        pub async fn set_param_float(
            &mut self,
            request: impl tonic::IntoRequest<super::SetParamFloatRequest>,
        ) -> Result<tonic::Response<super::SetParamFloatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = ""]
        #[doc = " Get all parameters."]
        pub async fn get_all_params(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllParamsRequest>,
        ) -> Result<tonic::Response<super::GetAllParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.param.ParamService/GetAllParams");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ParamServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ParamServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ParamServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod param_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ParamServiceServer."]
    #[async_trait]
    pub trait ParamService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Get an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn get_param_int(
            &self,
            request: tonic::Request<super::GetParamIntRequest>,
        ) -> Result<tonic::Response<super::GetParamIntResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set an int parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn set_param_int(
            &self,
            request: tonic::Request<super::SetParamIntRequest>,
        ) -> Result<tonic::Response<super::SetParamIntResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn get_param_float(
            &self,
            request: tonic::Request<super::GetParamFloatRequest>,
        ) -> Result<tonic::Response<super::GetParamFloatResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set a float parameter."]
        #[doc = ""]
        #[doc = " If the type is wrong, the result will be `WRONG_TYPE`."]
        async fn set_param_float(
            &self,
            request: tonic::Request<super::SetParamFloatRequest>,
        ) -> Result<tonic::Response<super::SetParamFloatResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get all parameters."]
        async fn get_all_params(
            &self,
            request: tonic::Request<super::GetAllParamsRequest>,
        ) -> Result<tonic::Response<super::GetAllParamsResponse>, tonic::Status>;
    }
    #[doc = " Provide raw access to get and set parameters."]
    #[derive(Debug)]
    pub struct ParamServiceServer<T: ParamService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ParamService> ParamServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ParamServiceServer<T>
    where
        T: ParamService,
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
                "/mavsdk.rpc.param.ParamService/GetParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct GetParamIntSvc<T: ParamService>(pub Arc<T>);
                    impl<T: ParamService> tonic::server::UnaryService<super::GetParamIntRequest> for GetParamIntSvc<T> {
                        type Response = super::GetParamIntResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_param_int(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetParamIntSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/SetParamInt" => {
                    #[allow(non_camel_case_types)]
                    struct SetParamIntSvc<T: ParamService>(pub Arc<T>);
                    impl<T: ParamService> tonic::server::UnaryService<super::SetParamIntRequest> for SetParamIntSvc<T> {
                        type Response = super::SetParamIntResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetParamIntRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_param_int(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetParamIntSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/GetParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct GetParamFloatSvc<T: ParamService>(pub Arc<T>);
                    impl<T: ParamService> tonic::server::UnaryService<super::GetParamFloatRequest>
                        for GetParamFloatSvc<T>
                    {
                        type Response = super::GetParamFloatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_param_float(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetParamFloatSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/SetParamFloat" => {
                    #[allow(non_camel_case_types)]
                    struct SetParamFloatSvc<T: ParamService>(pub Arc<T>);
                    impl<T: ParamService> tonic::server::UnaryService<super::SetParamFloatRequest>
                        for SetParamFloatSvc<T>
                    {
                        type Response = super::SetParamFloatResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetParamFloatRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_param_float(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetParamFloatSvc(inner);
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
                "/mavsdk.rpc.param.ParamService/GetAllParams" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllParamsSvc<T: ParamService>(pub Arc<T>);
                    impl<T: ParamService> tonic::server::UnaryService<super::GetAllParamsRequest>
                        for GetAllParamsSvc<T>
                    {
                        type Response = super::GetAllParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAllParamsSvc(inner);
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
    impl<T: ParamService> Clone for ParamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ParamService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ParamService> tonic::transport::NamedService for ParamServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.param.ParamService";
    }
}
