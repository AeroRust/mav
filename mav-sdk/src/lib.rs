/// Generated services
pub mod grpc;

pub type Result<T> = std::result::Result<T, tonic::transport::Error>;

use self::grpc::{
    action::ActionServiceClient, info::InfoServiceClient,
    mocap::mocap_service_client::MocapServiceClient, telemetry::TelemetryServiceClient,
};

use grpc::{
    calibration::CalibrationServiceClient, camera::CameraServiceClient, core::CoreServiceClient,
    geofence::GeofenceServiceClient, gimbal::GimbalServiceClient, mission::MissionServiceClient,
    offboard::OffboardServiceClient, param::ParamServiceClient, shell::ShellServiceClient,
};
use tonic::transport::{Channel, Endpoint};

pub const DEFAULT_URL: &str = "http://0.0.0.0:14540";
pub const OFFBOARD_PORT: u16 = 14540;

#[derive(Debug, Clone)]
pub struct Drone {
    pub action: ActionServiceClient<Channel>,
    pub calibration: CalibrationServiceClient<Channel>,
    pub camera: CameraServiceClient<Channel>,
    pub core: CoreServiceClient<Channel>,
    pub geofence: GeofenceServiceClient<Channel>,
    pub gimbal: GimbalServiceClient<Channel>,
    pub info: InfoServiceClient<Channel>,
    pub mission: MissionServiceClient<Channel>,
    pub mocap: MocapServiceClient<Channel>,
    pub offboard: OffboardServiceClient<Channel>,
    pub param: ParamServiceClient<Channel>,
    pub shell: ShellServiceClient<Channel>,
    pub telemetry: TelemetryServiceClient<Channel>,
}

impl Drone {
    pub async fn connect(url: &'static str) -> Result<Self> {
        let channel = Endpoint::new(url)?.connect().await?;

        Ok(Self {
            action: ActionServiceClient::new(channel.clone()),
            calibration: CalibrationServiceClient::new(channel.clone()),
            camera: CameraServiceClient::new(channel.clone()),
            core: CoreServiceClient::new(channel.clone()),
            geofence: GeofenceServiceClient::new(channel.clone()),
            gimbal: GimbalServiceClient::new(channel.clone()),
            info: InfoServiceClient::new(channel.clone()),
            mission: MissionServiceClient::new(channel.clone()),
            mocap: MocapServiceClient::new(channel.clone()),
            offboard: OffboardServiceClient::new(channel.clone()),
            param: ParamServiceClient::new(channel.clone()),
            shell: ShellServiceClient::new(channel.clone()),
            telemetry: TelemetryServiceClient::new(channel.clone()),
        })
    }
}
