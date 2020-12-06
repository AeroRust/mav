/// Generated services
pub mod grpc;

pub type Result<T> = std::result::Result<T, tonic::transport::Error>;

use self::grpc::{
    action::action_service_client::ActionServiceClient,
    info::info_service_client::InfoServiceClient, mocap::mocap_service_client::MocapServiceClient,
    telemetry::telemetry_service_client::TelemetryServiceClient,
};

use tonic::transport::Channel;

pub const DEFAULT_URL: &str = "http://0.0.0.0:14540";
pub const OFFBOARD_PORT: u16 = 14540;

/// pub mod service;
/// use grpc::info::info_service_server::InfoServiceServer;
// pub struct Server {
//     info: InfoServiceServer<service::Info>,
// }

#[derive(Debug)]
pub struct Drone {
    pub info: InfoServiceClient<Channel>,
    pub telemetry: TelemetryServiceClient<Channel>,
    pub mocap: MocapServiceClient<Channel>,
    pub action: ActionServiceClient<Channel>,
}

impl Drone {
    pub async fn connect<'a>(url: &'static str) -> Result<Self> {
        Ok(Self {
            info: InfoServiceClient::connect(url).await?,
            telemetry: TelemetryServiceClient::connect(url).await?,
            mocap: MocapServiceClient::connect(url).await?,
            action: ActionServiceClient::connect(url).await?,
        })
    }
}
