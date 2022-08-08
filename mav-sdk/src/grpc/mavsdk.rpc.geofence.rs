/// Point type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag="1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag="2")]
    pub longitude_deg: f64,
}
/// Polygon type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polygon {
    /// Points defining the polygon
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<Point>,
    /// Fence type
    #[prost(enumeration="polygon::FenceType", tag="2")]
    pub fence_type: i32,
}
/// Nested message and enum types in `Polygon`.
pub mod polygon {
    /// Geofence polygon types.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FenceType {
        /// Type representing an inclusion fence
        Inclusion = 0,
        /// Type representing an exclusion fence
        Exclusion = 1,
    }
    impl FenceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FenceType::Inclusion => "FENCE_TYPE_INCLUSION",
                FenceType::Exclusion => "FENCE_TYPE_EXCLUSION",
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceRequest {
    /// Polygon(s) representing the geofence(s)
    #[prost(message, repeated, tag="1")]
    pub polygons: ::prost::alloc::vec::Vec<Polygon>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceResponse {
    #[prost(message, optional, tag="1")]
    pub geofence_result: ::core::option::Option<GeofenceResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearGeofenceRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearGeofenceResponse {
    #[prost(message, optional, tag="1")]
    pub geofence_result: ::core::option::Option<GeofenceResult>,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeofenceResult {
    /// Result enum value
    #[prost(enumeration="geofence_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GeofenceResult`.
pub mod geofence_result {
    /// Possible results returned for geofence requests.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Error
        Error = 2,
        /// Too many Polygon objects in the geofence
        TooManyGeofenceItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
        /// No system connected
        NoSystem = 7,
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
                Result::Error => "RESULT_ERROR",
                Result::TooManyGeofenceItems => "RESULT_TOO_MANY_GEOFENCE_ITEMS",
                Result::Busy => "RESULT_BUSY",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::InvalidArgument => "RESULT_INVALID_ARGUMENT",
                Result::NoSystem => "RESULT_NO_SYSTEM",
            }
        }
    }
}
/// Generated client implementations.
pub mod geofence_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Enable setting a geofence.
    #[derive(Debug, Clone)]
    pub struct GeofenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GeofenceServiceClient<tonic::transport::Channel> {
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
    impl<T> GeofenceServiceClient<T>
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
        ) -> GeofenceServiceClient<InterceptedService<T, F>>
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
            GeofenceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        /// Upload a geofence.
        ///
        /// Polygons are uploaded to a drone. Once uploaded, the geofence will remain
        /// on the drone even if a connection is lost.
        pub async fn upload_geofence(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadGeofenceRequest>,
        ) -> Result<tonic::Response<super::UploadGeofenceResponse>, tonic::Status> {
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
                "/mavsdk.rpc.geofence.GeofenceService/UploadGeofence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Clear all geofences saved on the vehicle.
        pub async fn clear_geofence(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearGeofenceRequest>,
        ) -> Result<tonic::Response<super::ClearGeofenceResponse>, tonic::Status> {
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
                "/mavsdk.rpc.geofence.GeofenceService/ClearGeofence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod geofence_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GeofenceServiceServer.
    #[async_trait]
    pub trait GeofenceService: Send + Sync + 'static {
        ///
        /// Upload a geofence.
        ///
        /// Polygons are uploaded to a drone. Once uploaded, the geofence will remain
        /// on the drone even if a connection is lost.
        async fn upload_geofence(
            &self,
            request: tonic::Request<super::UploadGeofenceRequest>,
        ) -> Result<tonic::Response<super::UploadGeofenceResponse>, tonic::Status>;
        ///
        /// Clear all geofences saved on the vehicle.
        async fn clear_geofence(
            &self,
            request: tonic::Request<super::ClearGeofenceRequest>,
        ) -> Result<tonic::Response<super::ClearGeofenceResponse>, tonic::Status>;
    }
    /// Enable setting a geofence.
    #[derive(Debug)]
    pub struct GeofenceServiceServer<T: GeofenceService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GeofenceService> GeofenceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GeofenceServiceServer<T>
    where
        T: GeofenceService,
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
                "/mavsdk.rpc.geofence.GeofenceService/UploadGeofence" => {
                    #[allow(non_camel_case_types)]
                    struct UploadGeofenceSvc<T: GeofenceService>(pub Arc<T>);
                    impl<
                        T: GeofenceService,
                    > tonic::server::UnaryService<super::UploadGeofenceRequest>
                    for UploadGeofenceSvc<T> {
                        type Response = super::UploadGeofenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadGeofenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).upload_geofence(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadGeofenceSvc(inner);
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
                "/mavsdk.rpc.geofence.GeofenceService/ClearGeofence" => {
                    #[allow(non_camel_case_types)]
                    struct ClearGeofenceSvc<T: GeofenceService>(pub Arc<T>);
                    impl<
                        T: GeofenceService,
                    > tonic::server::UnaryService<super::ClearGeofenceRequest>
                    for ClearGeofenceSvc<T> {
                        type Response = super::ClearGeofenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearGeofenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clear_geofence(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearGeofenceSvc(inner);
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
    impl<T: GeofenceService> Clone for GeofenceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GeofenceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GeofenceService> tonic::server::NamedService for GeofenceServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.geofence.GeofenceService";
    }
}
