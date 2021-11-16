use std::path::PathBuf;

use chrono::{DateTime, Utc};
use log::{error, info};

use mav_sdk::{Drone, grpc::telemetry::{AttitudeQuaternionResponse, PositionResponse, SetRateAttitudeRequest, SetRatePositionRequest, TelemetryServiceClient}, transport::Channel};

use serde::{Deserialize, Serialize};
// use simple_logger::SimpleLogger;
use telemetry::{Entry, FileStore, Recorder};

const MAVSDK_SERVER: &str = "http://127.0.0.1:4000";

struct FlightLog<L, R> {
    pub log: L,
    pub entries: Vec<Entry<R>>,
}

#[derive(Serialize, Deserialize)]
pub struct AttitudeConfig {
    pub rate_hz: f64,
}

pub struct AttitudeLog {
    pub config: AttitudeConfig,
    pub rate_hz: f64,
    pub entries: Vec<Entry<AttitudeQuaternionResponse>>,
}

impl AttitudeLog {
    /// Load log from a JSON file
    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> { //io::Error> {
        todo!("Read from file + config for Rate hz")
    }
}


#[tokio::main]
async fn main() {
    // init logger
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let mut drone = Drone::connect(MAVSDK_SERVER).await.expect("Should connect");
    info!("We have a DRONE connection with {}", MAVSDK_SERVER);

    // use the same time for all logs files
    let time = Utc::now();

    drone.telemetry.set_rate_attitude(SetRateAttitudeRequest { rate_hz: 100.0}).await.expect("should set 100 hz for Attitude");
    drone.telemetry.set_rate_position(SetRatePositionRequest { rate_hz: 100.0}).await.expect("should set 100 hz for Position");


    tokio::spawn(run_quaternion_recording(drone.telemetry.clone(), time));
    tokio::spawn(run_position_recording(drone.telemetry.clone(), time));

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
}

/// Generates timestamped file name with the service name
fn get_file_path(service: &str, time: DateTime<Utc>) -> PathBuf {
    PathBuf::from(format!("./data/{}-{}.json", time, service))
}

/// Records the Quaternion response from MAVSDK into a file
async fn run_quaternion_recording(
    mut telemetry: TelemetryServiceClient<Channel>,
    time: DateTime<Utc>,
) -> anyhow::Result<()> {
    let file_path = get_file_path("quaternions", time);
    let recorder = FileStore::<AttitudeQuaternionResponse>::create(file_path).await?;

    let request = mav_sdk::grpc::telemetry::SubscribeAttitudeQuaternionRequest {};

    let mut response = telemetry.subscribe_attitude_quaternion(request).await?;

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response.get_mut().message().await? {
        // Log errors
        if let Err(err) = recorder.add(Entry::new(response)).await {
            error!("Recorder error: {}", err)
        }
    }

    Ok(())
}

/// Records the Position (GNSS - currently GPS) response from MAVSDK into a file
async fn run_position_recording(
    mut telemetry: TelemetryServiceClient<Channel>,
    time: DateTime<Utc>,
) -> anyhow::Result<()> {
    let file_path = get_file_path("position", time);

    let recorder = FileStore::<PositionResponse>::create(file_path).await?;

    let request = mav_sdk::grpc::telemetry::SubscribePositionRequest {};

    let mut response = telemetry.subscribe_position(request).await?;

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response.get_mut().message().await? {
        // Log errors
        if let Err(err) = recorder.add(Entry::new(response)).await {
            error!("Recorder error: {}", err)
        }
    }

    Ok(())
}
