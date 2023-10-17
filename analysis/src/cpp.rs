use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ErrorNode, ParseTreeVisitorCompat, TerminalNode};
use syntree::Tree;

use crate::gen::cpp14lexer::CPP14Lexer;
use crate::gen::cpp14parser::*;
use crate::gen::cpp14parservisitor::CPP14ParserVisitorCompat;
use crate::{SyntaxTree, VisitorReturn, TreeParseError, visitor_result};

use macros::auto_visitor;

#[derive(Debug)]
pub struct CppTree {
    /// Contains all necessary indices to reconstruct the source code from the original with
    /// symbols. This tree also contains whitespace and variable names, so it may not work as
    /// well for comparisons.
    /// TODO: Figure if non-token structure tree is needed
    pub symbol_tree: syntree::Builder<CppTreeItem, usize, usize>,
    /// Temporary variable for visitor
    tmp: VisitorReturn<()>,
}

impl Clone for CppTree{
    fn clone(&self) -> Self{
        Self{
            symbol_tree: self.symbol_tree.clone(),
            tmp: Default::default(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for CppTree {
    type Node = CPP14ParserContextType;
    type Return = VisitorReturn<()>;

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::default())
    }

    fn visit_terminal(&mut self, node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        if node.symbol.start - *self.symbol_tree.cursor() as isize > 0 {
            visitor_result!(self.symbol_tree.token(CppTreeItem::Whitespace, node.symbol.start as usize - self.symbol_tree.cursor()));
        }

        visitor_result!(self.symbol_tree.token(CppTreeItem::Terminal, node.symbol.text.len()));

        VisitorReturn(Ok(()))
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        VisitorReturn(Err(TreeParseError::InvalidNode))
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
        let lexer = CPP14Lexer::new(InputStream::new(value.as_str()));
        let mut parser = CPP14Parser::new(CommonTokenStream::new(lexer));

        let root = parser.translationUnit()?;

        let mut tree = CppTree {
            symbol_tree: Default::default(),
            tmp: Default::default(),
        };

        tree.visit(&*root).0?;

        Ok(tree)
    }
}