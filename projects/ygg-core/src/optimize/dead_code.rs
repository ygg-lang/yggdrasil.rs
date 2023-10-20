use super::*;
use yggdrasil_error::Validation;

pub struct DeadCodeEliminator {
    pub panic: bool,
    used: HashSet<String>,
    new: HashSet<String>,
    unvisited: HashSet<String>,
}

impl Default for DeadCodeEliminator {
    fn default() -> Self {
        Self { used: Default::default(), new: Default::default(), unvisited: Default::default(), panic: true }
    }
}

impl CodeOptimizer for DeadCodeEliminator {
    // TODO: Tri-Color mark and sweep algorithm
    fn optimize(&mut self, info: &GrammarInfo) -> Validation<GrammarInfo> {
        let errors = vec![];
        self.find_entry(info);
        self.find_unvisited();
        Validation::Success { value: info.clone(), diagnostics: errors }
    }
}

impl DeadCodeEliminator {
    fn find_entry(&mut self, info: &GrammarInfo) {
        todo!()
        // for (_, rule) in &info.rules {
        //     if info.exports.contains(&rule.name) || rule.entry {
        //         self.new.insert(rule.name.to_owned());
        //     }
        // }
    }
    fn find_unvisited(&mut self) {
        self.unvisited.clear();
        for rule in self.new.difference(&self.used) {
            self.unvisited.insert(rule.to_owned());
        }
        // println!("未访问节点: {:?}", self.unvisited);
        self.new.clear();
    }
    fn find_needed(&self, info: &GrammarInfo) -> IndexMap<String, GrammarRule> {
        info.rules.iter().filter(|r| self.used.contains(r.0)).map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    fn clear(&mut self) {
        self.used.clear();
        self.new.clear();
    }
}
