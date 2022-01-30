#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateGyroRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CalibrateGyroResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateAccelerometerRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CalibrateAccelerometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateMagnetometerRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CalibrateMagnetometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateLevelHorizonRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CalibrateLevelHorizonResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateGimbalAccelerometerRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CalibrateGimbalAccelerometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CalibrationResult {
    /// Result enum value
    #[prost(enumeration = "calibration_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CalibrationResult`.
pub mod calibration_result {
    /// Possible results returned for calibration commands
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
        /// The calibration succeeded
        Success = 1,
        /// Intermediate message showing progress or instructions on the next steps
        Next = 2,
        /// Calibration failed
        Failed = 3,
        /// No system is connected
        NoSystem = 4,
        /// Connection error
        ConnectionError = 5,
        /// Vehicle is busy
        Busy = 6,
        /// Command refused by vehicle
        CommandDenied = 7,
        /// Command timed out
        Timeout = 8,
        /// Calibration process was cancelled
        Cancelled = 9,
        /// Calibration process failed since the vehicle is armed
        FailedArmed = 10,
    }
}
///
/// Progress data coming from calibration.
///
/// Can be a progress percentage, or an instruction text.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProgressData {
    /// Whether this ProgressData contains a 'progress' status or not
    #[prost(bool, tag = "1")]
    pub has_progress: bool,
    /// Progress (percentage)
    #[prost(float, tag = "2")]
    pub progress: f32,
    /// Whether this ProgressData contains a 'status_text' or not
    #[prost(bool, tag = "3")]
    pub has_status_text: bool,
    /// Instruction text
    #[prost(string, tag = "4")]
    pub status_text: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod calibration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable to calibrate sensors of a drone such as gyro, accelerometer, and magnetometer."]
    #[derive(Debug, Clone)]
    pub struct CalibrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CalibrationServiceClient<tonic::transport::Channel> {
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
    impl<T> CalibrationServiceClient<T>
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
        ) -> CalibrationServiceClient<InterceptedService<T, F>>
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
            CalibrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Perform gyro calibration."]
        pub async fn subscribe_calibrate_gyro(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateGyroRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateGyroResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGyro",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform accelerometer calibration."]
        pub async fn subscribe_calibrate_accelerometer(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateAccelerometerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateAccelerometerResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateAccelerometer",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform magnetometer calibration."]
        pub async fn subscribe_calibrate_magnetometer(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateMagnetometerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateMagnetometerResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateMagnetometer",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform board level horizon calibration."]
        pub async fn subscribe_calibrate_level_horizon(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateLevelHorizonRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateLevelHorizonResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateLevelHorizon",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Perform gimbal accelerometer calibration."]
        pub async fn subscribe_calibrate_gimbal_accelerometer(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateGimbalAccelerometerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateGimbalAccelerometerResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGimbalAccelerometer",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Cancel ongoing calibration process."]
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/Cancel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod calibration_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CalibrationServiceServer."]
    #[async_trait]
    pub trait CalibrationService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeCalibrateGyro method."]
        type SubscribeCalibrateGyroStream: futures_core::Stream<Item = Result<super::CalibrateGyroResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Perform gyro calibration."]
        async fn subscribe_calibrate_gyro(
            &self,
            request: tonic::Request<super::SubscribeCalibrateGyroRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCalibrateGyroStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCalibrateAccelerometer method."]
        type SubscribeCalibrateAccelerometerStream: futures_core::Stream<
                Item = Result<super::CalibrateAccelerometerResponse, tonic::Status>,
            > + Send
            + 'static;
        #[doc = " Perform accelerometer calibration."]
        async fn subscribe_calibrate_accelerometer(
            &self,
            request: tonic::Request<super::SubscribeCalibrateAccelerometerRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCalibrateAccelerometerStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCalibrateMagnetometer method."]
        type SubscribeCalibrateMagnetometerStream: futures_core::Stream<Item = Result<super::CalibrateMagnetometerResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Perform magnetometer calibration."]
        async fn subscribe_calibrate_magnetometer(
            &self,
            request: tonic::Request<super::SubscribeCalibrateMagnetometerRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCalibrateMagnetometerStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCalibrateLevelHorizon method."]
        type SubscribeCalibrateLevelHorizonStream: futures_core::Stream<Item = Result<super::CalibrateLevelHorizonResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Perform board level horizon calibration."]
        async fn subscribe_calibrate_level_horizon(
            &self,
            request: tonic::Request<super::SubscribeCalibrateLevelHorizonRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCalibrateLevelHorizonStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCalibrateGimbalAccelerometer method."]
        type SubscribeCalibrateGimbalAccelerometerStream: futures_core::Stream<
                Item = Result<super::CalibrateGimbalAccelerometerResponse, tonic::Status>,
            > + Send
            + 'static;
        #[doc = " Perform gimbal accelerometer calibration."]
        async fn subscribe_calibrate_gimbal_accelerometer(
            &self,
            request: tonic::Request<super::SubscribeCalibrateGimbalAccelerometerRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCalibrateGimbalAccelerometerStream>, tonic::Status>;
        #[doc = " Cancel ongoing calibration process."]
        async fn cancel(
            &self,
            request: tonic::Request<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status>;
    }
    #[doc = " Enable to calibrate sensors of a drone such as gyro, accelerometer, and magnetometer."]
    #[derive(Debug)]
    pub struct CalibrationServiceServer<T: CalibrationService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CalibrationService> CalibrationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CalibrationServiceServer<T>
    where
        T: CalibrationService,
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
            match req . uri () . path () { "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGyro" => { # [allow (non_camel_case_types)] struct SubscribeCalibrateGyroSvc < T : CalibrationService > (pub Arc < T >) ; impl < T : CalibrationService > tonic :: server :: ServerStreamingService < super :: SubscribeCalibrateGyroRequest > for SubscribeCalibrateGyroSvc < T > { type Response = super :: CalibrateGyroResponse ; type ResponseStream = T :: SubscribeCalibrateGyroStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SubscribeCalibrateGyroRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . subscribe_calibrate_gyro (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SubscribeCalibrateGyroSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . server_streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateAccelerometer" => { # [allow (non_camel_case_types)] struct SubscribeCalibrateAccelerometerSvc < T : CalibrationService > (pub Arc < T >) ; impl < T : CalibrationService > tonic :: server :: ServerStreamingService < super :: SubscribeCalibrateAccelerometerRequest > for SubscribeCalibrateAccelerometerSvc < T > { type Response = super :: CalibrateAccelerometerResponse ; type ResponseStream = T :: SubscribeCalibrateAccelerometerStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SubscribeCalibrateAccelerometerRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . subscribe_calibrate_accelerometer (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SubscribeCalibrateAccelerometerSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . server_streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateMagnetometer" => { # [allow (non_camel_case_types)] struct SubscribeCalibrateMagnetometerSvc < T : CalibrationService > (pub Arc < T >) ; impl < T : CalibrationService > tonic :: server :: ServerStreamingService < super :: SubscribeCalibrateMagnetometerRequest > for SubscribeCalibrateMagnetometerSvc < T > { type Response = super :: CalibrateMagnetometerResponse ; type ResponseStream = T :: SubscribeCalibrateMagnetometerStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SubscribeCalibrateMagnetometerRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . subscribe_calibrate_magnetometer (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SubscribeCalibrateMagnetometerSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . server_streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateLevelHorizon" => { # [allow (non_camel_case_types)] struct SubscribeCalibrateLevelHorizonSvc < T : CalibrationService > (pub Arc < T >) ; impl < T : CalibrationService > tonic :: server :: ServerStreamingService < super :: SubscribeCalibrateLevelHorizonRequest > for SubscribeCalibrateLevelHorizonSvc < T > { type Response = super :: CalibrateLevelHorizonResponse ; type ResponseStream = T :: SubscribeCalibrateLevelHorizonStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SubscribeCalibrateLevelHorizonRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . subscribe_calibrate_level_horizon (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SubscribeCalibrateLevelHorizonSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . server_streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGimbalAccelerometer" => { # [allow (non_camel_case_types)] struct SubscribeCalibrateGimbalAccelerometerSvc < T : CalibrationService > (pub Arc < T >) ; impl < T : CalibrationService > tonic :: server :: ServerStreamingService < super :: SubscribeCalibrateGimbalAccelerometerRequest > for SubscribeCalibrateGimbalAccelerometerSvc < T > { type Response = super :: CalibrateGimbalAccelerometerResponse ; type ResponseStream = T :: SubscribeCalibrateGimbalAccelerometerStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SubscribeCalibrateGimbalAccelerometerRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . subscribe_calibrate_gimbal_accelerometer (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SubscribeCalibrateGimbalAccelerometerSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . server_streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/mavsdk.rpc.calibration.CalibrationService/Cancel" => { # [allow (non_camel_case_types)] struct CancelSvc < T : CalibrationService > (pub Arc < T >) ; impl < T : CalibrationService > tonic :: server :: UnaryService < super :: CancelRequest > for CancelSvc < T > { type Response = super :: CancelResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: CancelRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . cancel (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = CancelSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (empty_body ()) . unwrap ()) }) , }
        }
    }
    impl<T: CalibrationService> Clone for CalibrationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CalibrationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CalibrationService> tonic::transport::NamedService for CalibrationServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.calibration.CalibrationService";
    }
}
