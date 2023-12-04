use super::*;

impl<Language, Input> Iterator for ConcreteNodeIterator<Language, Input> {
    type Item = ConcreteNode<Language, Input>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.shared.arena.get(self.next?)?;
        self.prev = data.prev;
        self.next = data.next;
        Some(ConcreteNode { shared: self.shared.clone(), id: data.this })
    }
}

impl<Language, Input> DoubleEndedIterator for ConcreteNodeIterator<Language, Input> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let data = self.shared.arena.get(self.prev?)?;
        self.prev = data.prev;
        self.next = data.next;
        Some(ConcreteNode { shared: self.shared.clone(), id: data.this })
    }
}

impl<Language, Input> ConcreteNodeIterator<Language, Input> {
    pub fn skip_hidden(mut self) -> Self {
        self.skip_hidden = true;
        self
    }
    pub fn keep_hidden(mut self) -> Self {
        self.skip_hidden = false;
        self
    }
}
