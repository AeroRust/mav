#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectRequest {
    /// The failure unit to send
    #[prost(enumeration="FailureUnit", tag="1")]
    pub failure_unit: i32,
    /// The failure type to send
    #[prost(enumeration="FailureType", tag="2")]
    pub failure_type: i32,
    /// Instance to affect (0 for all)
    #[prost(int32, tag="3")]
    pub instance: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectResponse {
    #[prost(message, optional, tag="1")]
    pub failure_result: ::core::option::Option<FailureResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailureResult {
    /// Result enum value
    #[prost(enumeration="failure_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FailureResult`.
pub mod failure_result {
    /// Possible results returned for failure requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unknown => "RESULT_UNKNOWN",
                Result::Success => "RESULT_SUCCESS",
                Result::NoSystem => "RESULT_NO_SYSTEM",
                Result::ConnectionError => "RESULT_CONNECTION_ERROR",
                Result::Unsupported => "RESULT_UNSUPPORTED",
                Result::Denied => "RESULT_DENIED",
                Result::Disabled => "RESULT_DISABLED",
                Result::Timeout => "RESULT_TIMEOUT",
            }
        }
    }
}
/// A failure unit.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl FailureUnit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FailureUnit::SensorGyro => "FAILURE_UNIT_SENSOR_GYRO",
            FailureUnit::SensorAccel => "FAILURE_UNIT_SENSOR_ACCEL",
            FailureUnit::SensorMag => "FAILURE_UNIT_SENSOR_MAG",
            FailureUnit::SensorBaro => "FAILURE_UNIT_SENSOR_BARO",
            FailureUnit::SensorGps => "FAILURE_UNIT_SENSOR_GPS",
            FailureUnit::SensorOpticalFlow => "FAILURE_UNIT_SENSOR_OPTICAL_FLOW",
            FailureUnit::SensorVio => "FAILURE_UNIT_SENSOR_VIO",
            FailureUnit::SensorDistanceSensor => "FAILURE_UNIT_SENSOR_DISTANCE_SENSOR",
            FailureUnit::SensorAirspeed => "FAILURE_UNIT_SENSOR_AIRSPEED",
            FailureUnit::SystemBattery => "FAILURE_UNIT_SYSTEM_BATTERY",
            FailureUnit::SystemMotor => "FAILURE_UNIT_SYSTEM_MOTOR",
            FailureUnit::SystemServo => "FAILURE_UNIT_SYSTEM_SERVO",
            FailureUnit::SystemAvoidance => "FAILURE_UNIT_SYSTEM_AVOIDANCE",
            FailureUnit::SystemRcSignal => "FAILURE_UNIT_SYSTEM_RC_SIGNAL",
            FailureUnit::SystemMavlinkSignal => "FAILURE_UNIT_SYSTEM_MAVLINK_SIGNAL",
        }
    }
}
/// A failure type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl FailureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FailureType::Ok => "FAILURE_TYPE_OK",
            FailureType::Off => "FAILURE_TYPE_OFF",
            FailureType::Stuck => "FAILURE_TYPE_STUCK",
            FailureType::Garbage => "FAILURE_TYPE_GARBAGE",
            FailureType::Wrong => "FAILURE_TYPE_WRONG",
            FailureType::Slow => "FAILURE_TYPE_SLOW",
            FailureType::Delayed => "FAILURE_TYPE_DELAYED",
            FailureType::Intermittent => "FAILURE_TYPE_INTERMITTENT",
        }
    }
}
/// Generated client implementations.
pub mod failure_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Inject failures into system to test failsafes.
    #[derive(Debug, Clone)]
    pub struct FailureServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FailureServiceClient<tonic::transport::Channel> {
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
    impl<T> FailureServiceClient<T>
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
        ) -> FailureServiceClient<InterceptedService<T, F>>
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
            FailureServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Injects a failure.
        pub async fn inject(
            &mut self,
            request: impl tonic::IntoRequest<super::InjectRequest>,
        ) -> Result<tonic::Response<super::InjectResponse>, tonic::Status> {
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
                "/mavsdk.rpc.failure.FailureService/Inject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod failure_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with FailureServiceServer.
    #[async_trait]
    pub trait FailureService: Send + Sync + 'static {
        /// Injects a failure.
        async fn inject(
            &self,
            request: tonic::Request<super::InjectRequest>,
        ) -> Result<tonic::Response<super::InjectResponse>, tonic::Status>;
    }
    /// Inject failures into system to test failsafes.
    #[derive(Debug)]
    pub struct FailureServiceServer<T: FailureService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FailureService> FailureServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FailureServiceServer<T>
    where
        T: FailureService,
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
                "/mavsdk.rpc.failure.FailureService/Inject" => {
                    #[allow(non_camel_case_types)]
                    struct InjectSvc<T: FailureService>(pub Arc<T>);
                    impl<
                        T: FailureService,
                    > tonic::server::UnaryService<super::InjectRequest>
                    for InjectSvc<T> {
                        type Response = super::InjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
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
    impl<T: FailureService> tonic::server::NamedService for FailureServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.failure.FailureService";
    }
}
