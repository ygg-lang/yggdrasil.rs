use std::collections::HashSet;

use crate::{
    rule::node::ExpressionKind,
    traits::{FieldCount, FieldDescriptor},
    ChoiceExpression, ConcatExpression, DataKind, ExpressionNode, GrammarRule, RuleReference, UnaryExpression,
};

impl FieldDescriptor for GrammarRule {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.body.get_field_names(buffer)
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}

impl FieldDescriptor for ExpressionNode {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.kind.get_field_names(buffer)
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}

impl FieldDescriptor for ExpressionKind {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        match self {
            ExpressionKind::Choice(e) => e.get_field_names(buffer),
            ExpressionKind::Concat(e) => e.get_field_names(buffer),
            ExpressionKind::Unary(e) => e.get_field_names(buffer),
            ExpressionKind::Data(e) => e.get_field_names(buffer),
            ExpressionKind::Rule(e) => e.get_field_names(buffer),
        }
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}

impl FieldDescriptor for ChoiceExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.branches.iter().for_each(|f| f.get_field_names(buffer))
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}

impl FieldDescriptor for ConcatExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.sequence.iter().for_each(|f| f.get_field_names(buffer))
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}

impl FieldDescriptor for UnaryExpression {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        self.base.get_field_names(buffer)
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}

impl FieldDescriptor for DataKind {
    fn get_field_names<'a>(&'a self, _: &mut HashSet<&'a String>) {}

    fn get_field_count(&self, _: &mut HashSet<String, FieldCount>) {}
}

impl FieldDescriptor for RuleReference {
    fn get_field_names<'a>(&'a self, buffer: &mut HashSet<&'a String>) {
        buffer.insert(&self.name);
    }

    fn get_field_count(&self, buffer: &mut HashSet<String, FieldCount>) {
        todo!()
    }
}
