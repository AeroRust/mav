/// Generated services
pub mod grpc;

pub type Result<T> = std::result::Result<T, tonic::transport::Error>;

use self::grpc::{
    action::ActionServiceClient,
    info::InfoServiceClient, mocap::mocap_service_client::MocapServiceClient,
    telemetry::TelemetryServiceClient,
};

use tonic::transport::{Channel, Endpoint};

pub const DEFAULT_URL: &str = "http://0.0.0.0:14540";
pub const OFFBOARD_PORT: u16 = 14540;

#[derive(Debug)]
pub struct Drone {
    pub info: InfoServiceClient<Channel>,
    pub telemetry: TelemetryServiceClient<Channel>,
    pub mocap: MocapServiceClient<Channel>,
    pub action: ActionServiceClient<Channel>,
}

impl Drone {
    pub async fn connect<'a>(url: &'static str) -> Result<Self> {
        let channel = Endpoint::new(url)?.connect().await?;

        Ok(Self {
            info: InfoServiceClient::new(channel.clone()),
            telemetry: TelemetryServiceClient::new(channel.clone()),
            mocap: MocapServiceClient::new(channel.clone()),
            action: ActionServiceClient::new(channel.clone()),
        })
    }
}
