/// A low-level interface of either real or mocked blockchain that contract interacts with.
pub trait BlockchainInterface {
    // #############
    // # Registers #
    // #############
    unsafe fn read_register(&self, register_id: u64, ptr: u64);
    unsafe fn register_len(&self, register_id: u64) -> u64;
    // ###############
    // # Context API #
    // ###############
    unsafe fn current_account_id(&self, register_id: u64);
    unsafe fn signer_account_id(&self, register_id: u64);
    unsafe fn signer_account_pk(&self, register_id: u64);
    unsafe fn predecessor_account_id(&self, register_id: u64);
    unsafe fn input(&self, register_id: u64);
    unsafe fn block_index(&self) -> u64;
    unsafe fn storage_usage(&self) -> u64;
    // #################
    // # Economics API #
    // #################
    unsafe fn account_balance(&self, balance_ptr: u64);
    unsafe fn attached_deposit(&self, balance_ptr: u64);
    unsafe fn prepaid_gas(&self) -> u64;
    unsafe fn used_gas(&self) -> u64;
    // ############
    // # Math API #
    // ############
    unsafe fn random_seed(&self, register_id: u64);
    unsafe fn sha256(&self, value_len: u64, value_ptr: u64, register_id: u64);
    // #####################
    // # Miscellaneous API #
    // #####################
    unsafe fn value_return(&self, value_len: u64, value_ptr: u64);
    unsafe fn panic(&self);
    unsafe fn log_utf8(&self, len: u64, ptr: u64);
    unsafe fn log_utf16(&self, len: u64, ptr: u64);
    // ################
    // # Promises API #
    // ################
    unsafe fn promise_create(
        &self,
        account_id_len: u64,
        account_id_ptr: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64,
    ) -> u64;
    unsafe fn promise_then(
        &self,
        promise_index: u64,
        account_id_len: u64,
        account_id_ptr: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64,
    ) -> u64;
    unsafe fn promise_and(&self, promise_idx_ptr: u64, promise_idx_count: u64) -> u64;
    unsafe fn promise_results_count(&self) -> u64;
    unsafe fn promise_result(&self, result_idx: u64, register_id: u64) -> u64;
    unsafe fn promise_return(&self, promise_id: u64);
    // ###############
    // # Storage API #
    // ###############
    unsafe fn storage_write(
        &self,
        key_len: u64,
        key_ptr: u64,
        value_len: u64,
        value_ptr: u64,
        register_id: u64,
    ) -> u64;
    unsafe fn storage_read(&self, key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    unsafe fn storage_remove(&self, key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    unsafe fn storage_has_key(&self, key_len: u64, key_ptr: u64) -> u64;
    unsafe fn storage_iter_prefix(&self, prefix_len: u64, prefix_ptr: u64) -> u64;
    unsafe fn storage_iter_range(
        &self,
        start_len: u64,
        start_ptr: u64,
        end_len: u64,
        end_ptr: u64,
    ) -> u64;
    unsafe fn storage_iter_next(
        &self,
        iterator_id: u64,
        key_register_id: u64,
        value_register_id: u64,
    ) -> u64;
}
