// This file is @generated by prost-build.
/// Point type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
}
/// Polygon type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polygon {
    /// Points defining the polygon
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<Point>,
    /// Fence type
    #[prost(enumeration = "polygon::FenceType", tag = "2")]
    pub fence_type: i32,
}
/// Nested message and enum types in `Polygon`.
pub mod polygon {
    /// Geofence polygon types.
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
    pub enum FenceType {
        /// Type representing an inclusion fence
        Inclusion = 0,
        /// Type representing an exclusion fence
        Exclusion = 1,
    }
    impl FenceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FenceType::Inclusion => "FENCE_TYPE_INCLUSION",
                FenceType::Exclusion => "FENCE_TYPE_EXCLUSION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FENCE_TYPE_INCLUSION" => Some(Self::Inclusion),
                "FENCE_TYPE_EXCLUSION" => Some(Self::Exclusion),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceRequest {
    /// Polygon(s) representing the geofence(s)
    #[prost(message, repeated, tag = "1")]
    pub polygons: ::prost::alloc::vec::Vec<Polygon>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadGeofenceResponse {
    #[prost(message, optional, tag = "1")]
    pub geofence_result: ::core::option::Option<GeofenceResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearGeofenceRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearGeofenceResponse {
    #[prost(message, optional, tag = "1")]
    pub geofence_result: ::core::option::Option<GeofenceResult>,
}
/// Result type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeofenceResult {
    /// Result enum value
    #[prost(enumeration = "geofence_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GeofenceResult`.
pub mod geofence_result {
    /// Possible results returned for geofence requests.
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
        /// Request succeeded
        Success = 1,
        /// Error
        Error = 2,
        /// Too many Polygon objects in the geofence
        TooManyGeofenceItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
        /// No system connected
        NoSystem = 7,
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
                Result::Error => "RESULT_ERROR",
                Result::TooManyGeofenceItems => "RESULT_TOO_MANY_GEOFENCE_ITEMS",
                Result::Busy => "RESULT_BUSY",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::InvalidArgument => "RESULT_INVALID_ARGUMENT",
                Result::NoSystem => "RESULT_NO_SYSTEM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNKNOWN" => Some(Self::Unknown),
                "RESULT_SUCCESS" => Some(Self::Success),
                "RESULT_ERROR" => Some(Self::Error),
                "RESULT_TOO_MANY_GEOFENCE_ITEMS" => Some(Self::TooManyGeofenceItems),
                "RESULT_BUSY" => Some(Self::Busy),
                "RESULT_TIMEOUT" => Some(Self::Timeout),
                "RESULT_INVALID_ARGUMENT" => Some(Self::InvalidArgument),
                "RESULT_NO_SYSTEM" => Some(Self::NoSystem),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod geofence_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Enable setting a geofence.
    #[derive(Debug, Clone)]
    pub struct GeofenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GeofenceServiceClient<tonic::transport::Channel> {
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
    impl<T> GeofenceServiceClient<T>
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
        ) -> GeofenceServiceClient<InterceptedService<T, F>>
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
            GeofenceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        /// Upload a geofence.
        ///
        /// Polygons are uploaded to a drone. Once uploaded, the geofence will remain
        /// on the drone even if a connection is lost.
        pub async fn upload_geofence(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadGeofenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UploadGeofenceResponse>,
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
                "/mavsdk.rpc.geofence.GeofenceService/UploadGeofence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.geofence.GeofenceService",
                        "UploadGeofence",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Clear all geofences saved on the vehicle.
        pub async fn clear_geofence(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearGeofenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearGeofenceResponse>,
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
                "/mavsdk.rpc.geofence.GeofenceService/ClearGeofence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.geofence.GeofenceService",
                        "ClearGeofence",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}