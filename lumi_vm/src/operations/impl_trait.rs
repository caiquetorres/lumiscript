use crate::runtime_error::RuntimeError;
use crate::vm::{Vm, VmOperation};

pub(crate) struct ImplTrait;

impl VmOperation for ImplTrait {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        let trait_name = vm.const_stack.pop()?.as_str();
        let class_name = vm.const_stack.pop()?.as_str();

        let tr = vm.scope_stack.get(&trait_name)?.as_trait()?;
        let cls = vm.scope_stack.get(&class_name)?.as_class()?;

        vm.scope_stack.set_impl(cls, tr);

        Ok(())
    }
}
