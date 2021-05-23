use pest::iterators::Pair;

mod offset;


pub fn get_position<R>(pair:&Pair<R>) -> Self
    where R:PositionSystem<R>{
    PositionSystem::from(pair)
}

pub trait PositionSystem<R> {
    /// The middle way to avoid the orphan rule
    fn from(pair: &Pair<R>) -> Self;
}