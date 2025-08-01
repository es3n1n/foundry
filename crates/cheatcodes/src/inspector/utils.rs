use super::Ecx;
use crate::inspector::Cheatcodes;
use alloy_primitives::{Address, Bytes, U256};
use revm::interpreter::{CreateInputs, CreateScheme};

/// Common behaviour of legacy and EOF create inputs.
pub(crate) trait CommonCreateInput {
    fn caller(&self) -> Address;
    fn gas_limit(&self) -> u64;
    fn value(&self) -> U256;
    fn init_code(&self) -> Bytes;
    fn scheme(&self) -> Option<CreateScheme>;
    fn set_caller(&mut self, caller: Address);
    fn log_debug(&self, cheatcode: &mut Cheatcodes, scheme: &CreateScheme);
    fn allow_cheatcodes(&self, cheatcodes: &mut Cheatcodes, ecx: Ecx) -> Address;
}

impl CommonCreateInput for &mut CreateInputs {
    fn caller(&self) -> Address {
        self.caller
    }
    fn gas_limit(&self) -> u64 {
        self.gas_limit
    }
    fn value(&self) -> U256 {
        self.value
    }
    fn init_code(&self) -> Bytes {
        self.init_code.clone()
    }
    fn scheme(&self) -> Option<CreateScheme> {
        Some(self.scheme)
    }
    fn set_caller(&mut self, caller: Address) {
        self.caller = caller;
    }
    fn log_debug(&self, cheatcode: &mut Cheatcodes, scheme: &CreateScheme) {
        let kind = match scheme {
            CreateScheme::Create => "create",
            CreateScheme::Create2 { .. } => "create2",
            CreateScheme::Custom { .. } => "custom",
        };
        debug!(target: "cheatcodes", tx=?cheatcode.broadcastable_transactions.back().unwrap(), "broadcastable {kind}");
    }
    fn allow_cheatcodes(&self, cheatcodes: &mut Cheatcodes, ecx: Ecx) -> Address {
        let old_nonce = ecx
            .journaled_state
            .state
            .get(&self.caller)
            .map(|acc| acc.info.nonce)
            .unwrap_or_default();
        let created_address = self.created_address(old_nonce);
        cheatcodes.allow_cheatcodes_on_create(ecx, self.caller, created_address);
        created_address
    }
}
