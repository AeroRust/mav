#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDownloadRequest {
    /// The path of the remote file to download.
    #[prost(string, tag="1")]
    pub remote_file_path: ::prost::alloc::string::String,
    /// The local directory to download to.
    #[prost(string, tag="2")]
    pub local_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
    /// The progress data if result is next
    #[prost(message, optional, tag="2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeUploadRequest {
    /// The local file path to upload.
    #[prost(string, tag="1")]
    pub local_file_path: ::prost::alloc::string::String,
    /// The remote directory to upload to.
    #[prost(string, tag="2")]
    pub remote_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
    /// The progress data if result is next
    #[prost(message, optional, tag="2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDirectoryRequest {
    /// The remote directory to list the contents for.
    #[prost(string, tag="1")]
    pub remote_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDirectoryResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
    /// The found directory contents.
    #[prost(string, repeated, tag="2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDirectoryRequest {
    /// The remote directory to create.
    #[prost(string, tag="1")]
    pub remote_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDirectoryResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDirectoryRequest {
    /// The remote directory to remove.
    #[prost(string, tag="1")]
    pub remote_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDirectoryResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileRequest {
    /// The path of the remote file to remove.
    #[prost(string, tag="1")]
    pub remote_file_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRequest {
    /// The remote source path.
    #[prost(string, tag="1")]
    pub remote_from_path: ::prost::alloc::string::String,
    /// The remote destination path.
    #[prost(string, tag="2")]
    pub remote_to_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreFilesIdenticalRequest {
    /// The path of the local file.
    #[prost(string, tag="1")]
    pub local_file_path: ::prost::alloc::string::String,
    /// The path of the remote file.
    #[prost(string, tag="2")]
    pub remote_file_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreFilesIdenticalResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
    /// Whether the files are identical.
    #[prost(bool, tag="2")]
    pub are_identical: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRootDirectoryRequest {
    /// The root directory to set.
    #[prost(string, tag="1")]
    pub root_dir: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRootDirectoryResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTargetCompidRequest {
    /// The component ID to set.
    #[prost(uint32, tag="1")]
    pub compid: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTargetCompidResponse {
    #[prost(message, optional, tag="1")]
    pub ftp_result: ::core::option::Option<FtpResult>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOurCompidRequest {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOurCompidResponse {
    /// Our component ID.
    #[prost(uint32, tag="1")]
    pub compid: u32,
}
/// Progress data type for file transfer.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressData {
    /// The number of bytes already transferred.
    #[prost(uint32, tag="1")]
    pub bytes_transferred: u32,
    /// The total bytes to transfer.
    #[prost(uint32, tag="2")]
    pub total_bytes: u32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FtpResult {
    /// Result enum value
    #[prost(enumeration="ftp_result::Result", tag="1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag="2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FtpResult`.
pub mod ftp_result {
    /// Possible results returned for FTP commands
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Success
        Success = 1,
        /// Intermediate message showing progress
        Next = 2,
        /// Timeout
        Timeout = 3,
        /// Operation is already in progress
        Busy = 4,
        /// File IO operation error
        FileIoError = 5,
        /// File exists already
        FileExists = 6,
        /// File does not exist
        FileDoesNotExist = 7,
        /// File is write protected
        FileProtected = 8,
        /// Invalid parameter
        InvalidParameter = 9,
        /// Unsupported command
        Unsupported = 10,
        /// General protocol error
        ProtocolError = 11,
        /// No system connected
        NoSystem = 12,
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
                Result::Next => "RESULT_NEXT",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::Busy => "RESULT_BUSY",
                Result::FileIoError => "RESULT_FILE_IO_ERROR",
                Result::FileExists => "RESULT_FILE_EXISTS",
                Result::FileDoesNotExist => "RESULT_FILE_DOES_NOT_EXIST",
                Result::FileProtected => "RESULT_FILE_PROTECTED",
                Result::InvalidParameter => "RESULT_INVALID_PARAMETER",
                Result::Unsupported => "RESULT_UNSUPPORTED",
                Result::ProtocolError => "RESULT_PROTOCOL_ERROR",
                Result::NoSystem => "RESULT_NO_SYSTEM",
            }
        }
    }
}
/// Generated client implementations.
pub mod ftp_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    /// Implements file transfer functionality using MAVLink FTP.
    #[derive(Debug, Clone)]
    pub struct FtpServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FtpServiceClient<tonic::transport::Channel> {
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
    impl<T> FtpServiceClient<T>
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
        ) -> FtpServiceClient<InterceptedService<T, F>>
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
            FtpServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Resets FTP server in case there are stale open sessions.
        pub async fn reset(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetRequest>,
        ) -> Result<tonic::Response<super::ResetResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/Reset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Downloads a file to local directory.
        pub async fn subscribe_download(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeDownloadRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DownloadResponse>>,
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
                "/mavsdk.rpc.ftp.FtpService/SubscribeDownload",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///
        /// Uploads local file to remote directory.
        pub async fn subscribe_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeUploadRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::UploadResponse>>,
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
                "/mavsdk.rpc.ftp.FtpService/SubscribeUpload",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///
        /// Lists items from a remote directory.
        pub async fn list_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDirectoryRequest>,
        ) -> Result<tonic::Response<super::ListDirectoryResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/ListDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Creates a remote directory.
        pub async fn create_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDirectoryRequest>,
        ) -> Result<tonic::Response<super::CreateDirectoryResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/CreateDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Removes a remote directory.
        pub async fn remove_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDirectoryRequest>,
        ) -> Result<tonic::Response<super::RemoveDirectoryResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/RemoveDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Removes a remote file.
        pub async fn remove_file(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFileRequest>,
        ) -> Result<tonic::Response<super::RemoveFileResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/RemoveFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Renames a remote file or remote directory.
        pub async fn rename(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameRequest>,
        ) -> Result<tonic::Response<super::RenameResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/Rename",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Compares a local file to a remote file using a CRC32 checksum.
        pub async fn are_files_identical(
            &mut self,
            request: impl tonic::IntoRequest<super::AreFilesIdenticalRequest>,
        ) -> Result<tonic::Response<super::AreFilesIdenticalResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/AreFilesIdentical",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Set root directory for MAVLink FTP server.
        pub async fn set_root_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRootDirectoryRequest>,
        ) -> Result<tonic::Response<super::SetRootDirectoryResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/SetRootDirectory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Set target component ID. By default it is the autopilot.
        pub async fn set_target_compid(
            &mut self,
            request: impl tonic::IntoRequest<super::SetTargetCompidRequest>,
        ) -> Result<tonic::Response<super::SetTargetCompidResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/SetTargetCompid",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// Get our own component ID.
        pub async fn get_our_compid(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOurCompidRequest>,
        ) -> Result<tonic::Response<super::GetOurCompidResponse>, tonic::Status> {
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
                "/mavsdk.rpc.ftp.FtpService/GetOurCompid",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ftp_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with FtpServiceServer.
    #[async_trait]
    pub trait FtpService: Send + Sync + 'static {
        ///
        /// Resets FTP server in case there are stale open sessions.
        async fn reset(
            &self,
            request: tonic::Request<super::ResetRequest>,
        ) -> Result<tonic::Response<super::ResetResponse>, tonic::Status>;
        ///Server streaming response type for the SubscribeDownload method.
        type SubscribeDownloadStream: futures_core::Stream<
                Item = Result<super::DownloadResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// Downloads a file to local directory.
        async fn subscribe_download(
            &self,
            request: tonic::Request<super::SubscribeDownloadRequest>,
        ) -> Result<tonic::Response<Self::SubscribeDownloadStream>, tonic::Status>;
        ///Server streaming response type for the SubscribeUpload method.
        type SubscribeUploadStream: futures_core::Stream<
                Item = Result<super::UploadResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// Uploads local file to remote directory.
        async fn subscribe_upload(
            &self,
            request: tonic::Request<super::SubscribeUploadRequest>,
        ) -> Result<tonic::Response<Self::SubscribeUploadStream>, tonic::Status>;
        ///
        /// Lists items from a remote directory.
        async fn list_directory(
            &self,
            request: tonic::Request<super::ListDirectoryRequest>,
        ) -> Result<tonic::Response<super::ListDirectoryResponse>, tonic::Status>;
        ///
        /// Creates a remote directory.
        async fn create_directory(
            &self,
            request: tonic::Request<super::CreateDirectoryRequest>,
        ) -> Result<tonic::Response<super::CreateDirectoryResponse>, tonic::Status>;
        ///
        /// Removes a remote directory.
        async fn remove_directory(
            &self,
            request: tonic::Request<super::RemoveDirectoryRequest>,
        ) -> Result<tonic::Response<super::RemoveDirectoryResponse>, tonic::Status>;
        ///
        /// Removes a remote file.
        async fn remove_file(
            &self,
            request: tonic::Request<super::RemoveFileRequest>,
        ) -> Result<tonic::Response<super::RemoveFileResponse>, tonic::Status>;
        ///
        /// Renames a remote file or remote directory.
        async fn rename(
            &self,
            request: tonic::Request<super::RenameRequest>,
        ) -> Result<tonic::Response<super::RenameResponse>, tonic::Status>;
        ///
        /// Compares a local file to a remote file using a CRC32 checksum.
        async fn are_files_identical(
            &self,
            request: tonic::Request<super::AreFilesIdenticalRequest>,
        ) -> Result<tonic::Response<super::AreFilesIdenticalResponse>, tonic::Status>;
        ///
        /// Set root directory for MAVLink FTP server.
        async fn set_root_directory(
            &self,
            request: tonic::Request<super::SetRootDirectoryRequest>,
        ) -> Result<tonic::Response<super::SetRootDirectoryResponse>, tonic::Status>;
        ///
        /// Set target component ID. By default it is the autopilot.
        async fn set_target_compid(
            &self,
            request: tonic::Request<super::SetTargetCompidRequest>,
        ) -> Result<tonic::Response<super::SetTargetCompidResponse>, tonic::Status>;
        ///
        /// Get our own component ID.
        async fn get_our_compid(
            &self,
            request: tonic::Request<super::GetOurCompidRequest>,
        ) -> Result<tonic::Response<super::GetOurCompidResponse>, tonic::Status>;
    }
    ///
    /// Implements file transfer functionality using MAVLink FTP.
    #[derive(Debug)]
    pub struct FtpServiceServer<T: FtpService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FtpService> FtpServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FtpServiceServer<T>
    where
        T: FtpService,
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
                "/mavsdk.rpc.ftp.FtpService/Reset" => {
                    #[allow(non_camel_case_types)]
                    struct ResetSvc<T: FtpService>(pub Arc<T>);
                    impl<T: FtpService> tonic::server::UnaryService<super::ResetRequest>
                    for ResetSvc<T> {
                        type Response = super::ResetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResetSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/SubscribeDownload" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeDownloadSvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeDownloadRequest,
                    > for SubscribeDownloadSvc<T> {
                        type Response = super::DownloadResponse;
                        type ResponseStream = T::SubscribeDownloadStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeDownloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_download(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeDownloadSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/SubscribeUpload" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeUploadSvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeUploadRequest,
                    > for SubscribeUploadSvc<T> {
                        type Response = super::UploadResponse;
                        type ResponseStream = T::SubscribeUploadStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeUploadSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/ListDirectory" => {
                    #[allow(non_camel_case_types)]
                    struct ListDirectorySvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::ListDirectoryRequest>
                    for ListDirectorySvc<T> {
                        type Response = super::ListDirectoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDirectoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_directory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDirectorySvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/CreateDirectory" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDirectorySvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::CreateDirectoryRequest>
                    for CreateDirectorySvc<T> {
                        type Response = super::CreateDirectoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDirectoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_directory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDirectorySvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/RemoveDirectory" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveDirectorySvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::RemoveDirectoryRequest>
                    for RemoveDirectorySvc<T> {
                        type Response = super::RemoveDirectoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveDirectoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_directory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveDirectorySvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/RemoveFile" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFileSvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::RemoveFileRequest>
                    for RemoveFileSvc<T> {
                        type Response = super::RemoveFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFileSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/Rename" => {
                    #[allow(non_camel_case_types)]
                    struct RenameSvc<T: FtpService>(pub Arc<T>);
                    impl<T: FtpService> tonic::server::UnaryService<super::RenameRequest>
                    for RenameSvc<T> {
                        type Response = super::RenameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenameRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).rename(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RenameSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/AreFilesIdentical" => {
                    #[allow(non_camel_case_types)]
                    struct AreFilesIdenticalSvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::AreFilesIdenticalRequest>
                    for AreFilesIdenticalSvc<T> {
                        type Response = super::AreFilesIdenticalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AreFilesIdenticalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).are_files_identical(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AreFilesIdenticalSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/SetRootDirectory" => {
                    #[allow(non_camel_case_types)]
                    struct SetRootDirectorySvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::SetRootDirectoryRequest>
                    for SetRootDirectorySvc<T> {
                        type Response = super::SetRootDirectoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRootDirectoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_root_directory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRootDirectorySvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/SetTargetCompid" => {
                    #[allow(non_camel_case_types)]
                    struct SetTargetCompidSvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::SetTargetCompidRequest>
                    for SetTargetCompidSvc<T> {
                        type Response = super::SetTargetCompidResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetTargetCompidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_target_compid(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetTargetCompidSvc(inner);
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
                "/mavsdk.rpc.ftp.FtpService/GetOurCompid" => {
                    #[allow(non_camel_case_types)]
                    struct GetOurCompidSvc<T: FtpService>(pub Arc<T>);
                    impl<
                        T: FtpService,
                    > tonic::server::UnaryService<super::GetOurCompidRequest>
                    for GetOurCompidSvc<T> {
                        type Response = super::GetOurCompidResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOurCompidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_our_compid(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOurCompidSvc(inner);
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
    impl<T: FtpService> Clone for FtpServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: FtpService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FtpService> tonic::server::NamedService for FtpServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.ftp.FtpService";
    }
}
