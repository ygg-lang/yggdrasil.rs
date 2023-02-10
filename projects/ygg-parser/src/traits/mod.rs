use std::rc::Rc;

pub trait Extractor<R>
where
    Self: Sized,
{
    fn take(node: Option<Rc<R>>) -> Option<Self> {
        Self::take_one(&*node?)
    }

    fn take_one(node: &R) -> Option<Self>;

    fn take_one_of(nodes: Vec<Option<Rc<R>>>) -> Option<Self> {
        for term in nodes {
            let finish = Self::take(term);
            if finish.is_some() {
                return finish;
            }
        }
        return None;
    }

    fn take_many(nodes: &[Rc<R>]) -> Vec<Self> {
        let mut out = Vec::with_capacity(nodes.len());
        for node in nodes {
            if let Some(s) = Self::take_one(node) {
                out.push(s);
            }
        }
        out
    }
}
