use std::collections::HashMap;
use crate::{Error,Result};


pub trait CSTNode where Self: Sized {
    fn get_str(&self) -> &str;
    fn get_span(&self) -> (usize, usize);
    fn get_range(&self) -> (usize, usize, usize, usize);
    fn get_tag_map(&self) -> HashMap<String, Vec<Self>>;
    fn get_node_tag(&self) -> Option<&str>;
    fn get_branch_tag(&self) -> Option<&str>;
}

pub trait ASTNode<N>
    where N: CSTNode, Self: Sized {
    /// many
    fn many(node: Vec<N>, errors: &mut Vec<Error>) -> Vec<Self> {
        let mut out = Vec::with_capacity(node.len());
        for pair in node {
            match Self::parse(pair, errors) {
                Ok(o) => out.push(o),
                Err(Error::Unwinding) => (),
                Err(e) => errors.push(e),
            }
        }
        return out;
    }
    /// some
    fn some(node: N, errors: &mut Vec<Error>) -> Option<Self>{
        match Self::parse(node, errors) {
            Ok(o) => return Some(o),
            Err(Error::Unwinding) => (),
            Err(e) => errors.push(e)
        }
        return None
    }
    /// one
    fn one(node: N, errors: &mut Vec<Error>) -> Result<Self> {
        match Self::parse(node, errors) {
            Ok(o) => Ok(o),
            Err(Error::Unwinding) => Err(Error::Unwinding),
            Err(e) => {
                errors.push(e);
                Err(Error::Unwinding)
            }
        }
    }
    /// many
    fn named_many(map: &mut HashMap<String, Vec<N>>, tag: &str, errors: &mut Vec<Error>) -> Vec<Self> {
        match map.remove(tag) {
            Some(s) => Self::many(s, errors),
            _ => vec![],
        }
    }
    /// some
    fn named_some(map: &mut HashMap<String, Vec<N>>, tag: &str, errors: &mut Vec<Error>) -> Option<Self> {
        match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => Self::some(s,errors),
            _ => None,
        }
    }
    /// one
    fn named_one(map: &mut HashMap<String, Vec<N>>, tag: &str, errors: &mut Vec<Error>) -> Result<Self> {
        match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => Self::one(s, errors),
            _ => Err(Error::node_tag_missing(tag)),
        }
    }
    /// parse
    fn parse(pairs: N, errors: &mut Vec<Error>) -> Result<Self>;
}

pub trait PositionSystem<N> {
    /// The middle way to avoid the orphan rule
    fn from(node: N) -> Self;
    fn join(self, rhs: Self) -> Self;
}


macro_rules! ast_node_num {
    ($t:ty) => {
        impl<N: CSTNode> ASTNode<N> for $t {
            fn parse(node: N, _: &mut Vec<Error>) -> Result<Self> {
                Ok(node.get_str().parse::<$t>()?)
            }
        }
    };
    ($($t:ty),+ $(,)?) => {
        $(ast_node_num!($t);)+
    };
}

ast_node_num![u8,u16,u32,u64,u128,usize];
ast_node_num![i8,i16,i32,i64,i128,isize];
ast_node_num![f32,f64];


impl<N: CSTNode> ASTNode<N> for String
{
    fn parse(node: N, _: &mut Vec<Error>) -> Result<Self> {
        Ok(node.get_str().to_string())
    }
}

