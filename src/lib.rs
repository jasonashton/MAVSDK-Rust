use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;

static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| DescriptorPool::decode(
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref()
).unwrap());

pub mod action {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.action.rs"));
}

pub mod calibration {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.calibration.rs"));
}

pub mod camera {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.camera.rs"));
}

pub mod core {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.core.rs"));
}

pub mod geofence {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.geofence.rs"));
}

pub mod gimbal {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.gimbal.rs"));
}

pub mod info {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.info.rs"));
}

pub mod mission {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.mission.rs"));
}

pub mod mocap {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.mocap.rs"));
}

pub mod offboard {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.offboard.rs"));
}

pub mod param {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.param.rs"));
}

pub mod shell {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.shell.rs"));
}

pub mod telemetry {
    include!(concat!(env!("OUT_DIR"), "/mavsdk.rpc.telemetry.rs"));
}

// #[path = "grpc/mavsdk.rpc.action.rs"]
// pub mod action;
// #[path = "grpc/mavsdk.rpc.calibration.rs"]
// pub mod calibration;
// #[path = "grpc/mavsdk.rpc.camera.rs"]
// pub mod camera;
// #[path = "grpc/mavsdk.rpc.core.rs"]
// pub mod core;
// #[path = "grpc/mavsdk.rpc.geofence.rs"]
// pub mod geofence;
// #[path = "grpc/mavsdk.rpc.gimbal.rs"]
// pub mod gimbal;
// #[path = "grpc/mavsdk.rpc.info.rs"]
// pub mod info;
// #[path = "grpc/mavsdk.rpc.mission.rs"]
// pub mod mission;
// #[path = "grpc/mavsdk.rpc.mocap.rs"]
// pub mod mocap;
// #[path = "grpc/mavsdk.rpc.offboard.rs"]
// pub mod offboard;
// #[path = "grpc/mavsdk.rpc.param.rs"]
// pub mod param;
// #[path = "grpc/mavsdk.rpc.shell.rs"]
// pub mod shell;
// #[path = "grpc/mavsdk.rpc.telemetry.rs"]
// pub mod telemetry;