
use lsp_types::Diagnostic;

pub struct ExtraData {

}

pub struct MyVisitor<T> {
    warns: Vec<Diagnostic>,
    meta: T
}

impl MyVisitor<ExtraData> {

}

