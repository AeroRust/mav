use mav_sdk::{Drone, grpc::telemetry::AttitudeEulerResponse};

const MAVSDK_SERVER: &str = "http://127.0.0.1:4000";

#[tokio::main]
async fn main() {
    let mut drone = Drone::connect(MAVSDK_SERVER).await.expect("Should connect");
    println!("We have a DRONE connection with {}", MAVSDK_SERVER);

    let subscribe_euler_request = mav_sdk::grpc::telemetry::SubscribeAttitudeEulerRequest {};
    let mut response = drone
        .telemetry
        .subscribe_attitude_euler(subscribe_euler_request)
        .await
        .expect("Should subscribe");

    while let Some(AttitudeEulerResponse { attitude_euler }) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        if let Some(attitude_euler) = attitude_euler {
            println!("Euler Angles! {:?}", attitude_euler)
        }
    }
}
