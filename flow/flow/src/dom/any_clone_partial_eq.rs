pub(crate) trait AnyClonePartialEq {
    fn clone_box(&self) -> Box<dyn AnyClonePartialEq>;
    fn equals(&self, other: &dyn AnyClonePartialEq) -> bool;
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static + std::any::Any + Clone + PartialEq> AnyClonePartialEq for T {
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
}
