#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InjectRequest {
    /// The failure unit to send
    #[prost(enumeration = "FailureUnit", tag = "1")]
    pub failure_unit: i32,
    /// The failure type to send
    #[prost(enumeration = "FailureType", tag = "2")]
    pub failure_type: i32,
    /// Instance to affect (0 for all)
    #[prost(int32, tag = "3")]
    pub instance: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct InjectResponse {
    #[prost(message, optional, tag = "1")]
    pub failure_result: ::core::option::Option<FailureResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FailureResult {
    /// Result enum value
    #[prost(enumeration = "failure_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FailureResult`.
pub mod failure_result {
    /// Possible results returned for failure requests.
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
        /// Failure not supported
        Unsupported = 4,
        /// Failure injection denied
        Denied = 5,
        /// Failure injection is disabled
        Disabled = 6,
        /// Request timed out
        Timeout = 7,
    }
}
/// A failure unit.
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
pub enum FailureUnit {
    /// Gyro
    SensorGyro = 0,
    /// Accelerometer
    SensorAccel = 1,
    /// Magnetometer
    SensorMag = 2,
    /// Barometer
    SensorBaro = 3,
    /// GPS
    SensorGps = 4,
    /// Optical flow
    SensorOpticalFlow = 5,
    /// Visual inertial odometry
    SensorVio = 6,
    /// Distance sensor
    SensorDistanceSensor = 7,
    /// Airspeed
    SensorAirspeed = 8,
    /// Battery
    SystemBattery = 100,
    /// Motor
    SystemMotor = 101,
    /// Servo
    SystemServo = 102,
    /// Avoidance
    SystemAvoidance = 103,
    /// RC signal
    SystemRcSignal = 104,
    /// MAVLink signal
    SystemMavlinkSignal = 105,
}
/// A failure type
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
pub enum FailureType {
    /// No failure injected, used to reset a previous failure
    Ok = 0,
    /// Sets unit off, so completely non-responsive
    Off = 1,
    /// Unit is stuck e.g. keeps reporting the same value
    Stuck = 2,
    /// Unit is reporting complete garbage
    Garbage = 3,
    /// Unit is consistently wrong
    Wrong = 4,
    /// Unit is slow, so e.g. reporting at slower than expected rate
    Slow = 5,
    /// Data of unit is delayed in time
    Delayed = 6,
    /// Unit is sometimes working, sometimes not
    Intermittent = 7,
}
#[doc = r" Generated client implementations."]
pub mod failure_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Inject failures into system to test failsafes."]
    #[derive(Debug, Clone)]
    pub struct FailureServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FailureServiceClient<tonic::transport::Channel> {
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
    impl<T> FailureServiceClient<T>
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
        ) -> FailureServiceClient<InterceptedService<T, F>>
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
            FailureServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Injects a failure."]
        pub async fn inject(
            &mut self,
            request: impl tonic::IntoRequest<super::InjectRequest>,
        ) -> Result<tonic::Response<super::InjectResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.failure.FailureService/Inject");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod failure_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with FailureServiceServer."]
    #[async_trait]
    pub trait FailureService: Send + Sync + 'static {
        #[doc = " Injects a failure."]
        async fn inject(
            &self,
            request: tonic::Request<super::InjectRequest>,
        ) -> Result<tonic::Response<super::InjectResponse>, tonic::Status>;
    }
    #[doc = " Inject failures into system to test failsafes."]
    #[derive(Debug)]
    pub struct FailureServiceServer<T: FailureService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FailureService> FailureServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FailureServiceServer<T>
    where
        T: FailureService,
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
                "/mavsdk.rpc.failure.FailureService/Inject" => {
                    #[allow(non_camel_case_types)]
                    struct InjectSvc<T: FailureService>(pub Arc<T>);
                    impl<T: FailureService> tonic::server::UnaryService<super::InjectRequest> for InjectSvc<T> {
                        type Response = super::InjectResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).inject(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InjectSvc(inner);
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
    impl<T: FailureService> Clone for FailureServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: FailureService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FailureService> tonic::transport::NamedService for FailureServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.failure.FailureService";
    }
}
