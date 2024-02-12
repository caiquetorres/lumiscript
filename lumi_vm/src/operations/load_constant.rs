use crate::runtime_error::RuntimeError;
use crate::vm::Vm;

pub(crate) struct LoadConstant;

impl LoadConstant {
    /// Loads a constant from the current frame's constant pool onto the
    /// constant stack.
    ///
    /// # Parameters
    /// - `pos`: The position/index of the constant in the constant pool.
    /// - `vm`: A mutable reference to the virtual machine (`Vm`) where the
    /// operation is performed.
    ///
    /// # Returns
    /// - `Ok(())`: If the constant is successfully loaded onto the constant
    /// stack.
    /// - `Err(RuntimeError)`: If there is an error, such as constant stack
    /// underflow or an empty call frame stack.
    pub(crate) fn exec(pos: usize, vm: &mut Vm) -> Result<(), RuntimeError> {
        if let Some(current_frame) = vm.frame_stack.current() {
            let constant = current_frame
                .constant(pos)
                .ok_or(RuntimeError::new("Constant stack underflow"))?;

            vm.const_stack.push(constant.clone());
            Ok(())
        } else {
            Err(RuntimeError::new("Empty call frame stack"))
        }
    }
}
