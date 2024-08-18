// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A minimal runtime that includes the inventory [`pallet`](`pallet_inventory`).

#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

extern crate alloc;

use alloc::{vec, vec::Vec};
use frame_support::genesis_builder_helper::{build_config, create_default_config};
use frame_support::{
    construct_runtime,
    weights::{
        constants::WEIGHT_REF_TIME_PER_SECOND, Weight, WeightToFeeCoefficient,
        WeightToFeeCoefficients, WeightToFeePolynomial,
    },
};

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("minimal-inventory-runtime"),
    impl_name: create_runtime_str!("minimal-inventory-runtime"),
    authoring_version: 1,
    spec_version: 0,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// The signed extensions that are added to the runtime.
type SignedExtra = (
    // Checks that the sender is not the zero address.
    frame_system::CheckNonZeroSender<Runtime>,
    // Checks that the runtime version is correct.
    frame_system::CheckSpecVersion<Runtime>,
    // Checks that the transaction version is correct.
    frame_system::CheckTxVersion<Runtime>,
    // Checks that the genesis hash is correct.
    frame_system::CheckGenesis<Runtime>,
    // Checks that the era is valid.
    frame_system::CheckEra<Runtime>,
    // Checks that the nonce is valid.
    frame_system::CheckNonce<Runtime>,
    // Checks that the weight is valid.
    frame_system::CheckWeight<Runtime>,
    // Ensures that the sender has enough funds to pay for the transaction
    // and deducts the fee from the sender's account.
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

// Composes the runtime by adding all the used pallets and deriving necessary types.
#[runtime]
mod runtime {
    /// The main runtime type.
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Runtime;

    /// Mandatory system pallet that should always be included in a FRAME runtime.
    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Runtime>;

    /// Provides a way for consensus systems to set and check the onchain time.
    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp::Pallet<Runtime>;

    /// Provides the ability to keep track of balances.
    #[runtime::pallet_index(2)]
    pub type Balances = pallet_balances::Pallet<Runtime>;

    /// Provides a way to execute privileged functions.
    #[runtime::pallet_index(3)]
    pub type Sudo = pallet_sudo::Pallet<Runtime>;

    /// Provides the ability to charge for extrinsic execution.
    #[runtime::pallet_index(4)]
    pub type TransactionPayment = pallet_transaction_payment::Pallet<Runtime>;

    /// A minimal pallet inventory.
    #[runtime::pallet_index(5)]
    pub type Inventory = pallet_inventory::Pallet<Runtime>;
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

/// Implements the types required for the system pallet.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    type Block = Block;
    type Version = Version;
    // Use the account data from the balances pallet
    type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;

    #[doc = " The aggregated event type of the runtime."]
    type RuntimeEvent;

    #[doc = " The basic call filter to use in Origin. All origins are built with this filter as base,"]
    #[doc = " except Root."]
    #[doc = ""]
    #[doc = " This works as a filter for each incoming call. The call needs to pass this filter in"]
    #[doc = " order to dispatch. Otherwise it will be rejected with `CallFiltered`. This can be"]
    #[doc = " bypassed via `dispatch_bypass_filter` which should only be accessible by root. The"]
    #[doc = " filter can be composed of sub-filters by nesting for example"]
    #[doc = " [`frame_support::traits::InsideBoth`], [`frame_support::traits::TheseExcept`] or"]
    #[doc = " [`frame_support::traits::EverythingBut`] et al. The default would be"]
    #[doc = " [`frame_support::traits::Everything`]."]
    type BaseCallFilter;

    #[doc = " Block & extrinsics weights: base values and limits."]
    type BlockWeights;

    #[doc = " The maximum length of a block (in bytes)."]
    type BlockLength;

    #[doc = " The `RuntimeOrigin` type used by dispatchable calls."]
    type RuntimeOrigin;

    #[doc = " The aggregated `RuntimeCall` type."]
    type RuntimeCall;

    #[doc = " The aggregated `RuntimeTask` type."]
    type RuntimeTask;

    #[doc = " This stores the number of previous transactions associated with a sender account."]
    type Nonce;

    #[doc = " The output of the `Hashing` function."]
    type Hash;

    #[doc = " The hashing system (algorithm) being used in the runtime (e.g. Blake2)."]
    type Hashing;

    #[doc = " The user account identifier type for the runtime."]
    type AccountId;

    #[doc = " Converting trait to take a source type and convert to `AccountId`."]
    #[doc = ""]
    #[doc = " Used to define the type and conversion mechanism for referencing accounts in"]
    #[doc = " transactions. It\'s perfectly reasonable for this to be an identity conversion (with the"]
    #[doc = " source type being `AccountId`), but other pallets (e.g. Indices pallet) may provide more"]
    #[doc = " functional/efficient alternatives."]
    type Lookup;

    #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
    type BlockHashCount;

    #[doc = " The weight of runtime database operations the runtime can invoke."]
    type DbWeight;

    #[doc = " Provides information about the pallet setup in the runtime."]
    #[doc = ""]
    #[doc = " Expects the `PalletInfo` type that is being generated by `construct_runtime!` in the"]
    #[doc = " runtime."]
    #[doc = ""]
    #[doc = " For tests it is okay to use `()` as type, however it will provide \"useless\" data."]
    type PalletInfo;

    #[doc = " Handler for when a new account has just been created."]
    type OnNewAccount;

    #[doc = " A function that is invoked when an account has been determined to be dead."]
    #[doc = ""]
    #[doc = " All resources should be cleaned up associated with the given account."]
    type OnKilledAccount;

    type SystemWeightInfo;

    #[doc = " The designated SS58 prefix of this chain."]
    #[doc = ""]
    #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
    #[doc = " that the runtime should know about the prefix in order to make use of it as"]
    #[doc = " an identifier of the chain."]
    type SS58Prefix;

    #[doc = " What to do if the runtime wants to change the code to something new."]
    #[doc = ""]
    #[doc = " The default (`()`) implementation is responsible for setting the correct storage"]
    #[doc = " entry and emitting corresponding event and log item. (see"]
    #[doc = " [`Pallet::update_code_in_storage`])."]
    #[doc = " It\'s unlikely that this needs to be customized, unless you are writing a parachain using"]
    #[doc = " `Cumulus`, where the actual code change is deferred."]
    type OnSetCode;

    #[doc = " The maximum number of consumers allowed on a single account."]
    type MaxConsumers;

    #[doc = " All migrations that should run in the next runtime upgrade."]
    #[doc = ""]
    #[doc = " These used to be formerly configured in `Executive`. Parachains need to ensure that"]
    #[doc = " running all these migrations in one block will not overflow the weight limit of a block."]
    #[doc = " The migrations are run *before* the pallet `on_runtime_upgrade` hooks, just like the"]
    #[doc = " `OnRuntimeUpgrade` migrations."]
    type SingleBlockMigrations;

    #[doc = " The migrator that is used to run Multi-Block-Migrations."]
    #[doc = ""]
    #[doc = " Can be set to [`pallet-migrations`] or an alternative implementation of the interface."]
    #[doc = " The diagram in `frame_executive::block_flowchart` explains when it runs."]
    type MultiBlockMigrator;

    #[doc = " A callback that executes in *every block* directly before all inherents were applied."]
    #[doc = ""]
    #[doc = " See `frame_executive::block_flowchart` for a in-depth explanation when it runs."]
    type PreInherents;

    #[doc = " A callback that executes in *every block* directly after all inherents were applied."]
    #[doc = ""]
    #[doc = " See `frame_executive::block_flowchart` for a in-depth explanation when it runs."]
    type PostInherents;

    #[doc = " A callback that executes in *every block* directly after all transactions were applied."]
    #[doc = ""]
    #[doc = " See `frame_executive::block_flowchart` for a in-depth explanation when it runs."]
    type PostTransactions;
}

// Implements the types required for the balances pallet.
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
    type AccountStore = System;

    #[doc = " The overarching event type."]
    type RuntimeEvent;

    #[doc = " The overarching hold reason."]
    type RuntimeHoldReason;

    #[doc = " The overarching freeze reason."]
    type RuntimeFreezeReason;

    #[doc = " Weight information for extrinsics in this pallet."]
    type WeightInfo;

    #[doc = " The balance of an account."]
    type Balance;

    #[doc = " Handler for the unbalanced reduction when removing a dust account."]
    type DustRemoval;

    #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
    #[doc = ""]
    #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
    #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
    #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
    #[doc = " behaviour if you set this to zero."]
    #[doc = ""]
    #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
    type ExistentialDeposit;

    #[doc = " The ID type for reserves."]
    #[doc = ""]
    #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
    type ReserveIdentifier;

    #[doc = " The ID type for freezes."]
    type FreezeIdentifier;

    #[doc = " The maximum number of locks that should exist on an account."]
    #[doc = " Not strictly enforced, but used for weight estimation."]
    #[doc = ""]
    #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
    type MaxLocks;

    #[doc = " The maximum number of named reserves that can exist on an account."]
    #[doc = ""]
    #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
    type MaxReserves;

    #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
    type MaxFreezes;
}

// Implements the types required for the sudo pallet.
#[derive_impl(pallet_sudo::config_preludes::TestDefaultConfig)]
impl pallet_sudo::Config for Runtime {
    #[doc = " The overarching event type."]
    type RuntimeEvent;

    #[doc = " A sudo-able call."]
    type RuntimeCall;

    #[doc = " Type representing the weight of this pallet"]
    type WeightInfo;
}

// Implements the types required for the sudo pallet.
#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
impl pallet_timestamp::Config for Runtime {
    #[doc = " Type used for expressing a timestamp."]
    type Moment;

    #[doc = " Something which can be notified (e.g. another pallet) when the timestamp is set."]
    #[doc = ""]
    #[doc = " This can be set to `()` if it is not needed."]
    type OnTimestampSet;

    #[doc = " The minimum period between blocks."]
    #[doc = ""]
    #[doc = " Be aware that this is different to the *expected* period that the block production"]
    #[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
    #[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
    #[doc = " period on default settings."]
    type MinimumPeriod;

    #[doc = " Weight information for extrinsics in this pallet."]
    type WeightInfo;
}

// Implements the types required for the transaction payment pallet.
#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig)]
impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
    // Setting fee as independent of the weight of the extrinsic for demo purposes
    type WeightToFee = NoFee<<Self as pallet_balances::Config>::Balance>;
    // Setting fee as fixed for any length of the call data for demo purposes
    type LengthToFee = FixedFee<1, <Self as pallet_balances::Config>::Balance>;

    #[doc = " The overarching event type."]
    type RuntimeEvent;

    #[doc = " Update the multiplier of the next block, based on the previous block\'s weight."]
    type FeeMultiplierUpdate;

    #[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
    #[doc = " `priority`"]
    #[doc = ""]
    #[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
    #[doc = " added to a tip component in regular `priority` calculations."]
    #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
    #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
    #[doc = ""]
    #[doc = " ```rust,ignore"]
    #[doc = " // For `Normal`"]
    #[doc = " let priority = priority_calc(tip);"]
    #[doc = ""]
    #[doc = " // For `Operational`"]
    #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
    #[doc = " let priority = priority_calc(tip + virtual_tip);"]
    #[doc = " ```"]
    #[doc = ""]
    #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
    #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
    #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
    #[doc = " transactions."]
    type OperationalFeeMultiplier;
}

// Implements the types required for the inventory pallet.
impl pallet_inventory::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type LotNumber = ConstU32<40>;
    //type Sku = ConstU32<40>;
    type Qty = ConstU32<40>;
    type Weight = ConstU32<40>;
    type PurchaseDate = ConstU32<40>;
    type ExpirationDate = ConstU32<40>;

    #[doc = " Maximum number of items in the vector"]
    type MaxItems;
}

type Block = frame::runtime::types_common::BlockOf<Runtime, SignedExtra>;
type Header = HeaderFor<Runtime>;

type RuntimeExecutive =
    Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};

impl_runtime_apis! {
    impl apis::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            RuntimeExecutive::execute_block(block)
        }

        fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
            RuntimeExecutive::initialize_block(header)
        }
    }
    impl apis::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl apis::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
            RuntimeExecutive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> HeaderFor<Runtime> {
            RuntimeExecutive::finalize_block()
        }

        fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: InherentData,
        ) -> CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl apis::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: ExtrinsicFor<Runtime>,
            block_hash: <Runtime as frame_system::Config>::Hash,
        ) -> TransactionValidity {
            RuntimeExecutive::validate_transaction(source, tx, block_hash)
        }
    }

    impl apis::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &HeaderFor<Runtime>) {
            RuntimeExecutive::offchain_worker(header)
        }
    }

    impl apis::SessionKeys<Block> for Runtime {
        fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
            Default::default()
        }

        fn decode_session_keys(
            _encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, apis::KeyTypeId)>> {
            Default::default()
        }
    }

    impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
        fn account_nonce(account: interface::AccountId) -> interface::Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        interface::Balance,
    > for Runtime {
        fn query_info(uxt: ExtrinsicFor<Runtime>, len: u32) -> RuntimeDispatchInfo<interface::Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(uxt: ExtrinsicFor<Runtime>, len: u32) -> FeeDetails<interface::Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> interface::Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> interface::Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }
}

/// Some re-exports that the node side code needs to know. Some are useful in this context as well.
///
/// Other types should preferably be private.
// TODO: this should be standardized in some way, see:
// https://github.com/paritytech/substrate/issues/10579#issuecomment-1600537558
pub mod interface {
    use super::Runtime;
    use frame::deps::frame_system;

    pub type Block = super::Block;
    pub use frame::runtime::types_common::OpaqueBlock;
    pub type AccountId = <Runtime as frame_system::Config>::AccountId;
    pub type Nonce = <Runtime as frame_system::Config>::Nonce;
    pub type Hash = <Runtime as frame_system::Config>::Hash;
    pub type Balance = <Runtime as pallet_balances::Config>::Balance;
    pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
