pub trait Reduce {
    fn reduce(self, event: &dyn std::any::Any) -> Self;
}

pub trait ViewModel<View>: serde::Serialize + serde::de::DeserializeOwned {
    fn reduce(self, event: &dyn std::any::Any) -> Self;
    fn as_view(&self) -> View;
}
