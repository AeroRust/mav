#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPitchAndYawRequest {
    /// Pitch angle in degrees (negative points down)
    #[prost(float, tag = "1")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees (positive is clock-wise, range: -180 to 180 or 0 to 360)
    #[prost(float, tag = "2")]
    pub yaw_deg: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPitchAndYawResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::core::option::Option<GimbalResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPitchRateAndYawRateRequest {
    /// Angular rate around pitch axis in degrees/second (negative downward)
    #[prost(float, tag = "1")]
    pub pitch_rate_deg_s: f32,
    /// Angular rate around yaw axis in degrees/second (positive is clock-wise)
    #[prost(float, tag = "2")]
    pub yaw_rate_deg_s: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPitchRateAndYawRateResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::core::option::Option<GimbalResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetModeRequest {
    /// The mode to be set.
    #[prost(enumeration = "GimbalMode", tag = "1")]
    pub gimbal_mode: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetModeResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::core::option::Option<GimbalResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetRoiLocationRequest {
    /// Latitude in degrees
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude in metres (AMSL)
    #[prost(float, tag = "3")]
    pub altitude_m: f32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetRoiLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::core::option::Option<GimbalResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TakeControlRequest {
    /// Control mode (primary or secondary)
    #[prost(enumeration = "ControlMode", tag = "1")]
    pub control_mode: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TakeControlResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::core::option::Option<GimbalResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ReleaseControlRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ReleaseControlResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::core::option::Option<GimbalResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeControlRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ControlResponse {
    /// Control status
    #[prost(message, optional, tag = "1")]
    pub control_status: ::core::option::Option<ControlStatus>,
}
/// Control status
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ControlStatus {
    /// Control mode (none, primary or secondary)
    #[prost(enumeration = "ControlMode", tag = "1")]
    pub control_mode: i32,
    /// Sysid of the component that has primary control over the gimbal (0 if no one is in control)
    #[prost(int32, tag = "2")]
    pub sysid_primary_control: i32,
    /// Compid of the component that has primary control over the gimbal (0 if no one is in control)
    #[prost(int32, tag = "3")]
    pub compid_primary_control: i32,
    /// Sysid of the component that has secondary control over the gimbal (0 if no one is in control)
    #[prost(int32, tag = "4")]
    pub sysid_secondary_control: i32,
    /// Compid of the component that has secondary control over the gimbal (0 if no one is in control)
    #[prost(int32, tag = "5")]
    pub compid_secondary_control: i32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalResult {
    /// Result enum value
    #[prost(enumeration = "gimbal_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GimbalResult`.
pub mod gimbal_result {
    /// Possible results returned for gimbal commands.
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
        /// Command was accepted
        Success = 1,
        /// Error occurred sending the command
        Error = 2,
        /// Command timed out
        Timeout = 3,
        /// Functionality not supported
        Unsupported = 4,
        /// No system connected
        NoSystem = 5,
    }
}
/// Gimbal mode type.
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
pub enum GimbalMode {
    /// Yaw follow will point the gimbal to the vehicle heading
    YawFollow = 0,
    /// Yaw lock will fix the gimbal poiting to an absolute direction
    YawLock = 1,
}
/// Control mode
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
pub enum ControlMode {
    /// Indicates that the component does not have control over the gimbal
    None = 0,
    /// To take primary control over the gimbal
    Primary = 1,
    /// To take secondary control over the gimbal
    Secondary = 2,
}
#[doc = r" Generated client implementations."]
pub mod gimbal_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provide control over a gimbal."]
    #[derive(Debug, Clone)]
    pub struct GimbalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GimbalServiceClient<tonic::transport::Channel> {
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
    impl<T> GimbalServiceClient<T>
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
        ) -> GimbalServiceClient<InterceptedService<T, F>>
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
            GimbalServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = ""]
        #[doc = " Set gimbal pitch and yaw angles."]
        #[doc = ""]
        #[doc = " This sets the desired pitch and yaw angles of a gimbal."]
        #[doc = " Will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually be set to the new angles."]
        pub async fn set_pitch_and_yaw(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPitchAndYawRequest>,
        ) -> Result<tonic::Response<super::SetPitchAndYawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/SetPitchAndYaw",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = ""]
        #[doc = " Set gimbal angular rates around pitch and yaw axes."]
        #[doc = ""]
        #[doc = " This sets the desired angular rates around pitch and yaw axes of a gimbal."]
        #[doc = " Will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually reach the angular rate."]
        pub async fn set_pitch_rate_and_yaw_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPitchRateAndYawRateRequest>,
        ) -> Result<tonic::Response<super::SetPitchRateAndYawRateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/SetPitchRateAndYawRate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set gimbal mode."]
        #[doc = ""]
        #[doc = " This sets the desired yaw mode of a gimbal."]
        #[doc = " Will return when the command is accepted. However, it might"]
        #[doc = " take the gimbal longer to actually be set to the new angles."]
        pub async fn set_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.gimbal.GimbalService/SetMode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set gimbal region of interest (ROI)."]
        #[doc = ""]
        #[doc = " This sets a region of interest that the gimbal will point to."]
        #[doc = " The gimbal will continue to point to the specified region until it"]
        #[doc = " receives a new command."]
        #[doc = " The function will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually rotate to the ROI."]
        pub async fn set_roi_location(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRoiLocationRequest>,
        ) -> Result<tonic::Response<super::SetRoiLocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/SetRoiLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Take control."]
        #[doc = ""]
        #[doc = " There can be only two components in control of a gimbal at any given time."]
        #[doc = " One with \"primary\" control, and one with \"secondary\" control. The way the"]
        #[doc = " secondary control is implemented is not specified and hence depends on the"]
        #[doc = " vehicle."]
        #[doc = ""]
        #[doc = " Components are expected to be cooperative, which means that they can"]
        #[doc = " override each other and should therefore do it carefully."]
        pub async fn take_control(
            &mut self,
            request: impl tonic::IntoRequest<super::TakeControlRequest>,
        ) -> Result<tonic::Response<super::TakeControlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/TakeControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Release control."]
        #[doc = ""]
        #[doc = " Release control, such that other components can control the gimbal."]
        pub async fn release_control(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseControlRequest>,
        ) -> Result<tonic::Response<super::ReleaseControlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/ReleaseControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Subscribe to control status updates."]
        #[doc = ""]
        #[doc = " This allows a component to know if it has primary, secondary or"]
        #[doc = " no control over the gimbal. Also, it gives the system and component ids"]
        #[doc = " of the other components in control (if any)."]
        pub async fn subscribe_control(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeControlRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ControlResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.gimbal.GimbalService/SubscribeControl",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gimbal_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GimbalServiceServer."]
    #[async_trait]
    pub trait GimbalService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = ""]
        #[doc = " Set gimbal pitch and yaw angles."]
        #[doc = ""]
        #[doc = " This sets the desired pitch and yaw angles of a gimbal."]
        #[doc = " Will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually be set to the new angles."]
        async fn set_pitch_and_yaw(
            &self,
            request: tonic::Request<super::SetPitchAndYawRequest>,
        ) -> Result<tonic::Response<super::SetPitchAndYawResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = ""]
        #[doc = " Set gimbal angular rates around pitch and yaw axes."]
        #[doc = ""]
        #[doc = " This sets the desired angular rates around pitch and yaw axes of a gimbal."]
        #[doc = " Will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually reach the angular rate."]
        async fn set_pitch_rate_and_yaw_rate(
            &self,
            request: tonic::Request<super::SetPitchRateAndYawRateRequest>,
        ) -> Result<tonic::Response<super::SetPitchRateAndYawRateResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set gimbal mode."]
        #[doc = ""]
        #[doc = " This sets the desired yaw mode of a gimbal."]
        #[doc = " Will return when the command is accepted. However, it might"]
        #[doc = " take the gimbal longer to actually be set to the new angles."]
        async fn set_mode(
            &self,
            request: tonic::Request<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set gimbal region of interest (ROI)."]
        #[doc = ""]
        #[doc = " This sets a region of interest that the gimbal will point to."]
        #[doc = " The gimbal will continue to point to the specified region until it"]
        #[doc = " receives a new command."]
        #[doc = " The function will return when the command is accepted, however, it might"]
        #[doc = " take the gimbal longer to actually rotate to the ROI."]
        async fn set_roi_location(
            &self,
            request: tonic::Request<super::SetRoiLocationRequest>,
        ) -> Result<tonic::Response<super::SetRoiLocationResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Take control."]
        #[doc = ""]
        #[doc = " There can be only two components in control of a gimbal at any given time."]
        #[doc = " One with \"primary\" control, and one with \"secondary\" control. The way the"]
        #[doc = " secondary control is implemented is not specified and hence depends on the"]
        #[doc = " vehicle."]
        #[doc = ""]
        #[doc = " Components are expected to be cooperative, which means that they can"]
        #[doc = " override each other and should therefore do it carefully."]
        async fn take_control(
            &self,
            request: tonic::Request<super::TakeControlRequest>,
        ) -> Result<tonic::Response<super::TakeControlResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Release control."]
        #[doc = ""]
        #[doc = " Release control, such that other components can control the gimbal."]
        async fn release_control(
            &self,
            request: tonic::Request<super::ReleaseControlRequest>,
        ) -> Result<tonic::Response<super::ReleaseControlResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeControl method."]
        type SubscribeControlStream: futures_core::Stream<Item = Result<super::ControlResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to control status updates."]
        #[doc = ""]
        #[doc = " This allows a component to know if it has primary, secondary or"]
        #[doc = " no control over the gimbal. Also, it gives the system and component ids"]
        #[doc = " of the other components in control (if any)."]
        async fn subscribe_control(
            &self,
            request: tonic::Request<super::SubscribeControlRequest>,
        ) -> Result<tonic::Response<Self::SubscribeControlStream>, tonic::Status>;
    }
    #[doc = " Provide control over a gimbal."]
    #[derive(Debug)]
    pub struct GimbalServiceServer<T: GimbalService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GimbalService> GimbalServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GimbalServiceServer<T>
    where
        T: GimbalService,
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
                "/mavsdk.rpc.gimbal.GimbalService/SetPitchAndYaw" => {
                    #[allow(non_camel_case_types)]
                    struct SetPitchAndYawSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService> tonic::server::UnaryService<super::SetPitchAndYawRequest>
                        for SetPitchAndYawSvc<T>
                    {
                        type Response = super::SetPitchAndYawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPitchAndYawRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_pitch_and_yaw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPitchAndYawSvc(inner);
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
                "/mavsdk.rpc.gimbal.GimbalService/SetPitchRateAndYawRate" => {
                    #[allow(non_camel_case_types)]
                    struct SetPitchRateAndYawRateSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService>
                        tonic::server::UnaryService<super::SetPitchRateAndYawRateRequest>
                        for SetPitchRateAndYawRateSvc<T>
                    {
                        type Response = super::SetPitchRateAndYawRateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPitchRateAndYawRateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_pitch_rate_and_yaw_rate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPitchRateAndYawRateSvc(inner);
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
                "/mavsdk.rpc.gimbal.GimbalService/SetMode" => {
                    #[allow(non_camel_case_types)]
                    struct SetModeSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService> tonic::server::UnaryService<super::SetModeRequest> for SetModeSvc<T> {
                        type Response = super::SetModeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetModeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_mode(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetModeSvc(inner);
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
                "/mavsdk.rpc.gimbal.GimbalService/SetRoiLocation" => {
                    #[allow(non_camel_case_types)]
                    struct SetRoiLocationSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService> tonic::server::UnaryService<super::SetRoiLocationRequest>
                        for SetRoiLocationSvc<T>
                    {
                        type Response = super::SetRoiLocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRoiLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_roi_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRoiLocationSvc(inner);
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
                "/mavsdk.rpc.gimbal.GimbalService/TakeControl" => {
                    #[allow(non_camel_case_types)]
                    struct TakeControlSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService> tonic::server::UnaryService<super::TakeControlRequest>
                        for TakeControlSvc<T>
                    {
                        type Response = super::TakeControlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TakeControlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).take_control(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TakeControlSvc(inner);
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
                "/mavsdk.rpc.gimbal.GimbalService/ReleaseControl" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseControlSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService> tonic::server::UnaryService<super::ReleaseControlRequest>
                        for ReleaseControlSvc<T>
                    {
                        type Response = super::ReleaseControlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReleaseControlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).release_control(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReleaseControlSvc(inner);
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
                "/mavsdk.rpc.gimbal.GimbalService/SubscribeControl" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeControlSvc<T: GimbalService>(pub Arc<T>);
                    impl<T: GimbalService>
                        tonic::server::ServerStreamingService<super::SubscribeControlRequest>
                        for SubscribeControlSvc<T>
                    {
                        type Response = super::ControlResponse;
                        type ResponseStream = T::SubscribeControlStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeControlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_control(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeControlSvc(inner);
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
    impl<T: GimbalService> Clone for GimbalServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GimbalService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GimbalService> tonic::transport::NamedService for GimbalServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.gimbal.GimbalService";
    }
}
