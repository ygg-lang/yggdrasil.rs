use super::*;
use crate::data::RuleReference;

impl FieldDescriptor for GrammarRule {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        // self.body.get_field_names(buffer)
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount2>) {
        // self.body.get_field_count(buffer)
    }
}

impl FieldDescriptor for YggdrasilExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.kind.get_field_names(buffer)
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount2>) {
        self.kind.get_field_count(buffer)
    }
}

impl FieldDescriptor for ExpressionKind {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        match self {
            ExpressionKind::Choice(e) => e.get_field_names(buffer),
            ExpressionKind::Concat(e) => e.get_field_names(buffer),
            ExpressionKind::Unary(e) => e.get_field_names(buffer),
            ExpressionKind::Rule(e) => e.get_field_names(buffer),
            ExpressionKind::Function(e) => e.get_field_names(buffer),
            ExpressionKind::Regex(_) => {
                todo!()
            }
            _ => {}
        }
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount2>) {
        match self {
            ExpressionKind::Choice(e) => e.get_field_count(buffer),
            ExpressionKind::Concat(e) => e.get_field_count(buffer),
            ExpressionKind::Unary(e) => e.get_field_count(buffer),
            ExpressionKind::Rule(e) => e.get_field_count(buffer),
            ExpressionKind::Function(e) => e.get_field_count(buffer),
            ExpressionKind::Regex(_) => {
                todo!()
            }
            _ => {}
        }
    }
}

impl FieldDescriptor for FunctionExpression {
    fn get_field_names<'a>(&'a self, _buffer: &mut HashSet<&'a String>) {
        todo!()
    }

    fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
        todo!()
    }
}

impl FieldDescriptor for ChoiceExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.branches.iter().for_each(|f| f.get_field_names(buffer))
    }

    fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
        todo!()
    }
}

impl FieldDescriptor for ConcatExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        todo!()
    }

    fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
        todo!()
    }
}

impl FieldDescriptor for UnaryExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.base.get_field_names(buffer)
    }

    fn get_field_count(&self, _buffer: &mut HashSet<String, FieldCount2>) {
        todo!()
    }
}

impl FieldDescriptor for RuleReference {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        buffer.insert(&self.name.text);
    }

    fn get_field_count(&self, _: &mut HashSet<String, FieldCount2>) {
        unreachable!()
    }
}
