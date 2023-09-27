#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudiencePerType {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub audience_ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
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
pub struct KpiResultPerType {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub modified_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "ResultState", tag = "4")]
    pub state: i32,
    #[prost(message, repeated, tag = "5")]
    pub results: ::prost::alloc::vec::Vec<KpiResult>,
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
pub struct PredictedBuy {
    #[prost(message, optional, tag = "1")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub start: ::core::option::Option<super::super::common::v1::Date>,
    #[prost(message, optional, tag = "4")]
    pub end: ::core::option::Option<super::super::common::v1::Date>,
    #[prost(double, tag = "5")]
    pub insertions: f64,
    #[prost(double, tag = "6")]
    pub cost_per_insertion: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiMetadata {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "KpiValueType", tag = "3")]
    pub kpi_value_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachReachGoal {
    #[prost(message, optional, tag = "1")]
    pub audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub kpi_reach_goals: ::prost::alloc::vec::Vec<AttachKpiReachGoal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachKpiReachGoal {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<KpiValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachGoal {
    #[prost(message, optional, tag = "1")]
    pub audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub kpi_reach_goals: ::prost::alloc::vec::Vec<KpiReachGoal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiReachGoal {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<KpiValue>,
    #[prost(enumeration = "KpiValueType", tag = "4")]
    pub kpi_value_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KpiValue {
    #[prost(oneof = "kpi_value::KpiValue", tags = "1, 2, 3")]
    pub kpi_value: ::core::option::Option<kpi_value::KpiValue>,
}
/// Nested message and enum types in `KpiValue`.
pub mod kpi_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum KpiValue {
        #[prost(double, tag = "1")]
        KpiDoubleValue(f64),
        #[prost(double, tag = "2")]
        KpiPercentageValue(f64),
        #[prost(message, tag = "3")]
        KpiMoneyValue(super::super::super::common::v1::Money),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResultState {
    Unspecified = 0,
    Outdated = 1,
    UpToDate = 3,
}
impl ResultState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResultState::Unspecified => "RESULT_STATE_UNSPECIFIED",
            ResultState::Outdated => "RESULT_STATE_OUTDATED",
            ResultState::UpToDate => "RESULT_STATE_UP_TO_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESULT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESULT_STATE_OUTDATED" => Some(Self::Outdated),
            "RESULT_STATE_UP_TO_DATE" => Some(Self::UpToDate),
            _ => None,
        }
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsForPlanRequest {
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
pub struct PlanServiceGetStatisticsForPlanResponse {
    #[prost(message, repeated, tag = "1")]
    pub results_total: ::prost::alloc::vec::Vec<KpiResult>,
    #[prost(message, repeated, tag = "2")]
    pub results_per_buy: ::prost::alloc::vec::Vec<KpiResultForBuy>,
    #[prost(message, repeated, tag = "3")]
    pub results_per_label: ::prost::alloc::vec::Vec<KpiResultForLabel>,
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
    pub start_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "6")]
    pub evaluation_audiences: ::prost::alloc::vec::Vec<Audience>,
    #[prost(message, repeated, tag = "7")]
    pub buys: ::prost::alloc::vec::Vec<Buy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Buy {
    #[prost(oneof = "buy::Buy", tags = "1, 2")]
    pub buy: ::core::option::Option<buy::Buy>,
}
/// Nested message and enum types in `Buy`.
pub mod buy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Buy {
        #[prost(message, tag = "1")]
        LinearBuy(super::LinearBuy),
        #[prost(message, tag = "2")]
        AddressableBuy(super::AddressableBuy),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bm {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinearBuy {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub buyable_medium: ::core::option::Option<Bm>,
    #[prost(message, optional, tag = "3")]
    pub start_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub end_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub audience: ::core::option::Option<Audience>,
    #[prost(double, tag = "6")]
    pub grp: f64,
    #[prost(double, tag = "7")]
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
    pub start_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub end_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub audience: ::core::option::Option<Audience>,
    #[prost(double, tag = "6")]
    pub impressions000: f64,
    #[prost(int32, tag = "7")]
    pub frequency_cap: i32,
    #[prost(double, tag = "8")]
    pub cost_per_impressions000: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsForPlanVariantRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub predicted_buys: ::prost::alloc::vec::Vec<PredictedBuy>,
    #[prost(bool, tag = "3")]
    pub evaluate_results_total: bool,
    #[prost(bool, tag = "4")]
    pub evaluate_results_per_buy: bool,
    #[prost(bool, tag = "5")]
    pub evaluate_results_per_label: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsForPlanVariantsRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub predicted_plans: ::prost::alloc::vec::Vec<PredictedPlan>,
    #[prost(bool, tag = "3")]
    pub evaluate_results_total: bool,
    #[prost(bool, tag = "4")]
    pub evaluate_results_per_buy: bool,
    #[prost(bool, tag = "5")]
    pub evaluate_results_per_label: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictedPlan {
    #[prost(message, optional, tag = "1")]
    pub variant_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub predicted_buys: ::prost::alloc::vec::Vec<PredictedBuy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsForPlanVariantResponse {
    #[prost(message, repeated, tag = "1")]
    pub results_total: ::prost::alloc::vec::Vec<KpiResult>,
    #[prost(message, repeated, tag = "2")]
    pub results_per_buy: ::prost::alloc::vec::Vec<KpiResultForBuy>,
    #[prost(message, repeated, tag = "3")]
    pub results_per_label: ::prost::alloc::vec::Vec<KpiResultForLabel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsForPlanVariantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub variants_kpis: ::prost::alloc::vec::Vec<VariantKpis>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantKpis {
    #[prost(message, optional, tag = "1")]
    pub variant_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub results_total: ::prost::alloc::vec::Vec<KpiResult>,
    #[prost(message, repeated, tag = "3")]
    pub results_per_buy: ::prost::alloc::vec::Vec<KpiResultForBuy>,
    #[prost(message, repeated, tag = "4")]
    pub results_per_label: ::prost::alloc::vec::Vec<KpiResultForLabel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetPredictedBuysRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetPredictedBuysResponse {
    #[prost(message, repeated, tag = "1")]
    pub predicted_buys: ::prost::alloc::vec::Vec<PredictedBuy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlanReachGoalsRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlanReachGoalsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reach_goals: ::prost::alloc::vec::Vec<ReachGoal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceSetPrimaryAudienceRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub audience_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetPrimaryAudienceRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetPrimaryAudienceResponse {
    #[prost(message, optional, tag = "1")]
    pub audience_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceForceComputeStatisticsRequest {
    #[prost(message, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetEvaluationAudiencesRequest {
    #[prost(message, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetEvaluationAudiencesResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<AudiencePerType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceDetachEvaluationAudiencesRequest {
    #[prost(message, repeated, tag = "1")]
    pub audience_per_plan: ::prost::alloc::vec::Vec<AudiencePerType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceDetachEvaluationAudiencesResponse {
    #[prost(message, repeated, tag = "1")]
    pub audience_ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceAttachEvaluationAudiencesRequest {
    #[prost(message, repeated, tag = "1")]
    pub audience_per_plan: ::prost::alloc::vec::Vec<AudiencePerType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceAttachEvaluationAudiencesResponse {
    #[prost(message, repeated, tag = "1")]
    pub audience_ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsRequest {
    #[prost(message, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceGetStatisticsResponse {
    #[prost(message, repeated, tag = "1")]
    pub results_per_plan: ::prost::alloc::vec::Vec<KpiResultPerType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanServiceAttachPlanReachGoalsRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub reach_goals: ::prost::alloc::vec::Vec<AttachReachGoal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKpiMetadataResponse {
    #[prost(message, repeated, tag = "1")]
    pub kpis_metadata: ::prost::alloc::vec::Vec<KpiMetadata>,
}
/// Generated server implementations.
pub mod plan_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlanServiceServer.
    #[async_trait]
    pub trait PlanService: Send + Sync + 'static {
        /// commands
        async fn force_compute_statistics(
            &self,
            request: tonic::Request<super::PlanServiceForceComputeStatisticsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn attach_evaluation_audiences(
            &self,
            request: tonic::Request<super::PlanServiceAttachEvaluationAudiencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceAttachEvaluationAudiencesResponse>,
            tonic::Status,
        >;
        async fn detach_evaluation_audiences(
            &self,
            request: tonic::Request<super::PlanServiceDetachEvaluationAudiencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceDetachEvaluationAudiencesResponse>,
            tonic::Status,
        >;
        async fn attach_plan_reach_goals(
            &self,
            request: tonic::Request<super::PlanServiceAttachPlanReachGoalsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn set_primary_audience(
            &self,
            request: tonic::Request<super::PlanServiceSetPrimaryAudienceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn get_primary_audience(
            &self,
            request: tonic::Request<super::PlanServiceGetPrimaryAudienceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetPrimaryAudienceResponse>,
            tonic::Status,
        >;
        /// queries
        async fn get_statistics(
            &self,
            request: tonic::Request<super::PlanServiceGetStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetStatisticsResponse>,
            tonic::Status,
        >;
        async fn get_evaluation_audiences(
            &self,
            request: tonic::Request<super::PlanServiceGetEvaluationAudiencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetEvaluationAudiencesResponse>,
            tonic::Status,
        >;
        async fn get_plan_reach_goals(
            &self,
            request: tonic::Request<super::GetPlanReachGoalsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPlanReachGoalsResponse>,
            tonic::Status,
        >;
        async fn get_kpi_metadata(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::GetKpiMetadataResponse>,
            tonic::Status,
        >;
        /// long-running query
        async fn get_predicted_buys(
            &self,
            request: tonic::Request<super::PlanServiceGetPredictedBuysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetPredictedBuysResponse>,
            tonic::Status,
        >;
        async fn get_statistics_for_plan_variant(
            &self,
            request: tonic::Request<super::PlanServiceGetStatisticsForPlanVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetStatisticsForPlanVariantResponse>,
            tonic::Status,
        >;
        async fn get_statistics_for_plan_variants(
            &self,
            request: tonic::Request<
                super::PlanServiceGetStatisticsForPlanVariantsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetStatisticsForPlanVariantsResponse>,
            tonic::Status,
        >;
        async fn get_statistics_for_plan(
            &self,
            request: tonic::Request<super::PlanServiceGetStatisticsForPlanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlanServiceGetStatisticsForPlanResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlanServiceServer<T: PlanService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlanService> PlanServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlanServiceServer<T>
    where
        T: PlanService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/planoutcome.v3.PlanService/ForceComputeStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct ForceComputeStatisticsSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceForceComputeStatisticsRequest,
                    > for ForceComputeStatisticsSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceForceComputeStatisticsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::force_compute_statistics(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ForceComputeStatisticsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/AttachEvaluationAudiences" => {
                    #[allow(non_camel_case_types)]
                    struct AttachEvaluationAudiencesSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceAttachEvaluationAudiencesRequest,
                    > for AttachEvaluationAudiencesSvc<T> {
                        type Response = super::PlanServiceAttachEvaluationAudiencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceAttachEvaluationAudiencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::attach_evaluation_audiences(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AttachEvaluationAudiencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/DetachEvaluationAudiences" => {
                    #[allow(non_camel_case_types)]
                    struct DetachEvaluationAudiencesSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceDetachEvaluationAudiencesRequest,
                    > for DetachEvaluationAudiencesSvc<T> {
                        type Response = super::PlanServiceDetachEvaluationAudiencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceDetachEvaluationAudiencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::detach_evaluation_audiences(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DetachEvaluationAudiencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/AttachPlanReachGoals" => {
                    #[allow(non_camel_case_types)]
                    struct AttachPlanReachGoalsSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceAttachPlanReachGoalsRequest,
                    > for AttachPlanReachGoalsSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceAttachPlanReachGoalsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::attach_plan_reach_goals(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AttachPlanReachGoalsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/SetPrimaryAudience" => {
                    #[allow(non_camel_case_types)]
                    struct SetPrimaryAudienceSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceSetPrimaryAudienceRequest,
                    > for SetPrimaryAudienceSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceSetPrimaryAudienceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::set_primary_audience(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPrimaryAudienceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetPrimaryAudience" => {
                    #[allow(non_camel_case_types)]
                    struct GetPrimaryAudienceSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceGetPrimaryAudienceRequest,
                    > for GetPrimaryAudienceSvc<T> {
                        type Response = super::PlanServiceGetPrimaryAudienceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetPrimaryAudienceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_primary_audience(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPrimaryAudienceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatisticsSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<super::PlanServiceGetStatisticsRequest>
                    for GetStatisticsSvc<T> {
                        type Response = super::PlanServiceGetStatisticsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetStatisticsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_statistics(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatisticsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetEvaluationAudiences" => {
                    #[allow(non_camel_case_types)]
                    struct GetEvaluationAudiencesSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceGetEvaluationAudiencesRequest,
                    > for GetEvaluationAudiencesSvc<T> {
                        type Response = super::PlanServiceGetEvaluationAudiencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetEvaluationAudiencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_evaluation_audiences(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEvaluationAudiencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetPlanReachGoals" => {
                    #[allow(non_camel_case_types)]
                    struct GetPlanReachGoalsSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<super::GetPlanReachGoalsRequest>
                    for GetPlanReachGoalsSvc<T> {
                        type Response = super::GetPlanReachGoalsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPlanReachGoalsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_plan_reach_goals(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPlanReachGoalsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetKpiMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetKpiMetadataSvc<T: PlanService>(pub Arc<T>);
                    impl<T: PlanService> tonic::server::UnaryService<()>
                    for GetKpiMetadataSvc<T> {
                        type Response = super::GetKpiMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_kpi_metadata(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetKpiMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetPredictedBuys" => {
                    #[allow(non_camel_case_types)]
                    struct GetPredictedBuysSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceGetPredictedBuysRequest,
                    > for GetPredictedBuysSvc<T> {
                        type Response = super::PlanServiceGetPredictedBuysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetPredictedBuysRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_predicted_buys(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPredictedBuysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetStatisticsForPlanVariant" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatisticsForPlanVariantSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceGetStatisticsForPlanVariantRequest,
                    > for GetStatisticsForPlanVariantSvc<T> {
                        type Response = super::PlanServiceGetStatisticsForPlanVariantResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetStatisticsForPlanVariantRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_statistics_for_plan_variant(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatisticsForPlanVariantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetStatisticsForPlanVariants" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatisticsForPlanVariantsSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceGetStatisticsForPlanVariantsRequest,
                    > for GetStatisticsForPlanVariantsSvc<T> {
                        type Response = super::PlanServiceGetStatisticsForPlanVariantsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetStatisticsForPlanVariantsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_statistics_for_plan_variants(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatisticsForPlanVariantsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/planoutcome.v3.PlanService/GetStatisticsForPlan" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatisticsForPlanSvc<T: PlanService>(pub Arc<T>);
                    impl<
                        T: PlanService,
                    > tonic::server::UnaryService<
                        super::PlanServiceGetStatisticsForPlanRequest,
                    > for GetStatisticsForPlanSvc<T> {
                        type Response = super::PlanServiceGetStatisticsForPlanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PlanServiceGetStatisticsForPlanRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PlanService>::get_statistics_for_plan(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatisticsForPlanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlanService> Clone for PlanServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: PlanService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlanService> tonic::server::NamedService for PlanServiceServer<T> {
        const NAME: &'static str = "planoutcome.v3.PlanService";
    }
}
