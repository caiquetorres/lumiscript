use compiler::generator::constant::Constant;

use crate::obj::{Obj, ObjPrim};
use crate::runtime_error::RuntimeError;
use crate::vm::{ObjFactory, Vm, VmOperation};

pub(crate) struct Lit;

impl VmOperation for Lit {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        let constant = vm.const_stack.pop()?;

        let obj = match constant {
            Constant::Nil => {
                let nil_class = vm.scope_stack.get("Nil")?.as_class()?;
                ObjPrim::nil(nil_class)
            }
            Constant::Bool(b) => {
                let bool_class = vm.scope_stack.get("Bool")?.as_class()?;
                ObjPrim::bool(bool_class, b)
            }
            Constant::Num(num) => {
                let num_class = vm.scope_stack.get("Num")?.as_class()?;
                ObjPrim::num(num_class, num)
            }
            _ => {
                return Err(RuntimeError::new(
                    "Object cannot be converted to a bool, number or nil primitives",
                ))
            }
        };

        vm.obj_stack.push(Obj::Prim(ObjFactory::create(obj)));

        Ok(())
    }
}
