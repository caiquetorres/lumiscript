use crate::obj::{Obj, ObjTrait};
use crate::runtime_error::RuntimeError;
use crate::vm::{ObjFactory, Vm, VmOperation};

pub(crate) struct DeclareTrait;

impl VmOperation for DeclareTrait {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        let trait_name = vm.const_stack.pop()?.as_str();

        let trait_ptr = ObjFactory::create(ObjTrait::new(&trait_name));
        vm.scope_stack.insert(&trait_name, Obj::Trait(trait_ptr));

        Ok(())
    }
}
