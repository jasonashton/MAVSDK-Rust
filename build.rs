const PROTO_INCLUDE_PATH: &str = "./proto/protos";

const PLUGINS: &[&str] = &[
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files: Vec<String> = PLUGINS.iter()
        .map(|&plugin| format!("proto/protos/{}/{}.proto", plugin, plugin))
        .collect();

    let mut config = prost_build::Config::new();
    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .configure(
            &mut config,
            &proto_files,
            &[PROTO_INCLUDE_PATH],
        )?;

    tonic_build::configure()
        .build_server(false)
        .compile_with_config(config, &proto_files, &[PROTO_INCLUDE_PATH])?;

    Ok(())
}
