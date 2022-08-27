use mav_sdk::Drone;

const MAVSDK_SERVER: &str = "http://127.0.0.1:4000";

#[tokio::main]
async fn main() {
    let mut drone = Drone::connect(MAVSDK_SERVER).await.expect("Should connect");
    println!("We have a DRONE connection with {}", MAVSDK_SERVER);

    println!("Uploading Geofence...");

    let upload_geofence_request = mav_sdk::grpc::geofence::UploadGeofenceRequest { polygons: vec![
        mav_sdk::grpc::geofence::Polygon {
            points:vec![mav_sdk::grpc::geofence::Point{ latitude_deg: 0.0, longitude_deg: 0.0 },mav_sdk::grpc::geofence::Point{ latitude_deg: 0.0, longitude_deg: 0.0 },mav_sdk::grpc::geofence::Point{ latitude_deg: 0.0, longitude_deg: 0.0 },mav_sdk::grpc::geofence::Point{ latitude_deg: 0.0, longitude_deg: 0.0 },], fence_type: 0, },
    ]};
    let upload_geofence = drone
        .geofence
        .upload_geofence(upload_geofence_request)
        .await
        .expect("Should upload geofence");

        println!(
            "Upload Geofence: {}",
            &upload_geofence
                .get_ref()
                .geofence_result
                .as_ref()
                .unwrap()
                .result_str
        );

    println!("Clearing Geofence...");

    let clear_geofence_request = mav_sdk::grpc::geofence::ClearGeofenceRequest {};
    let clear_geofence = drone
        .geofence
        .clear_geofence(clear_geofence_request)
        .await
        .expect("Should clear geofence");
    
    println!(
        "Clear Geofence: {}",
        &clear_geofence
            .get_ref()
            .geofence_result
            .as_ref()
            .unwrap()
            .result_str
    );
}
