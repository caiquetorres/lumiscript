use crate::runtime_error::RuntimeError;
use crate::vm::Vm;
use crate::vm::VmOperation;

pub(crate) struct BeginScope;

impl VmOperation for BeginScope {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        vm.scope_stack.push(); // creates a new scope.

        if let Some(current) = vm.frame_stack.current() {
            for (key, object) in current.slots() {
                vm.scope_stack.insert(key, object.clone());
            }
            Ok(())
        } else {
            Err(RuntimeError::new("Empty call frame stack"))
        }
    }
}
