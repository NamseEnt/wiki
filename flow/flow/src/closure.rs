use crate::prelude::AnyClonePartialEqBox;
use std::sync::Arc;

pub fn closure<Param, Capture, Return>(
    capture: Capture,
    func: fn(&Param, &Capture) -> Option<Return>,
) -> Closure<Param, Capture, Return>
where
    Capture: std::any::Any + Clone + PartialEq + std::fmt::Debug,
{
    Closure { func, capture }
}

pub struct Closure<Param, Capture: std::any::Any + Clone + PartialEq + std::fmt::Debug, Return> {
    func: fn(&Param, &Capture) -> Option<Return>,
    capture: Capture,
}

impl<
        Param: 'static,
        Capture: std::any::Any + Clone + PartialEq + std::fmt::Debug + 'static,
        Return: 'static,
    > Closure<Param, Capture, Return>
{
    pub(crate) fn to_closured(self) -> Closured<Param> {
        Closured {
            func_ptr: self.func as *const (),
            func: Arc::new(move |param, capture| {
                let capture: &Capture = capture.downcast_ref().unwrap();
                let ret = (self.func)(param, capture);
                ret.map(|ret| Box::new(ret) as Box<dyn std::any::Any>)
            }),
            capture: AnyClonePartialEqBox::new(self.capture),
        }
    }
}

// PartialEq, Debug
#[derive(Clone)]
pub(crate) struct Closured<Param> {
    func_ptr: *const (),
    func: Arc<dyn Fn(&Param, &AnyClonePartialEqBox) -> Option<Box<dyn std::any::Any>>>,
    capture: AnyClonePartialEqBox,
}

impl<Param> PartialEq for Closured<Param> {
    fn eq(&self, other: &Self) -> bool {
        self.func_ptr == other.func_ptr && self.capture == other.capture
    }
}

impl<Param> std::fmt::Debug for Closured<Param> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Closured")
            .field("func_ptr", &self.func_ptr)
            .field("capture", &self.capture)
            .finish()
    }
}

impl<Param> Closured<Param> {
    pub(crate) fn invoke(&self, param: &Param) {
        let ret = (self.func)(param, &self.capture);
        let Some(ret) = ret else {
            return;
        };
        crate::emit_event(ret);
    }
}
