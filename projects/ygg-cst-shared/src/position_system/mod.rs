use pest::iterators::Pair;

mod offset;
pub use offset::OffsetRange;


pub fn get_position<T, R>(pair:&Pair<R>) -> T
    where T:PositionSystem<R>{
    PositionSystem::from(pair)
}

pub trait PositionSystem<R> {
    /// The middle way to avoid the orphan rule
    fn from(pair: &Pair<R>) -> Self;
}