use eyre::Result;
use ssz_rs::prelude::*;
use superstruct::superstruct;

use self::{
    primitives::{ByteList, ByteVector, U64},
    utils::{header_deserialize, superstruct_ssz, u256_deserialize},
};

pub mod primitives;
mod utils;

pub type Address = ByteVector<20>;
pub type Bytes32 = ByteVector<32>;
pub type LogsBloom = ByteVector<256>;
pub type BLSPubKey = ByteVector<48>;
pub type SignatureBytes = ByteVector<96>;
pub type Transaction = ByteList<1073741824>;

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct BeaconBlock {
    pub slot: U64,
    pub proposer_index: U64,
    pub parent_root: Bytes32,
    pub state_root: Bytes32,
    pub body: BeaconBlockBody,
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb),
    variant_attributes(
        derive(
            serde::Deserialize,
            serde::Serialize,
            Clone,
            Debug,
            SimpleSerialize,
            Default
        ),
        serde(deny_unknown_fields)
    )
)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub struct BeaconBlockBody {
    randao_reveal: SignatureBytes,
    eth1_data: Eth1Data,
    graffiti: Bytes32,
    proposer_slashings: List<ProposerSlashing, 16>,
    attester_slashings: List<AttesterSlashing, 2>,
    attestations: List<Attestation, 128>,
    deposits: List<Deposit, 16>,
    voluntary_exits: List<SignedVoluntaryExit, 16>,
    sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
    #[superstruct(only(Capella, Deneb))]
    bls_to_execution_changes: List<SignedBlsToExecutionChange, 16>,
    #[superstruct(only(Deneb))]
    blob_kzg_commitments: List<ByteVector<48>, 4096>,
}

impl Default for BeaconBlockBody {
    fn default() -> Self {
        BeaconBlockBody::Bellatrix(BeaconBlockBodyBellatrix::default())
    }
}

superstruct_ssz!(BeaconBlockBody);

#[derive(Default, Clone, Debug, SimpleSerialize, serde::Deserialize, serde::Serialize)]
pub struct SignedBlsToExecutionChange {
    message: BlsToExecutionChange,
    signature: SignatureBytes,
}

#[derive(Default, Clone, Debug, SimpleSerialize, serde::Deserialize, serde::Serialize)]
pub struct BlsToExecutionChange {
    validator_index: U64,
    from_bls_pubkey: BLSPubKey,
    to_execution_address: Address,
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb),
    variant_attributes(
        derive(
            serde::Deserialize,
            serde::Serialize,
            Debug,
            Default,
            SimpleSerialize,
            Clone
        ),
        serde(deny_unknown_fields)
    )
)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub struct ExecutionPayload {
    pub parent_hash: Bytes32,
    pub fee_recipient: Address,
    pub state_root: Bytes32,
    pub receipts_root: Bytes32,
    pub logs_bloom: LogsBloom,
    pub prev_randao: Bytes32,
    pub block_number: U64,
    pub gas_limit: U64,
    pub gas_used: U64,
    pub timestamp: U64,
    pub extra_data: ByteList<32>,
    #[serde(deserialize_with = "u256_deserialize")]
    pub base_fee_per_gas: U256,
    pub block_hash: Bytes32,
    pub transactions: List<Transaction, 1048576>,
    #[superstruct(only(Capella, Deneb))]
    withdrawals: List<Withdrawal, 16>,
    #[superstruct(only(Deneb))]
    blob_gas_used: U64,
    #[superstruct(only(Deneb))]
    excess_blob_gas: U64,
}

impl Default for ExecutionPayload {
    fn default() -> Self {
        ExecutionPayload::Bellatrix(ExecutionPayloadBellatrix::default())
    }
}

superstruct_ssz!(ExecutionPayload);

#[derive(Default, Clone, Debug, SimpleSerialize, serde::Deserialize, serde::Serialize)]
pub struct Withdrawal {
    index: U64,
    validator_index: U64,
    address: Address,
    amount: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct ProposerSlashing {
    signed_header_1: SignedBeaconBlockHeader,
    signed_header_2: SignedBeaconBlockHeader,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
struct SignedBeaconBlockHeader {
    message: BeaconBlockHeader,
    signature: SignatureBytes,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
struct BeaconBlockHeader {
    slot: U64,
    proposer_index: U64,
    parent_root: Bytes32,
    state_root: Bytes32,
    body_root: Bytes32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct AttesterSlashing {
    attestation_1: IndexedAttestation,
    attestation_2: IndexedAttestation,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
struct IndexedAttestation {
    attesting_indices: List<U64, 2048>,
    data: AttestationData,
    signature: SignatureBytes,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct Attestation {
    aggregation_bits: Bitlist<2048>,
    data: AttestationData,
    signature: SignatureBytes,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
struct AttestationData {
    slot: U64,
    index: U64,
    beacon_block_root: Bytes32,
    source: Checkpoint,
    target: Checkpoint,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
struct Checkpoint {
    epoch: U64,
    root: Bytes32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct SignedVoluntaryExit {
    message: VoluntaryExit,
    signature: SignatureBytes,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
struct VoluntaryExit {
    epoch: U64,
    validator_index: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct Deposit {
    proof: Vector<Bytes32, 33>,
    data: DepositData,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug, SimpleSerialize, Clone)]
struct DepositData {
    pubkey: BLSPubKey,
    withdrawal_credentials: Bytes32,
    amount: U64,
    signature: SignatureBytes,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, SimpleSerialize, Clone)]
pub struct Eth1Data {
    deposit_root: Bytes32,
    deposit_count: U64,
    block_hash: Bytes32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Bootstrap {
    #[serde(deserialize_with = "header_deserialize")]
    pub header: Header,
    pub current_sync_committee: SyncCommittee,
    pub current_sync_committee_branch: Vec<Bytes32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Update {
    #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: Header,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<Bytes32>,
    #[serde(deserialize_with = "header_deserialize")]
    pub finalized_header: Header,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

/// This struct is an equivalent to the `Update` struct, but with different header deserialization.
/// The deserialization function `header_deserialize` is only deserializing `Header` from json.
/// Since dWallet-network has a limit for transaction's input size, we need to send the updates as
/// bcs rather than json.
/// This is why we have this struct, which is used for deserializing the updates from bcs.
/// We cannot give up on any of the structs since when we fetch updates from RPC, it is in json
/// format. The same goes for `FinalityUpdateSerde`, `OptimisticUpdateSerde` and
/// `UpdatesResponseSerde` as they are also sent to dWallet-network as parameters to a function.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct UpdateSerde {
    pub attested_header: Header,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<Bytes32>,
    pub finalized_header: Header,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct FinalityUpdate {
    #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: Header,
    #[serde(deserialize_with = "header_deserialize")]
    pub finalized_header: Header,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct FinalityUpdateSerde {
    pub attested_header: Header,
    pub finalized_header: Header,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct OptimisticUpdate {
    #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: Header,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct OptimisticUpdateSerde {
    pub attested_header: Header,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default, SimpleSerialize)]
pub struct Header {
    pub slot: U64,
    pub proposer_index: U64,
    pub parent_root: Bytes32,
    pub state_root: Bytes32,
    pub body_root: Bytes32,
}

#[derive(Debug, Clone, Default, SimpleSerialize, serde::Deserialize, serde::Serialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubKey, 512>,
    pub aggregate_pubkey: BLSPubKey,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default, SimpleSerialize)]
pub struct SyncAggregate {
    pub sync_committee_bits: Bitvector<512>,
    pub sync_committee_signature: SignatureBytes,
}

pub struct GenericUpdate {
    pub attested_header: Header,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: u64,
    pub next_sync_committee: Option<SyncCommittee>,
    pub next_sync_committee_branch: Option<Vec<Bytes32>>,
    pub finalized_header: Option<Header>,
    pub finality_branch: Option<Vec<Bytes32>>,
}

impl From<&Update> for GenericUpdate {
    fn from(update: &Update) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: Some(update.next_sync_committee.clone()),
            next_sync_committee_branch: Some(update.next_sync_committee_branch.clone()),
            finalized_header: Some(update.finalized_header.clone()),
            finality_branch: Some(update.finality_branch.clone()),
        }
    }
}

impl From<&FinalityUpdate> for GenericUpdate {
    fn from(update: &FinalityUpdate) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: Some(update.finalized_header.clone()),
            finality_branch: Some(update.finality_branch.clone()),
        }
    }
}

impl From<&OptimisticUpdate> for GenericUpdate {
    fn from(update: &OptimisticUpdate) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: None,
            finality_branch: None,
        }
    }
}

impl From<Update> for UpdateSerde {
    fn from(value: Update) -> UpdateSerde {
        UpdateSerde {
            attested_header: value.attested_header,
            next_sync_committee: value.next_sync_committee,
            next_sync_committee_branch: value.next_sync_committee_branch,
            finalized_header: value.finalized_header,
            finality_branch: value.finality_branch,
            sync_aggregate: value.sync_aggregate,
            signature_slot: value.signature_slot,
        }
    }
}

impl From<FinalityUpdate> for FinalityUpdateSerde {
    fn from(value: FinalityUpdate) -> FinalityUpdateSerde {
        FinalityUpdateSerde {
            attested_header: value.attested_header,
            finalized_header: value.finalized_header,
            finality_branch: value.finality_branch,
            sync_aggregate: value.sync_aggregate,
            signature_slot: value.signature_slot,
        }
    }
}

impl From<OptimisticUpdate> for OptimisticUpdateSerde {
    fn from(value: OptimisticUpdate) -> OptimisticUpdateSerde {
        OptimisticUpdateSerde {
            attested_header: value.attested_header,
            sync_aggregate: value.sync_aggregate,
            signature_slot: value.signature_slot,
        }
    }
}

impl Into<Update> for UpdateSerde {
    fn into(self) -> Update {
        Update {
            attested_header: self.attested_header,
            next_sync_committee: self.next_sync_committee,
            next_sync_committee_branch: self.next_sync_committee_branch,
            finalized_header: self.finalized_header,
            finality_branch: self.finality_branch,
            sync_aggregate: self.sync_aggregate,
            signature_slot: self.signature_slot,
        }
    }
}

impl Into<FinalityUpdate> for FinalityUpdateSerde {
    fn into(self) -> FinalityUpdate {
        FinalityUpdate {
            attested_header: self.attested_header,
            finalized_header: self.finalized_header,
            finality_branch: self.finality_branch,
            sync_aggregate: self.sync_aggregate,
            signature_slot: self.signature_slot,
        }
    }
}

impl Into<OptimisticUpdate> for OptimisticUpdateSerde {
    fn into(self) -> OptimisticUpdate {
        OptimisticUpdate {
            attested_header: self.attested_header,
            sync_aggregate: self.sync_aggregate,
            signature_slot: self.signature_slot,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub(crate) struct UpdatesResponse {
    pub updates: Vec<Update>,
    pub finality_update: FinalityUpdate,
    pub optimistic_update: OptimisticUpdate,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct UpdatesResponseSerde {
    pub updates: Vec<UpdateSerde>,
    pub finality_update: FinalityUpdateSerde,
    pub optimistic_update: OptimisticUpdateSerde,
}

impl Default for UpdatesResponse {
    fn default() -> Self {
        UpdatesResponse {
            updates: vec![],
            finality_update: Default::default(),
            optimistic_update: Default::default(),
        }
    }
}

impl From<UpdatesResponseSerde> for UpdatesResponse {
    fn from(value: UpdatesResponseSerde) -> UpdatesResponse {
        UpdatesResponse {
            updates: value
                .updates
                .into_iter()
                .map(|update| update.into())
                .collect(),
            finality_update: value.finality_update.into(),
            optimistic_update: value.optimistic_update.into(),
        }
    }
}

impl Into<UpdatesResponseSerde> for UpdatesResponse {
    fn into(self) -> UpdatesResponseSerde {
        UpdatesResponseSerde {
            updates: self
                .updates
                .into_iter()
                .map(|update| update.into())
                .collect(),
            finality_update: self.finality_update.into(),
            optimistic_update: self.optimistic_update.into(),
        }
    }
}

impl UpdatesResponse {
    pub fn deserialize_from_bytes(bytes: Vec<u8>) -> Result<UpdatesResponse, anyhow::Error> {
        let updates_response_serde: UpdatesResponseSerde = bcs::from_bytes(&bytes)?;
        Ok(updates_response_serde.into())
    }
}
