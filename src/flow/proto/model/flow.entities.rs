// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub balance: u64,
    #[prost(bytes="vec", tag="3")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub keys: ::prost::alloc::vec::Vec<AccountKey>,
    #[prost(map="string, bytes", tag="5")]
    pub contracts: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountKey {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(bytes="vec", tag="2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub sign_algo: u32,
    #[prost(uint32, tag="4")]
    pub hash_algo: u32,
    #[prost(uint32, tag="5")]
    pub weight: u32,
    #[prost(uint32, tag="6")]
    pub sequence_number: u32,
    #[prost(bool, tag="7")]
    pub revoked: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub height: u64,
    #[prost(message, optional, tag="4")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bytes="vec", tag="5")]
    pub payload_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub view: u64,
    /// deprecated!! value will be empty. replaced by parent_vote_indices
    #[prost(bytes="vec", repeated, tag="7")]
    pub parent_voter_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="8")]
    pub parent_voter_sig_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub proposer_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub proposer_sig_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="11")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="12")]
    pub parent_voter_indices: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="13")]
    pub last_view_tc: ::core::option::Option<TimeoutCertificate>,
    #[prost(uint64, tag="14")]
    pub parent_view: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeoutCertificate {
    #[prost(uint64, tag="1")]
    pub view: u64,
    #[prost(uint64, repeated, tag="2")]
    pub high_qc_views: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag="3")]
    pub highest_qc: ::core::option::Option<QuorumCertificate>,
    #[prost(bytes="vec", tag="4")]
    pub signer_indices: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub sig_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuorumCertificate {
    #[prost(uint64, tag="1")]
    pub view: u64,
    #[prost(bytes="vec", tag="2")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub signer_indices: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub sig_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub transaction_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionGuarantee {
    #[prost(bytes="vec", tag="1")]
    pub collection_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub reference_block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// deprecated!! value will be empty. replaced by signer_indices
    #[prost(bytes="vec", repeated, tag="5")]
    pub signer_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub signer_indices: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSeal {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub execution_receipt_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub execution_receipt_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="4")]
    pub result_approval_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="5")]
    pub final_state: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub result_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="7")]
    pub aggregated_approval_sigs: ::prost::alloc::vec::Vec<AggregatedSignature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregatedSignature {
    #[prost(bytes="vec", repeated, tag="1")]
    pub verifier_signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signer_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionResult {
    #[prost(bytes="vec", tag="1")]
    pub previous_result_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
    #[prost(message, repeated, tag="4")]
    pub service_events: ::prost::alloc::vec::Vec<ServiceEvent>,
    #[deprecated]
    #[prost(bytes="vec", tag="5")]
    pub execution_data_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(uint32, tag="1")]
    pub collection_index: u32,
    /// state when starting executing this chunk
    #[prost(bytes="vec", tag="2")]
    pub start_state: ::prost::alloc::vec::Vec<u8>,
    /// Events generated by executing results
    #[prost(bytes="vec", tag="3")]
    pub event_collection: ::prost::alloc::vec::Vec<u8>,
    /// Block id of the execution result this chunk belongs to
    #[prost(bytes="vec", tag="4")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    /// total amount of computation used by running all txs in this chunk
    #[prost(uint64, tag="5")]
    pub total_computation_used: u64,
    /// number of transactions inside the collection
    #[prost(uint32, tag="6")]
    pub number_of_transactions: u32,
    /// chunk index inside the ER (starts from zero)
    #[prost(uint64, tag="7")]
    pub index: u64,
    /// EndState inferred from next chunk or from the ER
    #[prost(bytes="vec", tag="8")]
    pub end_state: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub execution_data_id: ::prost::alloc::vec::Vec<u8>,
    /// a commitment over sorted list of register changes
    #[prost(bytes="vec", tag="10")]
    pub state_delta_commitment: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceEvent {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionReceiptMeta {
    #[prost(bytes="vec", tag="1")]
    pub executor_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub result_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub spocks: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="4")]
    pub executor_signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub parent_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub height: u64,
    #[prost(message, optional, tag="4")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, repeated, tag="5")]
    pub collection_guarantees: ::prost::alloc::vec::Vec<CollectionGuarantee>,
    #[prost(message, repeated, tag="6")]
    pub block_seals: ::prost::alloc::vec::Vec<BlockSeal>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="8")]
    pub execution_receipt_meta_list: ::prost::alloc::vec::Vec<ExecutionReceiptMeta>,
    #[prost(message, repeated, tag="9")]
    pub execution_result_list: ::prost::alloc::vec::Vec<ExecutionResult>,
    #[prost(message, optional, tag="10")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockStatus {
    BlockUnknown = 0,
    BlockFinalized = 1,
    BlockSealed = 2,
}
impl BlockStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockStatus::BlockUnknown => "BLOCK_UNKNOWN",
            BlockStatus::BlockFinalized => "BLOCK_FINALIZED",
            BlockStatus::BlockSealed => "BLOCK_SEALED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_UNKNOWN" => Some(Self::BlockUnknown),
            "BLOCK_FINALIZED" => Some(Self::BlockFinalized),
            "BLOCK_SEALED" => Some(Self::BlockSealed),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub transaction_index: u32,
    #[prost(uint32, tag="4")]
    pub event_index: u32,
    #[prost(bytes="vec", tag="5")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(bytes="vec", tag="1")]
    pub latest_finalized_block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub latest_finalized_height: u64,
    #[prost(bytes="vec", tag="3")]
    pub node_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeVersionInfo {
    #[prost(string, tag="1")]
    pub semver: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub commit: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub spork_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub protocol_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes="vec", tag="1")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub arguments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub reference_block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub gas_limit: u64,
    #[prost(message, optional, tag="5")]
    pub proposal_key: ::core::option::Option<transaction::ProposalKey>,
    #[prost(bytes="vec", tag="6")]
    pub payer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub authorizers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="8")]
    pub payload_signatures: ::prost::alloc::vec::Vec<transaction::Signature>,
    #[prost(message, repeated, tag="9")]
    pub envelope_signatures: ::prost::alloc::vec::Vec<transaction::Signature>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProposalKey {
        #[prost(bytes="vec", tag="1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, tag="2")]
        pub key_id: u32,
        #[prost(uint64, tag="3")]
        pub sequence_number: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Signature {
        #[prost(bytes="vec", tag="1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(uint32, tag="2")]
        pub key_id: u32,
        #[prost(bytes="vec", tag="3")]
        pub signature: ::prost::alloc::vec::Vec<u8>,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStatus {
    Unknown = 0,
    Pending = 1,
    Finalized = 2,
    Executed = 3,
    Sealed = 4,
    Expired = 5,
}
impl TransactionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionStatus::Unknown => "UNKNOWN",
            TransactionStatus::Pending => "PENDING",
            TransactionStatus::Finalized => "FINALIZED",
            TransactionStatus::Executed => "EXECUTED",
            TransactionStatus::Sealed => "SEALED",
            TransactionStatus::Expired => "EXPIRED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PENDING" => Some(Self::Pending),
            "FINALIZED" => Some(Self::Finalized),
            "EXECUTED" => Some(Self::Executed),
            "SEALED" => Some(Self::Sealed),
            "EXPIRED" => Some(Self::Expired),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockExecutionData {
    #[prost(bytes="vec", tag="1")]
    pub block_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub chunk_execution_data: ::prost::alloc::vec::Vec<ChunkExecutionData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkExecutionData {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<ExecutionDataCollection>,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(message, optional, tag="3")]
    pub trie_update: ::core::option::Option<TrieUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionDataCollection {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrieUpdate {
    #[prost(bytes="vec", tag="1")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="3")]
    pub payloads: ::prost::alloc::vec::Vec<Payload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(message, repeated, tag="1")]
    pub key_part: ::prost::alloc::vec::Vec<KeyPart>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPart {
    #[prost(uint32, tag="1")]
    pub r#type: u32,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
include!("flow.entities.serde.rs");
// @@protoc_insertion_point(module)