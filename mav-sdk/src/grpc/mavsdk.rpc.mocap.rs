#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVisionPositionEstimateRequest {
    /// The vision position estimate
    #[prost(message, optional, tag="1")]
    pub vision_position_estimate: ::core::option::Option<VisionPositionEstimate>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVisionPositionEstimateResponse {
    #[prost(message, optional, tag="1")]
    pub mocap_result: ::core::option::Option<MocapResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudePositionMocapRequest {
    /// The attitude and position data
    #[prost(message, optional, tag="1")]
    pub attitude_position_mocap: ::core::option::Option<AttitudePositionMocap>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudePositionMocapResponse {
    #[prost(message, optional, tag="1")]
    pub mocap_result: ::core::option::Option<MocapResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOdometryRequest {
    /// The odometry data
    #[prost(message, optional, tag="1")]
    pub odometry: ::core::option::Option<Odometry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOdometryResponse {
    #[prost(message, optional, tag="1")]
    pub mocap_result: ::core::option::Option<MocapResult>,
}
/// Body position type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionBody {
    /// X position in metres.
    #[prost(float, tag="1")]
    pub x_m: f32,
    /// Y position in metres.
    #[prost(float, tag="2")]
    pub y_m: f32,
    /// Z position in metres.
    #[prost(float, tag="3")]
    pub z_m: f32,
}
/// Body angle type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngleBody {
    /// Roll angle in radians.
    #[prost(float, tag="1")]
    pub roll_rad: f32,
    /// Pitch angle in radians.
    #[prost(float, tag="2")]
    pub pitch_rad: f32,
    /// Yaw angle in radians.
    #[prost(float, tag="3")]
    pub yaw_rad: f32,
}
/// Speed type, represented in the Body (X Y Z) frame and in metres/second.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeedBody {
    /// Velocity in X in metres/second.
    #[prost(float, tag="1")]
    pub x_m_s: f32,
    /// Velocity in Y in metres/second.
    #[prost(float, tag="2")]
    pub y_m_s: f32,
    /// Velocity in Z in metres/second.
    #[prost(float, tag="3")]
    pub z_m_s: f32,
}
/// Angular velocity type
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocityBody {
    /// Roll angular velocity in radians/second.
    #[prost(float, tag="1")]
    pub roll_rad_s: f32,
    /// Pitch angular velocity in radians/second.
    #[prost(float, tag="2")]
    pub pitch_rad_s: f32,
    /// Yaw angular velocity in radians/second.
    #[prost(float, tag="3")]
    pub yaw_rad_s: f32,
}
/// Covariance type.
/// Row-major representation of a 6x6 cross-covariance matrix upper
/// right triangle.
/// Needs to be 21 entries or 1 entry with NaN if unknown.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Covariance {
    /// The covariance matrix
    #[prost(float, repeated, tag="1")]
    pub covariance_matrix: ::prost::alloc::vec::Vec<f32>,
}
///
/// Quaternion type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Hamilton quaternion product definition is used.
/// A zero-rotation quaternion is represented by (1,0,0,0).
/// The quaternion could also be written as w + xi + yj + zk.
///
/// For more info see: <https://en.wikipedia.org/wiki/Quaternion>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// Quaternion entry 0, also denoted as a
    #[prost(float, tag="1")]
    pub w: f32,
    /// Quaternion entry 1, also denoted as b
    #[prost(float, tag="2")]
    pub x: f32,
    /// Quaternion entry 2, also denoted as c
    #[prost(float, tag="3")]
    pub y: f32,
    /// Quaternion entry 3, also denoted as d
    #[prost(float, tag="4")]
    pub z: f32,
}
/// Global position/attitude estimate from a vision source.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisionPositionEstimate {
    /// PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)
    #[prost(uint64, tag="1")]
    pub time_usec: u64,
    /// Global position (m)
    #[prost(message, optional, tag="2")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Body angle (rad).
    #[prost(message, optional, tag="3")]
    pub angle_body: ::core::option::Option<AngleBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag="4")]
    pub pose_covariance: ::core::option::Option<Covariance>,
}
/// Motion capture attitude and position
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttitudePositionMocap {
    /// PositionBody frame timestamp UNIX Epoch time (0 to use Backend timestamp)
    #[prost(uint64, tag="1")]
    pub time_usec: u64,
    /// Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)
    #[prost(message, optional, tag="2")]
    pub q: ::core::option::Option<Quaternion>,
    /// Body Position (NED)
    #[prost(message, optional, tag="3")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag="4")]
    pub pose_covariance: ::core::option::Option<Covariance>,
}
/// Odometry message to communicate odometry information with an external interface.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// Timestamp (0 to use Backend timestamp).
    #[prost(uint64, tag="1")]
    pub time_usec: u64,
    /// Coordinate frame of reference for the pose data.
    #[prost(enumeration="odometry::MavFrame", tag="2")]
    pub frame_id: i32,
    /// Body Position.
    #[prost(message, optional, tag="3")]
    pub position_body: ::core::option::Option<PositionBody>,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation).
    #[prost(message, optional, tag="4")]
    pub q: ::core::option::Option<Quaternion>,
    /// Linear speed (m/s).
    #[prost(message, optional, tag="5")]
    pub speed_body: ::core::option::Option<SpeedBody>,
    /// Angular speed (rad/s).
    #[prost(message, optional, tag="6")]
    pub angular_velocity_body: ::core::option::Option<AngularVelocityBody>,
    /// Pose cross-covariance matrix.
    #[prost(message, optional, tag="7")]
    pub pose_covariance: ::core::option::Option<Covariance>,
    /// Velocity cross-covariance matrix.
    #[prost(message, optional, tag="8")]
    pub velocity_covariance: ::core::option::Option<Covariance>,
}
/// Nested message and enum types in `Odometry`.
pub mod odometry {
    /// Mavlink frame id
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MavFrame {
        /// MAVLink number: 14. Odometry local coordinate frame of data given by a motion capture system, Z-down (x: north, y: east, z: down).
        MocapNed = 0,
        /// MAVLink number: 20. Forward, Right, Down coordinate frame. This is a local frame with Z-down and arbitrary F/R alignment (i.e. not aligned with NED/earth frame). Replacement for MAV_FRAME_MOCAP_NED, MAV_FRAME_VISION_NED, MAV_FRAME_ESTIM_NED.
        LocalFrd = 1,
    }
    impl MavFrame {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MavFrame::MocapNed => "MAV_FRAME_MOCAP_NED",
                MavFrame::LocalFrd => "MAV_FRAME_LOCAL_FRD",
            }
        }
    }
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MocapResult {
    /// Result enum value
    #[prost(enumeration="mocap_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MocapResult`.
pub mod mocap_result {
    /// Possible results returned for mocap requests
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown error
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// No system is connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Invalid request data
        InvalidRequestData = 4,
        /// Function unsupported
        Unsupported = 5,
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
                Result::InvalidRequestData => "RESULT_INVALID_REQUEST_DATA",
                Result::Unsupported => "RESULT_UNSUPPORTED",
            }
        }
    }
}
/// Generated client implementations.
pub mod mocap_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///*
    /// Allows interfacing a vehicle with a motion capture system in
    /// order to allow navigation without global positioning sources available
    /// (e.g. indoors, or when flying under a bridge. etc.).
    #[derive(Debug, Clone)]
    pub struct MocapServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MocapServiceClient<tonic::transport::Channel> {
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
    impl<T> MocapServiceClient<T>
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
        ) -> MocapServiceClient<InterceptedService<T, F>>
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
            MocapServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Send Global position/attitude estimate from a vision source.
        pub async fn set_vision_position_estimate(
            &mut self,
            request: impl tonic::IntoRequest<super::SetVisionPositionEstimateRequest>,
        ) -> Result<
            tonic::Response<super::SetVisionPositionEstimateResponse>,
            tonic::Status,
        > {
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
                "/mavsdk.rpc.mocap.MocapService/SetVisionPositionEstimate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send motion capture attitude and position.
        pub async fn set_attitude_position_mocap(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAttitudePositionMocapRequest>,
        ) -> Result<
            tonic::Response<super::SetAttitudePositionMocapResponse>,
            tonic::Status,
        > {
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
                "/mavsdk.rpc.mocap.MocapService/SetAttitudePositionMocap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send odometry information with an external interface.
        pub async fn set_odometry(
            &mut self,
            request: impl tonic::IntoRequest<super::SetOdometryRequest>,
        ) -> Result<tonic::Response<super::SetOdometryResponse>, tonic::Status> {
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
                "/mavsdk.rpc.mocap.MocapService/SetOdometry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod mocap_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MocapServiceServer.
    #[async_trait]
    pub trait MocapService: Send + Sync + 'static {
        /// Send Global position/attitude estimate from a vision source.
        async fn set_vision_position_estimate(
            &self,
            request: tonic::Request<super::SetVisionPositionEstimateRequest>,
        ) -> Result<
            tonic::Response<super::SetVisionPositionEstimateResponse>,
            tonic::Status,
        >;
        /// Send motion capture attitude and position.
        async fn set_attitude_position_mocap(
            &self,
            request: tonic::Request<super::SetAttitudePositionMocapRequest>,
        ) -> Result<
            tonic::Response<super::SetAttitudePositionMocapResponse>,
            tonic::Status,
        >;
        /// Send odometry information with an external interface.
        async fn set_odometry(
            &self,
            request: tonic::Request<super::SetOdometryRequest>,
        ) -> Result<tonic::Response<super::SetOdometryResponse>, tonic::Status>;
    }
    ///*
    /// Allows interfacing a vehicle with a motion capture system in
    /// order to allow navigation without global positioning sources available
    /// (e.g. indoors, or when flying under a bridge. etc.).
    #[derive(Debug)]
    pub struct MocapServiceServer<T: MocapService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MocapService> MocapServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MocapServiceServer<T>
    where
        T: MocapService,
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
                "/mavsdk.rpc.mocap.MocapService/SetVisionPositionEstimate" => {
                    #[allow(non_camel_case_types)]
                    struct SetVisionPositionEstimateSvc<T: MocapService>(pub Arc<T>);
                    impl<
                        T: MocapService,
                    > tonic::server::UnaryService<
                        super::SetVisionPositionEstimateRequest,
                    > for SetVisionPositionEstimateSvc<T> {
                        type Response = super::SetVisionPositionEstimateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetVisionPositionEstimateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_vision_position_estimate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetVisionPositionEstimateSvc(inner);
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
                "/mavsdk.rpc.mocap.MocapService/SetAttitudePositionMocap" => {
                    #[allow(non_camel_case_types)]
                    struct SetAttitudePositionMocapSvc<T: MocapService>(pub Arc<T>);
                    impl<
                        T: MocapService,
                    > tonic::server::UnaryService<super::SetAttitudePositionMocapRequest>
                    for SetAttitudePositionMocapSvc<T> {
                        type Response = super::SetAttitudePositionMocapResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetAttitudePositionMocapRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_attitude_position_mocap(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAttitudePositionMocapSvc(inner);
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
                "/mavsdk.rpc.mocap.MocapService/SetOdometry" => {
                    #[allow(non_camel_case_types)]
                    struct SetOdometrySvc<T: MocapService>(pub Arc<T>);
                    impl<
                        T: MocapService,
                    > tonic::server::UnaryService<super::SetOdometryRequest>
                    for SetOdometrySvc<T> {
                        type Response = super::SetOdometryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetOdometryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_odometry(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetOdometrySvc(inner);
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
    impl<T: MocapService> Clone for MocapServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MocapService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MocapService> tonic::server::NamedService for MocapServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.mocap.MocapService";
    }
}
