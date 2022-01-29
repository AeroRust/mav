#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetFlightInformationRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetFlightInformationResponse {
    #[prost(message, optional, tag = "1")]
    pub info_result: ::core::option::Option<InfoResult>,
    /// Flight information of the system
    #[prost(message, optional, tag = "2")]
    pub flight_info: ::core::option::Option<FlightInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetIdentificationRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetIdentificationResponse {
    #[prost(message, optional, tag = "1")]
    pub info_result: ::core::option::Option<InfoResult>,
    /// Identification of the system
    #[prost(message, optional, tag = "2")]
    pub identification: ::core::option::Option<Identification>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetProductResponse {
    #[prost(message, optional, tag = "1")]
    pub info_result: ::core::option::Option<InfoResult>,
    /// Product information of the system
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<Product>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub info_result: ::core::option::Option<InfoResult>,
    /// Version information about the system
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetSpeedFactorRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetSpeedFactorResponse {
    #[prost(message, optional, tag = "1")]
    pub info_result: ::core::option::Option<InfoResult>,
    /// Speed factor of simulation
    #[prost(double, tag = "2")]
    pub speed_factor: f64,
}
/// System flight information.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlightInfo {
    /// Time since system boot
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Flight counter. Starts from zero, is incremented at every disarm and is never reset (even after reboot)
    #[prost(uint64, tag = "2")]
    pub flight_uid: u64,
}
/// System identification.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Identification {
    /// UID of the hardware. This refers to uid2 of MAVLink. If the system does not support uid2 yet, this is all zeros.
    #[prost(string, tag = "1")]
    pub hardware_uid: ::prost::alloc::string::String,
}
/// System product information.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// ID of the board vendor
    #[prost(int32, tag = "1")]
    pub vendor_id: i32,
    /// Name of the vendor
    #[prost(string, tag = "2")]
    pub vendor_name: ::prost::alloc::string::String,
    /// ID of the product
    #[prost(int32, tag = "3")]
    pub product_id: i32,
    /// Name of the product
    #[prost(string, tag = "4")]
    pub product_name: ::prost::alloc::string::String,
}
/// System version information.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Flight software major version
    #[prost(int32, tag = "1")]
    pub flight_sw_major: i32,
    /// Flight software minor version
    #[prost(int32, tag = "2")]
    pub flight_sw_minor: i32,
    /// Flight software patch version
    #[prost(int32, tag = "3")]
    pub flight_sw_patch: i32,
    /// Flight software vendor major version
    #[prost(int32, tag = "4")]
    pub flight_sw_vendor_major: i32,
    /// Flight software vendor minor version
    #[prost(int32, tag = "5")]
    pub flight_sw_vendor_minor: i32,
    /// Flight software vendor patch version
    #[prost(int32, tag = "6")]
    pub flight_sw_vendor_patch: i32,
    /// Operating system software major version
    #[prost(int32, tag = "7")]
    pub os_sw_major: i32,
    /// Operating system software minor version
    #[prost(int32, tag = "8")]
    pub os_sw_minor: i32,
    /// Operating system software patch version
    #[prost(int32, tag = "9")]
    pub os_sw_patch: i32,
    /// Flight software git hash
    #[prost(string, tag = "10")]
    pub flight_sw_git_hash: ::prost::alloc::string::String,
    /// Operating system software git hash
    #[prost(string, tag = "11")]
    pub os_sw_git_hash: ::prost::alloc::string::String,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InfoResult {
    /// Result enum value
    #[prost(enumeration = "info_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InfoResult`.
pub mod info_result {
    /// Possible results returned for info requests.
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
        /// Information has not been received yet
        InformationNotReceivedYet = 2,
    }
}
#[doc = r" Generated client implementations."]
pub mod info_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provide information about the hardware and/or software of a system."]
    #[derive(Debug, Clone)]
    pub struct InfoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InfoServiceClient<tonic::transport::Channel> {
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
    impl<T> InfoServiceClient<T>
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
        ) -> InfoServiceClient<InterceptedService<T, F>>
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
            InfoServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get flight information of the system."]
        pub async fn get_flight_information(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFlightInformationRequest>,
        ) -> Result<tonic::Response<super::GetFlightInformationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.info.InfoService/GetFlightInformation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the identification of the system."]
        pub async fn get_identification(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdentificationRequest>,
        ) -> Result<tonic::Response<super::GetIdentificationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.info.InfoService/GetIdentification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get product information of the system."]
        pub async fn get_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::GetProductResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.info.InfoService/GetProduct");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the version information of the system."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::GetVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.info.InfoService/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the speed factor of a simulation (with lockstep a simulation can run faster or slower than realtime)."]
        pub async fn get_speed_factor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpeedFactorRequest>,
        ) -> Result<tonic::Response<super::GetSpeedFactorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.info.InfoService/GetSpeedFactor");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod info_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InfoServiceServer."]
    #[async_trait]
    pub trait InfoService: Send + Sync + 'static {
        #[doc = " Get flight information of the system."]
        async fn get_flight_information(
            &self,
            request: tonic::Request<super::GetFlightInformationRequest>,
        ) -> Result<tonic::Response<super::GetFlightInformationResponse>, tonic::Status>;
        #[doc = " Get the identification of the system."]
        async fn get_identification(
            &self,
            request: tonic::Request<super::GetIdentificationRequest>,
        ) -> Result<tonic::Response<super::GetIdentificationResponse>, tonic::Status>;
        #[doc = " Get product information of the system."]
        async fn get_product(
            &self,
            request: tonic::Request<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::GetProductResponse>, tonic::Status>;
        #[doc = " Get the version information of the system."]
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::GetVersionResponse>, tonic::Status>;
        #[doc = " Get the speed factor of a simulation (with lockstep a simulation can run faster or slower than realtime)."]
        async fn get_speed_factor(
            &self,
            request: tonic::Request<super::GetSpeedFactorRequest>,
        ) -> Result<tonic::Response<super::GetSpeedFactorResponse>, tonic::Status>;
    }
    #[doc = " Provide information about the hardware and/or software of a system."]
    #[derive(Debug)]
    pub struct InfoServiceServer<T: InfoService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InfoService> InfoServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InfoServiceServer<T>
    where
        T: InfoService,
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
                "/mavsdk.rpc.info.InfoService/GetFlightInformation" => {
                    #[allow(non_camel_case_types)]
                    struct GetFlightInformationSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService>
                        tonic::server::UnaryService<super::GetFlightInformationRequest>
                        for GetFlightInformationSvc<T>
                    {
                        type Response = super::GetFlightInformationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFlightInformationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_flight_information(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFlightInformationSvc(inner);
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
                "/mavsdk.rpc.info.InfoService/GetIdentification" => {
                    #[allow(non_camel_case_types)]
                    struct GetIdentificationSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService>
                        tonic::server::UnaryService<super::GetIdentificationRequest>
                        for GetIdentificationSvc<T>
                    {
                        type Response = super::GetIdentificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIdentificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_identification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetIdentificationSvc(inner);
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
                "/mavsdk.rpc.info.InfoService/GetProduct" => {
                    #[allow(non_camel_case_types)]
                    struct GetProductSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService> tonic::server::UnaryService<super::GetProductRequest> for GetProductSvc<T> {
                        type Response = super::GetProductResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_product(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProductSvc(inner);
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
                "/mavsdk.rpc.info.InfoService/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService> tonic::server::UnaryService<super::GetVersionRequest> for GetVersionSvc<T> {
                        type Response = super::GetVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
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
                "/mavsdk.rpc.info.InfoService/GetSpeedFactor" => {
                    #[allow(non_camel_case_types)]
                    struct GetSpeedFactorSvc<T: InfoService>(pub Arc<T>);
                    impl<T: InfoService> tonic::server::UnaryService<super::GetSpeedFactorRequest>
                        for GetSpeedFactorSvc<T>
                    {
                        type Response = super::GetSpeedFactorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSpeedFactorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_speed_factor(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSpeedFactorSvc(inner);
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
    impl<T: InfoService> Clone for InfoServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InfoService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InfoService> tonic::transport::NamedService for InfoServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.info.InfoService";
    }
}
