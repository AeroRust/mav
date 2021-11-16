use super::*;


mod getter {
    use std::any::TypeId;

    use super::*;

    pub trait Get<L,R> {
        fn left() -> L;
        fn right() -> R;

        fn contains<E>() -> bool;
    }

    // impl Get<Telemetry> for (Telemetry) {
    //     fn get() -> Telemetry {
    //         Telemetry::default()
    //     }
    // }

    impl Get<Telemetry, Info> for (Telemetry, Info) {
        fn contains<E>() -> bool {
            TypeId::of::<E>() == TypeId::
        }
    }

    impl Get<Telemetry> for (Telemetry, Info) {

    }

    // pub trait Get<E> {
    // }
    // impl<E1, E2> Get<E1> for (E1, E2) {}
    // impl<E1, E2> Get<E2> for (E1, E2) {}
    // impl<E1, E2, E3> Get<E1> for (E1, E2, E3) {}
    // impl<E1, E2, E3> Get<E2> for (E1, E2, E3) {}
    // impl<E1, E2, E3> Get<E3> for (E1, E2, E3) {}

    // impl<E1, E2> Get<E1> for With<E1, E2>
    // where
    //     E1: Extension,
    //     E2: Extension,
    // {
    // }

    // impl<E1, E2> Get<E2> for With<E1, E2>
    // where
    //     E1: Extension,
    //     E2: Extension,
    // {
    // }
    

    // impl<E1: Extension, E2: Extension, E3: Extension> Get<E1> for With<With<E1, E2>, E3> {}
    // impl<E1: Extension, E2: Extension, E3: Extension> Get<E2> for With<With<E1, E2>, E3> {}
    // impl<E1: Extension, E2: Extension, E3: Extension> Get<E3q> for With<With<E1, E2>, E3> {}
}
use std::marker::PhantomData;

use tonic::transport::{Channel, Endpoint};

trait Extension {
    type Client;

    fn with_channel(channel: Channel) -> Self::Client;
}

#[derive(Default)]
struct With<E1, E2> {
    left: PhantomData<E1>,
    right: PhantomData<E2>,
}

// impl<E1, E2> Extension for With<E1, E2>
// where
//     E1: Extension,
//     E2: Extension,
// {
// }

#[derive(Debug, Default)]
pub struct Info;
impl Extension for Info {
    type Client = crate::grpc::info::InfoServiceClient<Channel>;

    fn with_channel(channel: Channel) -> Self::Client {
        Self::Client::new(channel)
    }
}

#[derive(Debug, Default)]
pub struct Telemetry;
impl Extension for Telemetry {
    type Client = crate::grpc::telemetry::TelemetryServiceClient<Channel>;

    fn with_channel(channel: Channel) -> Self::Client {
        Self::Client::new(channel)
    }
}

#[derive(Debug, Default)]
pub struct Mocap;
impl Extension for Mocap {
    type Client = crate::grpc::mocap::mocap_service_client::MocapServiceClient<Channel>;

    fn with_channel(channel: Channel) -> Self::Client {
        Self::Client::new(channel)
    }
}
#[derive(Debug, Default)]
pub struct Action;
impl Extension for Action {
    type Client = crate::grpc::action::ActionServiceClient<Channel>;

    fn with_channel(channel: Channel) -> Self::Client {
        Self::Client::new(channel)
    }
}

pub struct Builder<E1, E2> {
    extensions: With<E1, E2>,
}

// impl With<E1, E2> where E1: Extension,E2: Extension {
//     pub fn has::<>()
// }

// impl<E1, E2, E3> Builder<With<E1, E2>, E3>
// where
//     E1: Extension,
//     E2: Extension,
//     E3: Extension,
// {
//     pub async fn connect(url: &str) -> Result<Drone<3>, tonic::transport::Error> {
//         let channel = Endpoint::new(url)?.connect().await?;

//         Ok(Drone {
//             clients: [E1::with_channel(), E2::with_channel(), E3::with_channel()],
//         })
//     }
// }

impl<E1, E2> Builder<E1, E2>
where
    E1: Extension,
    E2: Extension,
{
    pub fn with<E3>(self) -> Builder<With<E1, E2>, E3>
    where
        E3: Extension,
    {
        Builder {
            extensions: With {
                left: Default::default(),
                right: Default::default(),
            },
        }
    }

    // pub fn connect(self, url: &str) -> Result<Drone<E1, E2>, Error> {
    //     Ok(Drone {
    //         clients: Service::<E1, E2>::connect(url)?
    //     })
    // }
}

// pub struct Drone<const N: usize> {
//     clients: []
// }

// impl<E1, E2> Drone<E1, E2> where E1: Extension, E2: Extension {
//     pub fn connect(url: &str) -> Result<Self, Error> {
//         // let channel = Endpoint::new(url)?.connect().await?;

//         Ok(Self {
//             services: Services {
//                 left: E1::connect()?,
//                 right: E2::connect()?,
//             }
//         })
//     }
// }

pub struct Service<E1, E2> {
    left: E1,
    right: E2,
}

pub struct Error {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_builder() {
        let builder = Builder {
            extensions: With::<Action, Info>::default(),
        }
        .with::<Telemetry>();

        builder.connect("");
        // .connect("http")?;
    }
}
