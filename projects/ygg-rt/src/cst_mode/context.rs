pub struct ConcreteContext<K> {
    kind: std::marker::PhantomData<K>,
}

impl<K> Default for ConcreteContext<K> {
    fn default() -> Self {
        Self { kind: Default::default() }
    }
}
