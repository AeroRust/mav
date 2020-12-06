#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeConnectionStateRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionStateResponse {
    /// Connection state
    #[prost(message, optional, tag = "1")]
    pub connection_state: ::std::option::Option<ConnectionState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunningPluginsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRunningPluginsResponse {
    /// Plugin info
    #[prost(message, repeated, tag = "1")]
    pub plugin_info: ::std::vec::Vec<PluginInfo>,
}
/// Connection state type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionState {
    /// UUID of the vehicle
    #[prost(uint64, tag = "1")]
    pub uuid: u64,
    /// Whether the vehicle got connected or disconnected
    #[prost(bool, tag = "2")]
    pub is_connected: bool,
}
/// Plugin info type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfo {
    /// Name of the plugin
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Address where the plugin is running
    #[prost(string, tag = "2")]
    pub address: std::string::String,
    /// Port where the plugin is running
    #[prost(int32, tag = "3")]
    pub port: i32,
}
#[doc = r" Generated client implementations."]
pub mod core_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Access to the connection state and running plugins."]
    pub struct CoreServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CoreServiceClient<tonic::transport::Channel> {
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
    impl<T> CoreServiceClient<T>
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
        #[doc = " Subscribe to 'connection state' updates."]
        pub async fn subscribe_connection_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeConnectionStateRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ConnectionStateResponse>>,
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
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Get a list of currently running plugins."]
        pub async fn list_running_plugins(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRunningPluginsRequest>,
        ) -> Result<tonic::Response<super::ListRunningPluginsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.core.CoreService/ListRunningPlugins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CoreServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CoreServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CoreServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod core_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CoreServiceServer."]
    #[async_trait]
    pub trait CoreService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeConnectionState method."]
        type SubscribeConnectionStateStream: Stream<Item = Result<super::ConnectionStateResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Subscribe to 'connection state' updates."]
        async fn subscribe_connection_state(
            &self,
            request: tonic::Request<super::SubscribeConnectionStateRequest>,
        ) -> Result<tonic::Response<Self::SubscribeConnectionStateStream>, tonic::Status>;
        #[doc = " Get a list of currently running plugins."]
        async fn list_running_plugins(
            &self,
            request: tonic::Request<super::ListRunningPluginsRequest>,
        ) -> Result<tonic::Response<super::ListRunningPluginsResponse>, tonic::Status>;
    }
    #[doc = " Access to the connection state and running plugins."]
    #[derive(Debug)]
    pub struct CoreServiceServer<T: CoreService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CoreService> CoreServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CoreServiceServer<T>
    where
        T: CoreService,
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
                "/mavsdk.rpc.core.CoreService/SubscribeConnectionState" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeConnectionStateSvc<T: CoreService>(pub Arc<T>);
                    impl<T: CoreService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeConnectionStateRequest,
                        > for SubscribeConnectionStateSvc<T>
                    {
                        type Response = super::ConnectionStateResponse;
                        type ResponseStream = T::SubscribeConnectionStateStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeConnectionStateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_connection_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = SubscribeConnectionStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mavsdk.rpc.core.CoreService/ListRunningPlugins" => {
                    #[allow(non_camel_case_types)]
                    struct ListRunningPluginsSvc<T: CoreService>(pub Arc<T>);
                    impl<T: CoreService>
                        tonic::server::UnaryService<super::ListRunningPluginsRequest>
                        for ListRunningPluginsSvc<T>
                    {
                        type Response = super::ListRunningPluginsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRunningPluginsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_running_plugins(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListRunningPluginsSvc(inner);
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
    impl<T: CoreService> Clone for CoreServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CoreService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CoreService> tonic::transport::NamedService for CoreServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.core.CoreService";
    }
}
