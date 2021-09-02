#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakePhotoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakePhotoResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPhotoIntervalRequest {
    /// Interval between photos (in seconds)
    #[prost(float, tag = "1")]
    pub interval_s: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartPhotoIntervalResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPhotoIntervalRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPhotoIntervalResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoStreamingRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartVideoStreamingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoStreamingRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopVideoStreamingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeRequest {
    /// Camera mode to set
    #[prost(enumeration = "Mode", tag = "1")]
    pub mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInformationRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InformationResponse {
    /// Camera information
    #[prost(message, optional, tag = "1")]
    pub information: ::core::option::Option<Information>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeModeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModeResponse {
    /// Camera mode
    #[prost(enumeration = "Mode", tag = "1")]
    pub mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeVideoStreamInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInfoResponse {
    /// Video stream info
    #[prost(message, optional, tag = "1")]
    pub video_stream_info: ::core::option::Option<VideoStreamInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCaptureInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInfoResponse {
    /// Capture info
    #[prost(message, optional, tag = "1")]
    pub capture_info: ::core::option::Option<CaptureInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeStatusRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    /// Camera status
    #[prost(message, optional, tag = "1")]
    pub camera_status: ::core::option::Option<Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCurrentSettingsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentSettingsResponse {
    /// List of current settings
    #[prost(message, repeated, tag = "1")]
    pub current_settings: ::prost::alloc::vec::Vec<Setting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribePossibleSettingOptionsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PossibleSettingOptionsResponse {
    /// List of settings that can be changed
    #[prost(message, repeated, tag = "1")]
    pub setting_options: ::prost::alloc::vec::Vec<SettingOptions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSettingRequest {
    /// Desired setting
    #[prost(message, optional, tag = "1")]
    pub setting: ::core::option::Option<Setting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSettingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingRequest {
    /// Requested setting
    #[prost(message, optional, tag = "1")]
    pub setting: ::core::option::Option<Setting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
    /// Setting
    #[prost(message, optional, tag = "2")]
    pub setting: ::core::option::Option<Setting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatStorageRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatStorageResponse {
    #[prost(message, optional, tag = "1")]
    pub camera_result: ::core::option::Option<CameraResult>,
}
/// Result type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CameraResult {
    /// Result enum value
    #[prost(enumeration = "camera_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CameraResult`.
pub mod camera_result {
    /// Possible results returned for camera commands
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Command executed successfully
        Success = 1,
        /// Command in progress
        InProgress = 2,
        /// Camera is busy and rejected command
        Busy = 3,
        /// Camera denied the command
        Denied = 4,
        /// An error has occured while executing the command
        Error = 5,
        /// Command timed out
        Timeout = 6,
        /// Command has wrong argument(s)
        WrongArgument = 7,
    }
}
/// Position type in global coordinates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude AMSL (above mean sea level) in metres
    #[prost(float, tag = "3")]
    pub absolute_altitude_m: f32,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag = "4")]
    pub relative_altitude_m: f32,
}
///
/// Quaternion type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Hamilton quaternion product definition is used.
/// A zero-rotation quaternion is represented by (1,0,0,0).
/// The quaternion could also be written as w + xi + yj + zk.
///
/// For more info see: https://en.wikipedia.org/wiki/Quaternion
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// Quaternion entry 0, also denoted as a
    #[prost(float, tag = "1")]
    pub w: f32,
    /// Quaternion entry 1, also denoted as b
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Quaternion entry 2, also denoted as c
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Quaternion entry 3, also denoted as d
    #[prost(float, tag = "4")]
    pub z: f32,
}
///
/// Euler angle type.
///
/// All rotations and axis systems follow the right-hand rule.
/// The Euler angles follow the convention of a 3-2-1 intrinsic Tait-Bryan rotation sequence.
///
/// For more info see https://en.wikipedia.org/wiki/Euler_angles
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EulerAngle {
    /// Roll angle in degrees, positive is banking to the right
    #[prost(float, tag = "1")]
    pub roll_deg: f32,
    /// Pitch angle in degrees, positive is pitching nose up
    #[prost(float, tag = "2")]
    pub pitch_deg: f32,
    /// Yaw angle in degrees, positive is clock-wise seen from above
    #[prost(float, tag = "3")]
    pub yaw_deg: f32,
}
/// Information about a picture just captured.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInfo {
    /// Location where the picture was taken
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<Position>,
    /// Attitude of the camera when the picture was taken (quaternion)
    #[prost(message, optional, tag = "2")]
    pub attitude_quaternion: ::core::option::Option<Quaternion>,
    /// Attitude of the camera when the picture was taken (euler angle)
    #[prost(message, optional, tag = "3")]
    pub attitude_euler_angle: ::core::option::Option<EulerAngle>,
    /// Timestamp in UTC (since UNIX epoch) in microseconds
    #[prost(uint64, tag = "4")]
    pub time_utc_us: u64,
    /// True if the capture was successful
    #[prost(bool, tag = "5")]
    pub is_success: bool,
    /// Zero-based index of this image since vehicle was armed
    #[prost(int32, tag = "6")]
    pub index: i32,
    /// Download URL of this image
    #[prost(string, tag = "7")]
    pub file_url: ::prost::alloc::string::String,
}
/// Type for video stream settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamSettings {
    /// Frames per second
    #[prost(float, tag = "1")]
    pub frame_rate_hz: f32,
    /// Horizontal resolution (in pixels)
    #[prost(uint32, tag = "2")]
    pub horizontal_resolution_pix: u32,
    /// Vertical resolution (in pixels)
    #[prost(uint32, tag = "3")]
    pub vertical_resolution_pix: u32,
    /// Bit rate (in bits per second)
    #[prost(uint32, tag = "4")]
    pub bit_rate_b_s: u32,
    /// Video image rotation (clockwise, 0-359 degrees)
    #[prost(uint32, tag = "5")]
    pub rotation_deg: u32,
    /// Video stream URI
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
}
/// Information about the video stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInfo {
    /// Video stream settings
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<VideoStreamSettings>,
    /// Current status of video streaming
    #[prost(enumeration = "video_stream_info::Status", tag = "2")]
    pub status: i32,
}
/// Nested message and enum types in `VideoStreamInfo`.
pub mod video_stream_info {
    /// Video stream status type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Video stream is not running
        NotRunning = 0,
        /// Video stream is running
        InProgress = 1,
    }
}
/// Information about the camera status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// Whether video recording is currently in process
    #[prost(bool, tag = "1")]
    pub video_on: bool,
    /// Whether a photo interval is currently in process
    #[prost(bool, tag = "2")]
    pub photo_interval_on: bool,
    /// Used storage (in MiB)
    #[prost(float, tag = "3")]
    pub used_storage_mib: f32,
    /// Available storage (in MiB)
    #[prost(float, tag = "4")]
    pub available_storage_mib: f32,
    /// Total storage (in MiB)
    #[prost(float, tag = "5")]
    pub total_storage_mib: f32,
    /// Elapsed time since starting the video recording (in seconds)
    #[prost(float, tag = "6")]
    pub recording_time_s: f32,
    /// Current folder name where media are saved
    #[prost(string, tag = "7")]
    pub media_folder_name: ::prost::alloc::string::String,
    /// Storage status
    #[prost(enumeration = "status::StorageStatus", tag = "8")]
    pub storage_status: i32,
}
/// Nested message and enum types in `Status`.
pub mod status {
    /// Storage status type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StorageStatus {
        /// Status not available
        NotAvailable = 0,
        /// Storage is not formatted (i.e. has no recognized file system)
        Unformatted = 1,
        /// Storage is formatted (i.e. has recognized a file system)
        Formatted = 2,
    }
}
/// Type to represent a setting option.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// Name of the option (machine readable)
    #[prost(string, tag = "1")]
    pub option_id: ::prost::alloc::string::String,
    /// Description of the option (human readable)
    #[prost(string, tag = "2")]
    pub option_description: ::prost::alloc::string::String,
}
/// Type to represent a setting with a selected option.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Setting {
    /// Name of a setting (machine readable)
    #[prost(string, tag = "1")]
    pub setting_id: ::prost::alloc::string::String,
    /// Description of the setting (human readable). This field is meant to be read from the drone, ignore it when setting.
    #[prost(string, tag = "2")]
    pub setting_description: ::prost::alloc::string::String,
    /// Selected option
    #[prost(message, optional, tag = "3")]
    pub option: ::core::option::Option<Option>,
    /// If option is given as a range. This field is meant to be read from the drone, ignore it when setting.
    #[prost(bool, tag = "4")]
    pub is_range: bool,
}
/// Type to represent a setting with a list of options to choose from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingOptions {
    /// Name of the setting (machine readable)
    #[prost(string, tag = "1")]
    pub setting_id: ::prost::alloc::string::String,
    /// Description of the setting (human readable)
    #[prost(string, tag = "2")]
    pub setting_description: ::prost::alloc::string::String,
    /// List of options or if range [min, max] or [min, max, interval]
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    /// If option is given as a range
    #[prost(bool, tag = "4")]
    pub is_range: bool,
}
/// Type to represent a camera information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Information {
    /// Name of the camera vendor
    #[prost(string, tag = "1")]
    pub vendor_name: ::prost::alloc::string::String,
    /// Name of the camera model
    #[prost(string, tag = "2")]
    pub model_name: ::prost::alloc::string::String,
}
/// Camera mode type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Mode {
    /// Unknown
    Unknown = 0,
    /// Photo mode
    Photo = 1,
    /// Video mode
    Video = 2,
}
#[doc = r" Generated client implementations."]
pub mod camera_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Can be used to manage cameras that implement the MAVLink"]
    #[doc = " Camera Protocol: https://mavlink.io/en/protocol/camera.html."]
    #[doc = ""]
    #[doc = " Currently only a single camera is supported."]
    #[doc = " When multiple cameras are supported the plugin will need to be"]
    #[doc = " instantiated separately for every camera and the camera selected using"]
    #[doc = " `select_camera`."]
    #[derive(Debug, Clone)]
    pub struct CameraServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CameraServiceClient<tonic::transport::Channel> {
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
    impl<T> CameraServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
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
        ) -> CameraServiceClient<InterceptedService<T, F>>
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
            CameraServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Take one photo."]
        pub async fn take_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::TakePhotoRequest>,
        ) -> Result<tonic::Response<super::TakePhotoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/TakePhoto");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start photo timelapse with a given interval."]
        pub async fn start_photo_interval(
            &mut self,
            request: impl tonic::IntoRequest<super::StartPhotoIntervalRequest>,
        ) -> Result<tonic::Response<super::StartPhotoIntervalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StartPhotoInterval",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop a running photo timelapse."]
        pub async fn stop_photo_interval(
            &mut self,
            request: impl tonic::IntoRequest<super::StopPhotoIntervalRequest>,
        ) -> Result<tonic::Response<super::StopPhotoIntervalResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StopPhotoInterval",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start a video recording."]
        pub async fn start_video(
            &mut self,
            request: impl tonic::IntoRequest<super::StartVideoRequest>,
        ) -> Result<tonic::Response<super::StartVideoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/StartVideo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop a running video recording."]
        pub async fn stop_video(
            &mut self,
            request: impl tonic::IntoRequest<super::StopVideoRequest>,
        ) -> Result<tonic::Response<super::StopVideoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/StopVideo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start video streaming."]
        pub async fn start_video_streaming(
            &mut self,
            request: impl tonic::IntoRequest<super::StartVideoStreamingRequest>,
        ) -> Result<tonic::Response<super::StartVideoStreamingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StartVideoStreaming",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Stop current video streaming."]
        pub async fn stop_video_streaming(
            &mut self,
            request: impl tonic::IntoRequest<super::StopVideoStreamingRequest>,
        ) -> Result<tonic::Response<super::StopVideoStreamingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/StopVideoStreaming",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set camera mode."]
        pub async fn set_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/SetMode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Subscribe to camera mode updates."]
        pub async fn subscribe_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeModeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ModeResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/SubscribeMode",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to camera information updates."]
        pub async fn subscribe_information(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeInformationRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::InformationResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeInformation",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to video stream info updates."]
        pub async fn subscribe_video_stream_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeVideoStreamInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VideoStreamInfoResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeVideoStreamInfo",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to capture info updates."]
        pub async fn subscribe_capture_info(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCaptureInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CaptureInfoResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCaptureInfo",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Subscribe to camera status updates."]
        pub async fn subscribe_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeStatusRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::StatusResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/SubscribeStatus",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Get the list of current camera settings."]
        pub async fn subscribe_current_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCurrentSettingsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CurrentSettingsResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCurrentSettings",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Get the list of settings that can be changed."]
        pub async fn subscribe_possible_setting_options(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribePossibleSettingOptionsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PossibleSettingOptionsResponse>>,
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
                "/mavsdk.rpc.camera.CameraService/SubscribePossibleSettingOptions",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Set a setting to some value."]
        #[doc = ""]
        #[doc = " Only setting_id of setting and option_id of option needs to be set."]
        pub async fn set_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::SetSettingRequest>,
        ) -> Result<tonic::Response<super::SetSettingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/SetSetting");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Get a setting."]
        #[doc = ""]
        #[doc = " Only setting_id of setting needs to be set."]
        pub async fn get_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingRequest>,
        ) -> Result<tonic::Response<super::GetSettingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/mavsdk.rpc.camera.CameraService/GetSetting");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Format storage (e.g. SD card) in camera."]
        #[doc = ""]
        #[doc = " This will delete all content of the camera storage!"]
        pub async fn format_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::FormatStorageRequest>,
        ) -> Result<tonic::Response<super::FormatStorageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.camera.CameraService/FormatStorage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod camera_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CameraServiceServer."]
    #[async_trait]
    pub trait CameraService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Take one photo."]
        async fn take_photo(
            &self,
            request: tonic::Request<super::TakePhotoRequest>,
        ) -> Result<tonic::Response<super::TakePhotoResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Start photo timelapse with a given interval."]
        async fn start_photo_interval(
            &self,
            request: tonic::Request<super::StartPhotoIntervalRequest>,
        ) -> Result<tonic::Response<super::StartPhotoIntervalResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Stop a running photo timelapse."]
        async fn stop_photo_interval(
            &self,
            request: tonic::Request<super::StopPhotoIntervalRequest>,
        ) -> Result<tonic::Response<super::StopPhotoIntervalResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Start a video recording."]
        async fn start_video(
            &self,
            request: tonic::Request<super::StartVideoRequest>,
        ) -> Result<tonic::Response<super::StartVideoResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Stop a running video recording."]
        async fn stop_video(
            &self,
            request: tonic::Request<super::StopVideoRequest>,
        ) -> Result<tonic::Response<super::StopVideoResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Start video streaming."]
        async fn start_video_streaming(
            &self,
            request: tonic::Request<super::StartVideoStreamingRequest>,
        ) -> Result<tonic::Response<super::StartVideoStreamingResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Stop current video streaming."]
        async fn stop_video_streaming(
            &self,
            request: tonic::Request<super::StopVideoStreamingRequest>,
        ) -> Result<tonic::Response<super::StopVideoStreamingResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set camera mode."]
        async fn set_mode(
            &self,
            request: tonic::Request<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeMode method."]
        type SubscribeModeStream: futures_core::Stream<Item = Result<super::ModeResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to camera mode updates."]
        async fn subscribe_mode(
            &self,
            request: tonic::Request<super::SubscribeModeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeModeStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeInformation method."]
        type SubscribeInformationStream: futures_core::Stream<Item = Result<super::InformationResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to camera information updates."]
        async fn subscribe_information(
            &self,
            request: tonic::Request<super::SubscribeInformationRequest>,
        ) -> Result<tonic::Response<Self::SubscribeInformationStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeVideoStreamInfo method."]
        type SubscribeVideoStreamInfoStream: futures_core::Stream<Item = Result<super::VideoStreamInfoResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to video stream info updates."]
        async fn subscribe_video_stream_info(
            &self,
            request: tonic::Request<super::SubscribeVideoStreamInfoRequest>,
        ) -> Result<tonic::Response<Self::SubscribeVideoStreamInfoStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCaptureInfo method."]
        type SubscribeCaptureInfoStream: futures_core::Stream<Item = Result<super::CaptureInfoResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to capture info updates."]
        async fn subscribe_capture_info(
            &self,
            request: tonic::Request<super::SubscribeCaptureInfoRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCaptureInfoStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeStatus method."]
        type SubscribeStatusStream: futures_core::Stream<Item = Result<super::StatusResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to camera status updates."]
        async fn subscribe_status(
            &self,
            request: tonic::Request<super::SubscribeStatusRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStatusStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeCurrentSettings method."]
        type SubscribeCurrentSettingsStream: futures_core::Stream<Item = Result<super::CurrentSettingsResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Get the list of current camera settings."]
        async fn subscribe_current_settings(
            &self,
            request: tonic::Request<super::SubscribeCurrentSettingsRequest>,
        ) -> Result<tonic::Response<Self::SubscribeCurrentSettingsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribePossibleSettingOptions method."]
        type SubscribePossibleSettingOptionsStream: futures_core::Stream<
                Item = Result<super::PossibleSettingOptionsResponse, tonic::Status>,
            > + Send
            + Sync
            + 'static;
        #[doc = ""]
        #[doc = " Get the list of settings that can be changed."]
        async fn subscribe_possible_setting_options(
            &self,
            request: tonic::Request<super::SubscribePossibleSettingOptionsRequest>,
        ) -> Result<tonic::Response<Self::SubscribePossibleSettingOptionsStream>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set a setting to some value."]
        #[doc = ""]
        #[doc = " Only setting_id of setting and option_id of option needs to be set."]
        async fn set_setting(
            &self,
            request: tonic::Request<super::SetSettingRequest>,
        ) -> Result<tonic::Response<super::SetSettingResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get a setting."]
        #[doc = ""]
        #[doc = " Only setting_id of setting needs to be set."]
        async fn get_setting(
            &self,
            request: tonic::Request<super::GetSettingRequest>,
        ) -> Result<tonic::Response<super::GetSettingResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Format storage (e.g. SD card) in camera."]
        #[doc = ""]
        #[doc = " This will delete all content of the camera storage!"]
        async fn format_storage(
            &self,
            request: tonic::Request<super::FormatStorageRequest>,
        ) -> Result<tonic::Response<super::FormatStorageResponse>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = " Can be used to manage cameras that implement the MAVLink"]
    #[doc = " Camera Protocol: https://mavlink.io/en/protocol/camera.html."]
    #[doc = ""]
    #[doc = " Currently only a single camera is supported."]
    #[doc = " When multiple cameras are supported the plugin will need to be"]
    #[doc = " instantiated separately for every camera and the camera selected using"]
    #[doc = " `select_camera`."]
    #[derive(Debug)]
    pub struct CameraServiceServer<T: CameraService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CameraService> CameraServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CameraServiceServer<T>
    where
        T: CameraService,
        B: Body + Send + Sync + 'static,
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
                "/mavsdk.rpc.camera.CameraService/TakePhoto" => {
                    #[allow(non_camel_case_types)]
                    struct TakePhotoSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::TakePhotoRequest> for TakePhotoSvc<T> {
                        type Response = super::TakePhotoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TakePhotoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).take_photo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TakePhotoSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/StartPhotoInterval" => {
                    #[allow(non_camel_case_types)]
                    struct StartPhotoIntervalSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::UnaryService<super::StartPhotoIntervalRequest>
                        for StartPhotoIntervalSvc<T>
                    {
                        type Response = super::StartPhotoIntervalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartPhotoIntervalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_photo_interval(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartPhotoIntervalSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/StopPhotoInterval" => {
                    #[allow(non_camel_case_types)]
                    struct StopPhotoIntervalSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::UnaryService<super::StopPhotoIntervalRequest>
                        for StopPhotoIntervalSvc<T>
                    {
                        type Response = super::StopPhotoIntervalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopPhotoIntervalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_photo_interval(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopPhotoIntervalSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/StartVideo" => {
                    #[allow(non_camel_case_types)]
                    struct StartVideoSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::StartVideoRequest> for StartVideoSvc<T> {
                        type Response = super::StartVideoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartVideoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_video(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartVideoSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/StopVideo" => {
                    #[allow(non_camel_case_types)]
                    struct StopVideoSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::StopVideoRequest> for StopVideoSvc<T> {
                        type Response = super::StopVideoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopVideoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_video(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopVideoSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/StartVideoStreaming" => {
                    #[allow(non_camel_case_types)]
                    struct StartVideoStreamingSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::UnaryService<super::StartVideoStreamingRequest>
                        for StartVideoStreamingSvc<T>
                    {
                        type Response = super::StartVideoStreamingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartVideoStreamingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_video_streaming(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartVideoStreamingSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/StopVideoStreaming" => {
                    #[allow(non_camel_case_types)]
                    struct StopVideoStreamingSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::UnaryService<super::StopVideoStreamingRequest>
                        for StopVideoStreamingSvc<T>
                    {
                        type Response = super::StopVideoStreamingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopVideoStreamingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_video_streaming(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopVideoStreamingSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SetMode" => {
                    #[allow(non_camel_case_types)]
                    struct SetModeSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::SetModeRequest> for SetModeSvc<T> {
                        type Response = super::SetModeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetModeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_mode(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetModeSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribeMode" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeModeSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<super::SubscribeModeRequest>
                        for SubscribeModeSvc<T>
                    {
                        type Response = super::ModeResponse;
                        type ResponseStream = T::SubscribeModeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeModeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_mode(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeModeSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribeInformation" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeInformationSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<super::SubscribeInformationRequest>
                        for SubscribeInformationSvc<T>
                    {
                        type Response = super::InformationResponse;
                        type ResponseStream = T::SubscribeInformationStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeInformationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_information(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeInformationSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribeVideoStreamInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeVideoStreamInfoSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeVideoStreamInfoRequest,
                        > for SubscribeVideoStreamInfoSvc<T>
                    {
                        type Response = super::VideoStreamInfoResponse;
                        type ResponseStream = T::SubscribeVideoStreamInfoStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeVideoStreamInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_video_stream_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeVideoStreamInfoSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCaptureInfo" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCaptureInfoSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<super::SubscribeCaptureInfoRequest>
                        for SubscribeCaptureInfoSvc<T>
                    {
                        type Response = super::CaptureInfoResponse;
                        type ResponseStream = T::SubscribeCaptureInfoStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeCaptureInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_capture_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCaptureInfoSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribeStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeStatusSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<super::SubscribeStatusRequest>
                        for SubscribeStatusSvc<T>
                    {
                        type Response = super::StatusResponse;
                        type ResponseStream = T::SubscribeStatusStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeStatusSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribeCurrentSettings" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeCurrentSettingsSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeCurrentSettingsRequest,
                        > for SubscribeCurrentSettingsSvc<T>
                    {
                        type Response = super::CurrentSettingsResponse;
                        type ResponseStream = T::SubscribeCurrentSettingsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeCurrentSettingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_current_settings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeCurrentSettingsSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SubscribePossibleSettingOptions" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribePossibleSettingOptionsSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService>
                        tonic::server::ServerStreamingService<
                            super::SubscribePossibleSettingOptionsRequest,
                        > for SubscribePossibleSettingOptionsSvc<T>
                    {
                        type Response = super::PossibleSettingOptionsResponse;
                        type ResponseStream = T::SubscribePossibleSettingOptionsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribePossibleSettingOptionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).subscribe_possible_setting_options(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribePossibleSettingOptionsSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/SetSetting" => {
                    #[allow(non_camel_case_types)]
                    struct SetSettingSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::SetSettingRequest> for SetSettingSvc<T> {
                        type Response = super::SetSettingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetSettingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_setting(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetSettingSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/GetSetting" => {
                    #[allow(non_camel_case_types)]
                    struct GetSettingSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::GetSettingRequest> for GetSettingSvc<T> {
                        type Response = super::GetSettingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSettingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_setting(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSettingSvc(inner);
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
                "/mavsdk.rpc.camera.CameraService/FormatStorage" => {
                    #[allow(non_camel_case_types)]
                    struct FormatStorageSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::FormatStorageRequest>
                        for FormatStorageSvc<T>
                    {
                        type Response = super::FormatStorageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FormatStorageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).format_storage(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FormatStorageSvc(inner);
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
    impl<T: CameraService> Clone for CameraServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CameraService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CameraService> tonic::transport::NamedService for CameraServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.camera.CameraService";
    }
}
