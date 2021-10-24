use mav_sdk::{
    grpc::telemetry::{AttitudeQuaternionResponse, TelemetryService, TelemetryServiceClient},
    Drone,
};
use tonic::transport::Channel;

const MAVSDK_SERVER: &str = "http://127.0.0.1:4000";

#[tokio::main]
async fn main() {
    let mut drone = Drone::connect(MAVSDK_SERVER).await.expect("Should connect");
    println!("We have a DRONE connection with {}", MAVSDK_SERVER);

    tokio::spawn(print_quaternion(drone.telemetry.clone()));
    tokio::spawn(print_position(drone.telemetry.clone()));

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
}

async fn print_quaternion(mut telemetry: TelemetryServiceClient<Channel>) {
    let request = mav_sdk::grpc::telemetry::SubscribeAttitudeQuaternionRequest {};

    let mut response = telemetry
        .subscribe_attitude_quaternion(request)
        .await
        .expect("Should subscribe");

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        let json = serde_json::to_string(&response).expect("Should serialize");
        println!("{}", json);
    }
}

async fn print_position(mut telemetry: TelemetryServiceClient<Channel>) {
    let request = mav_sdk::grpc::telemetry::SubscribePositionRequest {};

    // let rate = telemetry.rate
    // telemetry.set_rate_attitude();
    let mut response = telemetry
        .subscribe_position(request)
        .await
        .expect("Should subscribe");

    // AttitudeQuaternionResponse { attitude_quaternion }
    while let Some(response) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        let json = serde_json::to_string(&response).expect("Should serialize");
        println!("{}", json);
    }
}
