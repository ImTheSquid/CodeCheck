use antlr_rust::tree::ParseTreeVisitorCompat;
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
    fn compare(&self, other: &Box<dyn SyntaxTree>) -> f64 {
        todo!()
    }

    fn worst_runtime_complexity(&self) -> crate::RuntimeComplexity {
        todo!()
    }

    fn runtime_complexity_of_fn(&self, name: &str) -> Option<crate::RuntimeComplexity> {
        todo!()
    }
}

impl TryFrom<&str> for CppTree {
    type Error = TreeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
