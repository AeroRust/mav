#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetEntriesRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetEntriesResponse {
    #[prost(message, optional, tag = "1")]
    pub log_files_result: ::core::option::Option<LogFilesResult>,
    /// List of entries
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDownloadLogFileRequest {
    /// Entry of the log file to download.
    #[prost(message, optional, tag = "1")]
    pub entry: ::core::option::Option<Entry>,
    /// Path of where to download log file to.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DownloadLogFileResponse {
    #[prost(message, optional, tag = "1")]
    pub log_files_result: ::core::option::Option<LogFilesResult>,
    /// Progress if result is progress
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<ProgressData>,
}
///
/// Progress data coming when downloading a log file.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProgressData {
    /// Progress from 0 to 1
    #[prost(float, tag = "1")]
    pub progress: f32,
}
/// Log file entry type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// ID of the log file, to specify a file to be downloaded
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// Date of the log file in UTC in ISO 8601 format "yyyy-mm-ddThh:mm:ssZ"
    #[prost(string, tag = "2")]
    pub date: ::prost::alloc::string::String,
    /// Size of file in bytes
    #[prost(uint32, tag = "3")]
    pub size_bytes: u32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogFilesResult {
    /// Result enum value
    #[prost(enumeration = "log_files_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LogFilesResult`.
pub mod log_files_result {
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
        /// Request succeeded
        Success = 1,
        /// Progress update
        Next = 2,
        /// No log files found
        NoLogfiles = 3,
        /// A timeout happened
        Timeout = 4,
        /// Invalid argument
        InvalidArgument = 5,
        /// File open failed
        FileOpenFailed = 6,
        /// No system is connected
        NoSystem = 7,
    }
}
#[doc = r" Generated client implementations."]
pub mod log_files_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Allow to download log files from the vehicle after a flight is complete."]
    #[doc = " For log streaming during flight check the logging plugin."]
    #[derive(Debug, Clone)]
    pub struct LogFilesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LogFilesServiceClient<tonic::transport::Channel> {
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
    impl<T> LogFilesServiceClient<T>
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
        ) -> LogFilesServiceClient<InterceptedService<T, F>>
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
            LogFilesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get List of log files."]
        pub async fn get_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntriesRequest>,
        ) -> Result<tonic::Response<super::GetEntriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.log_files.LogFilesService/GetEntries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Download log file."]
        pub async fn subscribe_download_log_file(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeDownloadLogFileRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DownloadLogFileResponse>>,
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
                "/mavsdk.rpc.log_files.LogFilesService/SubscribeDownloadLogFile",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod log_files_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LogFilesServiceServer."]
    #[async_trait]
    pub trait LogFilesService: Send + Sync + 'static {
        #[doc = " Get List of log files."]
        async fn get_entries(
            &self,
            request: tonic::Request<super::GetEntriesRequest>,
        ) -> Result<tonic::Response<super::GetEntriesResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeDownloadLogFile method."]
        type SubscribeDownloadLogFileStream: futures_core::Stream<Item = Result<super::DownloadLogFileResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Download log file."]
        async fn subscribe_download_log_file(
            &self,
            request: tonic::Request<super::SubscribeDownloadLogFileRequest>,
        ) -> Result<tonic::Response<Self::SubscribeDownloadLogFileStream>, tonic::Status>;
    }
    #[doc = " Allow to download log files from the vehicle after a flight is complete."]
    #[doc = " For log streaming during flight check the logging plugin."]
    #[derive(Debug)]
    pub struct LogFilesServiceServer<T: LogFilesService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LogFilesService> LogFilesServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LogFilesServiceServer<T>
    where
        T: LogFilesService,
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
                "/mavsdk.rpc.log_files.LogFilesService/GetEntries" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntriesSvc<T: LogFilesService>(pub Arc<T>);
                    impl<T: LogFilesService> tonic::server::UnaryService<super::GetEntriesRequest>
                        for GetEntriesSvc<T>
                    {
                        type Response = super::GetEntriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEntriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_entries(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEntriesSvc(inner);
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
                "/mavsdk.rpc.log_files.LogFilesService/SubscribeDownloadLogFile" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeDownloadLogFileSvc<T: LogFilesService>(pub Arc<T>);
                    impl<T: LogFilesService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeDownloadLogFileRequest,
                        > for SubscribeDownloadLogFileSvc<T>
                    {
                        type Response = super::DownloadLogFileResponse;
                        type ResponseStream = T::SubscribeDownloadLogFileStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeDownloadLogFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_download_log_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeDownloadLogFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: LogFilesService> Clone for LogFilesServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: LogFilesService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LogFilesService> tonic::transport::NamedService for LogFilesServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.log_files.LogFilesService";
    }
}
