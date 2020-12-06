/// Point type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
}
/// Polygon type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polygon {
    /// Points defining the polygon
    #[prost(message, repeated, tag = "1")]
    pub points: ::std::vec::Vec<Point>,
    /// Fence type
    #[prost(enumeration = "polygon::FenceType", tag = "2")]
    pub fence_type: i32,
}
pub mod polygon {
    /// Geofence polygon types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FenceType {
        /// Type representing an inclusion fence
        Inclusion = 0,
        /// Type representing an exclusion fence
        Exclusion = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceRequest {
    /// Polygon(s) representing the geofence(s)
    #[prost(message, repeated, tag = "1")]
    pub polygons: ::std::vec::Vec<Polygon>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceResponse {
    #[prost(message, optional, tag = "1")]
    pub geofence_result: ::std::option::Option<GeofenceResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeofenceResult {
    /// Result enum value
    #[prost(enumeration = "geofence_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: std::string::String,
}
pub mod geofence_result {
    /// Possible results returned for geofence requests.
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
    }
}
#[doc = r" Generated client implementations."]
pub mod geofence_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Enable setting a geofence."]
    pub struct GeofenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GeofenceServiceClient<tonic::transport::Channel> {
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
    impl<T> GeofenceServiceClient<T>
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
        #[doc = " Upload a geofence."]
        #[doc = ""]
        #[doc = " Polygons are uploaded to a drone. Once uploaded, the geofence will remain"]
        #[doc = " on the drone even if a connection is lost."]
        pub async fn upload_geofence(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadGeofenceRequest>,
        ) -> Result<tonic::Response<super::UploadGeofenceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
    }
    impl<T: Clone> Clone for GeofenceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GeofenceServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GeofenceServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod geofence_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GeofenceServiceServer."]
    #[async_trait]
    pub trait GeofenceService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Upload a geofence."]
        #[doc = ""]
        #[doc = " Polygons are uploaded to a drone. Once uploaded, the geofence will remain"]
        #[doc = " on the drone even if a connection is lost."]
        async fn upload_geofence(
            &self,
            request: tonic::Request<super::UploadGeofenceRequest>,
        ) -> Result<tonic::Response<super::UploadGeofenceResponse>, tonic::Status>;
    }
    #[doc = " Enable setting a geofence."]
    #[derive(Debug)]
    pub struct GeofenceServiceServer<T: GeofenceService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GeofenceService> GeofenceServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GeofenceServiceServer<T>
    where
        T: GeofenceService,
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
                "/mavsdk.rpc.geofence.GeofenceService/UploadGeofence" => {
                    #[allow(non_camel_case_types)]
                    struct UploadGeofenceSvc<T: GeofenceService>(pub Arc<T>);
                    impl<T: GeofenceService>
                        tonic::server::UnaryService<super::UploadGeofenceRequest>
                        for UploadGeofenceSvc<T>
                    {
                        type Response = super::UploadGeofenceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadGeofenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upload_geofence(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UploadGeofenceSvc(inner);
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
    impl<T: GeofenceService> Clone for GeofenceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GeofenceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GeofenceService> tonic::transport::NamedService for GeofenceServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.geofence.GeofenceService";
    }
}
