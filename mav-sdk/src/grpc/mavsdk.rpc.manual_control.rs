#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPositionControlRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPositionControlResponse {
    #[prost(message, optional, tag = "1")]
    pub manual_control_result: ::core::option::Option<ManualControlResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartAltitudeControlRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartAltitudeControlResponse {
    #[prost(message, optional, tag = "1")]
    pub manual_control_result: ::core::option::Option<ManualControlResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetManualControlInputRequest {
    /// value between -1. to 1. negative -> backwards, positive -> forwards
    #[prost(float, tag = "1")]
    pub x: f32,
    /// value between -1. to 1. negative -> left, positive -> right
    #[prost(float, tag = "2")]
    pub y: f32,
    /// value between -1. to 1. negative -> down, positive -> up (usually for now, for multicopter 0 to 1 is expected)
    #[prost(float, tag = "3")]
    pub z: f32,
    /// value between -1. to 1. negative -> turn anti-clockwise (towards the left), positive -> turn clockwise (towards the right)
    #[prost(float, tag = "4")]
    pub r: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetManualControlInputResponse {
    #[prost(message, optional, tag = "1")]
    pub manual_control_result: ::core::option::Option<ManualControlResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualControlResult {
    /// Result enum value
    #[prost(enumeration = "manual_control_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ManualControlResult`.
pub mod manual_control_result {
    /// Possible results returned for manual control requests.
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
        /// Request was successful
        Success = 1,
        /// No system is connected
        NoSystem = 2,
        /// Connection error
        ConnectionError = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Command refused by vehicle
        CommandDenied = 5,
        /// Request timed out
        Timeout = 6,
        /// Input out of range
        InputOutOfRange = 7,
        /// No Input set
        InputNotSet = 8,
    }
}
#[doc = r" Generated client implementations."]
pub mod manual_control_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable manual control using e.g. a joystick or gamepad."]
    #[derive(Debug, Clone)]
    pub struct ManualControlServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ManualControlServiceClient<tonic::transport::Channel> {
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
    impl<T> ManualControlServiceClient<T>
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
        ) -> ManualControlServiceClient<InterceptedService<T, F>>
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
            ManualControlServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Start position control using e.g. joystick input."]
        #[doc = ""]
        #[doc = " Requires manual control input to be sent regularly already."]
        #[doc = " Requires a valid position using e.g. GPS, external vision, or optical flow."]
        pub async fn start_position_control(
            &mut self,
            request: impl tonic::IntoRequest<super::StartPositionControlRequest>,
        ) -> Result<tonic::Response<super::StartPositionControlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.manual_control.ManualControlService/StartPositionControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start altitude control"]
        #[doc = ""]
        #[doc = " Requires manual control input to be sent regularly already."]
        #[doc = " Does not require a  valid position e.g. GPS."]
        pub async fn start_altitude_control(
            &mut self,
            request: impl tonic::IntoRequest<super::StartAltitudeControlRequest>,
        ) -> Result<tonic::Response<super::StartAltitudeControlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.manual_control.ManualControlService/StartAltitudeControl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set manual control input"]
        #[doc = ""]
        #[doc = " The manual control input needs to be sent at a rate high enough to prevent"]
        #[doc = " triggering of RC loss, a good minimum rate is 10 Hz."]
        pub async fn set_manual_control_input(
            &mut self,
            request: impl tonic::IntoRequest<super::SetManualControlInputRequest>,
        ) -> Result<tonic::Response<super::SetManualControlInputResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.manual_control.ManualControlService/SetManualControlInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod manual_control_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ManualControlServiceServer."]
    #[async_trait]
    pub trait ManualControlService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Start position control using e.g. joystick input."]
        #[doc = ""]
        #[doc = " Requires manual control input to be sent regularly already."]
        #[doc = " Requires a valid position using e.g. GPS, external vision, or optical flow."]
        async fn start_position_control(
            &self,
            request: tonic::Request<super::StartPositionControlRequest>,
        ) -> Result<tonic::Response<super::StartPositionControlResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Start altitude control"]
        #[doc = ""]
        #[doc = " Requires manual control input to be sent regularly already."]
        #[doc = " Does not require a  valid position e.g. GPS."]
        async fn start_altitude_control(
            &self,
            request: tonic::Request<super::StartAltitudeControlRequest>,
        ) -> Result<tonic::Response<super::StartAltitudeControlResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set manual control input"]
        #[doc = ""]
        #[doc = " The manual control input needs to be sent at a rate high enough to prevent"]
        #[doc = " triggering of RC loss, a good minimum rate is 10 Hz."]
        async fn set_manual_control_input(
            &self,
            request: tonic::Request<super::SetManualControlInputRequest>,
        ) -> Result<tonic::Response<super::SetManualControlInputResponse>, tonic::Status>;
    }
    #[doc = " Enable manual control using e.g. a joystick or gamepad."]
    #[derive(Debug)]
    pub struct ManualControlServiceServer<T: ManualControlService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ManualControlService> ManualControlServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ManualControlServiceServer<T>
    where
        T: ManualControlService,
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
                "/mavsdk.rpc.manual_control.ManualControlService/StartPositionControl" => {
                    #[allow(non_camel_case_types)]
                    struct StartPositionControlSvc<T: ManualControlService>(pub Arc<T>);
                    impl<T: ManualControlService>
                        tonic::server::UnaryService<super::StartPositionControlRequest>
                        for StartPositionControlSvc<T>
                    {
                        type Response = super::StartPositionControlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartPositionControlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_position_control(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartPositionControlSvc(inner);
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
                "/mavsdk.rpc.manual_control.ManualControlService/StartAltitudeControl" => {
                    #[allow(non_camel_case_types)]
                    struct StartAltitudeControlSvc<T: ManualControlService>(pub Arc<T>);
                    impl<T: ManualControlService>
                        tonic::server::UnaryService<super::StartAltitudeControlRequest>
                        for StartAltitudeControlSvc<T>
                    {
                        type Response = super::StartAltitudeControlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartAltitudeControlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_altitude_control(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartAltitudeControlSvc(inner);
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
                "/mavsdk.rpc.manual_control.ManualControlService/SetManualControlInput" => {
                    #[allow(non_camel_case_types)]
                    struct SetManualControlInputSvc<T: ManualControlService>(pub Arc<T>);
                    impl<T: ManualControlService>
                        tonic::server::UnaryService<super::SetManualControlInputRequest>
                        for SetManualControlInputSvc<T>
                    {
                        type Response = super::SetManualControlInputResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetManualControlInputRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_manual_control_input(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetManualControlInputSvc(inner);
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
    impl<T: ManualControlService> Clone for ManualControlServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ManualControlService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ManualControlService> tonic::transport::NamedService for ManualControlServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.manual_control.ManualControlService";
    }
}
