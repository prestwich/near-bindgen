use crate::environment::blockchain_interface::BlockchainInterface;
use near_vm_logic::mocks::mock_external::MockedExternal;
use near_vm_logic::mocks::mock_memory::MockedMemory;
use near_vm_logic::types::PromiseResult;
use near_vm_logic::{Config, External, MemoryLike, VMContext, VMLogic};
use std::cell::RefCell;

/// Mocked blockchain that can be used in the tests for the smart contracts.
/// It implements `BlockchainInterface` by redirecting calls to `VMLogic`. It unwraps errors of
/// `VMLogic` to cause panic during the unit test similarly to how errors of `VMLogic` would cause
/// the termination of guest program execution. Unit tests can even assert the expected error
/// message.
pub struct MockedBlockchain {
    logic: RefCell<VMLogic<'static>>,
    // We keep ownership over logic fixture so that references in `VMLogic` are valid.
    #[allow(dead_code)]
    logic_fixture: LogicFixture,
}

struct LogicFixture {
    ext: Box<dyn External>,
    memory: Box<dyn MemoryLike>,
    promise_results: Box<Vec<PromiseResult>>,
    config: Box<Config>,
}

impl MockedBlockchain {
    pub fn new(context: VMContext, config: Config, promise_results: Vec<PromiseResult>) -> Self {
        let ext = Box::new(MockedExternal::new());
        let memory = Box::new(MockedMemory::new());
        let promise_results = Box::new(promise_results);
        let config = Box::new(config);

        let mut logic_fixture = LogicFixture { ext, memory, config, promise_results };

        let logic = unsafe {
            VMLogic::new(
                &mut *(logic_fixture.ext.as_mut() as *mut dyn External),
                context,
                & *(logic_fixture.config.as_mut() as *const Config),
                & *(logic_fixture.promise_results.as_ref().as_slice() as *const [PromiseResult]),
                &mut *(logic_fixture.memory.as_mut() as *mut dyn MemoryLike),
            )
        };

        let logic = RefCell::new(logic);
        Self { logic, logic_fixture }
    }
}

impl BlockchainInterface for MockedBlockchain {
    unsafe fn read_register(&self, register_id: u64, ptr: u64) {
        self.logic.borrow_mut().read_register(register_id, ptr).unwrap()
    }

    unsafe fn register_len(&self, register_id: u64) -> u64 {
        self.logic.borrow_mut().register_len(register_id).unwrap()
    }

    unsafe fn current_account_id(&self, register_id: u64) {
        self.logic.borrow_mut().current_account_id(register_id).unwrap()
    }

    unsafe fn signer_account_id(&self, register_id: u64) {
        self.logic.borrow_mut().signer_account_id(register_id).unwrap()
    }

    unsafe fn signer_account_pk(&self, register_id: u64) {
        self.logic.borrow_mut().signer_account_pk(register_id).unwrap()
    }

    unsafe fn predecessor_account_id(&self, register_id: u64) {
        self.logic.borrow_mut().predecessor_account_id(register_id).unwrap()
    }

    unsafe fn input(&self, register_id: u64) {
        self.logic.borrow_mut().input(register_id).unwrap()
    }

    unsafe fn block_index(&self) -> u64 {
        self.logic.borrow_mut().block_index().unwrap()
    }

    unsafe fn storage_usage(&self) -> u64 {
        self.logic.borrow_mut().storage_usage().unwrap()
    }

    unsafe fn account_balance(&self, balance_ptr: u64) {
        self.logic.borrow_mut().account_balance(balance_ptr).unwrap()
    }

    unsafe fn attached_deposit(&self, balance_ptr: u64) {
        self.logic.borrow_mut().attached_deposit(balance_ptr).unwrap()
    }

    unsafe fn prepaid_gas(&self) -> u64 {
        self.logic.borrow_mut().prepaid_gas().unwrap()
    }

    unsafe fn used_gas(&self) -> u64 {
        self.logic.borrow_mut().used_gas().unwrap()
    }

    unsafe fn random_seed(&self, register_id: u64) {
        self.logic.borrow_mut().random_seed(register_id).unwrap()
    }

    unsafe fn sha256(&self, value_len: u64, value_ptr: u64, register_id: u64) {
        self.logic.borrow_mut().sha256(value_len, value_ptr, register_id).unwrap()
    }

    unsafe fn value_return(&self, value_len: u64, value_ptr: u64) {
        self.logic.borrow_mut().value_return(value_len, value_ptr).unwrap()
    }

    unsafe fn panic(&self) {
        self.logic.borrow_mut().panic().unwrap()
    }

    unsafe fn log_utf8(&self, len: u64, ptr: u64) {
        self.logic.borrow_mut().log_utf8(len, ptr).unwrap()
    }

    unsafe fn log_utf16(&self, len: u64, ptr: u64) {
        self.logic.borrow_mut().log_utf16(len, ptr).unwrap()
    }

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
    ) -> u64 {
        self.logic
            .borrow_mut()
            .promise_create(
                account_id_len,
                account_id_ptr,
                method_name_len,
                method_name_ptr,
                arguments_len,
                arguments_ptr,
                amount_ptr,
                gas,
            )
            .unwrap()
    }

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
    ) -> u64 {
        self.logic
            .borrow_mut()
            .promise_then(
                promise_index,
                account_id_len,
                account_id_ptr,
                method_name_len,
                method_name_ptr,
                arguments_len,
                arguments_ptr,
                amount_ptr,
                gas,
            )
            .unwrap()
    }

    unsafe fn promise_and(&self, promise_idx_ptr: u64, promise_idx_count: u64) -> u64 {
        self.logic.borrow_mut().promise_and(promise_idx_ptr, promise_idx_count).unwrap()
    }

    unsafe fn promise_results_count(&self) -> u64 {
        self.logic.borrow_mut().promise_results_count().unwrap()
    }

    unsafe fn promise_result(&self, result_idx: u64, register_id: u64) -> u64 {
        self.logic.borrow_mut().promise_result(result_idx, register_id).unwrap()
    }

    unsafe fn promise_return(&self, promise_id: u64) {
        self.logic.borrow_mut().promise_return(promise_id).unwrap()
    }

    unsafe fn storage_write(
        &self,
        key_len: u64,
        key_ptr: u64,
        value_len: u64,
        value_ptr: u64,
        register_id: u64,
    ) -> u64 {
        self.logic
            .borrow_mut()
            .storage_write(key_len, key_ptr, value_len, value_ptr, register_id)
            .unwrap()
    }

    unsafe fn storage_read(&self, key_len: u64, key_ptr: u64, register_id: u64) -> u64 {
        self.logic.borrow_mut().storage_read(key_len, key_ptr, register_id).unwrap()
    }

    unsafe fn storage_remove(&self, key_len: u64, key_ptr: u64, register_id: u64) -> u64 {
        self.logic.borrow_mut().storage_remove(key_len, key_ptr, register_id).unwrap()
    }

    unsafe fn storage_has_key(&self, key_len: u64, key_ptr: u64) -> u64 {
        self.logic.borrow_mut().storage_has_key(key_len, key_ptr).unwrap()
    }

    unsafe fn storage_iter_prefix(&self, prefix_len: u64, prefix_ptr: u64) -> u64 {
        self.logic.borrow_mut().storage_iter_prefix(prefix_len, prefix_ptr).unwrap()
    }

    unsafe fn storage_iter_range(
        &self,
        start_len: u64,
        start_ptr: u64,
        end_len: u64,
        end_ptr: u64,
    ) -> u64 {
        self.logic.borrow_mut().storage_iter_range(start_len, start_ptr, end_len, end_ptr).unwrap()
    }

    unsafe fn storage_iter_next(
        &self,
        iterator_id: u64,
        key_register_id: u64,
        value_register_id: u64,
    ) -> u64 {
        self.logic
            .borrow_mut()
            .storage_iter_next(iterator_id, key_register_id, value_register_id)
            .unwrap()
    }
}
