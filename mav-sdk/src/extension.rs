pub trait Extension {
    type Client;
}

#[derive(Debug)]
pub struct Telemetry {}
#[derive(Debug)]
pub struct ClientTelemetry {}
impl Extension for Telemetry {
    type Client = ClientTelemetry;
}


#[derive(Debug)]
pub struct Info {}
#[derive(Debug)]
pub struct ClientInfo {}
impl Extension for Info {
    type Client = ClientInfo;
}

#[derive(Debug)]
pub struct Mocap {}
#[derive(Debug)]
pub struct ClientMocap {}

impl Extension for Mocap {
    type Client = ClientMocap;
}

pub struct Drone<const N: usize> {
    pub extensions: [Box<dyn Extension>; N]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_drone_extensions() {

        let drone = Drone {
            extensions: [Box::new(Telemetry {}),Box::new(Info {}), Box::new(Mocap {})],
        };

        let telemetry = drone.extensions[0];

        dbg!(telemetry);
    }
}
