use std::sync::Arc;

use tokio::sync::Mutex;
use web3::{
    signing::Key,
    types::{
        Address,
        BlockNumber,
        U256,
        U64,
    },
    Transport,
    Web3,
};

use crate::blockchain::key::PrivateKey;

use super::ProxyError;

#[derive(Clone)]
pub struct Nonce {
    inner: Arc<Mutex<U256>>,
}

impl Nonce {
    pub fn new(current: U256) -> Self {
        Self {
            inner: Arc::new(Mutex::new(current)),
        }
    }

    pub async fn next(&self) -> U256 {
        let mut inner = self.inner.lock().await;
        *inner += U256::from(1);
        inner.clone()
    }
}

#[derive(Clone)]
pub struct Account<T: Transport> {
    web3: Web3<T>,
    private_key: PrivateKey,
    nonce: Nonce,
}

impl<T: Transport> Account<T> {
    pub fn new(web3: Web3<T>, private_key: PrivateKey, nonce: U256) -> Self {
        Self {
            web3,
            private_key,
            nonce: Nonce::new(nonce),
        }
    }

    pub fn private_key(&self) -> PrivateKey {
        self.private_key.clone()
    }

    pub fn address(&self) -> Address {
        self.private_key.address()
    }

    pub async fn next_nonce(&self) -> U256 {
        self.nonce.next().await
    }

    pub async fn check_for_insufficient_eth(&self, required_gas: U256, block: U64) -> Result<(), ProxyError> {
        let actual_balance = self
            .web3
            .eth()
            .balance(self.address(), Some(BlockNumber::Number(block)))
            .await?;
        let gas_price = self.web3.eth().gas_price().await?;
        let required_balance = required_gas * gas_price;
        if required_balance < actual_balance {
            return Err(ProxyError::InsufficientEth(format!(
                "Balance is not enough to execute transaction. Current: {}, required: {}",
                actual_balance, required_balance,
            )));
        }
        Ok(())
    }
}
