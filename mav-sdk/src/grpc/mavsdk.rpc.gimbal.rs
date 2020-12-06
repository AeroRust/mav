#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPitchAndYawRequest {
    /// Pitch angle in degrees (negative points down)
    #[prost(float, tag = "1")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees (positive is clock-wise, range: -180 to 180 or 0 to 360)
    #[prost(float, tag = "2")]
    pub yaw_deg: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPitchAndYawResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::std::option::Option<GimbalResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeRequest {
    /// The mode to be set.
    #[prost(enumeration = "GimbalMode", tag = "1")]
    pub gimbal_mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::std::option::Option<GimbalResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoiLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub gimbal_result: ::std::option::Option<GimbalResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GimbalResult {
    /// Result enum value
    #[prost(enumeration = "gimbal_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod gimbal_result {
    /// Possible results returned for gimbal commands.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    }
}
/// Gimbal mode type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GimbalMode {
    /// Yaw follow will point the gimbal to the vehicle heading
    YawFollow = 0,
    /// Yaw lock will fix the gimbal poiting to an absolute direction
    YawLock = 1,
}
#[doc = r" Generated client implementations."]
pub mod gimbal_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provide control over a gimbal."]
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
    }
    impl<T: Clone> Clone for GimbalServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GimbalServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GimbalServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gimbal_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
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
    }
    #[doc = " Provide control over a gimbal."]
    #[derive(Debug)]
    pub struct GimbalServiceServer<T: GimbalService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GimbalService> GimbalServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GimbalServiceServer<T>
    where
        T: GimbalService,
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetPitchAndYawSvc(inner);
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetModeSvc(inner);
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetRoiLocationSvc(inner);
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
    impl<T: GimbalService> Clone for GimbalServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GimbalService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
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
