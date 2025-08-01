use alloy_network::{Ethereum, EthereumWallet};
use alloy_provider::{
    Identity, RootProvider,
    fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
};
use foundry_common::provider::{
    ProviderBuilder, RetryProvider, RetryProviderWithSigner, get_http_provider,
};

pub fn http_provider(http_endpoint: &str) -> RetryProvider {
    get_http_provider(http_endpoint)
}

pub fn http_provider_with_signer(
    http_endpoint: &str,
    signer: EthereumWallet,
) -> RetryProviderWithSigner {
    ProviderBuilder::new(http_endpoint)
        .build_with_wallet(signer)
        .expect("failed to build Alloy HTTP provider with signer")
}

pub fn ws_provider_with_signer(
    ws_endpoint: &str,
    signer: EthereumWallet,
) -> RetryProviderWithSigner {
    ProviderBuilder::new(ws_endpoint)
        .build_with_wallet(signer)
        .expect("failed to build Alloy WS provider with signer")
}

/// Currently required to get around <https://github.com/alloy-rs/alloy/issues/296>
pub async fn connect_pubsub(conn_str: &str) -> RootProvider {
    alloy_provider::ProviderBuilder::default().connect(conn_str).await.unwrap()
}

type PubsubSigner = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<
                GasFiller,
                JoinFill<
                    alloy_provider::fillers::BlobGasFiller,
                    JoinFill<NonceFiller, ChainIdFiller>,
                >,
            >,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
    Ethereum,
>;

pub async fn connect_pubsub_with_wallet(conn_str: &str, wallet: EthereumWallet) -> PubsubSigner {
    alloy_provider::ProviderBuilder::new().wallet(wallet).connect(conn_str).await.unwrap()
}

pub async fn ipc_provider_with_wallet(
    ipc_endpoint: &str,
    wallet: EthereumWallet,
) -> RetryProviderWithSigner {
    ProviderBuilder::new(ipc_endpoint)
        .build_with_wallet(wallet)
        .expect("failed to build Alloy IPC provider with signer")
}
