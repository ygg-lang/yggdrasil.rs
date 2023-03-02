use super::*;
use yggdrasil_ir::YggdrasilError;

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
        self.find_entry(info);
        self.find_unvisited();
        let mut errors = vec![];
        while !self.unvisited.is_empty() {
            for rule in &self.unvisited {
                match info.rules.get(rule) {
                    Some(s) => {
                        let mut new = HashSet::new();
                        s.get_field_names(&mut new);
                        self.used.insert(rule.to_owned());
                        self.new.extend(new.iter().map(|s| s.to_string()))
                    }
                    None => {
                        let error = format!("Undefined rule {}", rule);
                        if self.panic { panic!("{}", error) } else { errors.push(YggdrasilError::runtime_error(error)) }
                    }
                }
            }
            self.find_unvisited();
        }
        let rules = self.find_needed(info);
        self.clear();
        Validation::Success { value: GrammarInfo { rules, ..info.clone() }, diagnostics: errors }
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
