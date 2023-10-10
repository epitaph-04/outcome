#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatisticsForPlanRequest {
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<PlanData>,
    #[prost(bool, tag = "2")]
    pub evaluate_results_total: bool,
    #[prost(bool, tag = "3")]
    pub evaluate_results_per_buy: bool,
    #[prost(bool, tag = "4")]
    pub evaluate_results_per_label: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatisticsForPlanResponse {
    #[prost(message, repeated, tag = "1")]
    pub results_total: ::prost::alloc::vec::Vec<KpiResult>,
    #[prost(message, repeated, tag = "2")]
    pub results_per_buy: ::prost::alloc::vec::Vec<KpiResultForBuy>,
    #[prost(message, repeated, tag = "3")]
    pub results_per_label: ::prost::alloc::vec::Vec<KpiResultForLabel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiResult {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<KpiValue>,
    #[prost(bool, tag = "5")]
    pub success: bool,
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
    #[prost(enumeration = "KpiValueType", tag = "7")]
    pub kpi_value_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiResultForBuy {
    #[prost(message, optional, tag = "1")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<KpiResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiResultForLabel {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<KpiResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanData {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub period: ::core::option::Option<super::super::common::v1::Period>,
    #[prost(message, repeated, tag = "5")]
    pub evaluation_audiences: ::prost::alloc::vec::Vec<Audience>,
    #[prost(message, repeated, tag = "6")]
    pub buys: ::prost::alloc::vec::Vec<Buy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bm {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Buy {
    #[prost(oneof = "buy::Value", tags = "1, 2")]
    pub value: ::core::option::Option<buy::Value>,
}
/// Nested message and enum types in `Buy`.
pub mod buy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        LinearBuy(super::LinearBuy),
        #[prost(message, tag = "2")]
        AddressableBuy(super::AddressableBuy),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinearBuy {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub buyable_medium: ::core::option::Option<Bm>,
    #[prost(message, optional, tag = "3")]
    pub period: ::core::option::Option<super::super::common::v1::Period>,
    #[prost(message, optional, tag = "4")]
    pub audience: ::core::option::Option<Audience>,
    #[prost(double, tag = "5")]
    pub grp: f64,
    #[prost(double, tag = "6")]
    pub cost_per_grp: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressableBuy {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub buyable_medium: ::core::option::Option<Bm>,
    #[prost(message, optional, tag = "3")]
    pub period: ::core::option::Option<super::super::common::v1::Period>,
    #[prost(message, optional, tag = "4")]
    pub audience: ::core::option::Option<Audience>,
    #[prost(double, tag = "5")]
    pub impressions000: f64,
    #[prost(int32, tag = "6")]
    pub frequency_cap: i32,
    #[prost(double, tag = "7")]
    pub cost_per_impressions000: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiValue {
    #[prost(oneof = "kpi_value::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<kpi_value::Value>,
}
/// Nested message and enum types in `KpiValue`.
pub mod kpi_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        KpiDoubleValue(super::KpiDoubleValue),
        #[prost(message, tag = "2")]
        KpiPercentageValue(super::KpiPercentageValue),
        #[prost(message, tag = "3")]
        KpiMoneyValue(super::KpiMoneyValue),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiMoneyValue {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiDoubleValue {
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiPercentageValue {
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidPeriodErrorResponse {
    #[prost(enumeration = "ErrorType", tag = "1")]
    pub error_type: i32,
    #[prost(string, tag = "2")]
    pub explanation: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub start_date: ::core::option::Option<super::super::common::v1::Date>,
    #[prost(message, optional, tag = "4")]
    pub end_date: ::core::option::Option<super::super::common::v1::Date>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericErrorResponse {
    #[prost(enumeration = "ErrorType", tag = "1")]
    pub error_type: i32,
    #[prost(string, tag = "2")]
    pub explanation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    #[prost(oneof = "error_response::SealedValue", tags = "1, 2")]
    pub sealed_value: ::core::option::Option<error_response::SealedValue>,
}
/// Nested message and enum types in `ErrorResponse`.
pub mod error_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SealedValue {
        #[prost(message, tag = "1")]
        GenericError(super::GenericErrorResponse),
        #[prost(message, tag = "2")]
        InvalidPeriodError(super::InvalidPeriodErrorResponse),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KpiValueType {
    Unspecified = 0,
    Double = 1,
    Percentage = 2,
    Money = 3,
}
impl KpiValueType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KpiValueType::Unspecified => "KPI_VALUE_TYPE_UNSPECIFIED",
            KpiValueType::Double => "KPI_VALUE_TYPE_DOUBLE",
            KpiValueType::Percentage => "KPI_VALUE_TYPE_PERCENTAGE",
            KpiValueType::Money => "KPI_VALUE_TYPE_MONEY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KPI_VALUE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "KPI_VALUE_TYPE_DOUBLE" => Some(Self::Double),
            "KPI_VALUE_TYPE_PERCENTAGE" => Some(Self::Percentage),
            "KPI_VALUE_TYPE_MONEY" => Some(Self::Money),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorType {
    Unspecified = 0,
    EmptyPlan = 1,
    UndefinedPlanKey = 2,
    UndefinedEvaluationAudience = 3,
    UndefinedBuyableMedia = 4,
    InvalidPlanPeriod = 5,
    InvalidBuyPeriod = 6,
}
impl ErrorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorType::Unspecified => "ERROR_TYPE_UNSPECIFIED",
            ErrorType::EmptyPlan => "ERROR_TYPE_EMPTY_PLAN",
            ErrorType::UndefinedPlanKey => "ERROR_TYPE_UNDEFINED_PLAN_KEY",
            ErrorType::UndefinedEvaluationAudience => "ERROR_TYPE_UNDEFINED_EVALUATION_AUDIENCE",
            ErrorType::UndefinedBuyableMedia => "ERROR_TYPE_UNDEFINED_BUYABLE_MEDIA",
            ErrorType::InvalidPlanPeriod => "ERROR_TYPE_INVALID_PLAN_PERIOD",
            ErrorType::InvalidBuyPeriod => "ERROR_TYPE_INVALID_BUY_PERIOD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERROR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ERROR_TYPE_EMPTY_PLAN" => Some(Self::EmptyPlan),
            "ERROR_TYPE_UNDEFINED_PLAN_KEY" => Some(Self::UndefinedPlanKey),
            "ERROR_TYPE_UNDEFINED_EVALUATION_AUDIENCE" => Some(Self::UndefinedEvaluationAudience),
            "ERROR_TYPE_UNDEFINED_BUYABLE_MEDIA" => Some(Self::UndefinedBuyableMedia),
            "ERROR_TYPE_INVALID_PLAN_PERIOD" => Some(Self::InvalidPlanPeriod),
            "ERROR_TYPE_INVALID_BUY_PERIOD" => Some(Self::InvalidBuyPeriod),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod plan_outcome_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PlanOutcomeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlanOutcomeServiceClient<tonic::transport::Channel> {
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
    impl<T> PlanOutcomeServiceClient<T>
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
        ) -> PlanOutcomeServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PlanOutcomeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_statistics_for_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatisticsForPlanRequest>,
        ) -> std::result::Result<tonic::Response<super::GetStatisticsForPlanResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plan_outcome_predicting.v1.PlanOutcomeService/GetStatisticsForPlan",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "plan_outcome_predicting.v1.PlanOutcomeService",
                "GetStatisticsForPlan",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
