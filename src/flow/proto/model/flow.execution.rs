// @generated
// Ping

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
}
// Accounts

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountAtBlockIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountAtBlockIdResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<super::entities::Account>,
}
// Scripts

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteScriptAtBlockIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteScriptAtBlockIdResponse {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
// Events

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsForBlockIDsResponse {
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<get_events_for_block_i_ds_response::Result>,
    #[prost(enumeration="EventEncodingVersion", tag="2")]
    pub event_encoding_version: i32,
}
/// Nested message and enum types in `GetEventsForBlockIDsResponse`.
pub mod get_events_for_block_i_ds_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(bytes="vec", tag="1")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag="2")]
        pub block_height: u64,
        #[prost(message, repeated, tag="3")]
        pub events: ::prost::alloc::vec::Vec<super::super::entities::Event>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsForBlockIDsRequest {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="2")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
// Transactions

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResultRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionByIndexRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResultResponse {
    #[prost(uint32, tag="1")]
    pub status_code: u32,
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<super::entities::Event>,
    #[prost(enumeration="EventEncodingVersion", tag="4")]
    pub event_encoding_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsByBlockIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResultsResponse {
    #[prost(message, repeated, tag="1")]
    pub transaction_results: ::prost::alloc::vec::Vec<GetTransactionResultResponse>,
    #[prost(enumeration="EventEncodingVersion", tag="2")]
    pub event_encoding_version: i32,
}
// Registers

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegisterAtBlockIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub register_owner: ::prost::alloc::vec::Vec<u8>,
    /// bytes register_controller = 3; @deprecated
    #[prost(bytes="vec", tag="4")]
    pub register_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegisterAtBlockIdResponse {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
// Block Headers

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockHeaderRequest {
    #[prost(bool, tag="1")]
    pub is_sealed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeaderByIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeaderResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<super::entities::BlockHeader>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventEncodingVersion {
    JsonCdcV0 = 0,
    CcfV0 = 1,
}
impl EventEncodingVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventEncodingVersion::JsonCdcV0 => "JSON_CDC_V0",
            EventEncodingVersion::CcfV0 => "CCF_V0",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JSON_CDC_V0" => Some(Self::JsonCdcV0),
            "CCF_V0" => Some(Self::CcfV0),
            _ => None,
        }
    }
}
include!("flow.execution.serde.rs");
include!("flow.execution.tonic.rs");
// @@protoc_insertion_point(module)