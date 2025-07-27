//! Chain specification for the Subnet Zero Testnet network.

use alloc::{sync::Arc, vec};

use alloy_chains::Chain;
use alloy_primitives::{b256, U256};
use reth_chainspec::{BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_ethereum_forks::{EthereumHardfork, Hardfork};
use reth_optimism_forks::{OpHardfork, SUBNET_ZERO_TESTNET_HARDFORKS};
use reth_primitives_traits::SealedHeader;

use crate::{make_op_genesis_header, LazyLock, OpChainSpec};

/// The Subnet Zero Testnet spec
pub static SUBNET_ZERO_TESTNET: LazyLock<Arc<OpChainSpec>> = LazyLock::new(|| {
    let genesis = serde_json::from_str(include_str!("../res/genesis/subnet_zero_testnet.json"))
        .expect("Can't deserialize Subnet Zero Testnet genesis json");
    let hardforks = SUBNET_ZERO_TESTNET_HARDFORKS.clone();
    OpChainSpec {
        inner: ChainSpec {
            chain: Chain::from_id(131513),
            genesis_header: SealedHeader::new(
                make_op_genesis_header(&genesis, &hardforks),
                b256!("0x40882debb1677718942ae07c4163a7f6b5649aa6864ffcd2c48cd760230a752f"),
            ),
            genesis,
            paris_block_and_final_difficulty: Some((0, U256::from(0))),
            hardforks,
            base_fee_params: BaseFeeParamsKind::Variable(
                vec![
                    (EthereumHardfork::London.boxed(), BaseFeeParams::optimism()),
                    (OpHardfork::Canyon.boxed(), BaseFeeParams::optimism_canyon()),
                ]
                .into(),
            ),
            ..Default::default()
        },
    }
    .into()
});
