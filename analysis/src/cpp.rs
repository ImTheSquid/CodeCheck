use antlr_rust::tree::ParseTreeVisitorCompat;
use syntree::Tree;
use macros::auto_visitor;

use crate::gen::cpp14parser::*;
use crate::visitor_result;
use crate::gen::cpp14parservisitor::CPP14ParserVisitorCompat;
use crate::{VisitorReturn, SyntaxTree, TreeParseError};

#[derive(Debug, Clone)]
pub struct CppTree {
    symbol_tree: syntree::Builder<CppTreeItem, usize, usize>,
}

impl ParseTreeVisitorCompat<'_> for CppTree {
    type Node = CPP14ParserContextType;
    type Return = VisitorReturn<()>;

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::default())
    }
}

auto_visitor!(cpp14parser, CppTree, CppTreeItem);

impl SyntaxTree for CppTree {
    type Item = CppTreeItem;
    fn symbol_tree(self) -> anyhow::Result<Tree<Self::Item, usize, usize>, TreeParseError> {
        Ok(self.symbol_tree.build()?)
    }
}

impl TryFrom<String> for CppTree {
    type Error = TreeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
