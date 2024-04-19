// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateGyroRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateGyroResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateAccelerometerRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateAccelerometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateMagnetometerRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateMagnetometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateLevelHorizonRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateLevelHorizonResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCalibrateGimbalAccelerometerRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrateGimbalAccelerometerResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
    /// Progress data
    #[prost(message, optional, tag = "2")]
    pub progress_data: ::core::option::Option<ProgressData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelResponse {
    #[prost(message, optional, tag = "1")]
    pub calibration_result: ::core::option::Option<CalibrationResult>,
}
/// Result type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalibrationResult {
    /// Result enum value
    #[prost(enumeration = "calibration_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CalibrationResult`.
pub mod calibration_result {
    /// Possible results returned for calibration commands
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// The calibration succeeded
        Success = 1,
        /// Intermediate message showing progress or instructions on the next steps
        Next = 2,
        /// Calibration failed
        Failed = 3,
        /// No system is connected
        NoSystem = 4,
        /// Connection error
        ConnectionError = 5,
        /// Vehicle is busy
        Busy = 6,
        /// Command refused by vehicle
        CommandDenied = 7,
        /// Command timed out
        Timeout = 8,
        /// Calibration process was cancelled
        Cancelled = 9,
        /// Calibration process failed since the vehicle is armed
        FailedArmed = 10,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unknown => "RESULT_UNKNOWN",
                Result::Success => "RESULT_SUCCESS",
                Result::Next => "RESULT_NEXT",
                Result::Failed => "RESULT_FAILED",
                Result::NoSystem => "RESULT_NO_SYSTEM",
                Result::ConnectionError => "RESULT_CONNECTION_ERROR",
                Result::Busy => "RESULT_BUSY",
                Result::CommandDenied => "RESULT_COMMAND_DENIED",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::Cancelled => "RESULT_CANCELLED",
                Result::FailedArmed => "RESULT_FAILED_ARMED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNKNOWN" => Some(Self::Unknown),
                "RESULT_SUCCESS" => Some(Self::Success),
                "RESULT_NEXT" => Some(Self::Next),
                "RESULT_FAILED" => Some(Self::Failed),
                "RESULT_NO_SYSTEM" => Some(Self::NoSystem),
                "RESULT_CONNECTION_ERROR" => Some(Self::ConnectionError),
                "RESULT_BUSY" => Some(Self::Busy),
                "RESULT_COMMAND_DENIED" => Some(Self::CommandDenied),
                "RESULT_TIMEOUT" => Some(Self::Timeout),
                "RESULT_CANCELLED" => Some(Self::Cancelled),
                "RESULT_FAILED_ARMED" => Some(Self::FailedArmed),
                _ => None,
            }
        }
    }
}
///
/// Progress data coming from calibration.
///
/// Can be a progress percentage, or an instruction text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressData {
    /// Whether this ProgressData contains a 'progress' status or not
    #[prost(bool, tag = "1")]
    pub has_progress: bool,
    /// Progress (percentage)
    #[prost(float, tag = "2")]
    pub progress: f32,
    /// Whether this ProgressData contains a 'status_text' or not
    #[prost(bool, tag = "3")]
    pub has_status_text: bool,
    /// Instruction text
    #[prost(string, tag = "4")]
    pub status_text: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod calibration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Enable to calibrate sensors of a drone such as gyro, accelerometer, and magnetometer.
    #[derive(Debug, Clone)]
    pub struct CalibrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CalibrationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CalibrationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CalibrationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CalibrationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Perform gyro calibration.
        pub async fn subscribe_calibrate_gyro(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeCalibrateGyroRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::CalibrateGyroResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGyro",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.calibration.CalibrationService",
                        "SubscribeCalibrateGyro",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Perform accelerometer calibration.
        pub async fn subscribe_calibrate_accelerometer(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeCalibrateAccelerometerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::CalibrateAccelerometerResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateAccelerometer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.calibration.CalibrationService",
                        "SubscribeCalibrateAccelerometer",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Perform magnetometer calibration.
        pub async fn subscribe_calibrate_magnetometer(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeCalibrateMagnetometerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::CalibrateMagnetometerResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateMagnetometer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.calibration.CalibrationService",
                        "SubscribeCalibrateMagnetometer",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Perform board level horizon calibration.
        pub async fn subscribe_calibrate_level_horizon(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeCalibrateLevelHorizonRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::CalibrateLevelHorizonResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateLevelHorizon",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.calibration.CalibrationService",
                        "SubscribeCalibrateLevelHorizon",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Perform gimbal accelerometer calibration.
        pub async fn subscribe_calibrate_gimbal_accelerometer(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SubscribeCalibrateGimbalAccelerometerRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::CalibrateGimbalAccelerometerResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/SubscribeCalibrateGimbalAccelerometer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.calibration.CalibrationService",
                        "SubscribeCalibrateGimbalAccelerometer",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Cancel ongoing calibration process.
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRequest>,
        ) -> std::result::Result<tonic::Response<super::CancelResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.calibration.CalibrationService/Cancel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.calibration.CalibrationService",
                        "Cancel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}