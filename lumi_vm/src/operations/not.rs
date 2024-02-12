use std::collections::HashMap;

use crate::call_frame::CallFrame;
use crate::obj::Obj;
use crate::runtime_error::RuntimeError;
use crate::vm::{Vm, VmOperation};

pub(crate) struct Not;

impl VmOperation for Not {
    fn exec(vm: &mut Vm) -> Result<(), RuntimeError> {
        let operant = vm.obj_stack.pop()?;

        let neg_trait = vm.scope_stack.get("Neg")?.as_trait()?;

        if let Some(class_ptr) = operant.class_ptr() {
            if vm.scope_stack.has_impl(class_ptr, neg_trait) {
                let method = vm.scope_stack.method(class_ptr, "neg").unwrap();

                let mut slots = HashMap::new();
                slots.insert("this".to_owned(), operant);
                slots.insert("This".to_owned(), Obj::Class(class_ptr));

                match method {
                    Obj::Func(func) => {
                        vm.frame_stack.push(CallFrame::new(func, slots));
                        vm.set_frame_has_changed(true);
                    }
                    Obj::NativeFunc(native_func) => unsafe {
                        let native_func = &*native_func;
                        vm.obj_stack.push((native_func.func)(slots)?);
                    },
                    _ => unreachable!(),
                }

                Ok(())
            } else {
                Err(RuntimeError::new("Neg trait not implemented"))
            }
        } else {
            Err(RuntimeError::new("Neg trait not implemented"))
        }
    }
}
