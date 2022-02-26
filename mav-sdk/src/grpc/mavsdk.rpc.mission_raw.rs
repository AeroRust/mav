#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionRequest {
    /// The mission items
    #[prost(message, repeated, tag = "1")]
    pub mission_items: ::prost::alloc::vec::Vec<MissionItem>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
    /// The mission items
    #[prost(message, repeated, tag = "2")]
    pub mission_items: ::prost::alloc::vec::Vec<MissionItem>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StartMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
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
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
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
pub struct SubscribeMissionChangedRequest {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionChangedResponse {
    /// Mission has changed
    #[prost(bool, tag = "1")]
    pub mission_changed: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ImportQgroundcontrolMissionRequest {
    /// File path of the QGC plan
    #[prost(string, tag = "1")]
    pub qgc_plan_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ImportQgroundcontrolMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_raw_result: ::core::option::Option<MissionRawResult>,
    /// The imported mission data
    #[prost(message, optional, tag = "2")]
    pub mission_import_data: ::core::option::Option<MissionImportData>,
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
/// Mission item exactly identical to MAVLink MISSION_ITEM_INT.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionItem {
    /// Sequence (uint16_t)
    #[prost(uint32, tag = "1")]
    pub seq: u32,
    /// The coordinate system of the waypoint (actually uint8_t)
    #[prost(uint32, tag = "2")]
    pub frame: u32,
    /// The scheduled action for the waypoint (actually uint16_t)
    #[prost(uint32, tag = "3")]
    pub command: u32,
    /// false:0, true:1 (actually uint8_t)
    #[prost(uint32, tag = "4")]
    pub current: u32,
    /// Autocontinue to next waypoint (actually uint8_t)
    #[prost(uint32, tag = "5")]
    pub autocontinue: u32,
    /// PARAM1, see MAV_CMD enum
    #[prost(float, tag = "6")]
    pub param1: f32,
    /// PARAM2, see MAV_CMD enum
    #[prost(float, tag = "7")]
    pub param2: f32,
    /// PARAM3, see MAV_CMD enum
    #[prost(float, tag = "8")]
    pub param3: f32,
    /// PARAM4, see MAV_CMD enum
    #[prost(float, tag = "9")]
    pub param4: f32,
    /// PARAM5 / local: x position in meters * 1e4, global: latitude in degrees * 10^7
    #[prost(int32, tag = "10")]
    pub x: i32,
    /// PARAM6 / y position: local: x position in meters * 1e4, global: longitude in degrees *10^7
    #[prost(int32, tag = "11")]
    pub y: i32,
    /// PARAM7 / local: Z coordinate, global: altitude (relative or absolute, depending on frame)
    #[prost(float, tag = "12")]
    pub z: f32,
    /// Mission type (actually uint8_t)
    #[prost(uint32, tag = "13")]
    pub mission_type: u32,
}
/// Mission import data
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionImportData {
    /// Mission items
    #[prost(message, repeated, tag = "1")]
    pub mission_items: ::prost::alloc::vec::Vec<MissionItem>,
    /// Geofence items
    #[prost(message, repeated, tag = "2")]
    pub geofence_items: ::prost::alloc::vec::Vec<MissionItem>,
    /// Rally items
    #[prost(message, repeated, tag = "3")]
    pub rally_items: ::prost::alloc::vec::Vec<MissionItem>,
}
/// Result type.
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionRawResult {
    /// Result enum value
    #[prost(enumeration = "mission_raw_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MissionRawResult`.
pub mod mission_raw_result {
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
        /// Mission transfer (upload or download) has been cancelled
        TransferCancelled = 9,
        /// Failed to open the QGroundControl plan
        FailedToOpenQgcPlan = 10,
        /// Failed to parse the QGroundControl plan
        FailedToParseQgcPlan = 11,
        /// No system connected
        NoSystem = 12,
    }
}
#[doc = r" Generated client implementations."]
pub mod mission_raw_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Enable raw missions as exposed by MAVLink."]
    #[derive(Debug, Clone)]
    pub struct MissionRawServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MissionRawServiceClient<tonic::transport::Channel> {
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
    impl<T> MissionRawServiceClient<T>
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
        ) -> MissionRawServiceClient<InterceptedService<T, F>>
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
            MissionRawServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Upload a list of raw mission items to the system."]
        #[doc = ""]
        #[doc = " The raw mission items are uploaded to a drone. Once uploaded the mission"]
        #[doc = " can be started and executed even if the connection is lost."]
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
                "/mavsdk.rpc.mission_raw.MissionRawService/UploadMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
                "/mavsdk.rpc.mission_raw.MissionRawService/CancelMissionUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Download a list of raw mission items from the system (asynchronous)."]
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
                "/mavsdk.rpc.mission_raw.MissionRawService/DownloadMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
                "/mavsdk.rpc.mission_raw.MissionRawService/CancelMissionDownload",
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
                "/mavsdk.rpc.mission_raw.MissionRawService/StartMission",
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
                "/mavsdk.rpc.mission_raw.MissionRawService/PauseMission",
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
                "/mavsdk.rpc.mission_raw.MissionRawService/ClearMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = ""]
        #[doc = " Sets the raw mission item index to go to."]
        #[doc = ""]
        #[doc = " By setting the current index to 0, the mission is restarted from the beginning. If it is set"]
        #[doc = " to a specific index of a raw mission item, the mission will be set to this item."]
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
                "/mavsdk.rpc.mission_raw.MissionRawService/SetCurrentMissionItem",
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
                "/mavsdk.rpc.mission_raw.MissionRawService/SubscribeMissionProgress",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = "*"]
        #[doc = " Subscribes to mission changed."]
        #[doc = ""]
        #[doc = " This notification can be used to be informed if a ground station has"]
        #[doc = " been uploaded or changed by a ground station or companion computer."]
        #[doc = ""]
        #[doc = " @param callback Callback to notify about change."]
        pub async fn subscribe_mission_changed(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeMissionChangedRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MissionChangedResponse>>,
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
                "/mavsdk.rpc.mission_raw.MissionRawService/SubscribeMissionChanged",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = ""]
        #[doc = " Import a QGroundControl missions in JSON .plan format."]
        #[doc = ""]
        #[doc = " Supported:"]
        #[doc = " - Waypoints"]
        #[doc = " - Survey"]
        #[doc = " Not supported:"]
        #[doc = " - Structure Scan"]
        pub async fn import_qgroundcontrol_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportQgroundcontrolMissionRequest>,
        ) -> Result<tonic::Response<super::ImportQgroundcontrolMissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission_raw.MissionRawService/ImportQgroundcontrolMission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod mission_raw_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MissionRawServiceServer."]
    #[async_trait]
    pub trait MissionRawService: Send + Sync + 'static {
        #[doc = ""]
        #[doc = " Upload a list of raw mission items to the system."]
        #[doc = ""]
        #[doc = " The raw mission items are uploaded to a drone. Once uploaded the mission"]
        #[doc = " can be started and executed even if the connection is lost."]
        async fn upload_mission(
            &self,
            request: tonic::Request<super::UploadMissionRequest>,
        ) -> Result<tonic::Response<super::UploadMissionResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Cancel an ongoing mission upload."]
        async fn cancel_mission_upload(
            &self,
            request: tonic::Request<super::CancelMissionUploadRequest>,
        ) -> Result<tonic::Response<super::CancelMissionUploadResponse>, tonic::Status>;
        #[doc = ""]
        #[doc = " Download a list of raw mission items from the system (asynchronous)."]
        async fn download_mission(
            &self,
            request: tonic::Request<super::DownloadMissionRequest>,
        ) -> Result<tonic::Response<super::DownloadMissionResponse>, tonic::Status>;
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
        #[doc = " Sets the raw mission item index to go to."]
        #[doc = ""]
        #[doc = " By setting the current index to 0, the mission is restarted from the beginning. If it is set"]
        #[doc = " to a specific index of a raw mission item, the mission will be set to this item."]
        async fn set_current_mission_item(
            &self,
            request: tonic::Request<super::SetCurrentMissionItemRequest>,
        ) -> Result<tonic::Response<super::SetCurrentMissionItemResponse>, tonic::Status>;
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
        #[doc = "Server streaming response type for the SubscribeMissionChanged method."]
        type SubscribeMissionChangedStream: futures_core::Stream<Item = Result<super::MissionChangedResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = "*"]
        #[doc = " Subscribes to mission changed."]
        #[doc = ""]
        #[doc = " This notification can be used to be informed if a ground station has"]
        #[doc = " been uploaded or changed by a ground station or companion computer."]
        #[doc = ""]
        #[doc = " @param callback Callback to notify about change."]
        async fn subscribe_mission_changed(
            &self,
            request: tonic::Request<super::SubscribeMissionChangedRequest>,
        ) -> Result<tonic::Response<Self::SubscribeMissionChangedStream>, tonic::Status>;
        #[doc = ""]
        #[doc = " Import a QGroundControl missions in JSON .plan format."]
        #[doc = ""]
        #[doc = " Supported:"]
        #[doc = " - Waypoints"]
        #[doc = " - Survey"]
        #[doc = " Not supported:"]
        #[doc = " - Structure Scan"]
        async fn import_qgroundcontrol_mission(
            &self,
            request: tonic::Request<super::ImportQgroundcontrolMissionRequest>,
        ) -> Result<tonic::Response<super::ImportQgroundcontrolMissionResponse>, tonic::Status>;
    }
    #[doc = " Enable raw missions as exposed by MAVLink."]
    #[derive(Debug)]
    pub struct MissionRawServiceServer<T: MissionRawService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MissionRawService> MissionRawServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MissionRawServiceServer<T>
    where
        T: MissionRawService,
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
                "/mavsdk.rpc.mission_raw.MissionRawService/UploadMission" => {
                    #[allow(non_camel_case_types)]
                    struct UploadMissionSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
                        tonic::server::UnaryService<super::UploadMissionRequest>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/CancelMissionUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CancelMissionUploadSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/DownloadMission" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadMissionSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/CancelMissionDownload" => {
                    #[allow(non_camel_case_types)]
                    struct CancelMissionDownloadSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/StartMission" => {
                    #[allow(non_camel_case_types)]
                    struct StartMissionSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
                        tonic::server::UnaryService<super::StartMissionRequest>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/PauseMission" => {
                    #[allow(non_camel_case_types)]
                    struct PauseMissionSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
                        tonic::server::UnaryService<super::PauseMissionRequest>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/ClearMission" => {
                    #[allow(non_camel_case_types)]
                    struct ClearMissionSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
                        tonic::server::UnaryService<super::ClearMissionRequest>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/SetCurrentMissionItem" => {
                    #[allow(non_camel_case_types)]
                    struct SetCurrentMissionItemSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/SubscribeMissionProgress" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeMissionProgressSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
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
                "/mavsdk.rpc.mission_raw.MissionRawService/SubscribeMissionChanged" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeMissionChangedSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
                        tonic::server::ServerStreamingService<super::SubscribeMissionChangedRequest>
                        for SubscribeMissionChangedSvc<T>
                    {
                        type Response = super::MissionChangedResponse;
                        type ResponseStream = T::SubscribeMissionChangedStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeMissionChangedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).subscribe_mission_changed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeMissionChangedSvc(inner);
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
                "/mavsdk.rpc.mission_raw.MissionRawService/ImportQgroundcontrolMission" => {
                    #[allow(non_camel_case_types)]
                    struct ImportQgroundcontrolMissionSvc<T: MissionRawService>(pub Arc<T>);
                    impl<T: MissionRawService>
                        tonic::server::UnaryService<super::ImportQgroundcontrolMissionRequest>
                        for ImportQgroundcontrolMissionSvc<T>
                    {
                        type Response = super::ImportQgroundcontrolMissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportQgroundcontrolMissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).import_qgroundcontrol_mission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ImportQgroundcontrolMissionSvc(inner);
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
    impl<T: MissionRawService> Clone for MissionRawServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MissionRawService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MissionRawService> tonic::transport::NamedService for MissionRawServiceServer<T> {
        const NAME: &'static str = "mavsdk.rpc.mission_raw.MissionRawService";
    }
}
