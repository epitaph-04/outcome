#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Guid {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    /// Start date is inclusive
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<Date>,
    /// End date is exclusive
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<Date>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Date {
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    #[prost(int32, tag = "1")]
    pub year: i32,
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    #[prost(int32, tag = "2")]
    pub month: i32,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0
    /// if specifying a year by itself or a year and month where the day is not
    /// significant.
    #[prost(int32, tag = "3")]
    pub day: i32,
}
/// Based on <https://github.com/googleapis/googleapis/blob/master/google/type/money.proto>
/// Represents an amount of money with its currency type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[prost(string, tag = "1")]
    pub currency_code: ::prost::alloc::string::String,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    #[prost(int64, tag = "2")]
    pub units: i64,
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    #[prost(int32, tag = "3")]
    pub nanos: i32,
}
