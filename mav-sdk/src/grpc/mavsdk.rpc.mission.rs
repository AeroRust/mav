#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionRequest {
    /// The mission plan
    #[prost(message, optional, tag = "1")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeUploadMissionWithProgressRequest {
    /// The mission plan
    #[prost(message, optional, tag = "1")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionWithProgressResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// The progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// The mission plan
    #[prost(message, optional, tag = "2")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDownloadMissionWithProgressRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionWithProgressResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// The progress data, or the mission plan (when the download is finished)
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressDataOrMission>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentMissionItemRequest {
    /// Index of the mission item to be set as the next one (0-based)
    #[prost(int32, tag = "1")]
    pub index: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentMissionItemResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsMissionFinishedRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsMissionFinishedResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// True if the mission is finished and the last mission item has been reached
    #[prost(bool, tag = "2")]
    pub is_finished: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SubscribeMissionProgressRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionProgressResponse {
    /// Mission progress
    #[prost(message, optional, tag = "1")]
    pub mission_progress: ::core::option::Option<MissionProgress>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAfterMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAfterMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// If true, trigger an RTL at the end of the mission
    #[prost(bool, tag = "2")]
    pub enable: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAfterMissionRequest {
    /// If true, trigger an RTL at the end of the mission
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAfterMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
///
/// Type representing a mission item.
///
/// A MissionItem can contain a position and/or actions.
/// Mission items are building blocks to assemble a mission,
/// which can be sent to (or received from) a system.
/// They cannot be used independently.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionItem {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag = "3")]
    pub relative_altitude_m: f32,
    /// Speed to use after this mission item (in metres/second)
    #[prost(float, tag = "4")]
    pub speed_m_s: f32,
    /// True will make the drone fly through without stopping, while false will make the drone stop on the waypoint
    #[prost(bool, tag = "5")]
    pub is_fly_through: bool,
    /// Gimbal pitch (in degrees)
    #[prost(float, tag = "6")]
    pub gimbal_pitch_deg: f32,
    /// Gimbal yaw (in degrees)
    #[prost(float, tag = "7")]
    pub gimbal_yaw_deg: f32,
    /// Camera action to trigger at this mission item
    #[prost(enumeration = "mission_item::CameraAction", tag = "8")]
    pub camera_action: i32,
    /// Loiter time (in seconds)
    #[prost(float, tag = "9")]
    pub loiter_time_s: f32,
    /// Camera photo interval to use after this mission item (in seconds)
    #[prost(double, tag = "10")]
    pub camera_photo_interval_s: f64,
    /// Radius for completing a mission item (in metres)
    #[prost(float, tag = "11")]
    pub acceptance_radius_m: f32,
    /// Absolute yaw angle (in degrees)
    #[prost(float, tag = "12")]
    pub yaw_deg: f32,
    /// Camera photo distance to use after this mission item (in meters)
    #[prost(float, tag = "13")]
    pub camera_photo_distance_m: f32,
}
/// Nested message and enum types in `MissionItem`.
pub mod mission_item {
    /// Possible camera actions at a mission item.
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
    pub enum CameraAction {
        /// No action
        None = 0,
        /// Take a single photo
        TakePhoto = 1,
        /// Start capturing photos at regular intervals
        StartPhotoInterval = 2,
        /// Stop capturing photos at regular intervals
        StopPhotoInterval = 3,
        /// Start capturing video
        StartVideo = 4,
        /// Stop capturing video
        StopVideo = 5,
        /// Start capturing photos at regular distance
        StartPhotoDistance = 6,
        /// Stop capturing photos at regular distance
        StopPhotoDistance = 7,
    }
}
/// Mission plan type
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionPlan {
    /// The mission items
    #[prost(message, repeated, tag = "1")]
    pub mission_items: ::prost::alloc::vec::Vec<MissionItem>,
}
/// Mission progress type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionProgress {
    /// Current mission item index (0-based), if equal to total, the mission is finished
    #[prost(int32, tag = "1")]
    pub current: i32,
    /// Total number of mission items
    #[prost(int32, tag = "2")]
    pub total: i32,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionResult {
    /// Result enum value
    #[prost(enumeration = "mission_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MissionResult`.
pub mod mission_result {
    /// Possible results returned for action requests.
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
        /// Error
        Error = 2,
        /// Too many mission items in the mission
        TooManyMissionItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
        /// Mission downloaded from the system is not supported
        Unsupported = 7,
        /// No mission available on the system
        NoMissionAvailable = 8,
        /// Unsupported mission command
        UnsupportedMissionCmd = 11,
        /// Mission transfer (upload or download) has been cancelled
        TransferCancelled = 12,
        /// No system connected
        NoSystem = 13,
        /// Intermediate message showing progress
        Next = 14,
    }
}
/// Progress data coming from mission upload.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProgressData {
    /// Progress (0..1.0)
    #[prost(float, tag = "1")]
    pub progress: f32,
}
/// Progress data coming from mission download, or the mission itself (if the transfer succeeds).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressDataOrMission {
    /// Whether this ProgressData contains a 'progress' status or not
    #[prost(bool, tag = "1")]
    pub has_progress: bool,
    /// Progress (0..1.0)
    #[prost(float, tag = "2")]
    pub progress: f32,
    /// Whether this ProgressData contains a 'mission_plan' or not
    #[prost(bool, tag = "3")]
    pub has_mission: bool,
    /// Mission plan
    #[prost(message, optional, tag = "4")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[doc = r" Generated client implementations."]
pub mod mission_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable waypoint missions."]
    #[derive(Debug, Clone)]
    pub struct MissionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MissionServiceClient<tonic::transport::Channel> {
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
    impl<T> MissionServiceClient<T>
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
        ) -> MissionServiceClient<InterceptedService<T, F>>
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
            MissionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Upload a list of mission items to the system."]
        #[doc = ""]
        #[doc = " The mission items are uploaded to a drone. Once uploaded the mission can be started and"]
        #[doc = " executed even if the connection is lost."]
        pub async fn upload_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadMissionRequest>,
        ) -> Result<tonic::Response<super::UploadMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/UploadMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Upload a list of mission items to the system and report upload progress."]
        #[doc = ""]
        #[doc = " The mission items are uploaded to a drone. Once uploaded the mission can be started and"]
        #[doc = " executed even if the connection is lost."]
        pub async fn subscribe_upload_mission_with_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeUploadMissionWithProgressRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::UploadMissionWithProgressResponse>>,
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
                "/mavsdk.rpc.mission.MissionService/SubscribeUploadMissionWithProgress",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Cancel an ongoing mission upload."]
        pub async fn cancel_mission_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMissionUploadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionUploadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/CancelMissionUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Download a list of mission items from the system (asynchronous)."]
        #[doc = ""]
        #[doc = " Will fail if any of the downloaded mission items are not supported"]
        #[doc = " by the MAVSDK API."]
        pub async fn download_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadMissionRequest>,
        ) -> Result<tonic::Response<super::DownloadMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/DownloadMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Download a list of mission items from the system (asynchronous) and report progress."]
        #[doc = ""]
        #[doc = " Will fail if any of the downloaded mission items are not supported"]
        #[doc = " by the MAVSDK API."]
        pub async fn subscribe_download_mission_with_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeDownloadMissionWithProgressRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DownloadMissionWithProgressResponse>>,
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
                "/mavsdk.rpc.mission.MissionService/SubscribeDownloadMissionWithProgress",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Cancel an ongoing mission download."]
        pub async fn cancel_mission_download(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMissionDownloadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionDownloadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/CancelMissionDownload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Start the mission."]
        #[doc = ""]
        #[doc = " A mission must be uploaded to the vehicle before this can be called."]
        pub async fn start_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMissionRequest>,
        ) -> Result<tonic::Response<super::StartMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/StartMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Pause the mission."]
        #[doc = ""]
        #[doc = " Pausing the mission puts the vehicle into"]
        #[doc = " [HOLD mode](https://docs.px4.io/en/flight_modes/hold.html)."]
        #[doc = " A multicopter should just hover at the spot while a fixedwing vehicle should loiter"]
        #[doc = " around the location where it paused."]
        pub async fn pause_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseMissionRequest>,
        ) -> Result<tonic::Response<super::PauseMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/PauseMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Clear the mission saved on the vehicle."]
        pub async fn clear_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearMissionRequest>,
        ) -> Result<tonic::Response<super::ClearMissionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/ClearMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Sets the mission item index to go to."]
        #[doc = ""]
        #[doc = " By setting the current index to 0, the mission is restarted from the beginning. If it is set"]
        #[doc = " to a specific index of a mission item, the mission will be set to this item."]
        #[doc = ""]
        #[doc = " Note that this is not necessarily true for general missions using MAVLink if loop counters"]
        #[doc = " are used."]
        pub async fn set_current_mission_item(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCurrentMissionItemRequest>,
        ) -> Result<tonic::Response<super::SetCurrentMissionItemResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SetCurrentMissionItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Check if the mission has been finished."]
        pub async fn is_mission_finished(
            &mut self,
            request: impl tonic::IntoRequest<super::IsMissionFinishedRequest>,
        ) -> Result<tonic::Response<super::IsMissionFinishedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/IsMissionFinished",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Subscribe to mission progress updates."]
        pub async fn subscribe_mission_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeMissionProgressRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MissionProgressResponse>>,
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
                "/mavsdk.rpc.mission.MissionService/SubscribeMissionProgress",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Get whether to trigger Return-to-Launch (RTL) after mission is complete."]
        #[doc = ""]
        #[doc = " Before getting this option, it needs to be set, or a mission"]
        #[doc = " needs to be downloaded."]
        pub async fn get_return_to_launch_after_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReturnToLaunchAfterMissionRequest>,
        ) -> Result<tonic::Response<super::GetReturnToLaunchAfterMissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/GetReturnToLaunchAfterMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Set whether to trigger Return-to-Launch (RTL) after the mission is complete."]
        #[doc = ""]
        #[doc = " This will only take effect for the next mission upload, meaning that"]
        #[doc = " the mission may have to be uploaded again."]
        pub async fn set_return_to_launch_after_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReturnToLaunchAfterMissionRequest>,
        ) -> Result<tonic::Response<super::SetReturnToLaunchAfterMissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SetReturnToLaunchAfterMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod mission_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MissionServiceServer."]
    #[async_trait]
    pub trait MissionService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Upload a list of mission items to the system."]
        #[doc = ""]
        #[doc = " The mission items are uploaded to a drone. Once uploaded the mission can be started and"]
        #[doc = " executed even if the connection is lost."]
        async fn upload_mission(
            &self,
            request: tonic::Request<super::UploadMissionRequest>,
        ) -> Result<tonic::Response<super::UploadMissionResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeUploadMissionWithProgress method."]
        type SubscribeUploadMissionWithProgressStream: futures_core::Stream<
                Item = Result<super::UploadMissionWithProgressResponse, tonic::Status>,
            > + Send
            + 'static;
        #[doc = ""]
        #[doc = " Upload a list of mission items to the system and report upload progress."]
        #[doc = ""]
        #[doc = " The mission items are uploaded to a drone. Once uploaded the mission can be started and"]
        #[doc = " executed even if the connection is lost."]
        async fn subscribe_upload_mission_with_progress(
            &self,
            request: tonic::Request<super::SubscribeUploadMissionWithProgressRequest>,
        ) -> Result<tonic::Response<Self::SubscribeUploadMissionWithProgressStream>, tonic::Status>;
        #[doc = ""]
        #[doc = " Cancel an ongoing mission upload."]
        async fn cancel_mission_upload(
            &self,
            request: tonic::Request<super::CancelMissionUploadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionUploadResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Download a list of mission items from the system (asynchronous)."]
        #[doc = ""]
        #[doc = " Will fail if any of the downloaded mission items are not supported"]
        #[doc = " by the MAVSDK API."]
        async fn download_mission(
            &self,
            request: tonic::Request<super::DownloadMissionRequest>,
        ) -> Result<tonic::Response<super::DownloadMissionResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeDownloadMissionWithProgress method."]
        type SubscribeDownloadMissionWithProgressStream: futures_core::Stream<
                Item = Result<super::DownloadMissionWithProgressResponse, tonic::Status>,
            > + Send
            + 'static;
        #[doc = ""]
        #[doc = " Download a list of mission items from the system (asynchronous) and report progress."]
        #[doc = ""]
        #[doc = " Will fail if any of the downloaded mission items are not supported"]
        #[doc = " by the MAVSDK API."]
        async fn subscribe_download_mission_with_progress(
            &self,
            request: tonic::Request<super::SubscribeDownloadMissionWithProgressRequest>,
        ) -> Result<tonic::Response<Self::SubscribeDownloadMissionWithProgressStream>, tonic::Status>;
        #[doc = ""]
        #[doc = " Cancel an ongoing mission download."]
        async fn cancel_mission_download(
            &self,
            request: tonic::Request<super::CancelMissionDownloadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionDownloadResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Start the mission."]
        #[doc = ""]
        #[doc = " A mission must be uploaded to the vehicle before this can be called."]
        async fn start_mission(
            &self,
            request: tonic::Request<super::StartMissionRequest>,
        ) -> Result<tonic::Response<super::StartMissionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Pause the mission."]
        #[doc = ""]
        #[doc = " Pausing the mission puts the vehicle into"]
        #[doc = " [HOLD mode](https://docs.px4.io/en/flight_modes/hold.html)."]
        #[doc = " A multicopter should just hover at the spot while a fixedwing vehicle should loiter"]
        #[doc = " around the location where it paused."]
        async fn pause_mission(
            &self,
            request: tonic::Request<super::PauseMissionRequest>,
        ) -> Result<tonic::Response<super::PauseMissionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Clear the mission saved on the vehicle."]
        async fn clear_mission(
            &self,
            request: tonic::Request<super::ClearMissionRequest>,
        ) -> Result<tonic::Response<super::ClearMissionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Sets the mission item index to go to."]
        #[doc = ""]
        #[doc = " By setting the current index to 0, the mission is restarted from the beginning. If it is set"]
        #[doc = " to a specific index of a mission item, the mission will be set to this item."]
        #[doc = ""]
        #[doc = " Note that this is not necessarily true for general missions using MAVLink if loop counters"]
        #[doc = " are used."]
        async fn set_current_mission_item(
            &self,
            request: tonic::Request<super::SetCurrentMissionItemRequest>,
        ) -> Result<tonic::Response<super::SetCurrentMissionItemResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Check if the mission has been finished."]
        async fn is_mission_finished(
            &self,
            request: tonic::Request<super::IsMissionFinishedRequest>,
        ) -> Result<tonic::Response<super::IsMissionFinishedResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the SubscribeMissionProgress method."]
        type SubscribeMissionProgressStream: futures_core::Stream<Item = Result<super::MissionProgressResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = ""]
        #[doc = " Subscribe to mission progress updates."]
        async fn subscribe_mission_progress(
            &self,
            request: tonic::Request<super::SubscribeMissionProgressRequest>,
        ) -> Result<tonic::Response<Self::SubscribeMissionProgressStream>, tonic::Status>;
        #[doc = ""]
        #[doc = " Get whether to trigger Return-to-Launch (RTL) after mission is complete."]
        #[doc = ""]
        #[doc = " Before getting this option, it needs to be set, or a mission"]
        #[doc = " needs to be downloaded."]
        async fn get_return_to_launch_after_mission(
            &self,
            request: tonic::Request<super::GetReturnToLaunchAfterMissionRequest>,
        ) -> Result<tonic::Response<super::GetReturnToLaunchAfterMissionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Set whether to trigger Return-to-Launch (RTL) after the mission is complete."]
        #[doc = ""]
        #[doc = " This will only take effect for the next mission upload, meaning that"]
        #[doc = " the mission may have to be uploaded again."]
        async fn set_return_to_launch_after_mission(
            &self,
            request: tonic::Request<super::SetReturnToLaunchAfterMissionRequest>,
        ) -> Result<tonic::Response<super::SetReturnToLaunchAfterMissionResponse>, tonic::Status>;
    }
    #[doc = " Enable waypoint missions."]
    #[derive(Debug)]
    pub struct MissionServiceServer<T: MissionService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MissionService> MissionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MissionServiceServer<T>
    where
        T: MissionService,
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
                "/mavsdk.rpc.mission.MissionService/UploadMission" => {
                    #[allow(non_camel_case_types)]
                    struct UploadMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService> tonic::server::UnaryService<super::UploadMissionRequest>
                        for UploadMissionSvc<T>
                    {
                        type Response = super::UploadMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upload_mission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadMissionSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/SubscribeUploadMissionWithProgress" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeUploadMissionWithProgressSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeUploadMissionWithProgressRequest,
                        > for SubscribeUploadMissionWithProgressSvc<T>
                    {
                        type Response = super::UploadMissionWithProgressResponse;
                        type ResponseStream = T::SubscribeUploadMissionWithProgressStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeUploadMissionWithProgressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .subscribe_upload_mission_with_progress(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeUploadMissionWithProgressSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/CancelMissionUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CancelMissionUploadSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::CancelMissionUploadRequest>
                        for CancelMissionUploadSvc<T>
                    {
                        type Response = super::CancelMissionUploadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelMissionUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_mission_upload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelMissionUploadSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/DownloadMission" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::DownloadMissionRequest>
                        for DownloadMissionSvc<T>
                    {
                        type Response = super::DownloadMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).download_mission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DownloadMissionSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/SubscribeDownloadMissionWithProgress" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeDownloadMissionWithProgressSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeDownloadMissionWithProgressRequest,
                        > for SubscribeDownloadMissionWithProgressSvc<T>
                    {
                        type Response = super::DownloadMissionWithProgressResponse;
                        type ResponseStream = T::SubscribeDownloadMissionWithProgressStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubscribeDownloadMissionWithProgressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .subscribe_download_mission_with_progress(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeDownloadMissionWithProgressSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/CancelMissionDownload" => {
                    #[allow(non_camel_case_types)]
                    struct CancelMissionDownloadSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::CancelMissionDownloadRequest>
                        for CancelMissionDownloadSvc<T>
                    {
                        type Response = super::CancelMissionDownloadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelMissionDownloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).cancel_mission_download(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelMissionDownloadSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/StartMission" => {
                    #[allow(non_camel_case_types)]
                    struct StartMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService> tonic::server::UnaryService<super::StartMissionRequest>
                        for StartMissionSvc<T>
                    {
                        type Response = super::StartMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_mission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartMissionSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/PauseMission" => {
                    #[allow(non_camel_case_types)]
                    struct PauseMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService> tonic::server::UnaryService<super::PauseMissionRequest>
                        for PauseMissionSvc<T>
                    {
                        type Response = super::PauseMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PauseMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pause_mission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PauseMissionSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/ClearMission" => {
                    #[allow(non_camel_case_types)]
                    struct ClearMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService> tonic::server::UnaryService<super::ClearMissionRequest>
                        for ClearMissionSvc<T>
                    {
                        type Response = super::ClearMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).clear_mission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearMissionSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/SetCurrentMissionItem" => {
                    #[allow(non_camel_case_types)]
                    struct SetCurrentMissionItemSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::SetCurrentMissionItemRequest>
                        for SetCurrentMissionItemSvc<T>
                    {
                        type Response = super::SetCurrentMissionItemResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetCurrentMissionItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_current_mission_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetCurrentMissionItemSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/IsMissionFinished" => {
                    #[allow(non_camel_case_types)]
                    struct IsMissionFinishedSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::IsMissionFinishedRequest>
                        for IsMissionFinishedSvc<T>
                    {
                        type Response = super::IsMissionFinishedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsMissionFinishedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_mission_finished(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsMissionFinishedSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/SubscribeMissionProgress" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeMissionProgressSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::ServerStreamingService<
                            super::SubscribeMissionProgressRequest,
                        > for SubscribeMissionProgressSvc<T>
                    {
                        type Response = super::MissionProgressResponse;
                        type ResponseStream = T::SubscribeMissionProgressStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeMissionProgressRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_mission_progress(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeMissionProgressSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/GetReturnToLaunchAfterMission" => {
                    #[allow(non_camel_case_types)]
                    struct GetReturnToLaunchAfterMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::GetReturnToLaunchAfterMissionRequest>
                        for GetReturnToLaunchAfterMissionSvc<T>
                    {
                        type Response = super::GetReturnToLaunchAfterMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReturnToLaunchAfterMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_return_to_launch_after_mission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReturnToLaunchAfterMissionSvc(inner);
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
                "/mavsdk.rpc.mission.MissionService/SetReturnToLaunchAfterMission" => {
                    #[allow(non_camel_case_types)]
                    struct SetReturnToLaunchAfterMissionSvc<T: MissionService>(pub Arc<T>);
                    impl<T: MissionService>
                        tonic::server::UnaryService<super::SetReturnToLaunchAfterMissionRequest>
                        for SetReturnToLaunchAfterMissionSvc<T>
                    {
                        type Response = super::SetReturnToLaunchAfterMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetReturnToLaunchAfterMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_return_to_launch_after_mission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetReturnToLaunchAfterMissionSvc(inner);
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
    impl<T: MissionService> Clone for MissionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MissionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MissionService> tonic::transport::NamedService for MissionServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.mission.MissionService";
    }
}
