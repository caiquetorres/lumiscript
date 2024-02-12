use crate::obj::{Obj, ObjFunc};
use crate::runtime_error::RuntimeError;
use crate::vm::{ObjFactory, Vm, VmOperation};

pub(crate) struct DeclareMethod;

impl VmOperation for DeclareMethod {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        let class_name = vm.const_stack.pop()?.as_str();
        let func_name = vm.const_stack.pop()?.as_str();
        let func_const = vm.const_stack.pop()?.as_function();

        // TODO: We can improve this initialization right?
        let func = ObjFunc {
            chunk: func_const.chunk().clone(),
            name: func_const.name().clone(),
            params: func_const.params().clone(),
        };

        let class_ptr = vm.scope_stack.get(&class_name)?.as_class()?;
        let method_obj = Obj::Func(ObjFactory::create(func));
        vm.scope_stack.set_method(class_ptr, &func_name, method_obj);

        Ok(())
    }
}
