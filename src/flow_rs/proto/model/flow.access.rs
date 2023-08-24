// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeVersionInfoRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeVersionInfoResponse {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<super::entities::NodeVersionInfo>,
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
pub struct GetBlockHeaderByHeightRequest {
    #[prost(uint64, tag="1")]
    pub height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeaderResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<super::entities::BlockHeader>,
    #[prost(enumeration="super::entities::BlockStatus", tag="2")]
    pub block_status: i32,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
// Blocks

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockRequest {
    #[prost(bool, tag="1")]
    pub is_sealed: bool,
    #[prost(bool, tag="2")]
    pub full_block_response: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="2")]
    pub full_block_response: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightRequest {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(bool, tag="2")]
    pub full_block_response: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<super::entities::Block>,
    #[prost(enumeration="super::entities::BlockStatus", tag="2")]
    pub block_status: i32,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
// Collections

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionResponse {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<super::entities::Collection>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
// Transactions

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendTransactionRequest {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<super::entities::Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendTransactionResponse {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionRequest {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub collection_id: ::prost::alloc::vec::Vec<u8>,
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
pub struct GetTransactionsByBlockIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionResultsResponse {
    #[prost(message, repeated, tag="1")]
    pub transaction_results: ::prost::alloc::vec::Vec<TransactionResultResponse>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsResponse {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::entities::Transaction>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<super::entities::Transaction>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionResultResponse {
    #[prost(enumeration="super::entities::TransactionStatus", tag="1")]
    pub status: i32,
    #[prost(uint32, tag="2")]
    pub status_code: u32,
    #[prost(string, tag="3")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub events: ::prost::alloc::vec::Vec<super::entities::Event>,
    #[prost(bytes="vec", tag="5")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub collection_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="8")]
    pub block_height: u64,
    #[prost(message, optional, tag="9")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
// Accounts

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<super::entities::Account>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountAtLatestBlockRequest {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<super::entities::Account>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountAtBlockHeightRequest {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
}
// Scripts

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteScriptAtLatestBlockRequest {
    #[prost(bytes="vec", tag="1")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
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
pub struct ExecuteScriptAtBlockHeightRequest {
    #[prost(uint64, tag="1")]
    pub block_height: u64,
    #[prost(bytes="vec", tag="2")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteScriptResponse {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
// Events

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsForHeightRangeRequest {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub start_height: u64,
    #[prost(uint64, tag="3")]
    pub end_height: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsForBlockIDsRequest {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="2")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventsResponse {
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<events_response::Result>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
/// Nested message and enum types in `EventsResponse`.
pub mod events_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(bytes="vec", tag="1")]
        pub block_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint64, tag="2")]
        pub block_height: u64,
        #[prost(message, repeated, tag="3")]
        pub events: ::prost::alloc::vec::Vec<super::super::entities::Event>,
        #[prost(message, optional, tag="4")]
        pub block_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    }
}
// Network Parameters

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkParametersRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkParametersResponse {
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
}
// Protocol State

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestProtocolStateSnapshotRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolStateSnapshotResponse {
    #[prost(bytes="vec", tag="1")]
    pub serialized_snapshot: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
// Execution Results

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionResultForBlockIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionResultForBlockIdResponse {
    #[prost(message, optional, tag="1")]
    pub execution_result: ::core::option::Option<super::entities::ExecutionResult>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionResultByIdRequest {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionResultByIdResponse {
    #[prost(message, optional, tag="1")]
    pub execution_result: ::core::option::Option<super::entities::ExecutionResult>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::entities::Metadata>,
}
/// The request for GetExecutionDataByBlockID
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionDataByBlockIdRequest {
    /// Block ID of the block to get execution data for.
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
}
/// The response for GetExecutionDataByBlockID
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionDataByBlockIdResponse {
    /// BlockExecutionData for the block.
    #[prost(message, optional, tag="1")]
    pub block_execution_data: ::core::option::Option<super::entities::BlockExecutionData>,
}
/// The request for SubscribeExecutionData
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeExecutionDataRequest {
    /// Block ID of the first block to get execution data for.
    /// Only one of start_block_id and start_block_height may be provided,
    /// otherwise an InvalidArgument error is returned. If neither are provided,
    /// the latest sealed block is used.
    #[prost(bytes="vec", tag="1")]
    pub start_block_id: ::prost::alloc::vec::Vec<u8>,
    /// Block height of the first block to get execution data for.
    /// Only one of start_block_id and start_block_height may be provided,
    /// otherwise an InvalidArgument error is returned. If neither are provided,
    /// the latest sealed block is used.
    #[prost(uint64, tag="2")]
    pub start_block_height: u64,
}
/// The response for SubscribeExecutionData
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeExecutionDataResponse {
    /// Block height of the block containing the execution data.
    #[prost(uint64, tag="1")]
    pub block_height: u64,
    /// BlockExecutionData for the block.
    /// Note: The block's ID is included within the BlockExecutionData.
    #[prost(message, optional, tag="2")]
    pub block_execution_data: ::core::option::Option<super::entities::BlockExecutionData>,
    /// Timestamp from the block containing the execution data.
    #[prost(message, optional, tag="3")]
    pub block_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// The request for SubscribeEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeEventsRequest {
    /// Block ID of the first block to search for events.
    /// Only one of start_block_id and start_block_height may be provided,
    /// otherwise an InvalidArgument error is returned. If neither are provided,
    /// the latest sealed block is used.
    #[prost(bytes="vec", tag="1")]
    pub start_block_id: ::prost::alloc::vec::Vec<u8>,
    /// Block height of the first block to search for events.
    /// Only one of start_block_id and start_block_height may be provided,
    /// otherwise an InvalidArgument error is returned. If neither are provided,
    /// the latest sealed block is used.
    #[prost(uint64, tag="2")]
    pub start_block_height: u64,
    /// Filter to apply to events for each block searched.
    /// If no filter is provided, all events are returned.
    #[prost(message, optional, tag="3")]
    pub filter: ::core::option::Option<EventFilter>,
    /// Interval in block heights at which the server should return a heartbeat
    /// message to the client. The heartbeat is a normal SubscribeEventsResponse
    /// with no events, and allows clients to track which blocks were searched.
    /// Clients can use this information to determine which block to start from
    /// when reconnecting.
    ///
    /// The interval is calculated from the last response returned, which could be
    /// either another heartbeat or a response containing events.
    #[prost(uint64, tag="4")]
    pub heartbeat_interval: u64,
}
/// The response for SubscribeEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeEventsResponse {
    /// Block ID of the block containing the events.
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    /// Block height of the block containing the events.
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    /// Events matching the EventFilter in the request.
    /// The API may return no events which signals a periodic heartbeat. This
    /// allows clients to track which blocks were searched. Client can use this
    /// information to determine which block to start from when reconnecting.
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<super::entities::Event>,
    /// Timestamp from the block containing the events.
    #[prost(message, optional, tag="4")]
    pub block_timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// EventFilter defines the filter to apply to block events.
/// Filters are applied as an OR operation, i.e. any event matching any of the
/// filters is returned. If no filters are provided, all events are returned. If
/// there are any invalid filters, the API will return an InvalidArgument error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFilter {
    /// A list of full event types to include.
    ///
    /// All events exactly matching any of the provided event types will be
    /// returned.
    ///
    /// Event types have 2 formats:
    /// - Protocol events:
    ///      flow.[event name]
    /// - Smart contract events:
    ///      A.[contract address].[contract name].[event name]
    #[prost(string, repeated, tag="1")]
    pub event_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of contracts who's events should be included.
    ///
    /// All events emitted by any of the provided contracts will be returned.
    ///
    /// Contracts have the following name formats:
    /// - Protocol events:
    ///      flow
    /// - Smart contract events:
    ///      A.[contract address].[contract name]
    ///
    /// This filter matches on the full contract including its address, not just
    /// the contract's name.
    #[prost(string, repeated, tag="2")]
    pub contract: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of addresses who's events should be included.
    ///
    /// All events emitted by any contract held by any of the provided addresses
    /// will be returned.
    ///
    /// Addresses must be Flow account addresses in hex format and valid for the
    /// network the node is connected to. i.e. only a mainnet address is valid for
    /// a mainnet node. Addresses may optionally include the 0x prefix.
    #[prost(string, repeated, tag="3")]
    pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
include!("flow.access.serde.rs");
include!("flow.access.tonic.rs");
// @@protoc_insertion_point(module)