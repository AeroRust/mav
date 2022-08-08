#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTransponderRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransponderResponse {
    /// The next detection
    #[prost(message, optional, tag="1")]
    pub transponder: ::core::option::Option<AdsbVehicle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateTransponderRequest {
    /// The requested rate (in Hertz)
    #[prost(double, tag="1")]
    pub rate_hz: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRateTransponderResponse {
    #[prost(message, optional, tag="1")]
    pub transponder_result: ::core::option::Option<TransponderResult>,
}
/// ADSB Vehicle type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdsbVehicle {
    /// ICAO (International Civil Aviation Organization) unique worldwide identifier
    #[prost(uint32, tag="1")]
    pub icao_address: u32,
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag="2")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180).
    #[prost(double, tag="3")]
    pub longitude_deg: f64,
    /// ADSB altitude type.
    #[prost(enumeration="AdsbAltitudeType", tag="4")]
    pub altitude_type: i32,
    /// Altitude in metres according to altitude_type 
    #[prost(float, tag="5")]
    pub absolute_altitude_m: f32,
    /// Course over ground, in degrees
    #[prost(float, tag="6")]
    pub heading_deg: f32,
    /// The horizontal velocity in metres/second
    #[prost(float, tag="7")]
    pub horizontal_velocity_m_s: f32,
    /// The vertical velocity in metres/second. Positive is up.
    #[prost(float, tag="8")]
    pub vertical_velocity_m_s: f32,
    /// The callsign
    #[prost(string, tag="9")]
    pub callsign: ::prost::alloc::string::String,
    /// ADSB emitter type.
    #[prost(enumeration="AdsbEmitterType", tag="10")]
    pub emitter_type: i32,
    /// Squawk code.
    #[prost(uint32, tag="13")]
    pub squawk: u32,
    /// Time Since Last Communication in seconds.
    #[prost(uint32, tag="14")]
    pub tslc_s: u32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransponderResult {
    /// Result enum value
    #[prost(enumeration="transponder_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransponderResult`.
pub mod transponder_result {
    /// Possible results returned for transponder requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Success: the transponder command was accepted by the vehicle
        Success = 1,
        /// No system connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command refused by vehicle
        CommandDenied = 5,
        /// Request timed out
        Timeout = 6,
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
                Result::Busy => "RESULT_BUSY",
                Result::CommandDenied => "RESULT_COMMAND_DENIED",
                Result::Timeout => "RESULT_TIMEOUT",
            }
        }
    }
}
/// ADSB classification for the type of vehicle emitting the transponder signal.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdsbEmitterType {
    /// No emitter info.
    NoInfo = 0,
    /// Light emitter.
    Light = 1,
    /// Small emitter.
    Small = 2,
    /// Large emitter.
    Large = 3,
    /// High vortex emitter.
    HighVortexLarge = 4,
    /// Heavy emitter.
    Heavy = 5,
    /// Highly maneuverable emitter.
    HighlyManuv = 6,
    /// Rotorcraft emitter.
    Rotocraft = 7,
    /// Unassigned emitter.
    Unassigned = 8,
    /// Glider emitter.
    Glider = 9,
    /// Lighter air emitter.
    LighterAir = 10,
    /// Parachute emitter.
    Parachute = 11,
    /// Ultra light emitter.
    UltraLight = 12,
    /// Unassigned2 emitter.
    Unassigned2 = 13,
    /// UAV emitter.
    Uav = 14,
    /// Space emitter.
    Space = 15,
    /// Unassigned3 emitter.
    Unassgined3 = 16,
    /// Emergency emitter.
    EmergencySurface = 17,
    /// Service surface emitter.
    ServiceSurface = 18,
    /// Point obstacle emitter.
    PointObstacle = 19,
}
impl AdsbEmitterType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdsbEmitterType::NoInfo => "ADSB_EMITTER_TYPE_NO_INFO",
            AdsbEmitterType::Light => "ADSB_EMITTER_TYPE_LIGHT",
            AdsbEmitterType::Small => "ADSB_EMITTER_TYPE_SMALL",
            AdsbEmitterType::Large => "ADSB_EMITTER_TYPE_LARGE",
            AdsbEmitterType::HighVortexLarge => "ADSB_EMITTER_TYPE_HIGH_VORTEX_LARGE",
            AdsbEmitterType::Heavy => "ADSB_EMITTER_TYPE_HEAVY",
            AdsbEmitterType::HighlyManuv => "ADSB_EMITTER_TYPE_HIGHLY_MANUV",
            AdsbEmitterType::Rotocraft => "ADSB_EMITTER_TYPE_ROTOCRAFT",
            AdsbEmitterType::Unassigned => "ADSB_EMITTER_TYPE_UNASSIGNED",
            AdsbEmitterType::Glider => "ADSB_EMITTER_TYPE_GLIDER",
            AdsbEmitterType::LighterAir => "ADSB_EMITTER_TYPE_LIGHTER_AIR",
            AdsbEmitterType::Parachute => "ADSB_EMITTER_TYPE_PARACHUTE",
            AdsbEmitterType::UltraLight => "ADSB_EMITTER_TYPE_ULTRA_LIGHT",
            AdsbEmitterType::Unassigned2 => "ADSB_EMITTER_TYPE_UNASSIGNED2",
            AdsbEmitterType::Uav => "ADSB_EMITTER_TYPE_UAV",
            AdsbEmitterType::Space => "ADSB_EMITTER_TYPE_SPACE",
            AdsbEmitterType::Unassgined3 => "ADSB_EMITTER_TYPE_UNASSGINED3",
            AdsbEmitterType::EmergencySurface => "ADSB_EMITTER_TYPE_EMERGENCY_SURFACE",
            AdsbEmitterType::ServiceSurface => "ADSB_EMITTER_TYPE_SERVICE_SURFACE",
            AdsbEmitterType::PointObstacle => "ADSB_EMITTER_TYPE_POINT_OBSTACLE",
        }
    }
}
/// Altitude type used in AdsbVehicle message
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AdsbAltitudeType {
    /// Altitude reported from a Baro source using QNH reference
    PressureQnh = 0,
    /// Altitude reported from a GNSS source
    Geometric = 1,
}
impl AdsbAltitudeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AdsbAltitudeType::PressureQnh => "ADSB_ALTITUDE_TYPE_PRESSURE_QNH",
            AdsbAltitudeType::Geometric => "ADSB_ALTITUDE_TYPE_GEOMETRIC",
        }
    }
}
/// Generated client implementations.
pub mod transponder_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    /// Allow users to get ADS-B information
    /// and set ADS-B update rates.
    #[derive(Debug, Clone)]
    pub struct TransponderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TransponderServiceClient<tonic::transport::Channel> {
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
    impl<T> TransponderServiceClient<T>
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
        ) -> TransponderServiceClient<InterceptedService<T, F>>
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
            TransponderServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Subscribe to 'transponder' updates.
        pub async fn subscribe_transponder(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeTransponderRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TransponderResponse>>,
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
                "/mavsdk.rpc.transponder.TransponderService/SubscribeTransponder",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Set rate to 'transponder' updates.
        pub async fn set_rate_transponder(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRateTransponderRequest>,
        ) -> Result<tonic::Response<super::SetRateTransponderResponse>, tonic::Status> {
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
                "/mavsdk.rpc.transponder.TransponderService/SetRateTransponder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod transponder_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TransponderServiceServer.
    #[async_trait]
    pub trait TransponderService: Send + Sync + 'static {
        ///Server streaming response type for the SubscribeTransponder method.
        type SubscribeTransponderStream: futures_core::Stream<
                Item = Result<super::TransponderResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Subscribe to 'transponder' updates.
        async fn subscribe_transponder(
            &self,
            request: tonic::Request<super::SubscribeTransponderRequest>,
        ) -> Result<tonic::Response<Self::SubscribeTransponderStream>, tonic::Status>;
        /// Set rate to 'transponder' updates.
        async fn set_rate_transponder(
            &self,
            request: tonic::Request<super::SetRateTransponderRequest>,
        ) -> Result<tonic::Response<super::SetRateTransponderResponse>, tonic::Status>;
    }
    ///
    /// Allow users to get ADS-B information
    /// and set ADS-B update rates.
    #[derive(Debug)]
    pub struct TransponderServiceServer<T: TransponderService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TransponderService> TransponderServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TransponderServiceServer<T>
    where
        T: TransponderService,
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
                "/mavsdk.rpc.transponder.TransponderService/SubscribeTransponder" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeTransponderSvc<T: TransponderService>(pub Arc<T>);
                    impl<
                        T: TransponderService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeTransponderRequest,
                    > for SubscribeTransponderSvc<T> {
                        type Response = super::TransponderResponse;
                        type ResponseStream = T::SubscribeTransponderStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeTransponderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_transponder(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeTransponderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.transponder.TransponderService/SetRateTransponder" => {
                    #[allow(non_camel_case_types)]
                    struct SetRateTransponderSvc<T: TransponderService>(pub Arc<T>);
                    impl<
                        T: TransponderService,
                    > tonic::server::UnaryService<super::SetRateTransponderRequest>
                    for SetRateTransponderSvc<T> {
                        type Response = super::SetRateTransponderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRateTransponderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_rate_transponder(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRateTransponderSvc(inner);
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
    impl<T: TransponderService> Clone for TransponderServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TransponderService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TransponderService> tonic::server::NamedService
    for TransponderServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.transponder.TransponderService";
    }
}
