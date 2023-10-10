#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuyReportForVariantRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub updated_input_factors: ::prost::alloc::vec::Vec<UpdateInputFactor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuyReportForVariantResponse {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, repeated, tag = "2")]
    pub buys: ::prost::alloc::vec::Vec<GenericBuy>,
    #[prost(message, repeated, tag = "3")]
    pub by_publisher: ::prost::alloc::vec::Vec<ByPublisher>,
    #[prost(message, repeated, tag = "4")]
    pub by_label: ::prost::alloc::vec::Vec<ByLabel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInputFactor {
    #[prost(message, optional, tag = "1")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    /// to get new value we take original input (grps or impressions000, depending on buy type) and multiply by factor
    /// factor >=0
    /// factor = 0 => set new input value to 0
    /// factor < 1 => decrease
    /// factor = 1 => keep original value
    /// factor > 1 => increase
    #[prost(double, tag = "2")]
    pub factor: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginPlanRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub annotation: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginPlanResponse {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyPlanRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlanVariantRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub annotation: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub source: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub updated_input_factors: ::prost::alloc::vec::Vec<UpdateInputFactor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyPlanResponse {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlanVariantResponse {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlansRequest {
    #[prost(message, repeated, tag = "1")]
    pub plan_ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
    #[prost(enumeration = "PlanStatus", repeated, tag = "2")]
    pub allowed_status_list: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuysResponse {
    #[prost(message, repeated, tag = "1")]
    pub buy_details: ::prost::alloc::vec::Vec<BuyDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyDetail {
    #[prost(oneof = "buy_detail::BuyDetail", tags = "1, 2")]
    pub buy_detail: ::core::option::Option<buy_detail::BuyDetail>,
}
/// Nested message and enum types in `BuyDetail`.
pub mod buy_detail {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BuyDetail {
        #[prost(message, tag = "1")]
        LinearBuyDetail(super::LinearBuyDetail),
        #[prost(message, tag = "2")]
        AddressableBuyDetail(super::AddressableBuyDetail),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(enumeration = "PlanStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApprovePlanRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchivePlanRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReopenPlanRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinearBuyDetail {
    #[prost(enumeration = "BuyType", tag = "1")]
    pub buy_type: i32,
    #[prost(message, optional, tag = "2")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "3")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "4")]
    pub buyable_medium_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub buying_audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(double, tag = "8")]
    pub grp: f64,
    #[prost(message, optional, tag = "9")]
    pub cost_per_grp: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressableBuyDetail {
    #[prost(enumeration = "BuyType", tag = "1")]
    pub buy_type: i32,
    #[prost(message, optional, tag = "2")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "3")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "4")]
    pub buyable_medium_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub buying_audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(double, tag = "8")]
    pub impressions_000: f64,
    /// frequency_cap = 0 represent frequency_cap to be null
    #[prost(int32, tag = "9")]
    pub frequency_cap: i32,
    #[prost(message, optional, tag = "10")]
    pub cost_per_thousand_impressions: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlanDescriptionRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlanBudgetRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub budget: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlanNameRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePlanRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAddressableBuyRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(double, tag = "4")]
    pub impressions_000: f64,
    #[prost(message, optional, tag = "5")]
    pub buyable_medium_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "6")]
    pub buying_audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(int32, tag = "7")]
    pub frequency_cap: i32,
    #[prost(message, optional, tag = "8")]
    pub cost_per_thousand_impressions: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLinearBuyRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(double, tag = "4")]
    pub grp: f64,
    #[prost(message, optional, tag = "5")]
    pub buyable_medium_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "6")]
    pub buying_audience_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "7")]
    pub cost_per_grp: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAddressableBuyResponse {
    #[prost(message, optional, tag = "1")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLinearBuyResponse {
    #[prost(message, optional, tag = "1")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuysRequest {
    #[prost(message, repeated, tag = "1")]
    pub plan_ids: ::prost::alloc::vec::Vec<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBuyRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuyInputGrpRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(double, tag = "3")]
    pub grp: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuyInputImpressions000Request {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(double, tag = "3")]
    pub impressions_000: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCostPerGrpRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "3")]
    pub cost: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCostPerThousandImpressionsRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "3")]
    pub cost: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlansResponse {
    #[prost(message, repeated, tag = "1")]
    pub details: ::prost::alloc::vec::Vec<PlanDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllPlansResponse {
    #[prost(message, repeated, tag = "1")]
    pub details: ::prost::alloc::vec::Vec<PlanDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuyingPeriodRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "3")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuyingAudienceRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "3")]
    pub buying_audience_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFrequencyCappingRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "2")]
    pub buy_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(int32, tag = "3")]
    pub frequency_cap: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanDetail {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub modified_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int32, tag = "6")]
    pub number_of_buys: i32,
    #[prost(enumeration = "PlanStatus", tag = "7")]
    pub status: i32,
    #[prost(message, optional, tag = "8")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "9")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "10")]
    pub annotation: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub budget: ::core::option::Option<super::super::common::v1::Money>,
    #[prost(string, tag = "12")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub modified_by: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub source: ::prost::alloc::string::String,
    #[prost(int64, tag = "15")]
    pub version: i64,
    #[prost(int64, tag = "16")]
    pub owning_tenant_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlanAnnotationRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub annotation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArchivedPlansResponse {
    #[prost(message, repeated, tag = "1")]
    pub details: ::prost::alloc::vec::Vec<PlanDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlanSourceRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuyReportRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuyReportResponse {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub buys: ::prost::alloc::vec::Vec<GenericBuy>,
    #[prost(message, repeated, tag = "5")]
    pub by_publisher: ::prost::alloc::vec::Vec<ByPublisher>,
    #[prost(message, repeated, tag = "6")]
    pub by_label: ::prost::alloc::vec::Vec<ByLabel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyableMedium {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub publisher: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    #[prost(enumeration = "BuyableMediumType", tag = "5")]
    pub r#type: i32,
    #[prost(int32, tag = "6")]
    pub granularity_score: i32,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub market: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub created_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericBuy {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(enumeration = "BuyType", tag = "2")]
    pub buy_type: i32,
    #[prost(message, optional, tag = "3")]
    pub buyable_medium: ::core::option::Option<BuyableMedium>,
    #[prost(message, optional, tag = "4")]
    pub buying_audience: ::core::option::Option<Audience>,
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "7")]
    pub input_type: ::prost::alloc::string::String,
    #[prost(double, tag = "8")]
    pub input: f64,
    #[prost(message, optional, tag = "9")]
    pub total_cost: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByPublisher {
    #[prost(string, tag = "1")]
    pub publisher: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub total_cost: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByLabel {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub total_cost: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinearBuysOverviewRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinearBuysOverviewResponse {
    #[prost(message, repeated, tag = "1")]
    pub buys: ::prost::alloc::vec::Vec<LinearBuy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressableBuysOverviewRequest {
    #[prost(message, optional, tag = "1")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressableBuysOverviewResponse {
    #[prost(message, repeated, tag = "1")]
    pub buys: ::prost::alloc::vec::Vec<AddressableBuy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinearBuy {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(enumeration = "BuyType", tag = "2")]
    pub buy_type: i32,
    #[prost(message, optional, tag = "3")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "4")]
    pub buyable_medium: ::core::option::Option<BuyableMedium>,
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub buying_audience: ::core::option::Option<Audience>,
    #[prost(double, tag = "8")]
    pub grp: f64,
    #[prost(message, optional, tag = "9")]
    pub cost_per_grp: ::core::option::Option<super::super::common::v1::Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressableBuy {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(enumeration = "BuyType", tag = "2")]
    pub buy_type: i32,
    #[prost(message, optional, tag = "3")]
    pub plan_id: ::core::option::Option<super::super::common::v1::Guid>,
    #[prost(message, optional, tag = "4")]
    pub buyable_medium: ::core::option::Option<BuyableMedium>,
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub audience: ::core::option::Option<Audience>,
    #[prost(double, tag = "8")]
    pub impressions_000: f64,
    /// frequency_cap = 0 represent frequency_cap to be null
    #[prost(int32, tag = "9")]
    pub frequency_cap: i32,
    #[prost(message, optional, tag = "10")]
    pub cost_per_thousand_impressions: ::core::option::Option<super::super::common::v1::Money>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlanStatus {
    Unspecified = 0,
    Draft = 1,
    Approved = 2,
    Archived = 3,
    ToBeDeleted = 4,
}
impl PlanStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlanStatus::Unspecified => "PLAN_STATUS_UNSPECIFIED",
            PlanStatus::Draft => "PLAN_STATUS_DRAFT",
            PlanStatus::Approved => "PLAN_STATUS_APPROVED",
            PlanStatus::Archived => "PLAN_STATUS_ARCHIVED",
            PlanStatus::ToBeDeleted => "PLAN_STATUS_TO_BE_DELETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLAN_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PLAN_STATUS_DRAFT" => Some(Self::Draft),
            "PLAN_STATUS_APPROVED" => Some(Self::Approved),
            "PLAN_STATUS_ARCHIVED" => Some(Self::Archived),
            "PLAN_STATUS_TO_BE_DELETED" => Some(Self::ToBeDeleted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BuyType {
    Unspecified = 0,
    LinearBuy = 1,
    AddressableBuy = 2,
}
impl BuyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BuyType::Unspecified => "BUY_TYPE_UNSPECIFIED",
            BuyType::LinearBuy => "BUY_TYPE_LINEAR_BUY",
            BuyType::AddressableBuy => "BUY_TYPE_ADDRESSABLE_BUY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BUY_TYPE_LINEAR_BUY" => Some(Self::LinearBuy),
            "BUY_TYPE_ADDRESSABLE_BUY" => Some(Self::AddressableBuy),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BuyableMediumType {
    Unspecified = 0,
    Tactical = 1,
    HighlyTactical = 2,
    Strategical = 3,
    HighlyStrategical = 4,
}
impl BuyableMediumType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BuyableMediumType::Unspecified => "BUYABLE_MEDIUM_TYPE_UNSPECIFIED",
            BuyableMediumType::Tactical => "BUYABLE_MEDIUM_TYPE_TACTICAL",
            BuyableMediumType::HighlyTactical => "BUYABLE_MEDIUM_TYPE_HIGHLY_TACTICAL",
            BuyableMediumType::Strategical => "BUYABLE_MEDIUM_TYPE_STRATEGICAL",
            BuyableMediumType::HighlyStrategical => "BUYABLE_MEDIUM_TYPE_HIGHLY_STRATEGICAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUYABLE_MEDIUM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BUYABLE_MEDIUM_TYPE_TACTICAL" => Some(Self::Tactical),
            "BUYABLE_MEDIUM_TYPE_HIGHLY_TACTICAL" => Some(Self::HighlyTactical),
            "BUYABLE_MEDIUM_TYPE_STRATEGICAL" => Some(Self::Strategical),
            "BUYABLE_MEDIUM_TYPE_HIGHLY_STRATEGICAL" => Some(Self::HighlyStrategical),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod plan_build_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PlanBuildServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlanBuildServiceClient<tonic::transport::Channel> {
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
    impl<T> PlanBuildServiceClient<T>
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
        ) -> PlanBuildServiceClient<InterceptedService<T, F>>
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
            PlanBuildServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn begin_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::BeginPlanRequest>,
        ) -> std::result::Result<tonic::Response<super::BeginPlanResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/BeginPlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "BeginPlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_plan_description(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePlanDescriptionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdatePlanDescription",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdatePlanDescription",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_plan_annotation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePlanAnnotationRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdatePlanAnnotation",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdatePlanAnnotation",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_plan_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePlanBudgetRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdatePlanBudget",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdatePlanBudget",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_plan_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePlanNameRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdatePlanName",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdatePlanName",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePlanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/DeletePlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "DeletePlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn approve_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::ApprovePlanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/ApprovePlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "ApprovePlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn archive_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchivePlanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/ArchivePlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "ArchivePlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reopen_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::ReopenPlanRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/ReopenPlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "ReopenPlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_status(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateStatusRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/UpdateStatus");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateStatus",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn copy_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::CopyPlanRequest>,
        ) -> std::result::Result<tonic::Response<super::CopyPlanResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/CopyPlan");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("planbuild.v2.PlanBuildService", "CopyPlan"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_plan_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePlanSourceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdatePlanSource",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdatePlanSource",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_plan_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePlanVariantRequest>,
        ) -> std::result::Result<tonic::Response<super::CreatePlanVariantResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/CreatePlanVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "CreatePlanVariant",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_linear_buy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddLinearBuyRequest>,
        ) -> std::result::Result<tonic::Response<super::AddLinearBuyResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/AddLinearBuy");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "AddLinearBuy",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_addressable_buy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddAddressableBuyRequest>,
        ) -> std::result::Result<tonic::Response<super::AddAddressableBuyResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/AddAddressableBuy",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "AddAddressableBuy",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_buy_input_grp(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBuyInputGrpRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateBuyInputGrp",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateBuyInputGrp",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_buy_input_impressions000(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBuyInputImpressions000Request>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateBuyInputImpressions000",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateBuyInputImpressions000",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_cost_per_grp(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCostPerGrpRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateCostPerGrp",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateCostPerGrp",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_cost_per_thousand_impressions(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCostPerThousandImpressionsRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateCostPerThousandImpressions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateCostPerThousandImpressions",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_buying_audience(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBuyingAudienceRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateBuyingAudience",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateBuyingAudience",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_buying_period(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBuyingPeriodRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateBuyingPeriod",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateBuyingPeriod",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_frequency_capping(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFrequencyCappingRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/UpdateFrequencyCapping",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "UpdateFrequencyCapping",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_buy(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveBuyRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/RemoveBuy");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "RemoveBuy",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_buys(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBuysRequest>,
        ) -> std::result::Result<tonic::Response<super::GetBuysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/GetBuys");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("planbuild.v2.PlanBuildService", "GetBuys"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_plans(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPlansRequest>,
        ) -> std::result::Result<tonic::Response<super::GetPlansResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/GetPlans");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("planbuild.v2.PlanBuildService", "GetPlans"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_all_plans(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::GetAllPlansResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/GetAllPlans");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "GetAllPlans",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_archived_plans(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<tonic::Response<super::GetArchivedPlansResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/GetArchivedPlans",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "GetArchivedPlans",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_buy_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBuyReportRequest>,
        ) -> std::result::Result<tonic::Response<super::GetBuyReportResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/planbuild.v2.PlanBuildService/GetBuyReport");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "GetBuyReport",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_linear_buys_overview(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLinearBuysOverviewRequest>,
        ) -> std::result::Result<tonic::Response<super::GetLinearBuysOverviewResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/GetLinearBuysOverview",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "GetLinearBuysOverview",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_addressable_buys_overview(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAddressableBuysOverviewRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAddressableBuysOverviewResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/GetAddressableBuysOverview",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "GetAddressableBuysOverview",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_buy_report_for_variant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBuyReportForVariantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBuyReportForVariantResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/planbuild.v2.PlanBuildService/GetBuyReportForVariant",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "planbuild.v2.PlanBuildService",
                "GetBuyReportForVariant",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
