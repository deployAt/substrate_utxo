use super::Aura;
use codec::{Decode, Encode};
use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::{DispatchResult, Vec},
    ensure,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::sr25519::{Public, Signature};
use sp_core::{H256, H512};
use sp_runtime::traits::{BlakeTwo256, Hash, SaturatedConversion};
use sp_runtime::transaction_validity::{TransactionLongevity, ValidTransaction};
use sp_std::collections::btree_map::BTreeMap;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

#[cfg(feature = "std", derive(Serialize, Deserialize))]
[#derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct TransactionInput {
  pub outpoint: H256,
  pub sigscript: H512, //proof
}

#[cfg(feature = "std", derive(Serialize, Deserialize))]
[#derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct TransactionOutput {
  pub value: Value,
  pub pubkey: H256,
}

#[cfg(feature = "std", derive(Serialize, Deserialize))]
[#derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, Hash, Debug)]
pub struct Transaction {
  pub inputs: Vector<TransactionInput>,
  pub output: Vec<TransactionOutput>,
}

decl_storage! {
    trait Store for Module<T: Trait> as Utxo {
      UtxoStore build(|config: &GenesisConfig| {
        config.genesis_utxos
        .iter()
        .cloned()
        .map(|u| (BlakeTwo256::hash_of(&u), u))
        .collect::<Vec<_>>()
      }): map hasher(identity) H256 => Option<TransactionOutput>;

      pub RewardTotal get(reward_total): Value;
    }

    add_extra_genesis {
      config(genesis_utxos): Vec<TransactionOutputs>;
    }
}

// External functions: callable by the end user
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn spend(_origin, transaction: Transaction) -> DispatchResult {

        }

    }
}

decl_event! {
    pub enum Event {

    }
}

/// Tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    use frame_support::{
        assert_err, assert_ok, impl_outer_origin, parameter_types, weights::Weight,
    };
    use sp_core::testing::{KeyStore, SR25519};
    use sp_core::traits::KeystoreExt;
    use sp_runtime::{testing::Header, traits::IdentityLookup, Perbill};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const MaximumBlockWeight: Weight = 1024;
            pub const MaximumBlockLength: u32 = 2 * 1024;
            pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    }
    impl system::Trait for Test {
        type Origin = Origin;
        type Call = ();
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type BlockHashCount = BlockHashCount;
        type MaximumBlockWeight = MaximumBlockWeight;
        type MaximumBlockLength = MaximumBlockLength;
        type AvailableBlockRatio = AvailableBlockRatio;
        type Version = ();
        type ModuleToIndex = ();
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
    }
    impl Trait for Test {
        type Event = ();
    }

    type Utxo = Module<Test>;
}
