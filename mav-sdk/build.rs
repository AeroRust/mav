use std::path::PathBuf;

const PROTO_GIT_SUBMODULE: &str = "mavsdk-proto";
const MAVSDK_OPTIONS: &str = "mavsdk_options";

fn main() -> Result<(), std::io::Error> {
    let plugins = [
        "action",
        "calibration",
        "camera",
        "core",
        "geofence",
        "gimbal",
        "info",
        "mission",
        "mocap",
        "offboard",
        "param",
        "shell",
        "telemetry",
    ];

    let mavsdk_options_include = format!("{submodule}/protos", submodule = PROTO_GIT_SUBMODULE);

    // tonic_build(&plugins, mavsdk_options_include.into())

    tonic_build_single(&plugins, mavsdk_options_include.into())
}

fn proto_path(plugin_name: &str) -> PathBuf {
    let path_string = format!(
        "{submodule}/protos/{name}/{name}.proto",
        submodule = PROTO_GIT_SUBMODULE,
        name = plugin_name
    );

    PathBuf::from(path_string)
}

fn proto_include(plugin_name: &str) -> PathBuf {
    let path = format!(
        "{submodule}/protos/{name}",
        submodule = PROTO_GIT_SUBMODULE,
        name = plugin_name
    );

    PathBuf::from(path)
}

/// build in grpc, adding all files to the build instead generating them one by one
fn tonic_build_single(
    plugins: &[&str],
    mavsdk_options_include: PathBuf,
) -> Result<(), std::io::Error> {
    let mavsdk_options_path = PathBuf::from(format!(
        "{submodule}/protos/{name}.proto",
        submodule = PROTO_GIT_SUBMODULE,
        name = MAVSDK_OPTIONS
    ));

    let (proto_paths, proto_includes) = plugins.iter().fold(
        (vec![mavsdk_options_path], vec![mavsdk_options_include]),
        |(mut proto_paths, mut proto_includes), plugin| {
            proto_paths.push(proto_path(plugin));
            proto_includes.push(proto_include(plugin));

            (proto_paths, proto_includes)
        },
    );

    // let mut attributes = Attributes::default();
    // attributes.push_struct("AttitudeQuaternionResponse", "#[derive(Serialize, Deserialize)]");

    let derive_serde = "#[derive(serde::Serialize, serde::Deserialize)]";
    tonic_build::configure()
        // .build_server(false)
        // Quaternions
        .type_attribute("Quaternion", derive_serde)
        .type_attribute("AttitudeQuaternionResponse", derive_serde)
        // GPS position types
        .type_attribute("Position", derive_serde)
        .type_attribute("PositionResponse", derive_serde)
        .format(true)
        .out_dir("src/grpc")
        .compile(&proto_paths, &proto_includes)
}

/// build in sub-dirs each of the plugins and finally adding `MAVSDK_OPTIONS`

fn _tonic_build(plugins: &[&str], mavsdk_options_include: PathBuf) -> Result<(), std::io::Error> {
    plugins
        .iter()
        .map(|plugin| {
            // (proto_path(plugin), proto_include(plugin))
            tonic_build::configure()
                // .build_server(false)
                .format(true)
                .out_dir(format!("src/grpc/{}", plugin))
                .compile(
                    &[proto_path(plugin)],
                    &[proto_include(plugin), mavsdk_options_include.clone()],
                )
        })
        // Add the `mavsdk_options.proto`, since it's in the main `protos` directory
        .chain(std::iter::once_with(|| {
            let path = PathBuf::from(format!(
                "{submodule}/protos/{name}.proto",
                submodule = PROTO_GIT_SUBMODULE,
                name = MAVSDK_OPTIONS
            ));

            tonic_build::configure()
                .build_server(false)
                .format(true)
                .out_dir("src/grpc")
                .compile(&[path], &[mavsdk_options_include.clone()])
        }))
        .collect::<Result<_, _>>()
}
