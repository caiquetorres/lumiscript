use crate::runtime_error::RuntimeError;
use crate::vm::{Vm, VmOperation};

pub(crate) struct Pop;

impl VmOperation for Pop {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        vm.obj_stack.pop()?;
        Ok(())
    }
}
