use crate::obj::{Obj, ObjFunc};
use crate::runtime_error::RuntimeError;
use crate::vm::{ObjFactory, Vm, VmOperation};

pub(crate) struct DeclareFunc;

impl VmOperation for DeclareFunc {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        let func_name = vm.const_stack.pop()?.as_str();
        let func_const = vm.const_stack.pop()?.as_function();

        // TODO: We can improve this initialization right?
        let func = ObjFunc {
            chunk: func_const.chunk().clone(),
            name: func_const.name().clone(),
            params: func_const.params().clone(),
        };

        let func_obj = Obj::Func(ObjFactory::create(func));
        vm.scope_stack.insert(&func_name, func_obj);

        Ok(())
    }
}
