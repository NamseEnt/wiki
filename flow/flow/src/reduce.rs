pub trait Reduce {
    fn reduce(self, event: &dyn std::any::Any) -> Self;
}
