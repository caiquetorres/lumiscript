use crate::runtime_error::RuntimeError;
use crate::vm::Vm;
use crate::vm::VmOperation;

pub(crate) struct EndScope;

impl VmOperation for EndScope {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        vm.scope_stack.pop();
        Ok(())
    }
}
