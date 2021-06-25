use std::collections::HashMap;
use crate::{Error,Result};

/// It's a node contained in the Lossless Concrete Syntax Tree
/// All subsequent required information will be retained
/// Including spaces, line breaks, and comments or other semantically irrelevant content.
/// Macros and formatting can start at this level
pub trait CSTNode where Self: Sized {
    /// get str of the node
    fn get_str(&self) -> &str;
    /// Provide basic location information
    /// (start_offset, end_offset)
    fn get_span(&self) -> (usize, usize);
    /// Provide detailed row and column ranges
    /// (start_line, start_col, end_line, end_col)
    fn get_range(&self) -> (usize, usize, usize, usize);
    /// Get the tag of the current node
    fn get_node_tag(&self) -> Option<&'static str>;
    /// Get the tag of the current branch
    fn get_branch_tag(&self) -> Option<&'static str>;
    /// Find node tags in all of the children
    /// Then collect then into the vec, and store in hashmap with tag name
    fn get_tag_map(&self) -> HashMap<String, Vec<Self>>;
}


pub trait CSTNode2 where Self: Sized {
    fn get_str<'i>(&self,input: &'i str) -> &'i str;
    fn get_span(&self, input:&str) -> (usize, usize);
    fn get_range(&self, input:&str) -> (usize, usize, usize, usize);
    fn get_node_tag(&self) -> Option<&'static str>;
    fn get_branch_tag(&self) -> Option<&'static str>;
    fn get_tag_map(&self) -> HashMap<String, Vec<Self>>;
}

/// It's a node contained in the Strongly Typed Abstract Syntax Tree
/// Implement the `parse` method to express how to become a typed node
/// Semantic analysis, interpreter, LSP can start at this level
pub trait ASTNode<N>
    where N: CSTNode, Self: Sized {
    /// This node will appear multiple times
    /// If the child node fails, it will be abandoned
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
    /// This node is optional
    /// If the parsing fails, it will be abandoned and return null
    fn some(node: N, errors: &mut Vec<Error>) -> Option<Self>{
        match Self::parse(node, errors) {
            Ok(o) => return Some(o),
            Err(Error::Unwinding) => (),
            Err(e) => errors.push(e)
        }
        return None
    }
    /// This node is required
    /// If it fails, it will start to roll back
    /// Destruct level by level, until a node can tolerate this problem
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

