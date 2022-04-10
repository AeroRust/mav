/// Configuration type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Minimum height for the vehicle in meters (recommended minimum 8 meters)
    #[prost(float, tag = "1")]
    pub min_height_m: f32,
    /// Distance from target for vehicle to follow in meters (recommended minimum 1 meter)
    #[prost(float, tag = "2")]
    pub follow_distance_m: f32,
    /// Direction to follow in
    #[prost(enumeration = "config::FollowDirection", tag = "3")]
    pub follow_direction: i32,
    /// How responsive the vehicle is to the motion of the target (range 0.0 to 1.0)
    #[prost(float, tag = "4")]
    pub responsiveness: f32,
}
/// Nested message and enum types in `Config`.
pub mod config {
    /// Direction relative to the target that the vehicle should follow
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
    pub enum FollowDirection {
        /// Do not follow
        None = 0,
        /// Follow from behind
        Behind = 1,
        /// Follow from front
        Front = 2,
        /// Follow from front right
        FrontRight = 3,
        /// Follow from front left
        FrontLeft = 4,
    }
}
/// Target location for the vehicle to follow
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TargetLocation {
    /// Target latitude in degrees
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Target longitude in degrees
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Target altitude in meters above MSL
    #[prost(float, tag = "3")]
    pub absolute_altitude_m: f32,
    /// Target velocity in X axis, in meters per second
    #[prost(float, tag = "4")]
    pub velocity_x_m_s: f32,
    /// Target velocity in Y axis, in meters per second
    #[prost(float, tag = "5")]
    pub velocity_y_m_s: f32,
    /// Target velocity in Z axis, in meters per second
    #[prost(float, tag = "6")]
    pub velocity_z_m_s: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetConfigResponse {
    /// The current configuration
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<Config>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetConfigRequest {
    /// The new configuration to be set
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<Config>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetConfigResponse {
    #[prost(message, optional, tag = "1")]
    pub follow_me_result: ::core::option::Option<FollowMeResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsActiveRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsActiveResponse {
    /// Whether follow me is active or not
    #[prost(bool, tag = "1")]
    pub is_active: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetTargetLocationRequest {
    /// The new TargetLocation to follow
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<TargetLocation>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetTargetLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub follow_me_result: ::core::option::Option<FollowMeResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetLastLocationRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetLastLocationResponse {
    /// The last target location that was set
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<TargetLocation>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartResponse {
    #[prost(message, optional, tag = "1")]
    pub follow_me_result: ::core::option::Option<FollowMeResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
    #[prost(message, optional, tag = "1")]
    pub follow_me_result: ::core::option::Option<FollowMeResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FollowMeResult {
    /// Result enum value
    #[prost(enumeration = "follow_me_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FollowMeResult`.
pub mod follow_me_result {
    /// Possible results returned for followme operations
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
        /// No system connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command denied
        CommandDenied = 5,
        /// Request timed out
        Timeout = 6,
        /// FollowMe is not active
        NotActive = 7,
        /// Failed to set FollowMe configuration
        SetConfigFailed = 8,
    }
}
#[doc = r" Generated client implementations."]
pub mod follow_me_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Allow users to command the vehicle to follow a specific target."]
    #[doc = " The target is provided as a GPS coordinate and altitude."]
    #[derive(Debug, Clone)]
    pub struct FollowMeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FollowMeServiceClient<tonic::transport::Channel> {
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
    impl<T> FollowMeServiceClient<T>
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
        ) -> FollowMeServiceClient<InterceptedService<T, F>>
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
            FollowMeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get current configuration."]
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::GetConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.follow_me.FollowMeService/GetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Apply configuration by sending it to the system."]
        pub async fn set_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetConfigRequest>,
        ) -> Result<tonic::Response<super::SetConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.follow_me.FollowMeService/SetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Check if FollowMe is active."]
        pub async fn is_active(
            &mut self,
            request: impl tonic::IntoRequest<super::IsActiveRequest>,
        ) -> Result<tonic::Response<super::IsActiveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.follow_me.FollowMeService/IsActive",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Set location of the moving target."]
        pub async fn set_target_location(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTargetLocationRequest>,
        ) -> Result<tonic::Response<super::SetTargetLocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.follow_me.FollowMeService/SetTargetLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the last location of the target."]
        pub async fn get_last_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastLocationRequest>,
        ) -> Result<tonic::Response<super::GetLastLocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.follow_me.FollowMeService/GetLastLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Start FollowMe mode."]
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRequest>,
        ) -> Result<tonic::Response<super::StartResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.follow_me.FollowMeService/Start");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stop FollowMe mode."]
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.follow_me.FollowMeService/Stop");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod follow_me_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with FollowMeServiceServer."]
    #[async_trait]
    pub trait FollowMeService: Send + Sync + 'static {
        #[doc = " Get current configuration."]
        async fn get_config(
            &self,
            request: tonic::Request<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::GetConfigResponse>, tonic::Status>;
        #[doc = " Apply configuration by sending it to the system."]
        async fn set_config(
            &self,
            request: tonic::Request<super::SetConfigRequest>,
        ) -> Result<tonic::Response<super::SetConfigResponse>, tonic::Status>;
        #[doc = " Check if FollowMe is active."]
        async fn is_active(
            &self,
            request: tonic::Request<super::IsActiveRequest>,
        ) -> Result<tonic::Response<super::IsActiveResponse>, tonic::Status>;
        #[doc = " Set location of the moving target."]
        async fn set_target_location(
            &self,
            request: tonic::Request<super::SetTargetLocationRequest>,
        ) -> Result<tonic::Response<super::SetTargetLocationResponse>, tonic::Status>;
        #[doc = " Get the last location of the target."]
        async fn get_last_location(
            &self,
            request: tonic::Request<super::GetLastLocationRequest>,
        ) -> Result<tonic::Response<super::GetLastLocationResponse>, tonic::Status>;
        #[doc = " Start FollowMe mode."]
        async fn start(
            &self,
            request: tonic::Request<super::StartRequest>,
        ) -> Result<tonic::Response<super::StartResponse>, tonic::Status>;
        #[doc = " Stop FollowMe mode."]
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = " Allow users to command the vehicle to follow a specific target."]
    #[doc = " The target is provided as a GPS coordinate and altitude."]
    #[derive(Debug)]
    pub struct FollowMeServiceServer<T: FollowMeService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FollowMeService> FollowMeServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FollowMeServiceServer<T>
    where
        T: FollowMeService,
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
                "/mavsdk.rpc.follow_me.FollowMeService/GetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetConfigSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService> tonic::server::UnaryService<super::GetConfigRequest> for GetConfigSvc<T> {
                        type Response = super::GetConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConfigSvc(inner);
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
                "/mavsdk.rpc.follow_me.FollowMeService/SetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService> tonic::server::UnaryService<super::SetConfigRequest> for SetConfigSvc<T> {
                        type Response = super::SetConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetConfigSvc(inner);
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
                "/mavsdk.rpc.follow_me.FollowMeService/IsActive" => {
                    #[allow(non_camel_case_types)]
                    struct IsActiveSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService> tonic::server::UnaryService<super::IsActiveRequest> for IsActiveSvc<T> {
                        type Response = super::IsActiveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsActiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_active(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsActiveSvc(inner);
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
                "/mavsdk.rpc.follow_me.FollowMeService/SetTargetLocation" => {
                    #[allow(non_camel_case_types)]
                    struct SetTargetLocationSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService>
                        tonic::server::UnaryService<super::SetTargetLocationRequest>
                        for SetTargetLocationSvc<T>
                    {
                        type Response = super::SetTargetLocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetTargetLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_target_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetTargetLocationSvc(inner);
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
                "/mavsdk.rpc.follow_me.FollowMeService/GetLastLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetLastLocationSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService>
                        tonic::server::UnaryService<super::GetLastLocationRequest>
                        for GetLastLocationSvc<T>
                    {
                        type Response = super::GetLastLocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLastLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_last_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLastLocationSvc(inner);
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
                "/mavsdk.rpc.follow_me.FollowMeService/Start" => {
                    #[allow(non_camel_case_types)]
                    struct StartSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService> tonic::server::UnaryService<super::StartRequest> for StartSvc<T> {
                        type Response = super::StartResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartSvc(inner);
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
                "/mavsdk.rpc.follow_me.FollowMeService/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: FollowMeService>(pub Arc<T>);
                    impl<T: FollowMeService> tonic::server::UnaryService<super::StopRequest> for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
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
    impl<T: FollowMeService> Clone for FollowMeServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: FollowMeService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FollowMeService> tonic::transport::NamedService for FollowMeServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.follow_me.FollowMeService";
    }
}
