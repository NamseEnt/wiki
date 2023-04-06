use std::fmt::Debug;

pub(crate) trait AnyClonePartialEq {
    fn clone_box(&self) -> Box<dyn AnyClonePartialEq>;
    fn equals(&self, other: &dyn AnyClonePartialEq) -> bool;
    fn as_any(&self) -> &dyn std::any::Any;
    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T: 'static + std::any::Any + Clone + PartialEq + Debug> AnyClonePartialEq for T {
    fn clone_box(&self) -> Box<dyn AnyClonePartialEq> {
        Box::new(Clone::clone(self))
    }
    fn equals(&self, other: &dyn AnyClonePartialEq) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
