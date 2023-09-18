// Generated from Python3Parser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::python3parserlistener::*;
use super::python3parservisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const INDENT: isize = 1;
pub const DEDENT: isize = 2;
pub const STRING: isize = 3;
pub const NUMBER: isize = 4;
pub const INTEGER: isize = 5;
pub const AND: isize = 6;
pub const AS: isize = 7;
pub const ASSERT: isize = 8;
pub const ASYNC: isize = 9;
pub const AWAIT: isize = 10;
pub const BREAK: isize = 11;
pub const CASE: isize = 12;
pub const CLASS: isize = 13;
pub const CONTINUE: isize = 14;
pub const DEF: isize = 15;
pub const DEL: isize = 16;
pub const ELIF: isize = 17;
pub const ELSE: isize = 18;
pub const EXCEPT: isize = 19;
pub const FALSE: isize = 20;
pub const FINALLY: isize = 21;
pub const FOR: isize = 22;
pub const FROM: isize = 23;
pub const GLOBAL: isize = 24;
pub const IF: isize = 25;
pub const IMPORT: isize = 26;
pub const IN: isize = 27;
pub const IS: isize = 28;
pub const LAMBDA: isize = 29;
pub const MATCH: isize = 30;
pub const NONE: isize = 31;
pub const NONLOCAL: isize = 32;
pub const NOT: isize = 33;
pub const OR: isize = 34;
pub const PASS: isize = 35;
pub const RAISE: isize = 36;
pub const RETURN: isize = 37;
pub const TRUE: isize = 38;
pub const TRY: isize = 39;
pub const UNDERSCORE: isize = 40;
pub const WHILE: isize = 41;
pub const WITH: isize = 42;
pub const YIELD: isize = 43;
pub const NEWLINE: isize = 44;
pub const NAME: isize = 45;
pub const STRING_LITERAL: isize = 46;
pub const BYTES_LITERAL: isize = 47;
pub const DECIMAL_INTEGER: isize = 48;
pub const OCT_INTEGER: isize = 49;
pub const HEX_INTEGER: isize = 50;
pub const BIN_INTEGER: isize = 51;
pub const FLOAT_NUMBER: isize = 52;
pub const IMAG_NUMBER: isize = 53;
pub const DOT: isize = 54;
pub const ELLIPSIS: isize = 55;
pub const STAR: isize = 56;
pub const OPEN_PAREN: isize = 57;
pub const CLOSE_PAREN: isize = 58;
pub const COMMA: isize = 59;
pub const COLON: isize = 60;
pub const SEMI_COLON: isize = 61;
pub const POWER: isize = 62;
pub const ASSIGN: isize = 63;
pub const OPEN_BRACK: isize = 64;
pub const CLOSE_BRACK: isize = 65;
pub const OR_OP: isize = 66;
pub const XOR: isize = 67;
pub const AND_OP: isize = 68;
pub const LEFT_SHIFT: isize = 69;
pub const RIGHT_SHIFT: isize = 70;
pub const ADD: isize = 71;
pub const MINUS: isize = 72;
pub const DIV: isize = 73;
pub const MOD: isize = 74;
pub const IDIV: isize = 75;
pub const NOT_OP: isize = 76;
pub const OPEN_BRACE: isize = 77;
pub const CLOSE_BRACE: isize = 78;
pub const LESS_THAN: isize = 79;
pub const GREATER_THAN: isize = 80;
pub const EQUALS: isize = 81;
pub const GT_EQ: isize = 82;
pub const LT_EQ: isize = 83;
pub const NOT_EQ_1: isize = 84;
pub const NOT_EQ_2: isize = 85;
pub const AT: isize = 86;
pub const ARROW: isize = 87;
pub const ADD_ASSIGN: isize = 88;
pub const SUB_ASSIGN: isize = 89;
pub const MULT_ASSIGN: isize = 90;
pub const AT_ASSIGN: isize = 91;
pub const DIV_ASSIGN: isize = 92;
pub const MOD_ASSIGN: isize = 93;
pub const AND_ASSIGN: isize = 94;
pub const OR_ASSIGN: isize = 95;
pub const XOR_ASSIGN: isize = 96;
pub const LEFT_SHIFT_ASSIGN: isize = 97;
pub const RIGHT_SHIFT_ASSIGN: isize = 98;
pub const POWER_ASSIGN: isize = 99;
pub const IDIV_ASSIGN: isize = 100;
pub const SKIP_: isize = 101;
pub const UNKNOWN_CHAR: isize = 102;
pub const RULE_single_input: usize = 0;
pub const RULE_file_input: usize = 1;
pub const RULE_eval_input: usize = 2;
pub const RULE_decorator: usize = 3;
pub const RULE_decorators: usize = 4;
pub const RULE_decorated: usize = 5;
pub const RULE_async_funcdef: usize = 6;
pub const RULE_funcdef: usize = 7;
pub const RULE_parameters: usize = 8;
pub const RULE_typedargslist: usize = 9;
pub const RULE_tfpdef: usize = 10;
pub const RULE_varargslist: usize = 11;
pub const RULE_vfpdef: usize = 12;
pub const RULE_stmt: usize = 13;
pub const RULE_simple_stmts: usize = 14;
pub const RULE_simple_stmt: usize = 15;
pub const RULE_expr_stmt: usize = 16;
pub const RULE_annassign: usize = 17;
pub const RULE_testlist_star_expr: usize = 18;
pub const RULE_augassign: usize = 19;
pub const RULE_del_stmt: usize = 20;
pub const RULE_pass_stmt: usize = 21;
pub const RULE_flow_stmt: usize = 22;
pub const RULE_break_stmt: usize = 23;
pub const RULE_continue_stmt: usize = 24;
pub const RULE_return_stmt: usize = 25;
pub const RULE_yield_stmt: usize = 26;
pub const RULE_raise_stmt: usize = 27;
pub const RULE_import_stmt: usize = 28;
pub const RULE_import_name: usize = 29;
pub const RULE_import_from: usize = 30;
pub const RULE_import_as_name: usize = 31;
pub const RULE_dotted_as_name: usize = 32;
pub const RULE_import_as_names: usize = 33;
pub const RULE_dotted_as_names: usize = 34;
pub const RULE_dotted_name: usize = 35;
pub const RULE_global_stmt: usize = 36;
pub const RULE_nonlocal_stmt: usize = 37;
pub const RULE_assert_stmt: usize = 38;
pub const RULE_compound_stmt: usize = 39;
pub const RULE_async_stmt: usize = 40;
pub const RULE_if_stmt: usize = 41;
pub const RULE_while_stmt: usize = 42;
pub const RULE_for_stmt: usize = 43;
pub const RULE_try_stmt: usize = 44;
pub const RULE_with_stmt: usize = 45;
pub const RULE_with_item: usize = 46;
pub const RULE_except_clause: usize = 47;
pub const RULE_block: usize = 48;
pub const RULE_match_stmt: usize = 49;
pub const RULE_subject_expr: usize = 50;
pub const RULE_star_named_expressions: usize = 51;
pub const RULE_star_named_expression: usize = 52;
pub const RULE_case_block: usize = 53;
pub const RULE_guard: usize = 54;
pub const RULE_patterns: usize = 55;
pub const RULE_pattern: usize = 56;
pub const RULE_as_pattern: usize = 57;
pub const RULE_or_pattern: usize = 58;
pub const RULE_closed_pattern: usize = 59;
pub const RULE_literal_pattern: usize = 60;
pub const RULE_literal_expr: usize = 61;
pub const RULE_complex_number: usize = 62;
pub const RULE_signed_number: usize = 63;
pub const RULE_signed_real_number: usize = 64;
pub const RULE_real_number: usize = 65;
pub const RULE_imaginary_number: usize = 66;
pub const RULE_capture_pattern: usize = 67;
pub const RULE_pattern_capture_target: usize = 68;
pub const RULE_wildcard_pattern: usize = 69;
pub const RULE_value_pattern: usize = 70;
pub const RULE_attr: usize = 71;
pub const RULE_name_or_attr: usize = 72;
pub const RULE_group_pattern: usize = 73;
pub const RULE_sequence_pattern: usize = 74;
pub const RULE_open_sequence_pattern: usize = 75;
pub const RULE_maybe_sequence_pattern: usize = 76;
pub const RULE_maybe_star_pattern: usize = 77;
pub const RULE_star_pattern: usize = 78;
pub const RULE_mapping_pattern: usize = 79;
pub const RULE_items_pattern: usize = 80;
pub const RULE_key_value_pattern: usize = 81;
pub const RULE_double_star_pattern: usize = 82;
pub const RULE_class_pattern: usize = 83;
pub const RULE_positional_patterns: usize = 84;
pub const RULE_keyword_patterns: usize = 85;
pub const RULE_keyword_pattern: usize = 86;
pub const RULE_test: usize = 87;
pub const RULE_test_nocond: usize = 88;
pub const RULE_lambdef: usize = 89;
pub const RULE_lambdef_nocond: usize = 90;
pub const RULE_or_test: usize = 91;
pub const RULE_and_test: usize = 92;
pub const RULE_not_test: usize = 93;
pub const RULE_comparison: usize = 94;
pub const RULE_comp_op: usize = 95;
pub const RULE_star_expr: usize = 96;
pub const RULE_expr: usize = 97;
pub const RULE_atom_expr: usize = 98;
pub const RULE_atom: usize = 99;
pub const RULE_name: usize = 100;
pub const RULE_testlist_comp: usize = 101;
pub const RULE_trailer: usize = 102;
pub const RULE_subscriptlist: usize = 103;
pub const RULE_subscript_: usize = 104;
pub const RULE_sliceop: usize = 105;
pub const RULE_exprlist: usize = 106;
pub const RULE_testlist: usize = 107;
pub const RULE_dictorsetmaker: usize = 108;
pub const RULE_classdef: usize = 109;
pub const RULE_arglist: usize = 110;
pub const RULE_argument: usize = 111;
pub const RULE_comp_iter: usize = 112;
pub const RULE_comp_for: usize = 113;
pub const RULE_comp_if: usize = 114;
pub const RULE_encoding_decl: usize = 115;
pub const RULE_yield_expr: usize = 116;
pub const RULE_yield_arg: usize = 117;
pub const RULE_strings: usize = 118;
pub const ruleNames: [&'static str; 119] = [
    "single_input",
    "file_input",
    "eval_input",
    "decorator",
    "decorators",
    "decorated",
    "async_funcdef",
    "funcdef",
    "parameters",
    "typedargslist",
    "tfpdef",
    "varargslist",
    "vfpdef",
    "stmt",
    "simple_stmts",
    "simple_stmt",
    "expr_stmt",
    "annassign",
    "testlist_star_expr",
    "augassign",
    "del_stmt",
    "pass_stmt",
    "flow_stmt",
    "break_stmt",
    "continue_stmt",
    "return_stmt",
    "yield_stmt",
    "raise_stmt",
    "import_stmt",
    "import_name",
    "import_from",
    "import_as_name",
    "dotted_as_name",
    "import_as_names",
    "dotted_as_names",
    "dotted_name",
    "global_stmt",
    "nonlocal_stmt",
    "assert_stmt",
    "compound_stmt",
    "async_stmt",
    "if_stmt",
    "while_stmt",
    "for_stmt",
    "try_stmt",
    "with_stmt",
    "with_item",
    "except_clause",
    "block",
    "match_stmt",
    "subject_expr",
    "star_named_expressions",
    "star_named_expression",
    "case_block",
    "guard",
    "patterns",
    "pattern",
    "as_pattern",
    "or_pattern",
    "closed_pattern",
    "literal_pattern",
    "literal_expr",
    "complex_number",
    "signed_number",
    "signed_real_number",
    "real_number",
    "imaginary_number",
    "capture_pattern",
    "pattern_capture_target",
    "wildcard_pattern",
    "value_pattern",
    "attr",
    "name_or_attr",
    "group_pattern",
    "sequence_pattern",
    "open_sequence_pattern",
    "maybe_sequence_pattern",
    "maybe_star_pattern",
    "star_pattern",
    "mapping_pattern",
    "items_pattern",
    "key_value_pattern",
    "double_star_pattern",
    "class_pattern",
    "positional_patterns",
    "keyword_patterns",
    "keyword_pattern",
    "test",
    "test_nocond",
    "lambdef",
    "lambdef_nocond",
    "or_test",
    "and_test",
    "not_test",
    "comparison",
    "comp_op",
    "star_expr",
    "expr",
    "atom_expr",
    "atom",
    "name",
    "testlist_comp",
    "trailer",
    "subscriptlist",
    "subscript_",
    "sliceop",
    "exprlist",
    "testlist",
    "dictorsetmaker",
    "classdef",
    "arglist",
    "argument",
    "comp_iter",
    "comp_for",
    "comp_if",
    "encoding_decl",
    "yield_expr",
    "yield_arg",
    "strings",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 101] = [
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'and'"),
    Some("'as'"),
    Some("'assert'"),
    Some("'async'"),
    Some("'await'"),
    Some("'break'"),
    Some("'case'"),
    Some("'class'"),
    Some("'continue'"),
    Some("'def'"),
    Some("'del'"),
    Some("'elif'"),
    Some("'else'"),
    Some("'except'"),
    Some("'False'"),
    Some("'finally'"),
    Some("'for'"),
    Some("'from'"),
    Some("'global'"),
    Some("'if'"),
    Some("'import'"),
    Some("'in'"),
    Some("'is'"),
    Some("'lambda'"),
    Some("'match'"),
    Some("'None'"),
    Some("'nonlocal'"),
    Some("'not'"),
    Some("'or'"),
    Some("'pass'"),
    Some("'raise'"),
    Some("'return'"),
    Some("'True'"),
    Some("'try'"),
    Some("'_'"),
    Some("'while'"),
    Some("'with'"),
    Some("'yield'"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'.'"),
    Some("'...'"),
    Some("'*'"),
    Some("'('"),
    Some("')'"),
    Some("','"),
    Some("':'"),
    Some("';'"),
    Some("'**'"),
    Some("'='"),
    Some("'['"),
    Some("']'"),
    Some("'|'"),
    Some("'^'"),
    Some("'&'"),
    Some("'<<'"),
    Some("'>>'"),
    Some("'+'"),
    Some("'-'"),
    Some("'/'"),
    Some("'%'"),
    Some("'//'"),
    Some("'~'"),
    Some("'{'"),
    Some("'}'"),
    Some("'<'"),
    Some("'>'"),
    Some("'=='"),
    Some("'>='"),
    Some("'<='"),
    Some("'<>'"),
    Some("'!='"),
    Some("'@'"),
    Some("'->'"),
    Some("'+='"),
    Some("'-='"),
    Some("'*='"),
    Some("'@='"),
    Some("'/='"),
    Some("'%='"),
    Some("'&='"),
    Some("'|='"),
    Some("'^='"),
    Some("'<<='"),
    Some("'>>='"),
    Some("'**='"),
    Some("'//='"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 103] = [
    None,
    Some("INDENT"),
    Some("DEDENT"),
    Some("STRING"),
    Some("NUMBER"),
    Some("INTEGER"),
    Some("AND"),
    Some("AS"),
    Some("ASSERT"),
    Some("ASYNC"),
    Some("AWAIT"),
    Some("BREAK"),
    Some("CASE"),
    Some("CLASS"),
    Some("CONTINUE"),
    Some("DEF"),
    Some("DEL"),
    Some("ELIF"),
    Some("ELSE"),
    Some("EXCEPT"),
    Some("FALSE"),
    Some("FINALLY"),
    Some("FOR"),
    Some("FROM"),
    Some("GLOBAL"),
    Some("IF"),
    Some("IMPORT"),
    Some("IN"),
    Some("IS"),
    Some("LAMBDA"),
    Some("MATCH"),
    Some("NONE"),
    Some("NONLOCAL"),
    Some("NOT"),
    Some("OR"),
    Some("PASS"),
    Some("RAISE"),
    Some("RETURN"),
    Some("TRUE"),
    Some("TRY"),
    Some("UNDERSCORE"),
    Some("WHILE"),
    Some("WITH"),
    Some("YIELD"),
    Some("NEWLINE"),
    Some("NAME"),
    Some("STRING_LITERAL"),
    Some("BYTES_LITERAL"),
    Some("DECIMAL_INTEGER"),
    Some("OCT_INTEGER"),
    Some("HEX_INTEGER"),
    Some("BIN_INTEGER"),
    Some("FLOAT_NUMBER"),
    Some("IMAG_NUMBER"),
    Some("DOT"),
    Some("ELLIPSIS"),
    Some("STAR"),
    Some("OPEN_PAREN"),
    Some("CLOSE_PAREN"),
    Some("COMMA"),
    Some("COLON"),
    Some("SEMI_COLON"),
    Some("POWER"),
    Some("ASSIGN"),
    Some("OPEN_BRACK"),
    Some("CLOSE_BRACK"),
    Some("OR_OP"),
    Some("XOR"),
    Some("AND_OP"),
    Some("LEFT_SHIFT"),
    Some("RIGHT_SHIFT"),
    Some("ADD"),
    Some("MINUS"),
    Some("DIV"),
    Some("MOD"),
    Some("IDIV"),
    Some("NOT_OP"),
    Some("OPEN_BRACE"),
    Some("CLOSE_BRACE"),
    Some("LESS_THAN"),
    Some("GREATER_THAN"),
    Some("EQUALS"),
    Some("GT_EQ"),
    Some("LT_EQ"),
    Some("NOT_EQ_1"),
    Some("NOT_EQ_2"),
    Some("AT"),
    Some("ARROW"),
    Some("ADD_ASSIGN"),
    Some("SUB_ASSIGN"),
    Some("MULT_ASSIGN"),
    Some("AT_ASSIGN"),
    Some("DIV_ASSIGN"),
    Some("MOD_ASSIGN"),
    Some("AND_ASSIGN"),
    Some("OR_ASSIGN"),
    Some("XOR_ASSIGN"),
    Some("LEFT_SHIFT_ASSIGN"),
    Some("RIGHT_SHIFT_ASSIGN"),
    Some("POWER_ASSIGN"),
    Some("IDIV_ASSIGN"),
    Some("SKIP_"),
    Some("UNKNOWN_CHAR"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    Python3ParserExt<'input>,
    I,
    Python3ParserContextType,
    dyn Python3ParserListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type Python3ParserTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, Python3ParserContextType, dyn Python3ParserListener<'input> + 'a>;

/// Parser for Python3Parser grammar
pub struct Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                Python3ParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> Python3Parser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> Python3Parser<'input, I, DefaultErrorStrategy<'input, Python3ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for Python3Parser
pub trait Python3ParserContext<'input>:
    for<'x> Listenable<dyn Python3ParserListener<'input> + 'x>
    + for<'x> Visitable<dyn Python3ParserVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = Python3ParserContextType>
{
}

antlr_rust::coerce_from! { 'input : Python3ParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn Python3ParserContext<'input> + 'input
where
    T: Python3ParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn Python3ParserVisitor<'input> + 'x))
    }
}

impl<'input> Python3ParserContext<'input> for TerminalNode<'input, Python3ParserContextType> {}
impl<'input> Python3ParserContext<'input> for ErrorNode<'input, Python3ParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn Python3ParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn Python3ParserListener<'input> + 'input }

pub struct Python3ParserContextType;
antlr_rust::tid! {Python3ParserContextType}

impl<'input> ParserNodeType<'input> for Python3ParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn Python3ParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct Python3ParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> Python3ParserExt<'input> {
    pub const fn CannotBeDotLpEq(&self) -> bool {
        true
    }

    pub const fn CannotBePlusMinus(&self) -> bool {
        true
    }
}
antlr_rust::tid! { Python3ParserExt<'a> }

impl<'input> TokenAware<'input> for Python3ParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for Python3ParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for Python3ParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "Python3Parser.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn Python3ParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            60 => Python3Parser::<'input, I, _>::literal_pattern_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            61 => Python3Parser::<'input, I, _>::literal_expr_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            68 => Python3Parser::<'input, I, _>::pattern_capture_target_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            70 => Python3Parser::<'input, I, _>::value_pattern_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            97 => Python3Parser::<'input, I, _>::expr_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I> Python3Parser<'input, I, DefaultErrorStrategy<'input, Python3ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn literal_pattern_sempred(
        _localctx: Option<&Literal_patternContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.CannotBePlusMinus(),
            _ => true,
        }
    }
    fn literal_expr_sempred(
        _localctx: Option<&Literal_exprContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            1 => recog.CannotBePlusMinus(),
            _ => true,
        }
    }
    fn pattern_capture_target_sempred(
        _localctx: Option<&Pattern_capture_targetContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            2 => recog.CannotBeDotLpEq(),
            _ => true,
        }
    }
    fn value_pattern_sempred(
        _localctx: Option<&Value_patternContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            3 => recog.CannotBeDotLpEq(),
            _ => true,
        }
    }
    fn expr_sempred(
        _localctx: Option<&ExprContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            4 => recog.precpred(None, 8),
            5 => recog.precpred(None, 6),
            6 => recog.precpred(None, 5),
            7 => recog.precpred(None, 4),
            8 => recog.precpred(None, 3),
            9 => recog.precpred(None, 2),
            10 => recog.precpred(None, 1),
            _ => true,
        }
    }
}
//------------------- single_input ----------------
pub type Single_inputContextAll<'input> = Single_inputContext<'input>;

pub type Single_inputContext<'input> =
    BaseParserRuleContext<'input, Single_inputContextExt<'input>>;

#[derive(Clone)]
pub struct Single_inputContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Single_inputContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Single_inputContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_single_input(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_single_input(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Single_inputContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_single_input(self);
    }
}

impl<'input> CustomRuleContext<'input> for Single_inputContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_single_input
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_single_input }
}
antlr_rust::tid! {Single_inputContextExt<'a>}

impl<'input> Single_inputContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Single_inputContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Single_inputContextExt { ph: PhantomData },
        ))
    }
}

pub trait Single_inputContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Single_inputContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NEWLINE
    /// Returns `None` if there is no child corresponding to token NEWLINE
    fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, 0)
    }
    fn simple_stmts(&self) -> Option<Rc<Simple_stmtsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn compound_stmt(&self) -> Option<Rc<Compound_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Single_inputContextAttrs<'input> for Single_inputContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn single_input(&mut self) -> Result<Rc<Single_inputContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Single_inputContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 0, RULE_single_input);
        let mut _localctx: Rc<Single_inputContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(243);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(0, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(238);
                        recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule simple_stmts*/
                        recog.base.set_state(239);
                        recog.simple_stmts()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule compound_stmt*/
                        recog.base.set_state(240);
                        recog.compound_stmt()?;

                        recog.base.set_state(241);
                        recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- file_input ----------------
pub type File_inputContextAll<'input> = File_inputContext<'input>;

pub type File_inputContext<'input> = BaseParserRuleContext<'input, File_inputContextExt<'input>>;

#[derive(Clone)]
pub struct File_inputContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for File_inputContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for File_inputContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_file_input(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_file_input(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for File_inputContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_file_input(self);
    }
}

impl<'input> CustomRuleContext<'input> for File_inputContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_file_input
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_file_input }
}
antlr_rust::tid! {File_inputContextExt<'a>}

impl<'input> File_inputContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<File_inputContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            File_inputContextExt { ph: PhantomData },
        ))
    }
}

pub trait File_inputContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<File_inputContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
    fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
    /// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
    fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, i)
    }
    fn stmt_all(&self) -> Vec<Rc<StmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> File_inputContextAttrs<'input> for File_inputContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn file_input(&mut self) -> Result<Rc<File_inputContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = File_inputContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_file_input);
        let mut _localctx: Rc<File_inputContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(249);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << ASSERT)
                            | (1usize << ASYNC)
                            | (1usize << AWAIT)
                            | (1usize << BREAK)
                            | (1usize << CLASS)
                            | (1usize << CONTINUE)
                            | (1usize << DEF)
                            | (1usize << DEL)
                            | (1usize << FALSE)
                            | (1usize << FOR)
                            | (1usize << FROM)
                            | (1usize << GLOBAL)
                            | (1usize << IF)
                            | (1usize << IMPORT)
                            | (1usize << LAMBDA)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 32) & !0x3f) == 0
                        && ((1usize << (_la - 32))
                            & ((1usize << (NONLOCAL - 32))
                                | (1usize << (NOT - 32))
                                | (1usize << (PASS - 32))
                                | (1usize << (RAISE - 32))
                                | (1usize << (RETURN - 32))
                                | (1usize << (TRUE - 32))
                                | (1usize << (TRY - 32))
                                | (1usize << (UNDERSCORE - 32))
                                | (1usize << (WHILE - 32))
                                | (1usize << (WITH - 32))
                                | (1usize << (YIELD - 32))
                                | (1usize << (NEWLINE - 32))
                                | (1usize << (NAME - 32))
                                | (1usize << (ELLIPSIS - 32))
                                | (1usize << (STAR - 32))
                                | (1usize << (OPEN_PAREN - 32))))
                            != 0)
                    || (((_la - 64) & !0x3f) == 0
                        && ((1usize << (_la - 64))
                            & ((1usize << (OPEN_BRACK - 64))
                                | (1usize << (ADD - 64))
                                | (1usize << (MINUS - 64))
                                | (1usize << (NOT_OP - 64))
                                | (1usize << (OPEN_BRACE - 64))
                                | (1usize << (AT - 64))))
                            != 0)
                {
                    {
                        recog.base.set_state(247);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            NEWLINE => {
                                recog.base.set_state(245);
                                recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
                            }

                            STRING | NUMBER | ASSERT | ASYNC | AWAIT | BREAK | CLASS | CONTINUE
                            | DEF | DEL | FALSE | FOR | FROM | GLOBAL | IF | IMPORT | LAMBDA
                            | MATCH | NONE | NONLOCAL | NOT | PASS | RAISE | RETURN | TRUE
                            | TRY | UNDERSCORE | WHILE | WITH | YIELD | NAME | ELLIPSIS | STAR
                            | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE | AT => {
                                {
                                    /*InvokeRule stmt*/
                                    recog.base.set_state(246);
                                    recog.stmt()?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                    recog.base.set_state(251);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(252);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- eval_input ----------------
pub type Eval_inputContextAll<'input> = Eval_inputContext<'input>;

pub type Eval_inputContext<'input> = BaseParserRuleContext<'input, Eval_inputContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_inputContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Eval_inputContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Eval_inputContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_eval_input(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_eval_input(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Eval_inputContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_eval_input(self);
    }
}

impl<'input> CustomRuleContext<'input> for Eval_inputContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_eval_input
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_eval_input }
}
antlr_rust::tid! {Eval_inputContextExt<'a>}

impl<'input> Eval_inputContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Eval_inputContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Eval_inputContextExt { ph: PhantomData },
        ))
    }
}

pub trait Eval_inputContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Eval_inputContextExt<'input>>
{
    fn testlist(&self) -> Option<Rc<TestlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
    fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
    /// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
    fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, i)
    }
}

impl<'input> Eval_inputContextAttrs<'input> for Eval_inputContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn eval_input(&mut self) -> Result<Rc<Eval_inputContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Eval_inputContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_eval_input);
        let mut _localctx: Rc<Eval_inputContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule testlist*/
                recog.base.set_state(254);
                recog.testlist()?;

                recog.base.set_state(258);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == NEWLINE {
                    {
                        {
                            recog.base.set_state(255);
                            recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(260);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(261);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- decorator ----------------
pub type DecoratorContextAll<'input> = DecoratorContext<'input>;

pub type DecoratorContext<'input> = BaseParserRuleContext<'input, DecoratorContextExt<'input>>;

#[derive(Clone)]
pub struct DecoratorContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for DecoratorContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for DecoratorContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_decorator(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_decorator(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for DecoratorContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_decorator(self);
    }
}

impl<'input> CustomRuleContext<'input> for DecoratorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_decorator
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_decorator }
}
antlr_rust::tid! {DecoratorContextExt<'a>}

impl<'input> DecoratorContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DecoratorContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DecoratorContextExt { ph: PhantomData },
        ))
    }
}

pub trait DecoratorContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<DecoratorContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AT, 0)
    }
    fn dotted_name(&self) -> Option<Rc<Dotted_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NEWLINE
    /// Returns `None` if there is no child corresponding to token NEWLINE
    fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn arglist(&self) -> Option<Rc<ArglistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> DecoratorContextAttrs<'input> for DecoratorContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn decorator(&mut self) -> Result<Rc<DecoratorContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DecoratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_decorator);
        let mut _localctx: Rc<DecoratorContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(263);
                recog.base.match_token(AT, &mut recog.err_handler)?;

                /*InvokeRule dotted_name*/
                recog.base.set_state(264);
                recog.dotted_name()?;

                recog.base.set_state(270);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPEN_PAREN {
                    {
                        recog.base.set_state(265);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        recog.base.set_state(267);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (STAR - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (POWER - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule arglist*/
                                recog.base.set_state(266);
                                recog.arglist()?;
                            }
                        }

                        recog.base.set_state(269);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(272);
                recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- decorators ----------------
pub type DecoratorsContextAll<'input> = DecoratorsContext<'input>;

pub type DecoratorsContext<'input> = BaseParserRuleContext<'input, DecoratorsContextExt<'input>>;

#[derive(Clone)]
pub struct DecoratorsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for DecoratorsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for DecoratorsContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_decorators(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_decorators(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for DecoratorsContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_decorators(self);
    }
}

impl<'input> CustomRuleContext<'input> for DecoratorsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_decorators
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_decorators }
}
antlr_rust::tid! {DecoratorsContextExt<'a>}

impl<'input> DecoratorsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DecoratorsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DecoratorsContextExt { ph: PhantomData },
        ))
    }
}

pub trait DecoratorsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<DecoratorsContextExt<'input>>
{
    fn decorator_all(&self) -> Vec<Rc<DecoratorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn decorator(&self, i: usize) -> Option<Rc<DecoratorContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> DecoratorsContextAttrs<'input> for DecoratorsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn decorators(&mut self) -> Result<Rc<DecoratorsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DecoratorsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_decorators);
        let mut _localctx: Rc<DecoratorsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(275);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule decorator*/
                            recog.base.set_state(274);
                            recog.decorator()?;
                        }
                    }
                    recog.base.set_state(277);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == AT) {
                        break;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- decorated ----------------
pub type DecoratedContextAll<'input> = DecoratedContext<'input>;

pub type DecoratedContext<'input> = BaseParserRuleContext<'input, DecoratedContextExt<'input>>;

#[derive(Clone)]
pub struct DecoratedContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for DecoratedContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for DecoratedContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_decorated(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_decorated(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for DecoratedContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_decorated(self);
    }
}

impl<'input> CustomRuleContext<'input> for DecoratedContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_decorated
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_decorated }
}
antlr_rust::tid! {DecoratedContextExt<'a>}

impl<'input> DecoratedContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DecoratedContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DecoratedContextExt { ph: PhantomData },
        ))
    }
}

pub trait DecoratedContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<DecoratedContextExt<'input>>
{
    fn decorators(&self) -> Option<Rc<DecoratorsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn classdef(&self) -> Option<Rc<ClassdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn funcdef(&self) -> Option<Rc<FuncdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn async_funcdef(&self) -> Option<Rc<Async_funcdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> DecoratedContextAttrs<'input> for DecoratedContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn decorated(&mut self) -> Result<Rc<DecoratedContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DecoratedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_decorated);
        let mut _localctx: Rc<DecoratedContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule decorators*/
                recog.base.set_state(279);
                recog.decorators()?;

                recog.base.set_state(283);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    CLASS => {
                        {
                            /*InvokeRule classdef*/
                            recog.base.set_state(280);
                            recog.classdef()?;
                        }
                    }

                    DEF => {
                        {
                            /*InvokeRule funcdef*/
                            recog.base.set_state(281);
                            recog.funcdef()?;
                        }
                    }

                    ASYNC => {
                        {
                            /*InvokeRule async_funcdef*/
                            recog.base.set_state(282);
                            recog.async_funcdef()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- async_funcdef ----------------
pub type Async_funcdefContextAll<'input> = Async_funcdefContext<'input>;

pub type Async_funcdefContext<'input> =
    BaseParserRuleContext<'input, Async_funcdefContextExt<'input>>;

#[derive(Clone)]
pub struct Async_funcdefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Async_funcdefContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Async_funcdefContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_async_funcdef(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_async_funcdef(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Async_funcdefContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_async_funcdef(self);
    }
}

impl<'input> CustomRuleContext<'input> for Async_funcdefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_async_funcdef
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_async_funcdef }
}
antlr_rust::tid! {Async_funcdefContextExt<'a>}

impl<'input> Async_funcdefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Async_funcdefContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Async_funcdefContextExt { ph: PhantomData },
        ))
    }
}

pub trait Async_funcdefContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Async_funcdefContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ASYNC
    /// Returns `None` if there is no child corresponding to token ASYNC
    fn ASYNC(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASYNC, 0)
    }
    fn funcdef(&self) -> Option<Rc<FuncdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Async_funcdefContextAttrs<'input> for Async_funcdefContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn async_funcdef(&mut self) -> Result<Rc<Async_funcdefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Async_funcdefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_async_funcdef);
        let mut _localctx: Rc<Async_funcdefContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(285);
                recog.base.match_token(ASYNC, &mut recog.err_handler)?;

                /*InvokeRule funcdef*/
                recog.base.set_state(286);
                recog.funcdef()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- funcdef ----------------
pub type FuncdefContextAll<'input> = FuncdefContext<'input>;

pub type FuncdefContext<'input> = BaseParserRuleContext<'input, FuncdefContextExt<'input>>;

#[derive(Clone)]
pub struct FuncdefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for FuncdefContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for FuncdefContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_funcdef(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_funcdef(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for FuncdefContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_funcdef(self);
    }
}

impl<'input> CustomRuleContext<'input> for FuncdefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_funcdef
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_funcdef }
}
antlr_rust::tid! {FuncdefContextExt<'a>}

impl<'input> FuncdefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<FuncdefContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            FuncdefContextExt { ph: PhantomData },
        ))
    }
}

pub trait FuncdefContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<FuncdefContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEF
    /// Returns `None` if there is no child corresponding to token DEF
    fn DEF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEF, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn parameters(&self) -> Option<Rc<ParametersContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ARROW
    /// Returns `None` if there is no child corresponding to token ARROW
    fn ARROW(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ARROW, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> FuncdefContextAttrs<'input> for FuncdefContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn funcdef(&mut self) -> Result<Rc<FuncdefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = FuncdefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_funcdef);
        let mut _localctx: Rc<FuncdefContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(288);
                recog.base.match_token(DEF, &mut recog.err_handler)?;

                /*InvokeRule name*/
                recog.base.set_state(289);
                recog.name()?;

                /*InvokeRule parameters*/
                recog.base.set_state(290);
                recog.parameters()?;

                recog.base.set_state(293);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ARROW {
                    {
                        recog.base.set_state(291);
                        recog.base.match_token(ARROW, &mut recog.err_handler)?;

                        /*InvokeRule test*/
                        recog.base.set_state(292);
                        recog.test()?;
                    }
                }

                recog.base.set_state(295);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(296);
                recog.block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- parameters ----------------
pub type ParametersContextAll<'input> = ParametersContext<'input>;

pub type ParametersContext<'input> = BaseParserRuleContext<'input, ParametersContextExt<'input>>;

#[derive(Clone)]
pub struct ParametersContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ParametersContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ParametersContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_parameters(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_parameters(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ParametersContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_parameters(self);
    }
}

impl<'input> CustomRuleContext<'input> for ParametersContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_parameters
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_parameters }
}
antlr_rust::tid! {ParametersContextExt<'a>}

impl<'input> ParametersContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ParametersContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ParametersContextExt { ph: PhantomData },
        ))
    }
}

pub trait ParametersContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ParametersContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn typedargslist(&self) -> Option<Rc<TypedargslistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ParametersContextAttrs<'input> for ParametersContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn parameters(&mut self) -> Result<Rc<ParametersContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 16, RULE_parameters);
        let mut _localctx: Rc<ParametersContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(298);
                recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                recog.base.set_state(300);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == MATCH
                    || (((_la - 40) & !0x3f) == 0
                        && ((1usize << (_la - 40))
                            & ((1usize << (UNDERSCORE - 40))
                                | (1usize << (NAME - 40))
                                | (1usize << (STAR - 40))
                                | (1usize << (POWER - 40))))
                            != 0)
                {
                    {
                        /*InvokeRule typedargslist*/
                        recog.base.set_state(299);
                        recog.typedargslist()?;
                    }
                }

                recog.base.set_state(302);
                recog
                    .base
                    .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- typedargslist ----------------
pub type TypedargslistContextAll<'input> = TypedargslistContext<'input>;

pub type TypedargslistContext<'input> =
    BaseParserRuleContext<'input, TypedargslistContextExt<'input>>;

#[derive(Clone)]
pub struct TypedargslistContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for TypedargslistContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for TypedargslistContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_typedargslist(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_typedargslist(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for TypedargslistContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_typedargslist(self);
    }
}

impl<'input> CustomRuleContext<'input> for TypedargslistContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_typedargslist
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_typedargslist }
}
antlr_rust::tid! {TypedargslistContextExt<'a>}

impl<'input> TypedargslistContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TypedargslistContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TypedargslistContextExt { ph: PhantomData },
        ))
    }
}

pub trait TypedargslistContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<TypedargslistContextExt<'input>>
{
    fn tfpdef_all(&self) -> Vec<Rc<TfpdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn tfpdef(&self, i: usize) -> Option<Rc<TfpdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token POWER
    /// Returns `None` if there is no child corresponding to token POWER
    fn POWER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ASSIGN in current rule
    fn ASSIGN_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ASSIGN, starting from 0.
    /// Returns `None` if number of children corresponding to token ASSIGN is less or equal than `i`.
    fn ASSIGN(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, i)
    }
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> TypedargslistContextAttrs<'input> for TypedargslistContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn typedargslist(&mut self) -> Result<Rc<TypedargslistContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            TypedargslistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_typedargslist);
        let mut _localctx: Rc<TypedargslistContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(385);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    MATCH | UNDERSCORE | NAME => {
                        {
                            /*InvokeRule tfpdef*/
                            recog.base.set_state(304);
                            recog.tfpdef()?;

                            recog.base.set_state(307);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == ASSIGN {
                                {
                                    recog.base.set_state(305);
                                    recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                                    /*InvokeRule test*/
                                    recog.base.set_state(306);
                                    recog.test()?;
                                }
                            }

                            recog.base.set_state(317);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(12, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(309);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule tfpdef*/
                                            recog.base.set_state(310);
                                            recog.tfpdef()?;

                                            recog.base.set_state(313);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == ASSIGN {
                                                {
                                                    recog.base.set_state(311);
                                                    recog.base.match_token(
                                                        ASSIGN,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule test*/
                                                    recog.base.set_state(312);
                                                    recog.test()?;
                                                }
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(319);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(12, &mut recog.base)?;
                            }
                            recog.base.set_state(353);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(320);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    recog.base.set_state(351);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    match recog.base.input.la(1) {
                                        STAR => {
                                            {
                                                recog.base.set_state(321);
                                                recog
                                                    .base
                                                    .match_token(STAR, &mut recog.err_handler)?;

                                                recog.base.set_state(323);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if ((_la - 30) & !0x3f) == 0
                                                    && ((1usize << (_la - 30))
                                                        & ((1usize << (MATCH - 30))
                                                            | (1usize << (UNDERSCORE - 30))
                                                            | (1usize << (NAME - 30))))
                                                        != 0
                                                {
                                                    {
                                                        /*InvokeRule tfpdef*/
                                                        recog.base.set_state(322);
                                                        recog.tfpdef()?;
                                                    }
                                                }

                                                recog.base.set_state(333);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _alt = recog
                                                    .interpreter
                                                    .adaptive_predict(15, &mut recog.base)?;
                                                while { _alt != 2 && _alt != INVALID_ALT } {
                                                    if _alt == 1 {
                                                        {
                                                            {
                                                                recog.base.set_state(325);
                                                                recog.base.match_token(
                                                                    COMMA,
                                                                    &mut recog.err_handler,
                                                                )?;

                                                                /*InvokeRule tfpdef*/
                                                                recog.base.set_state(326);
                                                                recog.tfpdef()?;

                                                                recog.base.set_state(329);
                                                                recog
                                                                    .err_handler
                                                                    .sync(&mut recog.base)?;
                                                                _la = recog.base.input.la(1);
                                                                if _la == ASSIGN {
                                                                    {
                                                                        recog.base.set_state(327);
                                                                        recog.base.match_token(
                                                                            ASSIGN,
                                                                            &mut recog.err_handler,
                                                                        )?;

                                                                        /*InvokeRule test*/
                                                                        recog.base.set_state(328);
                                                                        recog.test()?;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    recog.base.set_state(335);
                                                    recog.err_handler.sync(&mut recog.base)?;
                                                    _alt = recog
                                                        .interpreter
                                                        .adaptive_predict(15, &mut recog.base)?;
                                                }
                                                recog.base.set_state(344);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == COMMA {
                                                    {
                                                        recog.base.set_state(336);
                                                        recog.base.match_token(
                                                            COMMA,
                                                            &mut recog.err_handler,
                                                        )?;

                                                        recog.base.set_state(342);
                                                        recog.err_handler.sync(&mut recog.base)?;
                                                        _la = recog.base.input.la(1);
                                                        if _la == POWER {
                                                            {
                                                                recog.base.set_state(337);
                                                                recog.base.match_token(
                                                                    POWER,
                                                                    &mut recog.err_handler,
                                                                )?;

                                                                /*InvokeRule tfpdef*/
                                                                recog.base.set_state(338);
                                                                recog.tfpdef()?;

                                                                recog.base.set_state(340);
                                                                recog
                                                                    .err_handler
                                                                    .sync(&mut recog.base)?;
                                                                _la = recog.base.input.la(1);
                                                                if _la == COMMA {
                                                                    {
                                                                        recog.base.set_state(339);
                                                                        recog.base.match_token(
                                                                            COMMA,
                                                                            &mut recog.err_handler,
                                                                        )?;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        POWER => {
                                            {
                                                recog.base.set_state(346);
                                                recog
                                                    .base
                                                    .match_token(POWER, &mut recog.err_handler)?;

                                                /*InvokeRule tfpdef*/
                                                recog.base.set_state(347);
                                                recog.tfpdef()?;

                                                recog.base.set_state(349);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == COMMA {
                                                    {
                                                        recog.base.set_state(348);
                                                        recog.base.match_token(
                                                            COMMA,
                                                            &mut recog.err_handler,
                                                        )?;
                                                    }
                                                }
                                            }
                                        }

                                        CLOSE_PAREN => {}

                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    STAR => {
                        {
                            recog.base.set_state(355);
                            recog.base.match_token(STAR, &mut recog.err_handler)?;

                            recog.base.set_state(357);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if ((_la - 30) & !0x3f) == 0
                                && ((1usize << (_la - 30))
                                    & ((1usize << (MATCH - 30))
                                        | (1usize << (UNDERSCORE - 30))
                                        | (1usize << (NAME - 30))))
                                    != 0
                            {
                                {
                                    /*InvokeRule tfpdef*/
                                    recog.base.set_state(356);
                                    recog.tfpdef()?;
                                }
                            }

                            recog.base.set_state(367);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(24, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(359);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule tfpdef*/
                                            recog.base.set_state(360);
                                            recog.tfpdef()?;

                                            recog.base.set_state(363);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == ASSIGN {
                                                {
                                                    recog.base.set_state(361);
                                                    recog.base.match_token(
                                                        ASSIGN,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule test*/
                                                    recog.base.set_state(362);
                                                    recog.test()?;
                                                }
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(369);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(24, &mut recog.base)?;
                            }
                            recog.base.set_state(378);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(370);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    recog.base.set_state(376);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == POWER {
                                        {
                                            recog.base.set_state(371);
                                            recog
                                                .base
                                                .match_token(POWER, &mut recog.err_handler)?;

                                            /*InvokeRule tfpdef*/
                                            recog.base.set_state(372);
                                            recog.tfpdef()?;

                                            recog.base.set_state(374);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == COMMA {
                                                {
                                                    recog.base.set_state(373);
                                                    recog.base.match_token(
                                                        COMMA,
                                                        &mut recog.err_handler,
                                                    )?;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    POWER => {
                        {
                            recog.base.set_state(380);
                            recog.base.match_token(POWER, &mut recog.err_handler)?;

                            /*InvokeRule tfpdef*/
                            recog.base.set_state(381);
                            recog.tfpdef()?;

                            recog.base.set_state(383);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(382);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- tfpdef ----------------
pub type TfpdefContextAll<'input> = TfpdefContext<'input>;

pub type TfpdefContext<'input> = BaseParserRuleContext<'input, TfpdefContextExt<'input>>;

#[derive(Clone)]
pub struct TfpdefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for TfpdefContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for TfpdefContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_tfpdef(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_tfpdef(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for TfpdefContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_tfpdef(self);
    }
}

impl<'input> CustomRuleContext<'input> for TfpdefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_tfpdef
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_tfpdef }
}
antlr_rust::tid! {TfpdefContextExt<'a>}

impl<'input> TfpdefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TfpdefContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TfpdefContextExt { ph: PhantomData },
        ))
    }
}

pub trait TfpdefContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<TfpdefContextExt<'input>>
{
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TfpdefContextAttrs<'input> for TfpdefContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn tfpdef(&mut self) -> Result<Rc<TfpdefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TfpdefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_tfpdef);
        let mut _localctx: Rc<TfpdefContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(387);
                recog.name()?;

                recog.base.set_state(390);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COLON {
                    {
                        recog.base.set_state(388);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        /*InvokeRule test*/
                        recog.base.set_state(389);
                        recog.test()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- varargslist ----------------
pub type VarargslistContextAll<'input> = VarargslistContext<'input>;

pub type VarargslistContext<'input> = BaseParserRuleContext<'input, VarargslistContextExt<'input>>;

#[derive(Clone)]
pub struct VarargslistContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for VarargslistContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for VarargslistContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_varargslist(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_varargslist(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for VarargslistContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_varargslist(self);
    }
}

impl<'input> CustomRuleContext<'input> for VarargslistContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_varargslist
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_varargslist }
}
antlr_rust::tid! {VarargslistContextExt<'a>}

impl<'input> VarargslistContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VarargslistContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VarargslistContextExt { ph: PhantomData },
        ))
    }
}

pub trait VarargslistContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<VarargslistContextExt<'input>>
{
    fn vfpdef_all(&self) -> Vec<Rc<VfpdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn vfpdef(&self, i: usize) -> Option<Rc<VfpdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token POWER
    /// Returns `None` if there is no child corresponding to token POWER
    fn POWER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ASSIGN in current rule
    fn ASSIGN_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ASSIGN, starting from 0.
    /// Returns `None` if number of children corresponding to token ASSIGN is less or equal than `i`.
    fn ASSIGN(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, i)
    }
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> VarargslistContextAttrs<'input> for VarargslistContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn varargslist(&mut self) -> Result<Rc<VarargslistContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = VarargslistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_varargslist);
        let mut _localctx: Rc<VarargslistContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(473);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    MATCH | UNDERSCORE | NAME => {
                        {
                            /*InvokeRule vfpdef*/
                            recog.base.set_state(392);
                            recog.vfpdef()?;

                            recog.base.set_state(395);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == ASSIGN {
                                {
                                    recog.base.set_state(393);
                                    recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                                    /*InvokeRule test*/
                                    recog.base.set_state(394);
                                    recog.test()?;
                                }
                            }

                            recog.base.set_state(405);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(33, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(397);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule vfpdef*/
                                            recog.base.set_state(398);
                                            recog.vfpdef()?;

                                            recog.base.set_state(401);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == ASSIGN {
                                                {
                                                    recog.base.set_state(399);
                                                    recog.base.match_token(
                                                        ASSIGN,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule test*/
                                                    recog.base.set_state(400);
                                                    recog.test()?;
                                                }
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(407);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(33, &mut recog.base)?;
                            }
                            recog.base.set_state(441);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(408);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    recog.base.set_state(439);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    match recog.base.input.la(1) {
                                        STAR => {
                                            {
                                                recog.base.set_state(409);
                                                recog
                                                    .base
                                                    .match_token(STAR, &mut recog.err_handler)?;

                                                recog.base.set_state(411);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if ((_la - 30) & !0x3f) == 0
                                                    && ((1usize << (_la - 30))
                                                        & ((1usize << (MATCH - 30))
                                                            | (1usize << (UNDERSCORE - 30))
                                                            | (1usize << (NAME - 30))))
                                                        != 0
                                                {
                                                    {
                                                        /*InvokeRule vfpdef*/
                                                        recog.base.set_state(410);
                                                        recog.vfpdef()?;
                                                    }
                                                }

                                                recog.base.set_state(421);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _alt = recog
                                                    .interpreter
                                                    .adaptive_predict(36, &mut recog.base)?;
                                                while { _alt != 2 && _alt != INVALID_ALT } {
                                                    if _alt == 1 {
                                                        {
                                                            {
                                                                recog.base.set_state(413);
                                                                recog.base.match_token(
                                                                    COMMA,
                                                                    &mut recog.err_handler,
                                                                )?;

                                                                /*InvokeRule vfpdef*/
                                                                recog.base.set_state(414);
                                                                recog.vfpdef()?;

                                                                recog.base.set_state(417);
                                                                recog
                                                                    .err_handler
                                                                    .sync(&mut recog.base)?;
                                                                _la = recog.base.input.la(1);
                                                                if _la == ASSIGN {
                                                                    {
                                                                        recog.base.set_state(415);
                                                                        recog.base.match_token(
                                                                            ASSIGN,
                                                                            &mut recog.err_handler,
                                                                        )?;

                                                                        /*InvokeRule test*/
                                                                        recog.base.set_state(416);
                                                                        recog.test()?;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    recog.base.set_state(423);
                                                    recog.err_handler.sync(&mut recog.base)?;
                                                    _alt = recog
                                                        .interpreter
                                                        .adaptive_predict(36, &mut recog.base)?;
                                                }
                                                recog.base.set_state(432);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == COMMA {
                                                    {
                                                        recog.base.set_state(424);
                                                        recog.base.match_token(
                                                            COMMA,
                                                            &mut recog.err_handler,
                                                        )?;

                                                        recog.base.set_state(430);
                                                        recog.err_handler.sync(&mut recog.base)?;
                                                        _la = recog.base.input.la(1);
                                                        if _la == POWER {
                                                            {
                                                                recog.base.set_state(425);
                                                                recog.base.match_token(
                                                                    POWER,
                                                                    &mut recog.err_handler,
                                                                )?;

                                                                /*InvokeRule vfpdef*/
                                                                recog.base.set_state(426);
                                                                recog.vfpdef()?;

                                                                recog.base.set_state(428);
                                                                recog
                                                                    .err_handler
                                                                    .sync(&mut recog.base)?;
                                                                _la = recog.base.input.la(1);
                                                                if _la == COMMA {
                                                                    {
                                                                        recog.base.set_state(427);
                                                                        recog.base.match_token(
                                                                            COMMA,
                                                                            &mut recog.err_handler,
                                                                        )?;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        POWER => {
                                            {
                                                recog.base.set_state(434);
                                                recog
                                                    .base
                                                    .match_token(POWER, &mut recog.err_handler)?;

                                                /*InvokeRule vfpdef*/
                                                recog.base.set_state(435);
                                                recog.vfpdef()?;

                                                recog.base.set_state(437);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == COMMA {
                                                    {
                                                        recog.base.set_state(436);
                                                        recog.base.match_token(
                                                            COMMA,
                                                            &mut recog.err_handler,
                                                        )?;
                                                    }
                                                }
                                            }
                                        }

                                        COLON => {}

                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    STAR => {
                        {
                            recog.base.set_state(443);
                            recog.base.match_token(STAR, &mut recog.err_handler)?;

                            recog.base.set_state(445);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if ((_la - 30) & !0x3f) == 0
                                && ((1usize << (_la - 30))
                                    & ((1usize << (MATCH - 30))
                                        | (1usize << (UNDERSCORE - 30))
                                        | (1usize << (NAME - 30))))
                                    != 0
                            {
                                {
                                    /*InvokeRule vfpdef*/
                                    recog.base.set_state(444);
                                    recog.vfpdef()?;
                                }
                            }

                            recog.base.set_state(455);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(45, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(447);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            /*InvokeRule vfpdef*/
                                            recog.base.set_state(448);
                                            recog.vfpdef()?;

                                            recog.base.set_state(451);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == ASSIGN {
                                                {
                                                    recog.base.set_state(449);
                                                    recog.base.match_token(
                                                        ASSIGN,
                                                        &mut recog.err_handler,
                                                    )?;

                                                    /*InvokeRule test*/
                                                    recog.base.set_state(450);
                                                    recog.test()?;
                                                }
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(457);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(45, &mut recog.base)?;
                            }
                            recog.base.set_state(466);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(458);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                    recog.base.set_state(464);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == POWER {
                                        {
                                            recog.base.set_state(459);
                                            recog
                                                .base
                                                .match_token(POWER, &mut recog.err_handler)?;

                                            /*InvokeRule vfpdef*/
                                            recog.base.set_state(460);
                                            recog.vfpdef()?;

                                            recog.base.set_state(462);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == COMMA {
                                                {
                                                    recog.base.set_state(461);
                                                    recog.base.match_token(
                                                        COMMA,
                                                        &mut recog.err_handler,
                                                    )?;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    POWER => {
                        {
                            recog.base.set_state(468);
                            recog.base.match_token(POWER, &mut recog.err_handler)?;

                            /*InvokeRule vfpdef*/
                            recog.base.set_state(469);
                            recog.vfpdef()?;

                            recog.base.set_state(471);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(470);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- vfpdef ----------------
pub type VfpdefContextAll<'input> = VfpdefContext<'input>;

pub type VfpdefContext<'input> = BaseParserRuleContext<'input, VfpdefContextExt<'input>>;

#[derive(Clone)]
pub struct VfpdefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for VfpdefContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for VfpdefContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_vfpdef(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_vfpdef(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for VfpdefContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_vfpdef(self);
    }
}

impl<'input> CustomRuleContext<'input> for VfpdefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_vfpdef
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_vfpdef }
}
antlr_rust::tid! {VfpdefContextExt<'a>}

impl<'input> VfpdefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VfpdefContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VfpdefContextExt { ph: PhantomData },
        ))
    }
}

pub trait VfpdefContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<VfpdefContextExt<'input>>
{
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> VfpdefContextAttrs<'input> for VfpdefContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn vfpdef(&mut self) -> Result<Rc<VfpdefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = VfpdefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_vfpdef);
        let mut _localctx: Rc<VfpdefContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(475);
                recog.name()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt ----------------
pub type StmtContextAll<'input> = StmtContext<'input>;

pub type StmtContext<'input> = BaseParserRuleContext<'input, StmtContextExt<'input>>;

#[derive(Clone)]
pub struct StmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for StmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for StmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for StmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for StmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}
antlr_rust::tid! {StmtContextExt<'a>}

impl<'input> StmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait StmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<StmtContextExt<'input>>
{
    fn simple_stmts(&self) -> Option<Rc<Simple_stmtsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn compound_stmt(&self) -> Option<Rc<Compound_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> StmtContextAttrs<'input> for StmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn stmt(&mut self) -> Result<Rc<StmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_stmt);
        let mut _localctx: Rc<StmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(479);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(51, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule simple_stmts*/
                        recog.base.set_state(477);
                        recog.simple_stmts()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule compound_stmt*/
                        recog.base.set_state(478);
                        recog.compound_stmt()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- simple_stmts ----------------
pub type Simple_stmtsContextAll<'input> = Simple_stmtsContext<'input>;

pub type Simple_stmtsContext<'input> =
    BaseParserRuleContext<'input, Simple_stmtsContextExt<'input>>;

#[derive(Clone)]
pub struct Simple_stmtsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Simple_stmtsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Simple_stmtsContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_simple_stmts(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_simple_stmts(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Simple_stmtsContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_simple_stmts(self);
    }
}

impl<'input> CustomRuleContext<'input> for Simple_stmtsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_simple_stmts
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_simple_stmts }
}
antlr_rust::tid! {Simple_stmtsContextExt<'a>}

impl<'input> Simple_stmtsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Simple_stmtsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Simple_stmtsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Simple_stmtsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Simple_stmtsContextExt<'input>>
{
    fn simple_stmt_all(&self) -> Vec<Rc<Simple_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn simple_stmt(&self, i: usize) -> Option<Rc<Simple_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token NEWLINE
    /// Returns `None` if there is no child corresponding to token NEWLINE
    fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMI_COLON in current rule
    fn SEMI_COLON_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SEMI_COLON, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMI_COLON is less or equal than `i`.
    fn SEMI_COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SEMI_COLON, i)
    }
}

impl<'input> Simple_stmtsContextAttrs<'input> for Simple_stmtsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn simple_stmts(&mut self) -> Result<Rc<Simple_stmtsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Simple_stmtsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_simple_stmts);
        let mut _localctx: Rc<Simple_stmtsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule simple_stmt*/
                recog.base.set_state(481);
                recog.simple_stmt()?;

                recog.base.set_state(486);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(52, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(482);
                                recog.base.match_token(SEMI_COLON, &mut recog.err_handler)?;

                                /*InvokeRule simple_stmt*/
                                recog.base.set_state(483);
                                recog.simple_stmt()?;
                            }
                        }
                    }
                    recog.base.set_state(488);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(52, &mut recog.base)?;
                }
                recog.base.set_state(490);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SEMI_COLON {
                    {
                        recog.base.set_state(489);
                        recog.base.match_token(SEMI_COLON, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(492);
                recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- simple_stmt ----------------
pub type Simple_stmtContextAll<'input> = Simple_stmtContext<'input>;

pub type Simple_stmtContext<'input> = BaseParserRuleContext<'input, Simple_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Simple_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Simple_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Simple_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_simple_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_simple_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Simple_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_simple_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Simple_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_simple_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_simple_stmt }
}
antlr_rust::tid! {Simple_stmtContextExt<'a>}

impl<'input> Simple_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Simple_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Simple_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Simple_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Simple_stmtContextExt<'input>>
{
    fn expr_stmt(&self) -> Option<Rc<Expr_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn del_stmt(&self) -> Option<Rc<Del_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn pass_stmt(&self) -> Option<Rc<Pass_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn flow_stmt(&self) -> Option<Rc<Flow_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn import_stmt(&self) -> Option<Rc<Import_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn global_stmt(&self) -> Option<Rc<Global_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn nonlocal_stmt(&self) -> Option<Rc<Nonlocal_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn assert_stmt(&self) -> Option<Rc<Assert_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Simple_stmtContextAttrs<'input> for Simple_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn simple_stmt(&mut self) -> Result<Rc<Simple_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Simple_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 30, RULE_simple_stmt);
        let mut _localctx: Rc<Simple_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(502);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT | TRUE
                    | UNDERSCORE | NAME | ELLIPSIS | STAR | OPEN_PAREN | OPEN_BRACK | ADD
                    | MINUS | NOT_OP | OPEN_BRACE => {
                        {
                            /*InvokeRule expr_stmt*/
                            recog.base.set_state(494);
                            recog.expr_stmt()?;
                        }
                    }

                    DEL => {
                        {
                            /*InvokeRule del_stmt*/
                            recog.base.set_state(495);
                            recog.del_stmt()?;
                        }
                    }

                    PASS => {
                        {
                            /*InvokeRule pass_stmt*/
                            recog.base.set_state(496);
                            recog.pass_stmt()?;
                        }
                    }

                    BREAK | CONTINUE | RAISE | RETURN | YIELD => {
                        {
                            /*InvokeRule flow_stmt*/
                            recog.base.set_state(497);
                            recog.flow_stmt()?;
                        }
                    }

                    FROM | IMPORT => {
                        {
                            /*InvokeRule import_stmt*/
                            recog.base.set_state(498);
                            recog.import_stmt()?;
                        }
                    }

                    GLOBAL => {
                        {
                            /*InvokeRule global_stmt*/
                            recog.base.set_state(499);
                            recog.global_stmt()?;
                        }
                    }

                    NONLOCAL => {
                        {
                            /*InvokeRule nonlocal_stmt*/
                            recog.base.set_state(500);
                            recog.nonlocal_stmt()?;
                        }
                    }

                    ASSERT => {
                        {
                            /*InvokeRule assert_stmt*/
                            recog.base.set_state(501);
                            recog.assert_stmt()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr_stmt ----------------
pub type Expr_stmtContextAll<'input> = Expr_stmtContext<'input>;

pub type Expr_stmtContext<'input> = BaseParserRuleContext<'input, Expr_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Expr_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Expr_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Expr_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expr_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_expr_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Expr_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_expr_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Expr_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr_stmt }
}
antlr_rust::tid! {Expr_stmtContextExt<'a>}

impl<'input> Expr_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Expr_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Expr_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Expr_stmtContextExt<'input>>
{
    fn testlist_star_expr_all(&self) -> Vec<Rc<Testlist_star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn testlist_star_expr(&self, i: usize) -> Option<Rc<Testlist_star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn annassign(&self) -> Option<Rc<AnnassignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn augassign(&self) -> Option<Rc<AugassignContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn yield_expr_all(&self) -> Vec<Rc<Yield_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn yield_expr(&self, i: usize) -> Option<Rc<Yield_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn testlist(&self) -> Option<Rc<TestlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ASSIGN in current rule
    fn ASSIGN_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ASSIGN, starting from 0.
    /// Returns `None` if number of children corresponding to token ASSIGN is less or equal than `i`.
    fn ASSIGN(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, i)
    }
}

impl<'input> Expr_stmtContextAttrs<'input> for Expr_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expr_stmt(&mut self) -> Result<Rc<Expr_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Expr_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_expr_stmt);
        let mut _localctx: Rc<Expr_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule testlist_star_expr*/
                recog.base.set_state(504);
                recog.testlist_star_expr()?;

                recog.base.set_state(521);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    COLON => {
                        {
                            /*InvokeRule annassign*/
                            recog.base.set_state(505);
                            recog.annassign()?;
                        }
                    }

                    ADD_ASSIGN | SUB_ASSIGN | MULT_ASSIGN | AT_ASSIGN | DIV_ASSIGN | MOD_ASSIGN
                    | AND_ASSIGN | OR_ASSIGN | XOR_ASSIGN | LEFT_SHIFT_ASSIGN
                    | RIGHT_SHIFT_ASSIGN | POWER_ASSIGN | IDIV_ASSIGN => {
                        {
                            /*InvokeRule augassign*/
                            recog.base.set_state(506);
                            recog.augassign()?;

                            recog.base.set_state(509);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.base.input.la(1) {
                                YIELD => {
                                    {
                                        /*InvokeRule yield_expr*/
                                        recog.base.set_state(507);
                                        recog.yield_expr()?;
                                    }
                                }

                                STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT
                                | TRUE | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN | OPEN_BRACK
                                | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                                    {
                                        /*InvokeRule testlist*/
                                        recog.base.set_state(508);
                                        recog.testlist()?;
                                    }
                                }

                                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                    &mut recog.base,
                                )))?,
                            }
                        }
                    }

                    NEWLINE | SEMI_COLON | ASSIGN => {
                        {
                            recog.base.set_state(518);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while _la == ASSIGN {
                                {
                                    {
                                        recog.base.set_state(511);
                                        recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                                        recog.base.set_state(514);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        match recog.base.input.la(1) {
                                            YIELD => {
                                                {
                                                    /*InvokeRule yield_expr*/
                                                    recog.base.set_state(512);
                                                    recog.yield_expr()?;
                                                }
                                            }

                                            STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH
                                            | NONE | NOT | TRUE | UNDERSCORE | NAME | ELLIPSIS
                                            | STAR | OPEN_PAREN | OPEN_BRACK | ADD | MINUS
                                            | NOT_OP | OPEN_BRACE => {
                                                {
                                                    /*InvokeRule testlist_star_expr*/
                                                    recog.base.set_state(513);
                                                    recog.testlist_star_expr()?;
                                                }
                                            }

                                            _ => Err(ANTLRError::NoAltError(
                                                NoViableAltError::new(&mut recog.base),
                                            ))?,
                                        }
                                    }
                                }
                                recog.base.set_state(520);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- annassign ----------------
pub type AnnassignContextAll<'input> = AnnassignContext<'input>;

pub type AnnassignContext<'input> = BaseParserRuleContext<'input, AnnassignContextExt<'input>>;

#[derive(Clone)]
pub struct AnnassignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for AnnassignContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for AnnassignContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_annassign(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_annassign(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for AnnassignContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_annassign(self);
    }
}

impl<'input> CustomRuleContext<'input> for AnnassignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_annassign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_annassign }
}
antlr_rust::tid! {AnnassignContextExt<'a>}

impl<'input> AnnassignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AnnassignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AnnassignContextExt { ph: PhantomData },
        ))
    }
}

pub trait AnnassignContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<AnnassignContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
}

impl<'input> AnnassignContextAttrs<'input> for AnnassignContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn annassign(&mut self) -> Result<Rc<AnnassignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AnnassignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_annassign);
        let mut _localctx: Rc<AnnassignContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(523);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule test*/
                recog.base.set_state(524);
                recog.test()?;

                recog.base.set_state(527);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ASSIGN {
                    {
                        recog.base.set_state(525);
                        recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                        /*InvokeRule test*/
                        recog.base.set_state(526);
                        recog.test()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- testlist_star_expr ----------------
pub type Testlist_star_exprContextAll<'input> = Testlist_star_exprContext<'input>;

pub type Testlist_star_exprContext<'input> =
    BaseParserRuleContext<'input, Testlist_star_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Testlist_star_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Testlist_star_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Testlist_star_exprContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_testlist_star_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_testlist_star_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Testlist_star_exprContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_testlist_star_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Testlist_star_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_testlist_star_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_testlist_star_expr }
}
antlr_rust::tid! {Testlist_star_exprContextExt<'a>}

impl<'input> Testlist_star_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Testlist_star_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Testlist_star_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Testlist_star_exprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Testlist_star_exprContextExt<'input>>
{
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn star_expr_all(&self) -> Vec<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn star_expr(&self, i: usize) -> Option<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Testlist_star_exprContextAttrs<'input> for Testlist_star_exprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn testlist_star_expr(
        &mut self,
    ) -> Result<Rc<Testlist_star_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Testlist_star_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_testlist_star_expr);
        let mut _localctx: Rc<Testlist_star_exprContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(531);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT | TRUE
                    | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS
                    | NOT_OP | OPEN_BRACE => {
                        {
                            /*InvokeRule test*/
                            recog.base.set_state(529);
                            recog.test()?;
                        }
                    }

                    STAR => {
                        {
                            /*InvokeRule star_expr*/
                            recog.base.set_state(530);
                            recog.star_expr()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(540);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(62, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(533);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                recog.base.set_state(536);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.base.input.la(1) {
                                    STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE
                                    | NOT | TRUE | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN
                                    | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                                        {
                                            /*InvokeRule test*/
                                            recog.base.set_state(534);
                                            recog.test()?;
                                        }
                                    }

                                    STAR => {
                                        {
                                            /*InvokeRule star_expr*/
                                            recog.base.set_state(535);
                                            recog.star_expr()?;
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                            }
                        }
                    }
                    recog.base.set_state(542);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(62, &mut recog.base)?;
                }
                recog.base.set_state(544);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(543);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- augassign ----------------
pub type AugassignContextAll<'input> = AugassignContext<'input>;

pub type AugassignContext<'input> = BaseParserRuleContext<'input, AugassignContextExt<'input>>;

#[derive(Clone)]
pub struct AugassignContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for AugassignContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for AugassignContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_augassign(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_augassign(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for AugassignContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_augassign(self);
    }
}

impl<'input> CustomRuleContext<'input> for AugassignContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_augassign
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_augassign }
}
antlr_rust::tid! {AugassignContextExt<'a>}

impl<'input> AugassignContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AugassignContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AugassignContextExt { ph: PhantomData },
        ))
    }
}

pub trait AugassignContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<AugassignContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ADD_ASSIGN
    /// Returns `None` if there is no child corresponding to token ADD_ASSIGN
    fn ADD_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SUB_ASSIGN
    /// Returns `None` if there is no child corresponding to token SUB_ASSIGN
    fn SUB_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SUB_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MULT_ASSIGN
    /// Returns `None` if there is no child corresponding to token MULT_ASSIGN
    fn MULT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MULT_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AT_ASSIGN
    /// Returns `None` if there is no child corresponding to token AT_ASSIGN
    fn AT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AT_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV_ASSIGN
    /// Returns `None` if there is no child corresponding to token DIV_ASSIGN
    fn DIV_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIV_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MOD_ASSIGN
    /// Returns `None` if there is no child corresponding to token MOD_ASSIGN
    fn MOD_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MOD_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AND_ASSIGN
    /// Returns `None` if there is no child corresponding to token AND_ASSIGN
    fn AND_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AND_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OR_ASSIGN
    /// Returns `None` if there is no child corresponding to token OR_ASSIGN
    fn OR_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XOR_ASSIGN
    /// Returns `None` if there is no child corresponding to token XOR_ASSIGN
    fn XOR_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(XOR_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEFT_SHIFT_ASSIGN
    /// Returns `None` if there is no child corresponding to token LEFT_SHIFT_ASSIGN
    fn LEFT_SHIFT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LEFT_SHIFT_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RIGHT_SHIFT_ASSIGN
    /// Returns `None` if there is no child corresponding to token RIGHT_SHIFT_ASSIGN
    fn RIGHT_SHIFT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RIGHT_SHIFT_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token POWER_ASSIGN
    /// Returns `None` if there is no child corresponding to token POWER_ASSIGN
    fn POWER_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER_ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IDIV_ASSIGN
    /// Returns `None` if there is no child corresponding to token IDIV_ASSIGN
    fn IDIV_ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IDIV_ASSIGN, 0)
    }
}

impl<'input> AugassignContextAttrs<'input> for AugassignContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn augassign(&mut self) -> Result<Rc<AugassignContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AugassignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_augassign);
        let mut _localctx: Rc<AugassignContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(546);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 88) & !0x3f) == 0
                        && ((1usize << (_la - 88))
                            & ((1usize << (ADD_ASSIGN - 88))
                                | (1usize << (SUB_ASSIGN - 88))
                                | (1usize << (MULT_ASSIGN - 88))
                                | (1usize << (AT_ASSIGN - 88))
                                | (1usize << (DIV_ASSIGN - 88))
                                | (1usize << (MOD_ASSIGN - 88))
                                | (1usize << (AND_ASSIGN - 88))
                                | (1usize << (OR_ASSIGN - 88))
                                | (1usize << (XOR_ASSIGN - 88))
                                | (1usize << (LEFT_SHIFT_ASSIGN - 88))
                                | (1usize << (RIGHT_SHIFT_ASSIGN - 88))
                                | (1usize << (POWER_ASSIGN - 88))
                                | (1usize << (IDIV_ASSIGN - 88))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- del_stmt ----------------
pub type Del_stmtContextAll<'input> = Del_stmtContext<'input>;

pub type Del_stmtContext<'input> = BaseParserRuleContext<'input, Del_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Del_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Del_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Del_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_del_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_del_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Del_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_del_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Del_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_del_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_del_stmt }
}
antlr_rust::tid! {Del_stmtContextExt<'a>}

impl<'input> Del_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Del_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Del_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Del_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Del_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DEL
    /// Returns `None` if there is no child corresponding to token DEL
    fn DEL(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEL, 0)
    }
    fn exprlist(&self) -> Option<Rc<ExprlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Del_stmtContextAttrs<'input> for Del_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn del_stmt(&mut self) -> Result<Rc<Del_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Del_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_del_stmt);
        let mut _localctx: Rc<Del_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(548);
                recog.base.match_token(DEL, &mut recog.err_handler)?;

                /*InvokeRule exprlist*/
                recog.base.set_state(549);
                recog.exprlist()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- pass_stmt ----------------
pub type Pass_stmtContextAll<'input> = Pass_stmtContext<'input>;

pub type Pass_stmtContext<'input> = BaseParserRuleContext<'input, Pass_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Pass_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Pass_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Pass_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pass_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_pass_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Pass_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_pass_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Pass_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pass_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pass_stmt }
}
antlr_rust::tid! {Pass_stmtContextExt<'a>}

impl<'input> Pass_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Pass_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Pass_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Pass_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Pass_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token PASS
    /// Returns `None` if there is no child corresponding to token PASS
    fn PASS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(PASS, 0)
    }
}

impl<'input> Pass_stmtContextAttrs<'input> for Pass_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pass_stmt(&mut self) -> Result<Rc<Pass_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Pass_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_pass_stmt);
        let mut _localctx: Rc<Pass_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(551);
                recog.base.match_token(PASS, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- flow_stmt ----------------
pub type Flow_stmtContextAll<'input> = Flow_stmtContext<'input>;

pub type Flow_stmtContext<'input> = BaseParserRuleContext<'input, Flow_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Flow_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Flow_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Flow_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_flow_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_flow_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Flow_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_flow_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Flow_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_flow_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_flow_stmt }
}
antlr_rust::tid! {Flow_stmtContextExt<'a>}

impl<'input> Flow_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Flow_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Flow_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Flow_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Flow_stmtContextExt<'input>>
{
    fn break_stmt(&self) -> Option<Rc<Break_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn continue_stmt(&self) -> Option<Rc<Continue_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn return_stmt(&self) -> Option<Rc<Return_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn raise_stmt(&self) -> Option<Rc<Raise_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn yield_stmt(&self) -> Option<Rc<Yield_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Flow_stmtContextAttrs<'input> for Flow_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn flow_stmt(&mut self) -> Result<Rc<Flow_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Flow_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_flow_stmt);
        let mut _localctx: Rc<Flow_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(558);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                BREAK => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule break_stmt*/
                        recog.base.set_state(553);
                        recog.break_stmt()?;
                    }
                }

                CONTINUE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule continue_stmt*/
                        recog.base.set_state(554);
                        recog.continue_stmt()?;
                    }
                }

                RETURN => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule return_stmt*/
                        recog.base.set_state(555);
                        recog.return_stmt()?;
                    }
                }

                RAISE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule raise_stmt*/
                        recog.base.set_state(556);
                        recog.raise_stmt()?;
                    }
                }

                YIELD => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule yield_stmt*/
                        recog.base.set_state(557);
                        recog.yield_stmt()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- break_stmt ----------------
pub type Break_stmtContextAll<'input> = Break_stmtContext<'input>;

pub type Break_stmtContext<'input> = BaseParserRuleContext<'input, Break_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Break_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Break_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Break_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_break_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_break_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Break_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_break_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Break_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_break_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_break_stmt }
}
antlr_rust::tid! {Break_stmtContextExt<'a>}

impl<'input> Break_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Break_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Break_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Break_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Break_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BREAK
    /// Returns `None` if there is no child corresponding to token BREAK
    fn BREAK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BREAK, 0)
    }
}

impl<'input> Break_stmtContextAttrs<'input> for Break_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn break_stmt(&mut self) -> Result<Rc<Break_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Break_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 46, RULE_break_stmt);
        let mut _localctx: Rc<Break_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(560);
                recog.base.match_token(BREAK, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- continue_stmt ----------------
pub type Continue_stmtContextAll<'input> = Continue_stmtContext<'input>;

pub type Continue_stmtContext<'input> =
    BaseParserRuleContext<'input, Continue_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Continue_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Continue_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Continue_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_continue_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_continue_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Continue_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_continue_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Continue_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_continue_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_continue_stmt }
}
antlr_rust::tid! {Continue_stmtContextExt<'a>}

impl<'input> Continue_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Continue_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Continue_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Continue_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Continue_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CONTINUE
    /// Returns `None` if there is no child corresponding to token CONTINUE
    fn CONTINUE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONTINUE, 0)
    }
}

impl<'input> Continue_stmtContextAttrs<'input> for Continue_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn continue_stmt(&mut self) -> Result<Rc<Continue_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Continue_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 48, RULE_continue_stmt);
        let mut _localctx: Rc<Continue_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(562);
                recog.base.match_token(CONTINUE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- return_stmt ----------------
pub type Return_stmtContextAll<'input> = Return_stmtContext<'input>;

pub type Return_stmtContext<'input> = BaseParserRuleContext<'input, Return_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Return_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Return_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Return_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_return_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_return_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Return_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_return_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Return_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_return_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_return_stmt }
}
antlr_rust::tid! {Return_stmtContextExt<'a>}

impl<'input> Return_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Return_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Return_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Return_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Return_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RETURN
    /// Returns `None` if there is no child corresponding to token RETURN
    fn RETURN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RETURN, 0)
    }
    fn testlist(&self) -> Option<Rc<TestlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Return_stmtContextAttrs<'input> for Return_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn return_stmt(&mut self) -> Result<Rc<Return_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Return_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 50, RULE_return_stmt);
        let mut _localctx: Rc<Return_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(564);
                recog.base.match_token(RETURN, &mut recog.err_handler)?;

                recog.base.set_state(566);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << AWAIT)
                            | (1usize << FALSE)
                            | (1usize << LAMBDA)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 33) & !0x3f) == 0
                        && ((1usize << (_la - 33))
                            & ((1usize << (NOT - 33))
                                | (1usize << (TRUE - 33))
                                | (1usize << (UNDERSCORE - 33))
                                | (1usize << (NAME - 33))
                                | (1usize << (ELLIPSIS - 33))
                                | (1usize << (OPEN_PAREN - 33))
                                | (1usize << (OPEN_BRACK - 33))))
                            != 0)
                    || (((_la - 71) & !0x3f) == 0
                        && ((1usize << (_la - 71))
                            & ((1usize << (ADD - 71))
                                | (1usize << (MINUS - 71))
                                | (1usize << (NOT_OP - 71))
                                | (1usize << (OPEN_BRACE - 71))))
                            != 0)
                {
                    {
                        /*InvokeRule testlist*/
                        recog.base.set_state(565);
                        recog.testlist()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- yield_stmt ----------------
pub type Yield_stmtContextAll<'input> = Yield_stmtContext<'input>;

pub type Yield_stmtContext<'input> = BaseParserRuleContext<'input, Yield_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Yield_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Yield_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Yield_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_yield_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_yield_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Yield_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_yield_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Yield_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_yield_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_yield_stmt }
}
antlr_rust::tid! {Yield_stmtContextExt<'a>}

impl<'input> Yield_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Yield_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Yield_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Yield_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Yield_stmtContextExt<'input>>
{
    fn yield_expr(&self) -> Option<Rc<Yield_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Yield_stmtContextAttrs<'input> for Yield_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn yield_stmt(&mut self) -> Result<Rc<Yield_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Yield_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 52, RULE_yield_stmt);
        let mut _localctx: Rc<Yield_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule yield_expr*/
                recog.base.set_state(568);
                recog.yield_expr()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- raise_stmt ----------------
pub type Raise_stmtContextAll<'input> = Raise_stmtContext<'input>;

pub type Raise_stmtContext<'input> = BaseParserRuleContext<'input, Raise_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Raise_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Raise_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Raise_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_raise_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_raise_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Raise_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_raise_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Raise_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_raise_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_raise_stmt }
}
antlr_rust::tid! {Raise_stmtContextExt<'a>}

impl<'input> Raise_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Raise_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Raise_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Raise_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Raise_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RAISE
    /// Returns `None` if there is no child corresponding to token RAISE
    fn RAISE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RAISE, 0)
    }
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FROM, 0)
    }
}

impl<'input> Raise_stmtContextAttrs<'input> for Raise_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn raise_stmt(&mut self) -> Result<Rc<Raise_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Raise_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 54, RULE_raise_stmt);
        let mut _localctx: Rc<Raise_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(570);
                recog.base.match_token(RAISE, &mut recog.err_handler)?;

                recog.base.set_state(576);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << AWAIT)
                            | (1usize << FALSE)
                            | (1usize << LAMBDA)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 33) & !0x3f) == 0
                        && ((1usize << (_la - 33))
                            & ((1usize << (NOT - 33))
                                | (1usize << (TRUE - 33))
                                | (1usize << (UNDERSCORE - 33))
                                | (1usize << (NAME - 33))
                                | (1usize << (ELLIPSIS - 33))
                                | (1usize << (OPEN_PAREN - 33))
                                | (1usize << (OPEN_BRACK - 33))))
                            != 0)
                    || (((_la - 71) & !0x3f) == 0
                        && ((1usize << (_la - 71))
                            & ((1usize << (ADD - 71))
                                | (1usize << (MINUS - 71))
                                | (1usize << (NOT_OP - 71))
                                | (1usize << (OPEN_BRACE - 71))))
                            != 0)
                {
                    {
                        /*InvokeRule test*/
                        recog.base.set_state(571);
                        recog.test()?;

                        recog.base.set_state(574);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == FROM {
                            {
                                recog.base.set_state(572);
                                recog.base.match_token(FROM, &mut recog.err_handler)?;

                                /*InvokeRule test*/
                                recog.base.set_state(573);
                                recog.test()?;
                            }
                        }
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_stmt ----------------
pub type Import_stmtContextAll<'input> = Import_stmtContext<'input>;

pub type Import_stmtContext<'input> = BaseParserRuleContext<'input, Import_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Import_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Import_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Import_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_import_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Import_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_import_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_import_stmt }
}
antlr_rust::tid! {Import_stmtContextExt<'a>}

impl<'input> Import_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Import_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Import_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Import_stmtContextExt<'input>>
{
    fn import_name(&self) -> Option<Rc<Import_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn import_from(&self) -> Option<Rc<Import_fromContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Import_stmtContextAttrs<'input> for Import_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_stmt(&mut self) -> Result<Rc<Import_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Import_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 56, RULE_import_stmt);
        let mut _localctx: Rc<Import_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(580);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                IMPORT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule import_name*/
                        recog.base.set_state(578);
                        recog.import_name()?;
                    }
                }

                FROM => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule import_from*/
                        recog.base.set_state(579);
                        recog.import_from()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_name ----------------
pub type Import_nameContextAll<'input> = Import_nameContext<'input>;

pub type Import_nameContext<'input> = BaseParserRuleContext<'input, Import_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Import_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Import_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Import_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_name(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_import_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Import_nameContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_import_name(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_import_name }
}
antlr_rust::tid! {Import_nameContextExt<'a>}

impl<'input> Import_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Import_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Import_nameContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Import_nameContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IMPORT
    /// Returns `None` if there is no child corresponding to token IMPORT
    fn IMPORT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IMPORT, 0)
    }
    fn dotted_as_names(&self) -> Option<Rc<Dotted_as_namesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Import_nameContextAttrs<'input> for Import_nameContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_name(&mut self) -> Result<Rc<Import_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Import_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 58, RULE_import_name);
        let mut _localctx: Rc<Import_nameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(582);
                recog.base.match_token(IMPORT, &mut recog.err_handler)?;

                /*InvokeRule dotted_as_names*/
                recog.base.set_state(583);
                recog.dotted_as_names()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_from ----------------
pub type Import_fromContextAll<'input> = Import_fromContext<'input>;

pub type Import_fromContext<'input> = BaseParserRuleContext<'input, Import_fromContextExt<'input>>;

#[derive(Clone)]
pub struct Import_fromContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Import_fromContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Import_fromContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_from(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_import_from(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Import_fromContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_import_from(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_fromContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_from
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_import_from }
}
antlr_rust::tid! {Import_fromContextExt<'a>}

impl<'input> Import_fromContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_fromContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Import_fromContextExt { ph: PhantomData },
        ))
    }
}

pub trait Import_fromContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Import_fromContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FROM, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IMPORT
    /// Returns `None` if there is no child corresponding to token IMPORT
    fn IMPORT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IMPORT, 0)
    }
    fn dotted_name(&self) -> Option<Rc<Dotted_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    fn import_as_names(&self) -> Option<Rc<Import_as_namesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ELLIPSIS in current rule
    fn ELLIPSIS_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ELLIPSIS, starting from 0.
    /// Returns `None` if number of children corresponding to token ELLIPSIS is less or equal than `i`.
    fn ELLIPSIS(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELLIPSIS, i)
    }
}

impl<'input> Import_fromContextAttrs<'input> for Import_fromContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_from(&mut self) -> Result<Rc<Import_fromContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Import_fromContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 60, RULE_import_from);
        let mut _localctx: Rc<Import_fromContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    recog.base.set_state(585);
                    recog.base.match_token(FROM, &mut recog.err_handler)?;

                    recog.base.set_state(598);
                    recog.err_handler.sync(&mut recog.base)?;
                    match recog.interpreter.adaptive_predict(71, &mut recog.base)? {
                        1 => {
                            {
                                recog.base.set_state(589);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while _la == DOT || _la == ELLIPSIS {
                                    {
                                        {
                                            recog.base.set_state(586);
                                            _la = recog.base.input.la(1);
                                            if { !(_la == DOT || _la == ELLIPSIS) } {
                                                recog
                                                    .err_handler
                                                    .recover_inline(&mut recog.base)?;
                                            } else {
                                                if recog.base.input.la(1) == TOKEN_EOF {
                                                    recog.base.matched_eof = true
                                                };
                                                recog.err_handler.report_match(&mut recog.base);
                                                recog.base.consume(&mut recog.err_handler);
                                            }
                                        }
                                    }
                                    recog.base.set_state(591);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                                /*InvokeRule dotted_name*/
                                recog.base.set_state(592);
                                recog.dotted_name()?;
                            }
                        }
                        2 => {
                            recog.base.set_state(594);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            loop {
                                {
                                    {
                                        recog.base.set_state(593);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == DOT || _la == ELLIPSIS) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                    }
                                }
                                recog.base.set_state(596);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if !(_la == DOT || _la == ELLIPSIS) {
                                    break;
                                }
                            }
                        }

                        _ => {}
                    }
                    recog.base.set_state(600);
                    recog.base.match_token(IMPORT, &mut recog.err_handler)?;

                    recog.base.set_state(607);
                    recog.err_handler.sync(&mut recog.base)?;
                    match recog.base.input.la(1) {
                        STAR => {
                            recog.base.set_state(601);
                            recog.base.match_token(STAR, &mut recog.err_handler)?;
                        }

                        OPEN_PAREN => {
                            {
                                recog.base.set_state(602);
                                recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                                /*InvokeRule import_as_names*/
                                recog.base.set_state(603);
                                recog.import_as_names()?;

                                recog.base.set_state(604);
                                recog
                                    .base
                                    .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                            }
                        }

                        MATCH | UNDERSCORE | NAME => {
                            {
                                /*InvokeRule import_as_names*/
                                recog.base.set_state(606);
                                recog.import_as_names()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_as_name ----------------
pub type Import_as_nameContextAll<'input> = Import_as_nameContext<'input>;

pub type Import_as_nameContext<'input> =
    BaseParserRuleContext<'input, Import_as_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Import_as_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Import_as_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Import_as_nameContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_as_name(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_import_as_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Import_as_nameContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_import_as_name(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_as_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_as_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_import_as_name }
}
antlr_rust::tid! {Import_as_nameContextExt<'a>}

impl<'input> Import_as_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_as_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Import_as_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Import_as_nameContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Import_as_nameContextExt<'input>>
{
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
}

impl<'input> Import_as_nameContextAttrs<'input> for Import_as_nameContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_as_name(&mut self) -> Result<Rc<Import_as_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Import_as_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 62, RULE_import_as_name);
        let mut _localctx: Rc<Import_as_nameContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(609);
                recog.name()?;

                recog.base.set_state(612);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == AS {
                    {
                        recog.base.set_state(610);
                        recog.base.match_token(AS, &mut recog.err_handler)?;

                        /*InvokeRule name*/
                        recog.base.set_state(611);
                        recog.name()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dotted_as_name ----------------
pub type Dotted_as_nameContextAll<'input> = Dotted_as_nameContext<'input>;

pub type Dotted_as_nameContext<'input> =
    BaseParserRuleContext<'input, Dotted_as_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Dotted_as_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Dotted_as_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Dotted_as_nameContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dotted_as_name(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_dotted_as_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Dotted_as_nameContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_dotted_as_name(self);
    }
}

impl<'input> CustomRuleContext<'input> for Dotted_as_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dotted_as_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dotted_as_name }
}
antlr_rust::tid! {Dotted_as_nameContextExt<'a>}

impl<'input> Dotted_as_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Dotted_as_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Dotted_as_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Dotted_as_nameContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Dotted_as_nameContextExt<'input>>
{
    fn dotted_name(&self) -> Option<Rc<Dotted_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Dotted_as_nameContextAttrs<'input> for Dotted_as_nameContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dotted_as_name(&mut self) -> Result<Rc<Dotted_as_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Dotted_as_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_dotted_as_name);
        let mut _localctx: Rc<Dotted_as_nameContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule dotted_name*/
                recog.base.set_state(614);
                recog.dotted_name()?;

                recog.base.set_state(617);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == AS {
                    {
                        recog.base.set_state(615);
                        recog.base.match_token(AS, &mut recog.err_handler)?;

                        /*InvokeRule name*/
                        recog.base.set_state(616);
                        recog.name()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- import_as_names ----------------
pub type Import_as_namesContextAll<'input> = Import_as_namesContext<'input>;

pub type Import_as_namesContext<'input> =
    BaseParserRuleContext<'input, Import_as_namesContextExt<'input>>;

#[derive(Clone)]
pub struct Import_as_namesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Import_as_namesContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Import_as_namesContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_import_as_names(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_import_as_names(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Import_as_namesContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_import_as_names(self);
    }
}

impl<'input> CustomRuleContext<'input> for Import_as_namesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_import_as_names
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_import_as_names }
}
antlr_rust::tid! {Import_as_namesContextExt<'a>}

impl<'input> Import_as_namesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Import_as_namesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Import_as_namesContextExt { ph: PhantomData },
        ))
    }
}

pub trait Import_as_namesContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Import_as_namesContextExt<'input>>
{
    fn import_as_name_all(&self) -> Vec<Rc<Import_as_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn import_as_name(&self, i: usize) -> Option<Rc<Import_as_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Import_as_namesContextAttrs<'input> for Import_as_namesContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn import_as_names(&mut self) -> Result<Rc<Import_as_namesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Import_as_namesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 66, RULE_import_as_names);
        let mut _localctx: Rc<Import_as_namesContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule import_as_name*/
                recog.base.set_state(619);
                recog.import_as_name()?;

                recog.base.set_state(624);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(75, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(620);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule import_as_name*/
                                recog.base.set_state(621);
                                recog.import_as_name()?;
                            }
                        }
                    }
                    recog.base.set_state(626);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(75, &mut recog.base)?;
                }
                recog.base.set_state(628);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(627);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dotted_as_names ----------------
pub type Dotted_as_namesContextAll<'input> = Dotted_as_namesContext<'input>;

pub type Dotted_as_namesContext<'input> =
    BaseParserRuleContext<'input, Dotted_as_namesContextExt<'input>>;

#[derive(Clone)]
pub struct Dotted_as_namesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Dotted_as_namesContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Dotted_as_namesContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dotted_as_names(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_dotted_as_names(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Dotted_as_namesContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_dotted_as_names(self);
    }
}

impl<'input> CustomRuleContext<'input> for Dotted_as_namesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dotted_as_names
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dotted_as_names }
}
antlr_rust::tid! {Dotted_as_namesContextExt<'a>}

impl<'input> Dotted_as_namesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Dotted_as_namesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Dotted_as_namesContextExt { ph: PhantomData },
        ))
    }
}

pub trait Dotted_as_namesContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Dotted_as_namesContextExt<'input>>
{
    fn dotted_as_name_all(&self) -> Vec<Rc<Dotted_as_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn dotted_as_name(&self, i: usize) -> Option<Rc<Dotted_as_nameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Dotted_as_namesContextAttrs<'input> for Dotted_as_namesContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dotted_as_names(&mut self) -> Result<Rc<Dotted_as_namesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Dotted_as_namesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 68, RULE_dotted_as_names);
        let mut _localctx: Rc<Dotted_as_namesContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule dotted_as_name*/
                recog.base.set_state(630);
                recog.dotted_as_name()?;

                recog.base.set_state(635);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(631);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule dotted_as_name*/
                            recog.base.set_state(632);
                            recog.dotted_as_name()?;
                        }
                    }
                    recog.base.set_state(637);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dotted_name ----------------
pub type Dotted_nameContextAll<'input> = Dotted_nameContext<'input>;

pub type Dotted_nameContext<'input> = BaseParserRuleContext<'input, Dotted_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Dotted_nameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Dotted_nameContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Dotted_nameContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dotted_name(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_dotted_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Dotted_nameContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_dotted_name(self);
    }
}

impl<'input> CustomRuleContext<'input> for Dotted_nameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dotted_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dotted_name }
}
antlr_rust::tid! {Dotted_nameContextExt<'a>}

impl<'input> Dotted_nameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Dotted_nameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Dotted_nameContextExt { ph: PhantomData },
        ))
    }
}

pub trait Dotted_nameContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Dotted_nameContextExt<'input>>
{
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> Dotted_nameContextAttrs<'input> for Dotted_nameContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dotted_name(&mut self) -> Result<Rc<Dotted_nameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Dotted_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 70, RULE_dotted_name);
        let mut _localctx: Rc<Dotted_nameContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(638);
                recog.name()?;

                recog.base.set_state(643);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == DOT {
                    {
                        {
                            recog.base.set_state(639);
                            recog.base.match_token(DOT, &mut recog.err_handler)?;

                            /*InvokeRule name*/
                            recog.base.set_state(640);
                            recog.name()?;
                        }
                    }
                    recog.base.set_state(645);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- global_stmt ----------------
pub type Global_stmtContextAll<'input> = Global_stmtContext<'input>;

pub type Global_stmtContext<'input> = BaseParserRuleContext<'input, Global_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Global_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Global_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Global_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_global_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_global_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Global_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_global_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Global_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_global_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_global_stmt }
}
antlr_rust::tid! {Global_stmtContextExt<'a>}

impl<'input> Global_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Global_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Global_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Global_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Global_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token GLOBAL
    /// Returns `None` if there is no child corresponding to token GLOBAL
    fn GLOBAL(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GLOBAL, 0)
    }
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Global_stmtContextAttrs<'input> for Global_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn global_stmt(&mut self) -> Result<Rc<Global_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Global_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 72, RULE_global_stmt);
        let mut _localctx: Rc<Global_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(646);
                recog.base.match_token(GLOBAL, &mut recog.err_handler)?;

                /*InvokeRule name*/
                recog.base.set_state(647);
                recog.name()?;

                recog.base.set_state(652);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(648);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule name*/
                            recog.base.set_state(649);
                            recog.name()?;
                        }
                    }
                    recog.base.set_state(654);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- nonlocal_stmt ----------------
pub type Nonlocal_stmtContextAll<'input> = Nonlocal_stmtContext<'input>;

pub type Nonlocal_stmtContext<'input> =
    BaseParserRuleContext<'input, Nonlocal_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Nonlocal_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Nonlocal_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Nonlocal_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_nonlocal_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_nonlocal_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Nonlocal_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_nonlocal_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Nonlocal_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_nonlocal_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_nonlocal_stmt }
}
antlr_rust::tid! {Nonlocal_stmtContextExt<'a>}

impl<'input> Nonlocal_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Nonlocal_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Nonlocal_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Nonlocal_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Nonlocal_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NONLOCAL
    /// Returns `None` if there is no child corresponding to token NONLOCAL
    fn NONLOCAL(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONLOCAL, 0)
    }
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Nonlocal_stmtContextAttrs<'input> for Nonlocal_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn nonlocal_stmt(&mut self) -> Result<Rc<Nonlocal_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Nonlocal_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 74, RULE_nonlocal_stmt);
        let mut _localctx: Rc<Nonlocal_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(655);
                recog.base.match_token(NONLOCAL, &mut recog.err_handler)?;

                /*InvokeRule name*/
                recog.base.set_state(656);
                recog.name()?;

                recog.base.set_state(661);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(657);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule name*/
                            recog.base.set_state(658);
                            recog.name()?;
                        }
                    }
                    recog.base.set_state(663);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- assert_stmt ----------------
pub type Assert_stmtContextAll<'input> = Assert_stmtContext<'input>;

pub type Assert_stmtContext<'input> = BaseParserRuleContext<'input, Assert_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Assert_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Assert_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Assert_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_assert_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_assert_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Assert_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_assert_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Assert_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_assert_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_assert_stmt }
}
antlr_rust::tid! {Assert_stmtContextExt<'a>}

impl<'input> Assert_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Assert_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Assert_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Assert_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Assert_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ASSERT
    /// Returns `None` if there is no child corresponding to token ASSERT
    fn ASSERT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSERT, 0)
    }
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, 0)
    }
}

impl<'input> Assert_stmtContextAttrs<'input> for Assert_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn assert_stmt(&mut self) -> Result<Rc<Assert_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Assert_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 76, RULE_assert_stmt);
        let mut _localctx: Rc<Assert_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(664);
                recog.base.match_token(ASSERT, &mut recog.err_handler)?;

                /*InvokeRule test*/
                recog.base.set_state(665);
                recog.test()?;

                recog.base.set_state(668);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(666);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                        /*InvokeRule test*/
                        recog.base.set_state(667);
                        recog.test()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- compound_stmt ----------------
pub type Compound_stmtContextAll<'input> = Compound_stmtContext<'input>;

pub type Compound_stmtContext<'input> =
    BaseParserRuleContext<'input, Compound_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Compound_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Compound_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Compound_stmtContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_compound_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_compound_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Compound_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_compound_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Compound_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_compound_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_compound_stmt }
}
antlr_rust::tid! {Compound_stmtContextExt<'a>}

impl<'input> Compound_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Compound_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Compound_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Compound_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Compound_stmtContextExt<'input>>
{
    fn if_stmt(&self) -> Option<Rc<If_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn while_stmt(&self) -> Option<Rc<While_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn for_stmt(&self) -> Option<Rc<For_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn try_stmt(&self) -> Option<Rc<Try_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn with_stmt(&self) -> Option<Rc<With_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn funcdef(&self) -> Option<Rc<FuncdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn classdef(&self) -> Option<Rc<ClassdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn decorated(&self) -> Option<Rc<DecoratedContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn async_stmt(&self) -> Option<Rc<Async_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn match_stmt(&self) -> Option<Rc<Match_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Compound_stmtContextAttrs<'input> for Compound_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn compound_stmt(&mut self) -> Result<Rc<Compound_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Compound_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 78, RULE_compound_stmt);
        let mut _localctx: Rc<Compound_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(680);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                IF => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule if_stmt*/
                        recog.base.set_state(670);
                        recog.if_stmt()?;
                    }
                }

                WHILE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule while_stmt*/
                        recog.base.set_state(671);
                        recog.while_stmt()?;
                    }
                }

                FOR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule for_stmt*/
                        recog.base.set_state(672);
                        recog.for_stmt()?;
                    }
                }

                TRY => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule try_stmt*/
                        recog.base.set_state(673);
                        recog.try_stmt()?;
                    }
                }

                WITH => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule with_stmt*/
                        recog.base.set_state(674);
                        recog.with_stmt()?;
                    }
                }

                DEF => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule funcdef*/
                        recog.base.set_state(675);
                        recog.funcdef()?;
                    }
                }

                CLASS => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule classdef*/
                        recog.base.set_state(676);
                        recog.classdef()?;
                    }
                }

                AT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule decorated*/
                        recog.base.set_state(677);
                        recog.decorated()?;
                    }
                }

                ASYNC => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        /*InvokeRule async_stmt*/
                        recog.base.set_state(678);
                        recog.async_stmt()?;
                    }
                }

                MATCH => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        /*InvokeRule match_stmt*/
                        recog.base.set_state(679);
                        recog.match_stmt()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- async_stmt ----------------
pub type Async_stmtContextAll<'input> = Async_stmtContext<'input>;

pub type Async_stmtContext<'input> = BaseParserRuleContext<'input, Async_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Async_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Async_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Async_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_async_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_async_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Async_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_async_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Async_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_async_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_async_stmt }
}
antlr_rust::tid! {Async_stmtContextExt<'a>}

impl<'input> Async_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Async_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Async_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Async_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Async_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ASYNC
    /// Returns `None` if there is no child corresponding to token ASYNC
    fn ASYNC(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASYNC, 0)
    }
    fn funcdef(&self) -> Option<Rc<FuncdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn with_stmt(&self) -> Option<Rc<With_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn for_stmt(&self) -> Option<Rc<For_stmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Async_stmtContextAttrs<'input> for Async_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn async_stmt(&mut self) -> Result<Rc<Async_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Async_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 80, RULE_async_stmt);
        let mut _localctx: Rc<Async_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(682);
                recog.base.match_token(ASYNC, &mut recog.err_handler)?;

                recog.base.set_state(686);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    DEF => {
                        {
                            /*InvokeRule funcdef*/
                            recog.base.set_state(683);
                            recog.funcdef()?;
                        }
                    }

                    WITH => {
                        {
                            /*InvokeRule with_stmt*/
                            recog.base.set_state(684);
                            recog.with_stmt()?;
                        }
                    }

                    FOR => {
                        {
                            /*InvokeRule for_stmt*/
                            recog.base.set_state(685);
                            recog.for_stmt()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- if_stmt ----------------
pub type If_stmtContextAll<'input> = If_stmtContext<'input>;

pub type If_stmtContext<'input> = BaseParserRuleContext<'input, If_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct If_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for If_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for If_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_if_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_if_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for If_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_if_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for If_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_if_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_if_stmt }
}
antlr_rust::tid! {If_stmtContextExt<'a>}

impl<'input> If_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<If_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            If_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait If_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<If_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
    fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
    /// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
    fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, i)
    }
    fn block_all(&self) -> Vec<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ELIF in current rule
    fn ELIF_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ELIF, starting from 0.
    /// Returns `None` if number of children corresponding to token ELIF is less or equal than `i`.
    fn ELIF(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELIF, i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
}

impl<'input> If_stmtContextAttrs<'input> for If_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn if_stmt(&mut self) -> Result<Rc<If_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = If_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_if_stmt);
        let mut _localctx: Rc<If_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(688);
                recog.base.match_token(IF, &mut recog.err_handler)?;

                /*InvokeRule test*/
                recog.base.set_state(689);
                recog.test()?;

                recog.base.set_state(690);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(691);
                recog.block()?;

                recog.base.set_state(699);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == ELIF {
                    {
                        {
                            recog.base.set_state(692);
                            recog.base.match_token(ELIF, &mut recog.err_handler)?;

                            /*InvokeRule test*/
                            recog.base.set_state(693);
                            recog.test()?;

                            recog.base.set_state(694);
                            recog.base.match_token(COLON, &mut recog.err_handler)?;

                            /*InvokeRule block*/
                            recog.base.set_state(695);
                            recog.block()?;
                        }
                    }
                    recog.base.set_state(701);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(705);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ELSE {
                    {
                        recog.base.set_state(702);
                        recog.base.match_token(ELSE, &mut recog.err_handler)?;

                        recog.base.set_state(703);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        /*InvokeRule block*/
                        recog.base.set_state(704);
                        recog.block()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- while_stmt ----------------
pub type While_stmtContextAll<'input> = While_stmtContext<'input>;

pub type While_stmtContext<'input> = BaseParserRuleContext<'input, While_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct While_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for While_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for While_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_while_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_while_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for While_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_while_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for While_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_while_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_while_stmt }
}
antlr_rust::tid! {While_stmtContextExt<'a>}

impl<'input> While_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<While_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            While_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait While_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<While_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token WHILE
    /// Returns `None` if there is no child corresponding to token WHILE
    fn WHILE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHILE, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
    fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
    /// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
    fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, i)
    }
    fn block_all(&self) -> Vec<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
}

impl<'input> While_stmtContextAttrs<'input> for While_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn while_stmt(&mut self) -> Result<Rc<While_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = While_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 84, RULE_while_stmt);
        let mut _localctx: Rc<While_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(707);
                recog.base.match_token(WHILE, &mut recog.err_handler)?;

                /*InvokeRule test*/
                recog.base.set_state(708);
                recog.test()?;

                recog.base.set_state(709);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(710);
                recog.block()?;

                recog.base.set_state(714);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ELSE {
                    {
                        recog.base.set_state(711);
                        recog.base.match_token(ELSE, &mut recog.err_handler)?;

                        recog.base.set_state(712);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        /*InvokeRule block*/
                        recog.base.set_state(713);
                        recog.block()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- for_stmt ----------------
pub type For_stmtContextAll<'input> = For_stmtContext<'input>;

pub type For_stmtContext<'input> = BaseParserRuleContext<'input, For_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct For_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for For_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for For_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_for_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_for_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for For_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_for_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for For_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_for_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_for_stmt }
}
antlr_rust::tid! {For_stmtContextExt<'a>}

impl<'input> For_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<For_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            For_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait For_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<For_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    fn exprlist(&self) -> Option<Rc<ExprlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    fn testlist(&self) -> Option<Rc<TestlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
    fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
    /// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
    fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, i)
    }
    fn block_all(&self) -> Vec<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
}

impl<'input> For_stmtContextAttrs<'input> for For_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn for_stmt(&mut self) -> Result<Rc<For_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = For_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_for_stmt);
        let mut _localctx: Rc<For_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(716);
                recog.base.match_token(FOR, &mut recog.err_handler)?;

                /*InvokeRule exprlist*/
                recog.base.set_state(717);
                recog.exprlist()?;

                recog.base.set_state(718);
                recog.base.match_token(IN, &mut recog.err_handler)?;

                /*InvokeRule testlist*/
                recog.base.set_state(719);
                recog.testlist()?;

                recog.base.set_state(720);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(721);
                recog.block()?;

                recog.base.set_state(725);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ELSE {
                    {
                        recog.base.set_state(722);
                        recog.base.match_token(ELSE, &mut recog.err_handler)?;

                        recog.base.set_state(723);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        /*InvokeRule block*/
                        recog.base.set_state(724);
                        recog.block()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- try_stmt ----------------
pub type Try_stmtContextAll<'input> = Try_stmtContext<'input>;

pub type Try_stmtContext<'input> = BaseParserRuleContext<'input, Try_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Try_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Try_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Try_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_try_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_try_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Try_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_try_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Try_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_try_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_try_stmt }
}
antlr_rust::tid! {Try_stmtContextExt<'a>}

impl<'input> Try_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Try_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Try_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Try_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Try_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token TRY
    /// Returns `None` if there is no child corresponding to token TRY
    fn TRY(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRY, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
    fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
    /// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
    fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, i)
    }
    fn block_all(&self) -> Vec<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn block(&self, i: usize) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token FINALLY
    /// Returns `None` if there is no child corresponding to token FINALLY
    fn FINALLY(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FINALLY, 0)
    }
    fn except_clause_all(&self) -> Vec<Rc<Except_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn except_clause(&self, i: usize) -> Option<Rc<Except_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
}

impl<'input> Try_stmtContextAttrs<'input> for Try_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn try_stmt(&mut self) -> Result<Rc<Try_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Try_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_try_stmt);
        let mut _localctx: Rc<Try_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    recog.base.set_state(727);
                    recog.base.match_token(TRY, &mut recog.err_handler)?;

                    recog.base.set_state(728);
                    recog.base.match_token(COLON, &mut recog.err_handler)?;

                    /*InvokeRule block*/
                    recog.base.set_state(729);
                    recog.block()?;

                    recog.base.set_state(751);
                    recog.err_handler.sync(&mut recog.base)?;
                    match recog.base.input.la(1) {
                        EXCEPT => {
                            {
                                recog.base.set_state(734);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                loop {
                                    {
                                        {
                                            /*InvokeRule except_clause*/
                                            recog.base.set_state(730);
                                            recog.except_clause()?;

                                            recog.base.set_state(731);
                                            recog
                                                .base
                                                .match_token(COLON, &mut recog.err_handler)?;

                                            /*InvokeRule block*/
                                            recog.base.set_state(732);
                                            recog.block()?;
                                        }
                                    }
                                    recog.base.set_state(736);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if !(_la == EXCEPT) {
                                        break;
                                    }
                                }
                                recog.base.set_state(741);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == ELSE {
                                    {
                                        recog.base.set_state(738);
                                        recog.base.match_token(ELSE, &mut recog.err_handler)?;

                                        recog.base.set_state(739);
                                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                                        /*InvokeRule block*/
                                        recog.base.set_state(740);
                                        recog.block()?;
                                    }
                                }

                                recog.base.set_state(746);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == FINALLY {
                                    {
                                        recog.base.set_state(743);
                                        recog.base.match_token(FINALLY, &mut recog.err_handler)?;

                                        recog.base.set_state(744);
                                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                                        /*InvokeRule block*/
                                        recog.base.set_state(745);
                                        recog.block()?;
                                    }
                                }
                            }
                        }

                        FINALLY => {
                            {
                                recog.base.set_state(748);
                                recog.base.match_token(FINALLY, &mut recog.err_handler)?;

                                recog.base.set_state(749);
                                recog.base.match_token(COLON, &mut recog.err_handler)?;

                                /*InvokeRule block*/
                                recog.base.set_state(750);
                                recog.block()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- with_stmt ----------------
pub type With_stmtContextAll<'input> = With_stmtContext<'input>;

pub type With_stmtContext<'input> = BaseParserRuleContext<'input, With_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct With_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for With_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for With_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_with_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_with_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for With_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_with_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for With_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_with_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_with_stmt }
}
antlr_rust::tid! {With_stmtContextExt<'a>}

impl<'input> With_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<With_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            With_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait With_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<With_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token WITH
    /// Returns `None` if there is no child corresponding to token WITH
    fn WITH(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WITH, 0)
    }
    fn with_item_all(&self) -> Vec<Rc<With_itemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn with_item(&self, i: usize) -> Option<Rc<With_itemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> With_stmtContextAttrs<'input> for With_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn with_stmt(&mut self) -> Result<Rc<With_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = With_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_with_stmt);
        let mut _localctx: Rc<With_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(753);
                recog.base.match_token(WITH, &mut recog.err_handler)?;

                /*InvokeRule with_item*/
                recog.base.set_state(754);
                recog.with_item()?;

                recog.base.set_state(759);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(755);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule with_item*/
                            recog.base.set_state(756);
                            recog.with_item()?;
                        }
                    }
                    recog.base.set_state(761);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(762);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(763);
                recog.block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- with_item ----------------
pub type With_itemContextAll<'input> = With_itemContext<'input>;

pub type With_itemContext<'input> = BaseParserRuleContext<'input, With_itemContextExt<'input>>;

#[derive(Clone)]
pub struct With_itemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for With_itemContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for With_itemContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_with_item(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_with_item(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for With_itemContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_with_item(self);
    }
}

impl<'input> CustomRuleContext<'input> for With_itemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_with_item
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_with_item }
}
antlr_rust::tid! {With_itemContextExt<'a>}

impl<'input> With_itemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<With_itemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            With_itemContextExt { ph: PhantomData },
        ))
    }
}

pub trait With_itemContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<With_itemContextExt<'input>>
{
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> With_itemContextAttrs<'input> for With_itemContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn with_item(&mut self) -> Result<Rc<With_itemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = With_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_with_item);
        let mut _localctx: Rc<With_itemContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule test*/
                recog.base.set_state(765);
                recog.test()?;

                recog.base.set_state(768);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == AS {
                    {
                        recog.base.set_state(766);
                        recog.base.match_token(AS, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(767);
                        recog.expr_rec(0)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- except_clause ----------------
pub type Except_clauseContextAll<'input> = Except_clauseContext<'input>;

pub type Except_clauseContext<'input> =
    BaseParserRuleContext<'input, Except_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Except_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Except_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Except_clauseContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_except_clause(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_except_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Except_clauseContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_except_clause(self);
    }
}

impl<'input> CustomRuleContext<'input> for Except_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_except_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_except_clause }
}
antlr_rust::tid! {Except_clauseContextExt<'a>}

impl<'input> Except_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Except_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Except_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Except_clauseContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Except_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token EXCEPT
    /// Returns `None` if there is no child corresponding to token EXCEPT
    fn EXCEPT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXCEPT, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Except_clauseContextAttrs<'input> for Except_clauseContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn except_clause(&mut self) -> Result<Rc<Except_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Except_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 94, RULE_except_clause);
        let mut _localctx: Rc<Except_clauseContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(770);
                recog.base.match_token(EXCEPT, &mut recog.err_handler)?;

                recog.base.set_state(776);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << AWAIT)
                            | (1usize << FALSE)
                            | (1usize << LAMBDA)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 33) & !0x3f) == 0
                        && ((1usize << (_la - 33))
                            & ((1usize << (NOT - 33))
                                | (1usize << (TRUE - 33))
                                | (1usize << (UNDERSCORE - 33))
                                | (1usize << (NAME - 33))
                                | (1usize << (ELLIPSIS - 33))
                                | (1usize << (OPEN_PAREN - 33))
                                | (1usize << (OPEN_BRACK - 33))))
                            != 0)
                    || (((_la - 71) & !0x3f) == 0
                        && ((1usize << (_la - 71))
                            & ((1usize << (ADD - 71))
                                | (1usize << (MINUS - 71))
                                | (1usize << (NOT_OP - 71))
                                | (1usize << (OPEN_BRACE - 71))))
                            != 0)
                {
                    {
                        /*InvokeRule test*/
                        recog.base.set_state(771);
                        recog.test()?;

                        recog.base.set_state(774);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == AS {
                            {
                                recog.base.set_state(772);
                                recog.base.match_token(AS, &mut recog.err_handler)?;

                                /*InvokeRule name*/
                                recog.base.set_state(773);
                                recog.name()?;
                            }
                        }
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;

pub type BlockContext<'input> = BaseParserRuleContext<'input, BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for BlockContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for BlockContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_block(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for BlockContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid! {BlockContextExt<'a>}

impl<'input> BlockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BlockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BlockContextExt { ph: PhantomData },
        ))
    }
}

pub trait BlockContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<BlockContextExt<'input>>
{
    fn simple_stmts(&self) -> Option<Rc<Simple_stmtsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NEWLINE
    /// Returns `None` if there is no child corresponding to token NEWLINE
    fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INDENT
    /// Returns `None` if there is no child corresponding to token INDENT
    fn INDENT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DEDENT
    /// Returns `None` if there is no child corresponding to token DEDENT
    fn DEDENT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEDENT, 0)
    }
    fn stmt_all(&self) -> Vec<Rc<StmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn block(&mut self) -> Result<Rc<BlockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(788);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STRING | NUMBER | ASSERT | AWAIT | BREAK | CONTINUE | DEL | FALSE | FROM
                | GLOBAL | IMPORT | LAMBDA | MATCH | NONE | NONLOCAL | NOT | PASS | RAISE
                | RETURN | TRUE | UNDERSCORE | YIELD | NAME | ELLIPSIS | STAR | OPEN_PAREN
                | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule simple_stmts*/
                        recog.base.set_state(778);
                        recog.simple_stmts()?;
                    }
                }

                NEWLINE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(779);
                        recog.base.match_token(NEWLINE, &mut recog.err_handler)?;

                        recog.base.set_state(780);
                        recog.base.match_token(INDENT, &mut recog.err_handler)?;

                        recog.base.set_state(782);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        loop {
                            {
                                {
                                    /*InvokeRule stmt*/
                                    recog.base.set_state(781);
                                    recog.stmt()?;
                                }
                            }
                            recog.base.set_state(784);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if !((((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << STRING)
                                        | (1usize << NUMBER)
                                        | (1usize << ASSERT)
                                        | (1usize << ASYNC)
                                        | (1usize << AWAIT)
                                        | (1usize << BREAK)
                                        | (1usize << CLASS)
                                        | (1usize << CONTINUE)
                                        | (1usize << DEF)
                                        | (1usize << DEL)
                                        | (1usize << FALSE)
                                        | (1usize << FOR)
                                        | (1usize << FROM)
                                        | (1usize << GLOBAL)
                                        | (1usize << IF)
                                        | (1usize << IMPORT)
                                        | (1usize << LAMBDA)
                                        | (1usize << MATCH)
                                        | (1usize << NONE)))
                                    != 0)
                                || (((_la - 32) & !0x3f) == 0
                                    && ((1usize << (_la - 32))
                                        & ((1usize << (NONLOCAL - 32))
                                            | (1usize << (NOT - 32))
                                            | (1usize << (PASS - 32))
                                            | (1usize << (RAISE - 32))
                                            | (1usize << (RETURN - 32))
                                            | (1usize << (TRUE - 32))
                                            | (1usize << (TRY - 32))
                                            | (1usize << (UNDERSCORE - 32))
                                            | (1usize << (WHILE - 32))
                                            | (1usize << (WITH - 32))
                                            | (1usize << (YIELD - 32))
                                            | (1usize << (NAME - 32))
                                            | (1usize << (ELLIPSIS - 32))
                                            | (1usize << (STAR - 32))
                                            | (1usize << (OPEN_PAREN - 32))))
                                        != 0)
                                || (((_la - 64) & !0x3f) == 0
                                    && ((1usize << (_la - 64))
                                        & ((1usize << (OPEN_BRACK - 64))
                                            | (1usize << (ADD - 64))
                                            | (1usize << (MINUS - 64))
                                            | (1usize << (NOT_OP - 64))
                                            | (1usize << (OPEN_BRACE - 64))
                                            | (1usize << (AT - 64))))
                                        != 0))
                            {
                                break;
                            }
                        }
                        recog.base.set_state(786);
                        recog.base.match_token(DEDENT, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- match_stmt ----------------
pub type Match_stmtContextAll<'input> = Match_stmtContext<'input>;

pub type Match_stmtContext<'input> = BaseParserRuleContext<'input, Match_stmtContextExt<'input>>;

#[derive(Clone)]
pub struct Match_stmtContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Match_stmtContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Match_stmtContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_match_stmt(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_match_stmt(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Match_stmtContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_match_stmt(self);
    }
}

impl<'input> CustomRuleContext<'input> for Match_stmtContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_match_stmt
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_match_stmt }
}
antlr_rust::tid! {Match_stmtContextExt<'a>}

impl<'input> Match_stmtContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Match_stmtContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Match_stmtContextExt { ph: PhantomData },
        ))
    }
}

pub trait Match_stmtContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Match_stmtContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token MATCH
    /// Returns `None` if there is no child corresponding to token MATCH
    fn MATCH(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH, 0)
    }
    fn subject_expr(&self) -> Option<Rc<Subject_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NEWLINE
    /// Returns `None` if there is no child corresponding to token NEWLINE
    fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token INDENT
    /// Returns `None` if there is no child corresponding to token INDENT
    fn INDENT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DEDENT
    /// Returns `None` if there is no child corresponding to token DEDENT
    fn DEDENT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEDENT, 0)
    }
    fn case_block_all(&self) -> Vec<Rc<Case_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn case_block(&self, i: usize) -> Option<Rc<Case_blockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Match_stmtContextAttrs<'input> for Match_stmtContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn match_stmt(&mut self) -> Result<Rc<Match_stmtContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Match_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 98, RULE_match_stmt);
        let mut _localctx: Rc<Match_stmtContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(790);
                recog.base.match_token(MATCH, &mut recog.err_handler)?;

                /*InvokeRule subject_expr*/
                recog.base.set_state(791);
                recog.subject_expr()?;

                recog.base.set_state(792);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                recog.base.set_state(793);
                recog.base.match_token(NEWLINE, &mut recog.err_handler)?;

                recog.base.set_state(794);
                recog.base.match_token(INDENT, &mut recog.err_handler)?;

                recog.base.set_state(796);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule case_block*/
                            recog.base.set_state(795);
                            recog.case_block()?;
                        }
                    }
                    recog.base.set_state(798);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == CASE) {
                        break;
                    }
                }
                recog.base.set_state(800);
                recog.base.match_token(DEDENT, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- subject_expr ----------------
pub type Subject_exprContextAll<'input> = Subject_exprContext<'input>;

pub type Subject_exprContext<'input> =
    BaseParserRuleContext<'input, Subject_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Subject_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Subject_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Subject_exprContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_subject_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_subject_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Subject_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_subject_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Subject_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_subject_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_subject_expr }
}
antlr_rust::tid! {Subject_exprContextExt<'a>}

impl<'input> Subject_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Subject_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Subject_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Subject_exprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Subject_exprContextExt<'input>>
{
    fn star_named_expression(&self) -> Option<Rc<Star_named_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, 0)
    }
    fn star_named_expressions(&self) -> Option<Rc<Star_named_expressionsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Subject_exprContextAttrs<'input> for Subject_exprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn subject_expr(&mut self) -> Result<Rc<Subject_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Subject_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 100, RULE_subject_expr);
        let mut _localctx: Rc<Subject_exprContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(808);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(100, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule star_named_expression*/
                        recog.base.set_state(802);
                        recog.star_named_expression()?;

                        recog.base.set_state(803);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                        recog.base.set_state(805);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                /*InvokeRule star_named_expressions*/
                                recog.base.set_state(804);
                                recog.star_named_expressions()?;
                            }
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule test*/
                        recog.base.set_state(807);
                        recog.test()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- star_named_expressions ----------------
pub type Star_named_expressionsContextAll<'input> = Star_named_expressionsContext<'input>;

pub type Star_named_expressionsContext<'input> =
    BaseParserRuleContext<'input, Star_named_expressionsContextExt<'input>>;

#[derive(Clone)]
pub struct Star_named_expressionsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Star_named_expressionsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Star_named_expressionsContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_star_named_expressions(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_star_named_expressions(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Star_named_expressionsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_star_named_expressions(self);
    }
}

impl<'input> CustomRuleContext<'input> for Star_named_expressionsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_star_named_expressions
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_star_named_expressions }
}
antlr_rust::tid! {Star_named_expressionsContextExt<'a>}

impl<'input> Star_named_expressionsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Star_named_expressionsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Star_named_expressionsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Star_named_expressionsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Star_named_expressionsContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    fn star_named_expression_all(&self) -> Vec<Rc<Star_named_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn star_named_expression(&self, i: usize) -> Option<Rc<Star_named_expressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Star_named_expressionsContextAttrs<'input> for Star_named_expressionsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn star_named_expressions(
        &mut self,
    ) -> Result<Rc<Star_named_expressionsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Star_named_expressionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 102, RULE_star_named_expressions);
        let mut _localctx: Rc<Star_named_expressionsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(810);
                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                recog.base.set_state(812);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule star_named_expression*/
                            recog.base.set_state(811);
                            recog.star_named_expression()?;
                        }
                    }
                    recog.base.set_state(814);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !((((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << STRING)
                                | (1usize << NUMBER)
                                | (1usize << AWAIT)
                                | (1usize << FALSE)
                                | (1usize << LAMBDA)
                                | (1usize << MATCH)
                                | (1usize << NONE)))
                            != 0)
                        || (((_la - 33) & !0x3f) == 0
                            && ((1usize << (_la - 33))
                                & ((1usize << (NOT - 33))
                                    | (1usize << (TRUE - 33))
                                    | (1usize << (UNDERSCORE - 33))
                                    | (1usize << (NAME - 33))
                                    | (1usize << (ELLIPSIS - 33))
                                    | (1usize << (STAR - 33))
                                    | (1usize << (OPEN_PAREN - 33))
                                    | (1usize << (OPEN_BRACK - 33))))
                                != 0)
                        || (((_la - 71) & !0x3f) == 0
                            && ((1usize << (_la - 71))
                                & ((1usize << (ADD - 71))
                                    | (1usize << (MINUS - 71))
                                    | (1usize << (NOT_OP - 71))
                                    | (1usize << (OPEN_BRACE - 71))))
                                != 0))
                    {
                        break;
                    }
                }
                recog.base.set_state(817);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(816);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- star_named_expression ----------------
pub type Star_named_expressionContextAll<'input> = Star_named_expressionContext<'input>;

pub type Star_named_expressionContext<'input> =
    BaseParserRuleContext<'input, Star_named_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Star_named_expressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Star_named_expressionContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Star_named_expressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_star_named_expression(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_star_named_expression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Star_named_expressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_star_named_expression(self);
    }
}

impl<'input> CustomRuleContext<'input> for Star_named_expressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_star_named_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_star_named_expression }
}
antlr_rust::tid! {Star_named_expressionContextExt<'a>}

impl<'input> Star_named_expressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Star_named_expressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Star_named_expressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Star_named_expressionContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Star_named_expressionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Star_named_expressionContextAttrs<'input> for Star_named_expressionContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn star_named_expression(
        &mut self,
    ) -> Result<Rc<Star_named_expressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Star_named_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 104, RULE_star_named_expression);
        let mut _localctx: Rc<Star_named_expressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(822);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STAR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(819);
                        recog.base.match_token(STAR, &mut recog.err_handler)?;

                        /*InvokeRule expr*/
                        recog.base.set_state(820);
                        recog.expr_rec(0)?;
                    }
                }

                STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT | TRUE
                | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP
                | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule test*/
                        recog.base.set_state(821);
                        recog.test()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- case_block ----------------
pub type Case_blockContextAll<'input> = Case_blockContext<'input>;

pub type Case_blockContext<'input> = BaseParserRuleContext<'input, Case_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Case_blockContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Case_blockContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Case_blockContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_case_block(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_case_block(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Case_blockContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_case_block(self);
    }
}

impl<'input> CustomRuleContext<'input> for Case_blockContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_case_block
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_case_block }
}
antlr_rust::tid! {Case_blockContextExt<'a>}

impl<'input> Case_blockContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Case_blockContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Case_blockContextExt { ph: PhantomData },
        ))
    }
}

pub trait Case_blockContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Case_blockContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CASE, 0)
    }
    fn patterns(&self) -> Option<Rc<PatternsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn guard(&self) -> Option<Rc<GuardContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Case_blockContextAttrs<'input> for Case_blockContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn case_block(&mut self) -> Result<Rc<Case_blockContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Case_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 106, RULE_case_block);
        let mut _localctx: Rc<Case_blockContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(824);
                recog.base.match_token(CASE, &mut recog.err_handler)?;

                /*InvokeRule patterns*/
                recog.base.set_state(825);
                recog.patterns()?;

                recog.base.set_state(827);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == IF {
                    {
                        /*InvokeRule guard*/
                        recog.base.set_state(826);
                        recog.guard()?;
                    }
                }

                recog.base.set_state(829);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(830);
                recog.block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- guard ----------------
pub type GuardContextAll<'input> = GuardContext<'input>;

pub type GuardContext<'input> = BaseParserRuleContext<'input, GuardContextExt<'input>>;

#[derive(Clone)]
pub struct GuardContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for GuardContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for GuardContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_guard(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_guard(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for GuardContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_guard(self);
    }
}

impl<'input> CustomRuleContext<'input> for GuardContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_guard
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_guard }
}
antlr_rust::tid! {GuardContextExt<'a>}

impl<'input> GuardContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<GuardContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            GuardContextExt { ph: PhantomData },
        ))
    }
}

pub trait GuardContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<GuardContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> GuardContextAttrs<'input> for GuardContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn guard(&mut self) -> Result<Rc<GuardContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = GuardContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_guard);
        let mut _localctx: Rc<GuardContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(832);
                recog.base.match_token(IF, &mut recog.err_handler)?;

                /*InvokeRule test*/
                recog.base.set_state(833);
                recog.test()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- patterns ----------------
pub type PatternsContextAll<'input> = PatternsContext<'input>;

pub type PatternsContext<'input> = BaseParserRuleContext<'input, PatternsContextExt<'input>>;

#[derive(Clone)]
pub struct PatternsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for PatternsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for PatternsContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_patterns(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_patterns(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for PatternsContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_patterns(self);
    }
}

impl<'input> CustomRuleContext<'input> for PatternsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_patterns
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_patterns }
}
antlr_rust::tid! {PatternsContextExt<'a>}

impl<'input> PatternsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PatternsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PatternsContextExt { ph: PhantomData },
        ))
    }
}

pub trait PatternsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<PatternsContextExt<'input>>
{
    fn open_sequence_pattern(&self) -> Option<Rc<Open_sequence_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> PatternsContextAttrs<'input> for PatternsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn patterns(&mut self) -> Result<Rc<PatternsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PatternsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_patterns);
        let mut _localctx: Rc<PatternsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(837);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(105, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule open_sequence_pattern*/
                        recog.base.set_state(835);
                        recog.open_sequence_pattern()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule pattern*/
                        recog.base.set_state(836);
                        recog.pattern()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;

pub type PatternContext<'input> = BaseParserRuleContext<'input, PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for PatternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for PatternContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for PatternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::tid! {PatternContextExt<'a>}

impl<'input> PatternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<PatternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            PatternContextExt { ph: PhantomData },
        ))
    }
}

pub trait PatternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<PatternContextExt<'input>>
{
    fn as_pattern(&self) -> Option<Rc<As_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn or_pattern(&self) -> Option<Rc<Or_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pattern(&mut self) -> Result<Rc<PatternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(841);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(106, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule as_pattern*/
                        recog.base.set_state(839);
                        recog.as_pattern()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule or_pattern*/
                        recog.base.set_state(840);
                        recog.or_pattern()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- as_pattern ----------------
pub type As_patternContextAll<'input> = As_patternContext<'input>;

pub type As_patternContext<'input> = BaseParserRuleContext<'input, As_patternContextExt<'input>>;

#[derive(Clone)]
pub struct As_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for As_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for As_patternContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_as_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_as_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for As_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_as_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for As_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_as_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_as_pattern }
}
antlr_rust::tid! {As_patternContextExt<'a>}

impl<'input> As_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<As_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            As_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait As_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<As_patternContextExt<'input>>
{
    fn or_pattern(&self) -> Option<Rc<Or_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    fn pattern_capture_target(&self) -> Option<Rc<Pattern_capture_targetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> As_patternContextAttrs<'input> for As_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn as_pattern(&mut self) -> Result<Rc<As_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = As_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 114, RULE_as_pattern);
        let mut _localctx: Rc<As_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule or_pattern*/
                recog.base.set_state(843);
                recog.or_pattern()?;

                recog.base.set_state(844);
                recog.base.match_token(AS, &mut recog.err_handler)?;

                /*InvokeRule pattern_capture_target*/
                recog.base.set_state(845);
                recog.pattern_capture_target()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- or_pattern ----------------
pub type Or_patternContextAll<'input> = Or_patternContext<'input>;

pub type Or_patternContext<'input> = BaseParserRuleContext<'input, Or_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Or_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Or_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Or_patternContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_or_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_or_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Or_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_or_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Or_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_or_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_or_pattern }
}
antlr_rust::tid! {Or_patternContextExt<'a>}

impl<'input> Or_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Or_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Or_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Or_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Or_patternContextExt<'input>>
{
    fn closed_pattern_all(&self) -> Vec<Rc<Closed_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn closed_pattern(&self, i: usize) -> Option<Rc<Closed_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR_OP in current rule
    fn OR_OP_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OR_OP, starting from 0.
    /// Returns `None` if number of children corresponding to token OR_OP is less or equal than `i`.
    fn OR_OP(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR_OP, i)
    }
}

impl<'input> Or_patternContextAttrs<'input> for Or_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn or_pattern(&mut self) -> Result<Rc<Or_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Or_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 116, RULE_or_pattern);
        let mut _localctx: Rc<Or_patternContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule closed_pattern*/
                recog.base.set_state(847);
                recog.closed_pattern()?;

                recog.base.set_state(852);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OR_OP {
                    {
                        {
                            recog.base.set_state(848);
                            recog.base.match_token(OR_OP, &mut recog.err_handler)?;

                            /*InvokeRule closed_pattern*/
                            recog.base.set_state(849);
                            recog.closed_pattern()?;
                        }
                    }
                    recog.base.set_state(854);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- closed_pattern ----------------
pub type Closed_patternContextAll<'input> = Closed_patternContext<'input>;

pub type Closed_patternContext<'input> =
    BaseParserRuleContext<'input, Closed_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Closed_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Closed_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Closed_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_closed_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_closed_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Closed_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_closed_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Closed_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_closed_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_closed_pattern }
}
antlr_rust::tid! {Closed_patternContextExt<'a>}

impl<'input> Closed_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Closed_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Closed_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Closed_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Closed_patternContextExt<'input>>
{
    fn literal_pattern(&self) -> Option<Rc<Literal_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn capture_pattern(&self) -> Option<Rc<Capture_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn wildcard_pattern(&self) -> Option<Rc<Wildcard_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn value_pattern(&self) -> Option<Rc<Value_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn group_pattern(&self) -> Option<Rc<Group_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn sequence_pattern(&self) -> Option<Rc<Sequence_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn mapping_pattern(&self) -> Option<Rc<Mapping_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn class_pattern(&self) -> Option<Rc<Class_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Closed_patternContextAttrs<'input> for Closed_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn closed_pattern(&mut self) -> Result<Rc<Closed_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Closed_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 118, RULE_closed_pattern);
        let mut _localctx: Rc<Closed_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(863);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(108, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule literal_pattern*/
                        recog.base.set_state(855);
                        recog.literal_pattern()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule capture_pattern*/
                        recog.base.set_state(856);
                        recog.capture_pattern()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule wildcard_pattern*/
                        recog.base.set_state(857);
                        recog.wildcard_pattern()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule value_pattern*/
                        recog.base.set_state(858);
                        recog.value_pattern()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule group_pattern*/
                        recog.base.set_state(859);
                        recog.group_pattern()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule sequence_pattern*/
                        recog.base.set_state(860);
                        recog.sequence_pattern()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule mapping_pattern*/
                        recog.base.set_state(861);
                        recog.mapping_pattern()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule class_pattern*/
                        recog.base.set_state(862);
                        recog.class_pattern()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- literal_pattern ----------------
pub type Literal_patternContextAll<'input> = Literal_patternContext<'input>;

pub type Literal_patternContext<'input> =
    BaseParserRuleContext<'input, Literal_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Literal_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Literal_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_literal_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_literal_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Literal_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_literal_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Literal_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal_pattern }
}
antlr_rust::tid! {Literal_patternContextExt<'a>}

impl<'input> Literal_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Literal_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Literal_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Literal_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Literal_patternContextExt<'input>>
{
    fn signed_number(&self) -> Option<Rc<Signed_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn complex_number(&self) -> Option<Rc<Complex_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn strings(&self) -> Option<Rc<StringsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NONE
    /// Returns `None` if there is no child corresponding to token NONE
    fn NONE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
}

impl<'input> Literal_patternContextAttrs<'input> for Literal_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn literal_pattern(&mut self) -> Result<Rc<Literal_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Literal_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 120, RULE_literal_pattern);
        let mut _localctx: Rc<Literal_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(873);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(109, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule signed_number*/
                        recog.base.set_state(865);
                        recog.signed_number()?;

                        recog.base.set_state(866);
                        if !({ recog.CannotBePlusMinus() }) {
                            Err(FailedPredicateError::new(
                                &mut recog.base,
                                Some(" recog.CannotBePlusMinus() ".to_owned()),
                                None,
                            ))?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule complex_number*/
                        recog.base.set_state(868);
                        recog.complex_number()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule strings*/
                        recog.base.set_state(869);
                        recog.strings()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(870);
                        recog.base.match_token(NONE, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(871);
                        recog.base.match_token(TRUE, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(872);
                        recog.base.match_token(FALSE, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- literal_expr ----------------
pub type Literal_exprContextAll<'input> = Literal_exprContext<'input>;

pub type Literal_exprContext<'input> =
    BaseParserRuleContext<'input, Literal_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Literal_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Literal_exprContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_literal_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_literal_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Literal_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_literal_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Literal_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_literal_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_literal_expr }
}
antlr_rust::tid! {Literal_exprContextExt<'a>}

impl<'input> Literal_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Literal_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Literal_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Literal_exprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Literal_exprContextExt<'input>>
{
    fn signed_number(&self) -> Option<Rc<Signed_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn complex_number(&self) -> Option<Rc<Complex_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn strings(&self) -> Option<Rc<StringsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NONE
    /// Returns `None` if there is no child corresponding to token NONE
    fn NONE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
}

impl<'input> Literal_exprContextAttrs<'input> for Literal_exprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn literal_expr(&mut self) -> Result<Rc<Literal_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Literal_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 122, RULE_literal_expr);
        let mut _localctx: Rc<Literal_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(883);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(110, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule signed_number*/
                        recog.base.set_state(875);
                        recog.signed_number()?;

                        recog.base.set_state(876);
                        if !({ recog.CannotBePlusMinus() }) {
                            Err(FailedPredicateError::new(
                                &mut recog.base,
                                Some(" recog.CannotBePlusMinus() ".to_owned()),
                                None,
                            ))?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule complex_number*/
                        recog.base.set_state(878);
                        recog.complex_number()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule strings*/
                        recog.base.set_state(879);
                        recog.strings()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(880);
                        recog.base.match_token(NONE, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(881);
                        recog.base.match_token(TRUE, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(882);
                        recog.base.match_token(FALSE, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- complex_number ----------------
pub type Complex_numberContextAll<'input> = Complex_numberContext<'input>;

pub type Complex_numberContext<'input> =
    BaseParserRuleContext<'input, Complex_numberContextExt<'input>>;

#[derive(Clone)]
pub struct Complex_numberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Complex_numberContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Complex_numberContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_complex_number(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_complex_number(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Complex_numberContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_complex_number(self);
    }
}

impl<'input> CustomRuleContext<'input> for Complex_numberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_complex_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_complex_number }
}
antlr_rust::tid! {Complex_numberContextExt<'a>}

impl<'input> Complex_numberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Complex_numberContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Complex_numberContextExt { ph: PhantomData },
        ))
    }
}

pub trait Complex_numberContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Complex_numberContextExt<'input>>
{
    fn signed_real_number(&self) -> Option<Rc<Signed_real_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ADD
    /// Returns `None` if there is no child corresponding to token ADD
    fn ADD(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD, 0)
    }
    fn imaginary_number(&self) -> Option<Rc<Imaginary_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> Complex_numberContextAttrs<'input> for Complex_numberContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn complex_number(&mut self) -> Result<Rc<Complex_numberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Complex_numberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 124, RULE_complex_number);
        let mut _localctx: Rc<Complex_numberContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(893);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(111, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule signed_real_number*/
                        recog.base.set_state(885);
                        recog.signed_real_number()?;

                        recog.base.set_state(886);
                        recog.base.match_token(ADD, &mut recog.err_handler)?;

                        /*InvokeRule imaginary_number*/
                        recog.base.set_state(887);
                        recog.imaginary_number()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule signed_real_number*/
                        recog.base.set_state(889);
                        recog.signed_real_number()?;

                        recog.base.set_state(890);
                        recog.base.match_token(MINUS, &mut recog.err_handler)?;

                        /*InvokeRule imaginary_number*/
                        recog.base.set_state(891);
                        recog.imaginary_number()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- signed_number ----------------
pub type Signed_numberContextAll<'input> = Signed_numberContext<'input>;

pub type Signed_numberContext<'input> =
    BaseParserRuleContext<'input, Signed_numberContextExt<'input>>;

#[derive(Clone)]
pub struct Signed_numberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Signed_numberContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Signed_numberContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_signed_number(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_signed_number(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Signed_numberContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_signed_number(self);
    }
}

impl<'input> CustomRuleContext<'input> for Signed_numberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_signed_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_signed_number }
}
antlr_rust::tid! {Signed_numberContextExt<'a>}

impl<'input> Signed_numberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Signed_numberContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Signed_numberContextExt { ph: PhantomData },
        ))
    }
}

pub trait Signed_numberContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Signed_numberContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> Signed_numberContextAttrs<'input> for Signed_numberContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn signed_number(&mut self) -> Result<Rc<Signed_numberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Signed_numberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 126, RULE_signed_number);
        let mut _localctx: Rc<Signed_numberContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(898);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                NUMBER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(895);
                        recog.base.match_token(NUMBER, &mut recog.err_handler)?;
                    }
                }

                MINUS => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(896);
                        recog.base.match_token(MINUS, &mut recog.err_handler)?;

                        recog.base.set_state(897);
                        recog.base.match_token(NUMBER, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- signed_real_number ----------------
pub type Signed_real_numberContextAll<'input> = Signed_real_numberContext<'input>;

pub type Signed_real_numberContext<'input> =
    BaseParserRuleContext<'input, Signed_real_numberContextExt<'input>>;

#[derive(Clone)]
pub struct Signed_real_numberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Signed_real_numberContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Signed_real_numberContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_signed_real_number(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_signed_real_number(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Signed_real_numberContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_signed_real_number(self);
    }
}

impl<'input> CustomRuleContext<'input> for Signed_real_numberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_signed_real_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_signed_real_number }
}
antlr_rust::tid! {Signed_real_numberContextExt<'a>}

impl<'input> Signed_real_numberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Signed_real_numberContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Signed_real_numberContextExt { ph: PhantomData },
        ))
    }
}

pub trait Signed_real_numberContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Signed_real_numberContextExt<'input>>
{
    fn real_number(&self) -> Option<Rc<Real_numberContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token MINUS
    /// Returns `None` if there is no child corresponding to token MINUS
    fn MINUS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, 0)
    }
}

impl<'input> Signed_real_numberContextAttrs<'input> for Signed_real_numberContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn signed_real_number(
        &mut self,
    ) -> Result<Rc<Signed_real_numberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Signed_real_numberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 128, RULE_signed_real_number);
        let mut _localctx: Rc<Signed_real_numberContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(903);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                NUMBER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule real_number*/
                        recog.base.set_state(900);
                        recog.real_number()?;
                    }
                }

                MINUS => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(901);
                        recog.base.match_token(MINUS, &mut recog.err_handler)?;

                        /*InvokeRule real_number*/
                        recog.base.set_state(902);
                        recog.real_number()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- real_number ----------------
pub type Real_numberContextAll<'input> = Real_numberContext<'input>;

pub type Real_numberContext<'input> = BaseParserRuleContext<'input, Real_numberContextExt<'input>>;

#[derive(Clone)]
pub struct Real_numberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Real_numberContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Real_numberContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_real_number(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_real_number(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Real_numberContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_real_number(self);
    }
}

impl<'input> CustomRuleContext<'input> for Real_numberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_real_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_real_number }
}
antlr_rust::tid! {Real_numberContextExt<'a>}

impl<'input> Real_numberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Real_numberContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Real_numberContextExt { ph: PhantomData },
        ))
    }
}

pub trait Real_numberContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Real_numberContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
}

impl<'input> Real_numberContextAttrs<'input> for Real_numberContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn real_number(&mut self) -> Result<Rc<Real_numberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Real_numberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 130, RULE_real_number);
        let mut _localctx: Rc<Real_numberContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(905);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- imaginary_number ----------------
pub type Imaginary_numberContextAll<'input> = Imaginary_numberContext<'input>;

pub type Imaginary_numberContext<'input> =
    BaseParserRuleContext<'input, Imaginary_numberContextExt<'input>>;

#[derive(Clone)]
pub struct Imaginary_numberContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Imaginary_numberContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Imaginary_numberContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_imaginary_number(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_imaginary_number(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Imaginary_numberContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_imaginary_number(self);
    }
}

impl<'input> CustomRuleContext<'input> for Imaginary_numberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_imaginary_number
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_imaginary_number }
}
antlr_rust::tid! {Imaginary_numberContextExt<'a>}

impl<'input> Imaginary_numberContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Imaginary_numberContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Imaginary_numberContextExt { ph: PhantomData },
        ))
    }
}

pub trait Imaginary_numberContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Imaginary_numberContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
}

impl<'input> Imaginary_numberContextAttrs<'input> for Imaginary_numberContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn imaginary_number(
        &mut self,
    ) -> Result<Rc<Imaginary_numberContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Imaginary_numberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 132, RULE_imaginary_number);
        let mut _localctx: Rc<Imaginary_numberContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(907);
                recog.base.match_token(NUMBER, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- capture_pattern ----------------
pub type Capture_patternContextAll<'input> = Capture_patternContext<'input>;

pub type Capture_patternContext<'input> =
    BaseParserRuleContext<'input, Capture_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Capture_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Capture_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Capture_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_capture_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_capture_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Capture_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_capture_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Capture_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_capture_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_capture_pattern }
}
antlr_rust::tid! {Capture_patternContextExt<'a>}

impl<'input> Capture_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Capture_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Capture_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Capture_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Capture_patternContextExt<'input>>
{
    fn pattern_capture_target(&self) -> Option<Rc<Pattern_capture_targetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Capture_patternContextAttrs<'input> for Capture_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn capture_pattern(&mut self) -> Result<Rc<Capture_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Capture_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 134, RULE_capture_pattern);
        let mut _localctx: Rc<Capture_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule pattern_capture_target*/
                recog.base.set_state(909);
                recog.pattern_capture_target()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- pattern_capture_target ----------------
pub type Pattern_capture_targetContextAll<'input> = Pattern_capture_targetContext<'input>;

pub type Pattern_capture_targetContext<'input> =
    BaseParserRuleContext<'input, Pattern_capture_targetContextExt<'input>>;

#[derive(Clone)]
pub struct Pattern_capture_targetContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Pattern_capture_targetContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Pattern_capture_targetContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_pattern_capture_target(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_pattern_capture_target(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Pattern_capture_targetContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_pattern_capture_target(self);
    }
}

impl<'input> CustomRuleContext<'input> for Pattern_capture_targetContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_pattern_capture_target
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_pattern_capture_target }
}
antlr_rust::tid! {Pattern_capture_targetContextExt<'a>}

impl<'input> Pattern_capture_targetContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Pattern_capture_targetContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Pattern_capture_targetContextExt { ph: PhantomData },
        ))
    }
}

pub trait Pattern_capture_targetContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Pattern_capture_targetContextExt<'input>>
{
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Pattern_capture_targetContextAttrs<'input> for Pattern_capture_targetContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn pattern_capture_target(
        &mut self,
    ) -> Result<Rc<Pattern_capture_targetContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Pattern_capture_targetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 136, RULE_pattern_capture_target);
        let mut _localctx: Rc<Pattern_capture_targetContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(911);
                recog.name()?;

                recog.base.set_state(912);
                if !({ recog.CannotBeDotLpEq() }) {
                    Err(FailedPredicateError::new(
                        &mut recog.base,
                        Some(" recog.CannotBeDotLpEq() ".to_owned()),
                        None,
                    ))?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- wildcard_pattern ----------------
pub type Wildcard_patternContextAll<'input> = Wildcard_patternContext<'input>;

pub type Wildcard_patternContext<'input> =
    BaseParserRuleContext<'input, Wildcard_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Wildcard_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Wildcard_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Wildcard_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_wildcard_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_wildcard_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Wildcard_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_wildcard_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Wildcard_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_wildcard_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_wildcard_pattern }
}
antlr_rust::tid! {Wildcard_patternContextExt<'a>}

impl<'input> Wildcard_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Wildcard_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Wildcard_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Wildcard_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Wildcard_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UNDERSCORE
    /// Returns `None` if there is no child corresponding to token UNDERSCORE
    fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNDERSCORE, 0)
    }
}

impl<'input> Wildcard_patternContextAttrs<'input> for Wildcard_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn wildcard_pattern(
        &mut self,
    ) -> Result<Rc<Wildcard_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Wildcard_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 138, RULE_wildcard_pattern);
        let mut _localctx: Rc<Wildcard_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(914);
                recog.base.match_token(UNDERSCORE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- value_pattern ----------------
pub type Value_patternContextAll<'input> = Value_patternContext<'input>;

pub type Value_patternContext<'input> =
    BaseParserRuleContext<'input, Value_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Value_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Value_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Value_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_value_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_value_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Value_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_value_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Value_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value_pattern }
}
antlr_rust::tid! {Value_patternContextExt<'a>}

impl<'input> Value_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Value_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Value_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Value_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Value_patternContextExt<'input>>
{
    fn attr(&self) -> Option<Rc<AttrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Value_patternContextAttrs<'input> for Value_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value_pattern(&mut self) -> Result<Rc<Value_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Value_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 140, RULE_value_pattern);
        let mut _localctx: Rc<Value_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule attr*/
                recog.base.set_state(916);
                recog.attr()?;

                recog.base.set_state(917);
                if !({ recog.CannotBeDotLpEq() }) {
                    Err(FailedPredicateError::new(
                        &mut recog.base,
                        Some(" recog.CannotBeDotLpEq() ".to_owned()),
                        None,
                    ))?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- attr ----------------
pub type AttrContextAll<'input> = AttrContext<'input>;

pub type AttrContext<'input> = BaseParserRuleContext<'input, AttrContextExt<'input>>;

#[derive(Clone)]
pub struct AttrContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for AttrContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for AttrContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_attr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_attr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for AttrContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_attr(self);
    }
}

impl<'input> CustomRuleContext<'input> for AttrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_attr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_attr }
}
antlr_rust::tid! {AttrContextExt<'a>}

impl<'input> AttrContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AttrContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AttrContextExt { ph: PhantomData },
        ))
    }
}

pub trait AttrContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<AttrContextExt<'input>>
{
    fn name_all(&self) -> Vec<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
    fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, i)
    }
}

impl<'input> AttrContextAttrs<'input> for AttrContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn attr(&mut self) -> Result<Rc<AttrContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AttrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_attr);
        let mut _localctx: Rc<AttrContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(919);
                recog.name()?;

                recog.base.set_state(922);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1;
                loop {
                    match _alt {
                        x if x == 1 => {
                            {
                                recog.base.set_state(920);
                                recog.base.match_token(DOT, &mut recog.err_handler)?;

                                /*InvokeRule name*/
                                recog.base.set_state(921);
                                recog.name()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(924);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(114, &mut recog.base)?;
                    if _alt == 2 || _alt == INVALID_ALT {
                        break;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- name_or_attr ----------------
pub type Name_or_attrContextAll<'input> = Name_or_attrContext<'input>;

pub type Name_or_attrContext<'input> =
    BaseParserRuleContext<'input, Name_or_attrContextExt<'input>>;

#[derive(Clone)]
pub struct Name_or_attrContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Name_or_attrContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Name_or_attrContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_name_or_attr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_name_or_attr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Name_or_attrContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_name_or_attr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Name_or_attrContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_name_or_attr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_name_or_attr }
}
antlr_rust::tid! {Name_or_attrContextExt<'a>}

impl<'input> Name_or_attrContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Name_or_attrContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Name_or_attrContextExt { ph: PhantomData },
        ))
    }
}

pub trait Name_or_attrContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Name_or_attrContextExt<'input>>
{
    fn attr(&self) -> Option<Rc<AttrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Name_or_attrContextAttrs<'input> for Name_or_attrContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn name_or_attr(&mut self) -> Result<Rc<Name_or_attrContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Name_or_attrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 144, RULE_name_or_attr);
        let mut _localctx: Rc<Name_or_attrContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(928);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(115, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule attr*/
                        recog.base.set_state(926);
                        recog.attr()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule name*/
                        recog.base.set_state(927);
                        recog.name()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- group_pattern ----------------
pub type Group_patternContextAll<'input> = Group_patternContext<'input>;

pub type Group_patternContext<'input> =
    BaseParserRuleContext<'input, Group_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Group_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Group_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Group_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_group_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_group_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Group_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_group_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Group_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_group_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_group_pattern }
}
antlr_rust::tid! {Group_patternContextExt<'a>}

impl<'input> Group_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Group_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Group_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Group_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Group_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
}

impl<'input> Group_patternContextAttrs<'input> for Group_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn group_pattern(&mut self) -> Result<Rc<Group_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Group_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 146, RULE_group_pattern);
        let mut _localctx: Rc<Group_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(930);
                recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                /*InvokeRule pattern*/
                recog.base.set_state(931);
                recog.pattern()?;

                recog.base.set_state(932);
                recog
                    .base
                    .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sequence_pattern ----------------
pub type Sequence_patternContextAll<'input> = Sequence_patternContext<'input>;

pub type Sequence_patternContext<'input> =
    BaseParserRuleContext<'input, Sequence_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Sequence_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Sequence_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Sequence_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sequence_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_sequence_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Sequence_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_sequence_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Sequence_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sequence_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sequence_pattern }
}
antlr_rust::tid! {Sequence_patternContextExt<'a>}

impl<'input> Sequence_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Sequence_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Sequence_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Sequence_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Sequence_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_BRACK
    /// Returns `None` if there is no child corresponding to token OPEN_BRACK
    fn OPEN_BRACK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_BRACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_BRACK
    /// Returns `None` if there is no child corresponding to token CLOSE_BRACK
    fn CLOSE_BRACK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_BRACK, 0)
    }
    fn maybe_sequence_pattern(&self) -> Option<Rc<Maybe_sequence_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn open_sequence_pattern(&self) -> Option<Rc<Open_sequence_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Sequence_patternContextAttrs<'input> for Sequence_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sequence_pattern(
        &mut self,
    ) -> Result<Rc<Sequence_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Sequence_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 148, RULE_sequence_pattern);
        let mut _localctx: Rc<Sequence_patternContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(944);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                OPEN_BRACK => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(934);
                        recog.base.match_token(OPEN_BRACK, &mut recog.err_handler)?;

                        recog.base.set_state(936);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << FALSE)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 38) & !0x3f) == 0
                                && ((1usize << (_la - 38))
                                    & ((1usize << (TRUE - 38))
                                        | (1usize << (UNDERSCORE - 38))
                                        | (1usize << (NAME - 38))
                                        | (1usize << (STAR - 38))
                                        | (1usize << (OPEN_PAREN - 38))
                                        | (1usize << (OPEN_BRACK - 38))))
                                    != 0)
                            || _la == MINUS
                            || _la == OPEN_BRACE
                        {
                            {
                                /*InvokeRule maybe_sequence_pattern*/
                                recog.base.set_state(935);
                                recog.maybe_sequence_pattern()?;
                            }
                        }

                        recog.base.set_state(938);
                        recog
                            .base
                            .match_token(CLOSE_BRACK, &mut recog.err_handler)?;
                    }
                }

                OPEN_PAREN => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(939);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        recog.base.set_state(941);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << FALSE)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 38) & !0x3f) == 0
                                && ((1usize << (_la - 38))
                                    & ((1usize << (TRUE - 38))
                                        | (1usize << (UNDERSCORE - 38))
                                        | (1usize << (NAME - 38))
                                        | (1usize << (STAR - 38))
                                        | (1usize << (OPEN_PAREN - 38))
                                        | (1usize << (OPEN_BRACK - 38))))
                                    != 0)
                            || _la == MINUS
                            || _la == OPEN_BRACE
                        {
                            {
                                /*InvokeRule open_sequence_pattern*/
                                recog.base.set_state(940);
                                recog.open_sequence_pattern()?;
                            }
                        }

                        recog.base.set_state(943);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- open_sequence_pattern ----------------
pub type Open_sequence_patternContextAll<'input> = Open_sequence_patternContext<'input>;

pub type Open_sequence_patternContext<'input> =
    BaseParserRuleContext<'input, Open_sequence_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Open_sequence_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Open_sequence_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Open_sequence_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_open_sequence_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_open_sequence_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Open_sequence_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_open_sequence_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Open_sequence_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_open_sequence_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_open_sequence_pattern }
}
antlr_rust::tid! {Open_sequence_patternContextExt<'a>}

impl<'input> Open_sequence_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Open_sequence_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Open_sequence_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Open_sequence_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Open_sequence_patternContextExt<'input>>
{
    fn maybe_star_pattern(&self) -> Option<Rc<Maybe_star_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMA
    /// Returns `None` if there is no child corresponding to token COMMA
    fn COMMA(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, 0)
    }
    fn maybe_sequence_pattern(&self) -> Option<Rc<Maybe_sequence_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Open_sequence_patternContextAttrs<'input> for Open_sequence_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn open_sequence_pattern(
        &mut self,
    ) -> Result<Rc<Open_sequence_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Open_sequence_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 150, RULE_open_sequence_pattern);
        let mut _localctx: Rc<Open_sequence_patternContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule maybe_star_pattern*/
                recog.base.set_state(946);
                recog.maybe_star_pattern()?;

                recog.base.set_state(947);
                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                recog.base.set_state(949);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << FALSE)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 38) & !0x3f) == 0
                        && ((1usize << (_la - 38))
                            & ((1usize << (TRUE - 38))
                                | (1usize << (UNDERSCORE - 38))
                                | (1usize << (NAME - 38))
                                | (1usize << (STAR - 38))
                                | (1usize << (OPEN_PAREN - 38))
                                | (1usize << (OPEN_BRACK - 38))))
                            != 0)
                    || _la == MINUS
                    || _la == OPEN_BRACE
                {
                    {
                        /*InvokeRule maybe_sequence_pattern*/
                        recog.base.set_state(948);
                        recog.maybe_sequence_pattern()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- maybe_sequence_pattern ----------------
pub type Maybe_sequence_patternContextAll<'input> = Maybe_sequence_patternContext<'input>;

pub type Maybe_sequence_patternContext<'input> =
    BaseParserRuleContext<'input, Maybe_sequence_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Maybe_sequence_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Maybe_sequence_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Maybe_sequence_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_maybe_sequence_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_maybe_sequence_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Maybe_sequence_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_maybe_sequence_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Maybe_sequence_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_maybe_sequence_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_maybe_sequence_pattern }
}
antlr_rust::tid! {Maybe_sequence_patternContextExt<'a>}

impl<'input> Maybe_sequence_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Maybe_sequence_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Maybe_sequence_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Maybe_sequence_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Maybe_sequence_patternContextExt<'input>>
{
    fn maybe_star_pattern_all(&self) -> Vec<Rc<Maybe_star_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn maybe_star_pattern(&self, i: usize) -> Option<Rc<Maybe_star_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Maybe_sequence_patternContextAttrs<'input> for Maybe_sequence_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn maybe_sequence_pattern(
        &mut self,
    ) -> Result<Rc<Maybe_sequence_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Maybe_sequence_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 152, RULE_maybe_sequence_pattern);
        let mut _localctx: Rc<Maybe_sequence_patternContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule maybe_star_pattern*/
                recog.base.set_state(951);
                recog.maybe_star_pattern()?;

                recog.base.set_state(956);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(120, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(952);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule maybe_star_pattern*/
                                recog.base.set_state(953);
                                recog.maybe_star_pattern()?;
                            }
                        }
                    }
                    recog.base.set_state(958);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(120, &mut recog.base)?;
                }
                recog.base.set_state(960);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(959);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- maybe_star_pattern ----------------
pub type Maybe_star_patternContextAll<'input> = Maybe_star_patternContext<'input>;

pub type Maybe_star_patternContext<'input> =
    BaseParserRuleContext<'input, Maybe_star_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Maybe_star_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Maybe_star_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Maybe_star_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_maybe_star_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_maybe_star_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Maybe_star_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_maybe_star_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Maybe_star_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_maybe_star_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_maybe_star_pattern }
}
antlr_rust::tid! {Maybe_star_patternContextExt<'a>}

impl<'input> Maybe_star_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Maybe_star_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Maybe_star_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Maybe_star_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Maybe_star_patternContextExt<'input>>
{
    fn star_pattern(&self) -> Option<Rc<Star_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Maybe_star_patternContextAttrs<'input> for Maybe_star_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn maybe_star_pattern(
        &mut self,
    ) -> Result<Rc<Maybe_star_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Maybe_star_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 154, RULE_maybe_star_pattern);
        let mut _localctx: Rc<Maybe_star_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(964);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STAR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule star_pattern*/
                        recog.base.set_state(962);
                        recog.star_pattern()?;
                    }
                }

                STRING | NUMBER | FALSE | MATCH | NONE | TRUE | UNDERSCORE | NAME | OPEN_PAREN
                | OPEN_BRACK | MINUS | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule pattern*/
                        recog.base.set_state(963);
                        recog.pattern()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- star_pattern ----------------
pub type Star_patternContextAll<'input> = Star_patternContext<'input>;

pub type Star_patternContext<'input> =
    BaseParserRuleContext<'input, Star_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Star_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Star_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Star_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_star_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_star_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Star_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_star_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Star_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_star_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_star_pattern }
}
antlr_rust::tid! {Star_patternContextExt<'a>}

impl<'input> Star_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Star_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Star_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Star_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Star_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    fn pattern_capture_target(&self) -> Option<Rc<Pattern_capture_targetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn wildcard_pattern(&self) -> Option<Rc<Wildcard_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Star_patternContextAttrs<'input> for Star_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn star_pattern(&mut self) -> Result<Rc<Star_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Star_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 156, RULE_star_pattern);
        let mut _localctx: Rc<Star_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(970);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(123, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(966);
                        recog.base.match_token(STAR, &mut recog.err_handler)?;

                        /*InvokeRule pattern_capture_target*/
                        recog.base.set_state(967);
                        recog.pattern_capture_target()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(968);
                        recog.base.match_token(STAR, &mut recog.err_handler)?;

                        /*InvokeRule wildcard_pattern*/
                        recog.base.set_state(969);
                        recog.wildcard_pattern()?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- mapping_pattern ----------------
pub type Mapping_patternContextAll<'input> = Mapping_patternContext<'input>;

pub type Mapping_patternContext<'input> =
    BaseParserRuleContext<'input, Mapping_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Mapping_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Mapping_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Mapping_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_mapping_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_mapping_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Mapping_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_mapping_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Mapping_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_mapping_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_mapping_pattern }
}
antlr_rust::tid! {Mapping_patternContextExt<'a>}

impl<'input> Mapping_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Mapping_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Mapping_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Mapping_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Mapping_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_BRACE
    /// Returns `None` if there is no child corresponding to token OPEN_BRACE
    fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_BRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
    /// Returns `None` if there is no child corresponding to token CLOSE_BRACE
    fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_BRACE, 0)
    }
    fn double_star_pattern(&self) -> Option<Rc<Double_star_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    fn items_pattern(&self) -> Option<Rc<Items_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Mapping_patternContextAttrs<'input> for Mapping_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn mapping_pattern(&mut self) -> Result<Rc<Mapping_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Mapping_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 158, RULE_mapping_pattern);
        let mut _localctx: Rc<Mapping_patternContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(997);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(127, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(972);
                        recog.base.match_token(OPEN_BRACE, &mut recog.err_handler)?;

                        recog.base.set_state(973);
                        recog
                            .base
                            .match_token(CLOSE_BRACE, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(974);
                        recog.base.match_token(OPEN_BRACE, &mut recog.err_handler)?;

                        /*InvokeRule double_star_pattern*/
                        recog.base.set_state(975);
                        recog.double_star_pattern()?;

                        recog.base.set_state(977);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(976);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(979);
                        recog
                            .base
                            .match_token(CLOSE_BRACE, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(981);
                        recog.base.match_token(OPEN_BRACE, &mut recog.err_handler)?;

                        /*InvokeRule items_pattern*/
                        recog.base.set_state(982);
                        recog.items_pattern()?;

                        recog.base.set_state(983);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                        /*InvokeRule double_star_pattern*/
                        recog.base.set_state(984);
                        recog.double_star_pattern()?;

                        recog.base.set_state(986);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(985);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(988);
                        recog
                            .base
                            .match_token(CLOSE_BRACE, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(990);
                        recog.base.match_token(OPEN_BRACE, &mut recog.err_handler)?;

                        /*InvokeRule items_pattern*/
                        recog.base.set_state(991);
                        recog.items_pattern()?;

                        recog.base.set_state(993);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(992);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(995);
                        recog
                            .base
                            .match_token(CLOSE_BRACE, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- items_pattern ----------------
pub type Items_patternContextAll<'input> = Items_patternContext<'input>;

pub type Items_patternContext<'input> =
    BaseParserRuleContext<'input, Items_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Items_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Items_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Items_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_items_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_items_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Items_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_items_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Items_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_items_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_items_pattern }
}
antlr_rust::tid! {Items_patternContextExt<'a>}

impl<'input> Items_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Items_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Items_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Items_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Items_patternContextExt<'input>>
{
    fn key_value_pattern_all(&self) -> Vec<Rc<Key_value_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn key_value_pattern(&self, i: usize) -> Option<Rc<Key_value_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Items_patternContextAttrs<'input> for Items_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn items_pattern(&mut self) -> Result<Rc<Items_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Items_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 160, RULE_items_pattern);
        let mut _localctx: Rc<Items_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule key_value_pattern*/
                recog.base.set_state(999);
                recog.key_value_pattern()?;

                recog.base.set_state(1004);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(128, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1000);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule key_value_pattern*/
                                recog.base.set_state(1001);
                                recog.key_value_pattern()?;
                            }
                        }
                    }
                    recog.base.set_state(1006);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(128, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- key_value_pattern ----------------
pub type Key_value_patternContextAll<'input> = Key_value_patternContext<'input>;

pub type Key_value_patternContext<'input> =
    BaseParserRuleContext<'input, Key_value_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Key_value_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Key_value_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Key_value_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_key_value_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_key_value_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Key_value_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_key_value_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Key_value_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_key_value_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_key_value_pattern }
}
antlr_rust::tid! {Key_value_patternContextExt<'a>}

impl<'input> Key_value_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Key_value_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Key_value_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Key_value_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Key_value_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn literal_expr(&self) -> Option<Rc<Literal_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn attr(&self) -> Option<Rc<AttrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Key_value_patternContextAttrs<'input> for Key_value_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn key_value_pattern(
        &mut self,
    ) -> Result<Rc<Key_value_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Key_value_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 162, RULE_key_value_pattern);
        let mut _localctx: Rc<Key_value_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1009);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    STRING | NUMBER | FALSE | NONE | TRUE | MINUS => {
                        {
                            /*InvokeRule literal_expr*/
                            recog.base.set_state(1007);
                            recog.literal_expr()?;
                        }
                    }

                    MATCH | UNDERSCORE | NAME => {
                        {
                            /*InvokeRule attr*/
                            recog.base.set_state(1008);
                            recog.attr()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(1011);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule pattern*/
                recog.base.set_state(1012);
                recog.pattern()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- double_star_pattern ----------------
pub type Double_star_patternContextAll<'input> = Double_star_patternContext<'input>;

pub type Double_star_patternContext<'input> =
    BaseParserRuleContext<'input, Double_star_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Double_star_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Double_star_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Double_star_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_double_star_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_double_star_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Double_star_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_double_star_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Double_star_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_double_star_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_double_star_pattern }
}
antlr_rust::tid! {Double_star_patternContextExt<'a>}

impl<'input> Double_star_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Double_star_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Double_star_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Double_star_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Double_star_patternContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token POWER
    /// Returns `None` if there is no child corresponding to token POWER
    fn POWER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER, 0)
    }
    fn pattern_capture_target(&self) -> Option<Rc<Pattern_capture_targetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Double_star_patternContextAttrs<'input> for Double_star_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn double_star_pattern(
        &mut self,
    ) -> Result<Rc<Double_star_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Double_star_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 164, RULE_double_star_pattern);
        let mut _localctx: Rc<Double_star_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1014);
                recog.base.match_token(POWER, &mut recog.err_handler)?;

                /*InvokeRule pattern_capture_target*/
                recog.base.set_state(1015);
                recog.pattern_capture_target()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- class_pattern ----------------
pub type Class_patternContextAll<'input> = Class_patternContext<'input>;

pub type Class_patternContext<'input> =
    BaseParserRuleContext<'input, Class_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Class_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Class_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Class_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_class_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_class_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Class_patternContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_class_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Class_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_class_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_class_pattern }
}
antlr_rust::tid! {Class_patternContextExt<'a>}

impl<'input> Class_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Class_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Class_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Class_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Class_patternContextExt<'input>>
{
    fn name_or_attr(&self) -> Option<Rc<Name_or_attrContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn positional_patterns(&self) -> Option<Rc<Positional_patternsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
    fn keyword_patterns(&self) -> Option<Rc<Keyword_patternsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Class_patternContextAttrs<'input> for Class_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn class_pattern(&mut self) -> Result<Rc<Class_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Class_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 166, RULE_class_pattern);
        let mut _localctx: Rc<Class_patternContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1047);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(133, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule name_or_attr*/
                        recog.base.set_state(1017);
                        recog.name_or_attr()?;

                        recog.base.set_state(1018);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        recog.base.set_state(1019);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule name_or_attr*/
                        recog.base.set_state(1021);
                        recog.name_or_attr()?;

                        recog.base.set_state(1022);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        /*InvokeRule positional_patterns*/
                        recog.base.set_state(1023);
                        recog.positional_patterns()?;

                        recog.base.set_state(1025);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(1024);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1027);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule name_or_attr*/
                        recog.base.set_state(1029);
                        recog.name_or_attr()?;

                        recog.base.set_state(1030);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        /*InvokeRule keyword_patterns*/
                        recog.base.set_state(1031);
                        recog.keyword_patterns()?;

                        recog.base.set_state(1033);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(1032);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1035);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule name_or_attr*/
                        recog.base.set_state(1037);
                        recog.name_or_attr()?;

                        recog.base.set_state(1038);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        /*InvokeRule positional_patterns*/
                        recog.base.set_state(1039);
                        recog.positional_patterns()?;

                        recog.base.set_state(1040);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;

                        /*InvokeRule keyword_patterns*/
                        recog.base.set_state(1041);
                        recog.keyword_patterns()?;

                        recog.base.set_state(1043);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COMMA {
                            {
                                recog.base.set_state(1042);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1045);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- positional_patterns ----------------
pub type Positional_patternsContextAll<'input> = Positional_patternsContext<'input>;

pub type Positional_patternsContext<'input> =
    BaseParserRuleContext<'input, Positional_patternsContextExt<'input>>;

#[derive(Clone)]
pub struct Positional_patternsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Positional_patternsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Positional_patternsContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_positional_patterns(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_positional_patterns(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Positional_patternsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_positional_patterns(self);
    }
}

impl<'input> CustomRuleContext<'input> for Positional_patternsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_positional_patterns
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_positional_patterns }
}
antlr_rust::tid! {Positional_patternsContextExt<'a>}

impl<'input> Positional_patternsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Positional_patternsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Positional_patternsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Positional_patternsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Positional_patternsContextExt<'input>>
{
    fn pattern_all(&self) -> Vec<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Positional_patternsContextAttrs<'input> for Positional_patternsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn positional_patterns(
        &mut self,
    ) -> Result<Rc<Positional_patternsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Positional_patternsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 168, RULE_positional_patterns);
        let mut _localctx: Rc<Positional_patternsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule pattern*/
                recog.base.set_state(1049);
                recog.pattern()?;

                recog.base.set_state(1054);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(134, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1050);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule pattern*/
                                recog.base.set_state(1051);
                                recog.pattern()?;
                            }
                        }
                    }
                    recog.base.set_state(1056);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(134, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- keyword_patterns ----------------
pub type Keyword_patternsContextAll<'input> = Keyword_patternsContext<'input>;

pub type Keyword_patternsContext<'input> =
    BaseParserRuleContext<'input, Keyword_patternsContextExt<'input>>;

#[derive(Clone)]
pub struct Keyword_patternsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Keyword_patternsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Keyword_patternsContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_keyword_patterns(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_keyword_patterns(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Keyword_patternsContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_keyword_patterns(self);
    }
}

impl<'input> CustomRuleContext<'input> for Keyword_patternsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_keyword_patterns
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_keyword_patterns }
}
antlr_rust::tid! {Keyword_patternsContextExt<'a>}

impl<'input> Keyword_patternsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Keyword_patternsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Keyword_patternsContextExt { ph: PhantomData },
        ))
    }
}

pub trait Keyword_patternsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Keyword_patternsContextExt<'input>>
{
    fn keyword_pattern_all(&self) -> Vec<Rc<Keyword_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn keyword_pattern(&self, i: usize) -> Option<Rc<Keyword_patternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Keyword_patternsContextAttrs<'input> for Keyword_patternsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn keyword_patterns(
        &mut self,
    ) -> Result<Rc<Keyword_patternsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Keyword_patternsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 170, RULE_keyword_patterns);
        let mut _localctx: Rc<Keyword_patternsContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule keyword_pattern*/
                recog.base.set_state(1057);
                recog.keyword_pattern()?;

                recog.base.set_state(1062);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(135, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1058);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule keyword_pattern*/
                                recog.base.set_state(1059);
                                recog.keyword_pattern()?;
                            }
                        }
                    }
                    recog.base.set_state(1064);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(135, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- keyword_pattern ----------------
pub type Keyword_patternContextAll<'input> = Keyword_patternContext<'input>;

pub type Keyword_patternContext<'input> =
    BaseParserRuleContext<'input, Keyword_patternContextExt<'input>>;

#[derive(Clone)]
pub struct Keyword_patternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Keyword_patternContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Keyword_patternContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_keyword_pattern(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_keyword_pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Keyword_patternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_keyword_pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for Keyword_patternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_keyword_pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_keyword_pattern }
}
antlr_rust::tid! {Keyword_patternContextExt<'a>}

impl<'input> Keyword_patternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Keyword_patternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Keyword_patternContextExt { ph: PhantomData },
        ))
    }
}

pub trait Keyword_patternContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Keyword_patternContextExt<'input>>
{
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Keyword_patternContextAttrs<'input> for Keyword_patternContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn keyword_pattern(&mut self) -> Result<Rc<Keyword_patternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Keyword_patternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 172, RULE_keyword_pattern);
        let mut _localctx: Rc<Keyword_patternContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(1065);
                recog.name()?;

                recog.base.set_state(1066);
                recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                /*InvokeRule pattern*/
                recog.base.set_state(1067);
                recog.pattern()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- test ----------------
pub type TestContextAll<'input> = TestContext<'input>;

pub type TestContext<'input> = BaseParserRuleContext<'input, TestContextExt<'input>>;

#[derive(Clone)]
pub struct TestContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for TestContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for TestContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_test(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_test(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for TestContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_test(self);
    }
}

impl<'input> CustomRuleContext<'input> for TestContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_test
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_test }
}
antlr_rust::tid! {TestContextExt<'a>}

impl<'input> TestContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TestContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TestContextExt { ph: PhantomData },
        ))
    }
}

pub trait TestContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<TestContextExt<'input>>
{
    fn or_test_all(&self) -> Vec<Rc<Or_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn or_test(&self, i: usize) -> Option<Rc<Or_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn lambdef(&self) -> Option<Rc<LambdefContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TestContextAttrs<'input> for TestContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn test(&mut self) -> Result<Rc<TestContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_test);
        let mut _localctx: Rc<TestContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1078);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STRING | NUMBER | AWAIT | FALSE | MATCH | NONE | NOT | TRUE | UNDERSCORE | NAME
                | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule or_test*/
                        recog.base.set_state(1069);
                        recog.or_test()?;

                        recog.base.set_state(1075);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == IF {
                            {
                                recog.base.set_state(1070);
                                recog.base.match_token(IF, &mut recog.err_handler)?;

                                /*InvokeRule or_test*/
                                recog.base.set_state(1071);
                                recog.or_test()?;

                                recog.base.set_state(1072);
                                recog.base.match_token(ELSE, &mut recog.err_handler)?;

                                /*InvokeRule test*/
                                recog.base.set_state(1073);
                                recog.test()?;
                            }
                        }
                    }
                }

                LAMBDA => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule lambdef*/
                        recog.base.set_state(1077);
                        recog.lambdef()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- test_nocond ----------------
pub type Test_nocondContextAll<'input> = Test_nocondContext<'input>;

pub type Test_nocondContext<'input> = BaseParserRuleContext<'input, Test_nocondContextExt<'input>>;

#[derive(Clone)]
pub struct Test_nocondContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Test_nocondContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Test_nocondContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_test_nocond(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_test_nocond(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Test_nocondContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_test_nocond(self);
    }
}

impl<'input> CustomRuleContext<'input> for Test_nocondContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_test_nocond
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_test_nocond }
}
antlr_rust::tid! {Test_nocondContextExt<'a>}

impl<'input> Test_nocondContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Test_nocondContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Test_nocondContextExt { ph: PhantomData },
        ))
    }
}

pub trait Test_nocondContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Test_nocondContextExt<'input>>
{
    fn or_test(&self) -> Option<Rc<Or_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn lambdef_nocond(&self) -> Option<Rc<Lambdef_nocondContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Test_nocondContextAttrs<'input> for Test_nocondContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn test_nocond(&mut self) -> Result<Rc<Test_nocondContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Test_nocondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 176, RULE_test_nocond);
        let mut _localctx: Rc<Test_nocondContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1082);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                STRING | NUMBER | AWAIT | FALSE | MATCH | NONE | NOT | TRUE | UNDERSCORE | NAME
                | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule or_test*/
                        recog.base.set_state(1080);
                        recog.or_test()?;
                    }
                }

                LAMBDA => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule lambdef_nocond*/
                        recog.base.set_state(1081);
                        recog.lambdef_nocond()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- lambdef ----------------
pub type LambdefContextAll<'input> = LambdefContext<'input>;

pub type LambdefContext<'input> = BaseParserRuleContext<'input, LambdefContextExt<'input>>;

#[derive(Clone)]
pub struct LambdefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for LambdefContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for LambdefContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_lambdef(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_lambdef(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for LambdefContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_lambdef(self);
    }
}

impl<'input> CustomRuleContext<'input> for LambdefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_lambdef
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_lambdef }
}
antlr_rust::tid! {LambdefContextExt<'a>}

impl<'input> LambdefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<LambdefContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            LambdefContextExt { ph: PhantomData },
        ))
    }
}

pub trait LambdefContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<LambdefContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LAMBDA
    /// Returns `None` if there is no child corresponding to token LAMBDA
    fn LAMBDA(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LAMBDA, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn varargslist(&self) -> Option<Rc<VarargslistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> LambdefContextAttrs<'input> for LambdefContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn lambdef(&mut self) -> Result<Rc<LambdefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = LambdefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_lambdef);
        let mut _localctx: Rc<LambdefContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1084);
                recog.base.match_token(LAMBDA, &mut recog.err_handler)?;

                recog.base.set_state(1086);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == MATCH
                    || (((_la - 40) & !0x3f) == 0
                        && ((1usize << (_la - 40))
                            & ((1usize << (UNDERSCORE - 40))
                                | (1usize << (NAME - 40))
                                | (1usize << (STAR - 40))
                                | (1usize << (POWER - 40))))
                            != 0)
                {
                    {
                        /*InvokeRule varargslist*/
                        recog.base.set_state(1085);
                        recog.varargslist()?;
                    }
                }

                recog.base.set_state(1088);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule test*/
                recog.base.set_state(1089);
                recog.test()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- lambdef_nocond ----------------
pub type Lambdef_nocondContextAll<'input> = Lambdef_nocondContext<'input>;

pub type Lambdef_nocondContext<'input> =
    BaseParserRuleContext<'input, Lambdef_nocondContextExt<'input>>;

#[derive(Clone)]
pub struct Lambdef_nocondContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Lambdef_nocondContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Lambdef_nocondContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_lambdef_nocond(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_lambdef_nocond(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for Lambdef_nocondContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_lambdef_nocond(self);
    }
}

impl<'input> CustomRuleContext<'input> for Lambdef_nocondContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_lambdef_nocond
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_lambdef_nocond }
}
antlr_rust::tid! {Lambdef_nocondContextExt<'a>}

impl<'input> Lambdef_nocondContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Lambdef_nocondContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Lambdef_nocondContextExt { ph: PhantomData },
        ))
    }
}

pub trait Lambdef_nocondContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Lambdef_nocondContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LAMBDA
    /// Returns `None` if there is no child corresponding to token LAMBDA
    fn LAMBDA(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LAMBDA, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn test_nocond(&self) -> Option<Rc<Test_nocondContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn varargslist(&self) -> Option<Rc<VarargslistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Lambdef_nocondContextAttrs<'input> for Lambdef_nocondContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn lambdef_nocond(&mut self) -> Result<Rc<Lambdef_nocondContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Lambdef_nocondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 180, RULE_lambdef_nocond);
        let mut _localctx: Rc<Lambdef_nocondContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1091);
                recog.base.match_token(LAMBDA, &mut recog.err_handler)?;

                recog.base.set_state(1093);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == MATCH
                    || (((_la - 40) & !0x3f) == 0
                        && ((1usize << (_la - 40))
                            & ((1usize << (UNDERSCORE - 40))
                                | (1usize << (NAME - 40))
                                | (1usize << (STAR - 40))
                                | (1usize << (POWER - 40))))
                            != 0)
                {
                    {
                        /*InvokeRule varargslist*/
                        recog.base.set_state(1092);
                        recog.varargslist()?;
                    }
                }

                recog.base.set_state(1095);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule test_nocond*/
                recog.base.set_state(1096);
                recog.test_nocond()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- or_test ----------------
pub type Or_testContextAll<'input> = Or_testContext<'input>;

pub type Or_testContext<'input> = BaseParserRuleContext<'input, Or_testContextExt<'input>>;

#[derive(Clone)]
pub struct Or_testContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Or_testContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Or_testContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_or_test(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_or_test(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Or_testContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_or_test(self);
    }
}

impl<'input> CustomRuleContext<'input> for Or_testContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_or_test
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_or_test }
}
antlr_rust::tid! {Or_testContextExt<'a>}

impl<'input> Or_testContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Or_testContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Or_testContextExt { ph: PhantomData },
        ))
    }
}

pub trait Or_testContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Or_testContextExt<'input>>
{
    fn and_test_all(&self) -> Vec<Rc<And_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn and_test(&self, i: usize) -> Option<Rc<And_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
    fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR, i)
    }
}

impl<'input> Or_testContextAttrs<'input> for Or_testContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn or_test(&mut self) -> Result<Rc<Or_testContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Or_testContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_or_test);
        let mut _localctx: Rc<Or_testContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule and_test*/
                recog.base.set_state(1098);
                recog.and_test()?;

                recog.base.set_state(1103);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == OR {
                    {
                        {
                            recog.base.set_state(1099);
                            recog.base.match_token(OR, &mut recog.err_handler)?;

                            /*InvokeRule and_test*/
                            recog.base.set_state(1100);
                            recog.and_test()?;
                        }
                    }
                    recog.base.set_state(1105);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- and_test ----------------
pub type And_testContextAll<'input> = And_testContext<'input>;

pub type And_testContext<'input> = BaseParserRuleContext<'input, And_testContextExt<'input>>;

#[derive(Clone)]
pub struct And_testContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for And_testContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for And_testContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_and_test(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_and_test(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for And_testContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_and_test(self);
    }
}

impl<'input> CustomRuleContext<'input> for And_testContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_and_test
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_and_test }
}
antlr_rust::tid! {And_testContextExt<'a>}

impl<'input> And_testContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<And_testContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            And_testContextExt { ph: PhantomData },
        ))
    }
}

pub trait And_testContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<And_testContextExt<'input>>
{
    fn not_test_all(&self) -> Vec<Rc<Not_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn not_test(&self, i: usize) -> Option<Rc<Not_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token AND in current rule
    fn AND_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
    /// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
    fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AND, i)
    }
}

impl<'input> And_testContextAttrs<'input> for And_testContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn and_test(&mut self) -> Result<Rc<And_testContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = And_testContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_and_test);
        let mut _localctx: Rc<And_testContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule not_test*/
                recog.base.set_state(1106);
                recog.not_test()?;

                recog.base.set_state(1111);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == AND {
                    {
                        {
                            recog.base.set_state(1107);
                            recog.base.match_token(AND, &mut recog.err_handler)?;

                            /*InvokeRule not_test*/
                            recog.base.set_state(1108);
                            recog.not_test()?;
                        }
                    }
                    recog.base.set_state(1113);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- not_test ----------------
pub type Not_testContextAll<'input> = Not_testContext<'input>;

pub type Not_testContext<'input> = BaseParserRuleContext<'input, Not_testContextExt<'input>>;

#[derive(Clone)]
pub struct Not_testContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Not_testContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Not_testContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_not_test(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_not_test(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Not_testContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_not_test(self);
    }
}

impl<'input> CustomRuleContext<'input> for Not_testContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_not_test
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_not_test }
}
antlr_rust::tid! {Not_testContextExt<'a>}

impl<'input> Not_testContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Not_testContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Not_testContextExt { ph: PhantomData },
        ))
    }
}

pub trait Not_testContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Not_testContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT, 0)
    }
    fn not_test(&self) -> Option<Rc<Not_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn comparison(&self) -> Option<Rc<ComparisonContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Not_testContextAttrs<'input> for Not_testContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn not_test(&mut self) -> Result<Rc<Not_testContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Not_testContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_not_test);
        let mut _localctx: Rc<Not_testContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1117);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                NOT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1114);
                        recog.base.match_token(NOT, &mut recog.err_handler)?;

                        /*InvokeRule not_test*/
                        recog.base.set_state(1115);
                        recog.not_test()?;
                    }
                }

                STRING | NUMBER | AWAIT | FALSE | MATCH | NONE | TRUE | UNDERSCORE | NAME
                | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule comparison*/
                        recog.base.set_state(1116);
                        recog.comparison()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- comparison ----------------
pub type ComparisonContextAll<'input> = ComparisonContext<'input>;

pub type ComparisonContext<'input> = BaseParserRuleContext<'input, ComparisonContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ComparisonContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ComparisonContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_comparison(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_comparison(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ComparisonContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_comparison(self);
    }
}

impl<'input> CustomRuleContext<'input> for ComparisonContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_comparison
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_comparison }
}
antlr_rust::tid! {ComparisonContextExt<'a>}

impl<'input> ComparisonContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ComparisonContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ComparisonContextExt { ph: PhantomData },
        ))
    }
}

pub trait ComparisonContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ComparisonContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn comp_op_all(&self) -> Vec<Rc<Comp_opContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn comp_op(&self, i: usize) -> Option<Rc<Comp_opContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> ComparisonContextAttrs<'input> for ComparisonContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn comparison(&mut self) -> Result<Rc<ComparisonContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ComparisonContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 188, RULE_comparison);
        let mut _localctx: Rc<ComparisonContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expr*/
                recog.base.set_state(1119);
                recog.expr_rec(0)?;

                recog.base.set_state(1125);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(144, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule comp_op*/
                                recog.base.set_state(1120);
                                recog.comp_op()?;

                                /*InvokeRule expr*/
                                recog.base.set_state(1121);
                                recog.expr_rec(0)?;
                            }
                        }
                    }
                    recog.base.set_state(1127);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(144, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- comp_op ----------------
pub type Comp_opContextAll<'input> = Comp_opContext<'input>;

pub type Comp_opContext<'input> = BaseParserRuleContext<'input, Comp_opContextExt<'input>>;

#[derive(Clone)]
pub struct Comp_opContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Comp_opContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Comp_opContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_comp_op(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_comp_op(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Comp_opContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_comp_op(self);
    }
}

impl<'input> CustomRuleContext<'input> for Comp_opContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_comp_op
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_comp_op }
}
antlr_rust::tid! {Comp_opContextExt<'a>}

impl<'input> Comp_opContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Comp_opContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Comp_opContextExt { ph: PhantomData },
        ))
    }
}

pub trait Comp_opContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Comp_opContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LESS_THAN
    /// Returns `None` if there is no child corresponding to token LESS_THAN
    fn LESS_THAN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LESS_THAN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GREATER_THAN
    /// Returns `None` if there is no child corresponding to token GREATER_THAN
    fn GREATER_THAN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GREATER_THAN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EQUALS
    /// Returns `None` if there is no child corresponding to token EQUALS
    fn EQUALS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token GT_EQ
    /// Returns `None` if there is no child corresponding to token GT_EQ
    fn GT_EQ(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(GT_EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LT_EQ
    /// Returns `None` if there is no child corresponding to token LT_EQ
    fn LT_EQ(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LT_EQ, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT_EQ_1
    /// Returns `None` if there is no child corresponding to token NOT_EQ_1
    fn NOT_EQ_1(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT_EQ_1, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT_EQ_2
    /// Returns `None` if there is no child corresponding to token NOT_EQ_2
    fn NOT_EQ_2(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT_EQ_2, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IS
    /// Returns `None` if there is no child corresponding to token IS
    fn IS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IS, 0)
    }
}

impl<'input> Comp_opContextAttrs<'input> for Comp_opContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn comp_op(&mut self) -> Result<Rc<Comp_opContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Comp_opContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_comp_op);
        let mut _localctx: Rc<Comp_opContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1141);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(145, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1128);
                        recog.base.match_token(LESS_THAN, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1129);
                        recog
                            .base
                            .match_token(GREATER_THAN, &mut recog.err_handler)?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1130);
                        recog.base.match_token(EQUALS, &mut recog.err_handler)?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(1131);
                        recog.base.match_token(GT_EQ, &mut recog.err_handler)?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(1132);
                        recog.base.match_token(LT_EQ, &mut recog.err_handler)?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(1133);
                        recog.base.match_token(NOT_EQ_1, &mut recog.err_handler)?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(1134);
                        recog.base.match_token(NOT_EQ_2, &mut recog.err_handler)?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        recog.base.set_state(1135);
                        recog.base.match_token(IN, &mut recog.err_handler)?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        recog.base.set_state(1136);
                        recog.base.match_token(NOT, &mut recog.err_handler)?;

                        recog.base.set_state(1137);
                        recog.base.match_token(IN, &mut recog.err_handler)?;
                    }
                }
                10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        recog.base.set_state(1138);
                        recog.base.match_token(IS, &mut recog.err_handler)?;
                    }
                }
                11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 11);
                    recog.base.enter_outer_alt(None, 11);
                    {
                        recog.base.set_state(1139);
                        recog.base.match_token(IS, &mut recog.err_handler)?;

                        recog.base.set_state(1140);
                        recog.base.match_token(NOT, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- star_expr ----------------
pub type Star_exprContextAll<'input> = Star_exprContext<'input>;

pub type Star_exprContext<'input> = BaseParserRuleContext<'input, Star_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Star_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Star_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Star_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_star_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_star_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Star_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_star_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Star_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_star_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_star_expr }
}
antlr_rust::tid! {Star_exprContextExt<'a>}

impl<'input> Star_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Star_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Star_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Star_exprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Star_exprContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Star_exprContextAttrs<'input> for Star_exprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn star_expr(&mut self) -> Result<Rc<Star_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Star_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 192, RULE_star_expr);
        let mut _localctx: Rc<Star_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1143);
                recog.base.match_token(STAR, &mut recog.err_handler)?;

                /*InvokeRule expr*/
                recog.base.set_state(1144);
                recog.expr_rec(0)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;

pub type ExprContext<'input> = BaseParserRuleContext<'input, ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ExprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ExprContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ExprContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid! {ExprContextExt<'a>}

impl<'input> ExprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExprContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ExprContextExt<'input>>
{
    fn atom_expr(&self) -> Option<Rc<Atom_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ADD in current rule
    fn ADD_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token ADD, starting from 0.
    /// Returns `None` if number of children corresponding to token ADD is less or equal than `i`.
    fn ADD(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token MINUS in current rule
    fn MINUS_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token MINUS, starting from 0.
    /// Returns `None` if number of children corresponding to token MINUS is less or equal than `i`.
    fn MINUS(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MINUS, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token NOT_OP in current rule
    fn NOT_OP_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token NOT_OP, starting from 0.
    /// Returns `None` if number of children corresponding to token NOT_OP is less or equal than `i`.
    fn NOT_OP(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT_OP, i)
    }
    /// Retrieves first TerminalNode corresponding to token POWER
    /// Returns `None` if there is no child corresponding to token POWER
    fn POWER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DIV
    /// Returns `None` if there is no child corresponding to token DIV
    fn DIV(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MOD
    /// Returns `None` if there is no child corresponding to token MOD
    fn MOD(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MOD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IDIV
    /// Returns `None` if there is no child corresponding to token IDIV
    fn IDIV(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IDIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LEFT_SHIFT
    /// Returns `None` if there is no child corresponding to token LEFT_SHIFT
    fn LEFT_SHIFT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LEFT_SHIFT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RIGHT_SHIFT
    /// Returns `None` if there is no child corresponding to token RIGHT_SHIFT
    fn RIGHT_SHIFT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RIGHT_SHIFT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AND_OP
    /// Returns `None` if there is no child corresponding to token AND_OP
    fn AND_OP(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AND_OP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XOR
    /// Returns `None` if there is no child corresponding to token XOR
    fn XOR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(XOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OR_OP
    /// Returns `None` if there is no child corresponding to token OR_OP
    fn OR_OP(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR_OP, 0)
    }
}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expr(&mut self) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        self.expr_rec(0)
    }

    fn expr_rec(&mut self, _p: isize) -> Result<Rc<ExprContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 194, RULE_expr, _p);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 194;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1154);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    STRING | NUMBER | AWAIT | FALSE | MATCH | NONE | TRUE | UNDERSCORE | NAME
                    | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | OPEN_BRACE => {
                        {
                            /*InvokeRule atom_expr*/
                            recog.base.set_state(1147);
                            recog.atom_expr()?;
                        }
                    }

                    ADD | MINUS | NOT_OP => {
                        {
                            recog.base.set_state(1149);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = 1;
                            loop {
                                match _alt {
                                    x if x == 1 => {
                                        recog.base.set_state(1148);
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 71) & !0x3f) == 0
                                                && ((1usize << (_la - 71))
                                                    & ((1usize << (ADD - 71))
                                                        | (1usize << (MINUS - 71))
                                                        | (1usize << (NOT_OP - 71))))
                                                    != 0)
                                        } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                                recog.base.set_state(1151);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(146, &mut recog.base)?;
                                if _alt == 2 || _alt == INVALID_ALT {
                                    break;
                                }
                            }
                            /*InvokeRule expr*/
                            recog.base.set_state(1153);
                            recog.expr_rec(7)?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(1179);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(149, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(1177);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(148, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1156);
                                        if !({ recog.precpred(None, 8) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 8)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1157);
                                        recog.base.match_token(POWER, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1158);
                                        recog.expr_rec(9)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1159);
                                        if !({ recog.precpred(None, 6) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 6)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1160);
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 56) & !0x3f) == 0
                                                && ((1usize << (_la - 56))
                                                    & ((1usize << (STAR - 56))
                                                        | (1usize << (DIV - 56))
                                                        | (1usize << (MOD - 56))
                                                        | (1usize << (IDIV - 56))
                                                        | (1usize << (AT - 56))))
                                                    != 0)
                                        } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1161);
                                        recog.expr_rec(7)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1162);
                                        if !({ recog.precpred(None, 5) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 5)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1163);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == ADD || _la == MINUS) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1164);
                                        recog.expr_rec(6)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1165);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1166);
                                        _la = recog.base.input.la(1);
                                        if { !(_la == LEFT_SHIFT || _la == RIGHT_SHIFT) } {
                                            recog.err_handler.recover_inline(&mut recog.base)?;
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expr*/
                                        recog.base.set_state(1167);
                                        recog.expr_rec(5)?;
                                    }
                                }
                                5 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1168);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1169);
                                        recog.base.match_token(AND_OP, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1170);
                                        recog.expr_rec(4)?;
                                    }
                                }
                                6 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1171);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1172);
                                        recog.base.match_token(XOR, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1173);
                                        recog.expr_rec(3)?;
                                    }
                                }
                                7 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp =
                                            ExprContextExt::new(_parentctx.clone(), _parentState);
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(1174);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(1175);
                                        recog.base.match_token(OR_OP, &mut recog.err_handler)?;

                                        /*InvokeRule expr*/
                                        recog.base.set_state(1176);
                                        recog.expr_rec(2)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(1181);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(149, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- atom_expr ----------------
pub type Atom_exprContextAll<'input> = Atom_exprContext<'input>;

pub type Atom_exprContext<'input> = BaseParserRuleContext<'input, Atom_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Atom_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Atom_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Atom_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_atom_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_atom_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Atom_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_atom_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Atom_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atom_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_atom_expr }
}
antlr_rust::tid! {Atom_exprContextExt<'a>}

impl<'input> Atom_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Atom_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Atom_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Atom_exprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Atom_exprContextExt<'input>>
{
    fn atom(&self) -> Option<Rc<AtomContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token AWAIT
    /// Returns `None` if there is no child corresponding to token AWAIT
    fn AWAIT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AWAIT, 0)
    }
    fn trailer_all(&self) -> Vec<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn trailer(&self, i: usize) -> Option<Rc<TrailerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Atom_exprContextAttrs<'input> for Atom_exprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn atom_expr(&mut self) -> Result<Rc<Atom_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Atom_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 196, RULE_atom_expr);
        let mut _localctx: Rc<Atom_exprContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1183);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == AWAIT {
                    {
                        recog.base.set_state(1182);
                        recog.base.match_token(AWAIT, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule atom*/
                recog.base.set_state(1185);
                recog.atom()?;

                recog.base.set_state(1189);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(151, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule trailer*/
                                recog.base.set_state(1186);
                                recog.trailer()?;
                            }
                        }
                    }
                    recog.base.set_state(1191);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(151, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;

pub type AtomContext<'input> = BaseParserRuleContext<'input, AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for AtomContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for AtomContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_atom(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_atom(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for AtomContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_atom(self);
    }
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_atom
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr_rust::tid! {AtomContextExt<'a>}

impl<'input> AtomContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<AtomContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            AtomContextExt { ph: PhantomData },
        ))
    }
}

pub trait AtomContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<AtomContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn yield_expr(&self) -> Option<Rc<Yield_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn testlist_comp(&self) -> Option<Rc<Testlist_compContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_BRACK
    /// Returns `None` if there is no child corresponding to token OPEN_BRACK
    fn OPEN_BRACK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_BRACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_BRACK
    /// Returns `None` if there is no child corresponding to token CLOSE_BRACK
    fn CLOSE_BRACK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_BRACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_BRACE
    /// Returns `None` if there is no child corresponding to token OPEN_BRACE
    fn OPEN_BRACE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_BRACE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_BRACE
    /// Returns `None` if there is no child corresponding to token CLOSE_BRACE
    fn CLOSE_BRACE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_BRACE, 0)
    }
    fn dictorsetmaker(&self) -> Option<Rc<DictorsetmakerContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
    fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
    /// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
    fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, i)
    }
    /// Retrieves first TerminalNode corresponding to token ELLIPSIS
    /// Returns `None` if there is no child corresponding to token ELLIPSIS
    fn ELLIPSIS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELLIPSIS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NONE
    /// Returns `None` if there is no child corresponding to token NONE
    fn NONE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn atom(&mut self) -> Result<Rc<AtomContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            recog.base.set_state(1219);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                OPEN_PAREN => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1192);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        recog.base.set_state(1195);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            YIELD => {
                                {
                                    /*InvokeRule yield_expr*/
                                    recog.base.set_state(1193);
                                    recog.yield_expr()?;
                                }
                            }

                            STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT
                            | TRUE | UNDERSCORE | NAME | ELLIPSIS | STAR | OPEN_PAREN
                            | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                                {
                                    /*InvokeRule testlist_comp*/
                                    recog.base.set_state(1194);
                                    recog.testlist_comp()?;
                                }
                            }

                            CLOSE_PAREN => {}

                            _ => {}
                        }
                        recog.base.set_state(1197);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }

                OPEN_BRACK => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1198);
                        recog.base.match_token(OPEN_BRACK, &mut recog.err_handler)?;

                        recog.base.set_state(1200);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (STAR - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule testlist_comp*/
                                recog.base.set_state(1199);
                                recog.testlist_comp()?;
                            }
                        }

                        recog.base.set_state(1202);
                        recog
                            .base
                            .match_token(CLOSE_BRACK, &mut recog.err_handler)?;
                    }
                }

                OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1203);
                        recog.base.match_token(OPEN_BRACE, &mut recog.err_handler)?;

                        recog.base.set_state(1205);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (STAR - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (POWER - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule dictorsetmaker*/
                                recog.base.set_state(1204);
                                recog.dictorsetmaker()?;
                            }
                        }

                        recog.base.set_state(1207);
                        recog
                            .base
                            .match_token(CLOSE_BRACE, &mut recog.err_handler)?;
                    }
                }

                MATCH | UNDERSCORE | NAME => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule name*/
                        recog.base.set_state(1208);
                        recog.name()?;
                    }
                }

                NUMBER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        recog.base.set_state(1209);
                        recog.base.match_token(NUMBER, &mut recog.err_handler)?;
                    }
                }

                STRING => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        recog.base.set_state(1211);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = 1;
                        loop {
                            match _alt {
                                x if x == 1 => {
                                    recog.base.set_state(1210);
                                    recog.base.match_token(STRING, &mut recog.err_handler)?;
                                }

                                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                    &mut recog.base,
                                )))?,
                            }
                            recog.base.set_state(1213);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(155, &mut recog.base)?;
                            if _alt == 2 || _alt == INVALID_ALT {
                                break;
                            }
                        }
                    }
                }

                ELLIPSIS => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        recog.base.set_state(1215);
                        recog.base.match_token(ELLIPSIS, &mut recog.err_handler)?;
                    }
                }

                NONE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        recog.base.set_state(1216);
                        recog.base.match_token(NONE, &mut recog.err_handler)?;
                    }
                }

                TRUE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        recog.base.set_state(1217);
                        recog.base.match_token(TRUE, &mut recog.err_handler)?;
                    }
                }

                FALSE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        recog.base.set_state(1218);
                        recog.base.match_token(FALSE, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- name ----------------
pub type NameContextAll<'input> = NameContext<'input>;

pub type NameContext<'input> = BaseParserRuleContext<'input, NameContextExt<'input>>;

#[derive(Clone)]
pub struct NameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for NameContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for NameContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_name(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_name(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for NameContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_name(self);
    }
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_name
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr_rust::tid! {NameContextExt<'a>}

impl<'input> NameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NameContextExt { ph: PhantomData },
        ))
    }
}

pub trait NameContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<NameContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token NAME
    /// Returns `None` if there is no child corresponding to token NAME
    fn NAME(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NAME, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UNDERSCORE
    /// Returns `None` if there is no child corresponding to token UNDERSCORE
    fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNDERSCORE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MATCH
    /// Returns `None` if there is no child corresponding to token MATCH
    fn MATCH(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH, 0)
    }
}

impl<'input> NameContextAttrs<'input> for NameContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn name(&mut self) -> Result<Rc<NameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1221);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 30) & !0x3f) == 0
                        && ((1usize << (_la - 30))
                            & ((1usize << (MATCH - 30))
                                | (1usize << (UNDERSCORE - 30))
                                | (1usize << (NAME - 30))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- testlist_comp ----------------
pub type Testlist_compContextAll<'input> = Testlist_compContext<'input>;

pub type Testlist_compContext<'input> =
    BaseParserRuleContext<'input, Testlist_compContextExt<'input>>;

#[derive(Clone)]
pub struct Testlist_compContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Testlist_compContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Testlist_compContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_testlist_comp(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_testlist_comp(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Testlist_compContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_testlist_comp(self);
    }
}

impl<'input> CustomRuleContext<'input> for Testlist_compContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_testlist_comp
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_testlist_comp }
}
antlr_rust::tid! {Testlist_compContextExt<'a>}

impl<'input> Testlist_compContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Testlist_compContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Testlist_compContextExt { ph: PhantomData },
        ))
    }
}

pub trait Testlist_compContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Testlist_compContextExt<'input>>
{
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn star_expr_all(&self) -> Vec<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn star_expr(&self, i: usize) -> Option<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn comp_for(&self) -> Option<Rc<Comp_forContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Testlist_compContextAttrs<'input> for Testlist_compContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn testlist_comp(&mut self) -> Result<Rc<Testlist_compContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Testlist_compContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 202, RULE_testlist_comp);
        let mut _localctx: Rc<Testlist_compContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1225);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT | TRUE
                    | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS
                    | NOT_OP | OPEN_BRACE => {
                        {
                            /*InvokeRule test*/
                            recog.base.set_state(1223);
                            recog.test()?;
                        }
                    }

                    STAR => {
                        {
                            /*InvokeRule star_expr*/
                            recog.base.set_state(1224);
                            recog.star_expr()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(1241);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    ASYNC | FOR => {
                        {
                            /*InvokeRule comp_for*/
                            recog.base.set_state(1227);
                            recog.comp_for()?;
                        }
                    }

                    CLOSE_PAREN | COMMA | CLOSE_BRACK => {
                        {
                            recog.base.set_state(1235);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(159, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(1228);
                                            recog
                                                .base
                                                .match_token(COMMA, &mut recog.err_handler)?;

                                            recog.base.set_state(1231);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            match recog.base.input.la(1) {
                                                STRING | NUMBER | AWAIT | FALSE | LAMBDA
                                                | MATCH | NONE | NOT | TRUE | UNDERSCORE | NAME
                                                | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD
                                                | MINUS | NOT_OP | OPEN_BRACE => {
                                                    {
                                                        /*InvokeRule test*/
                                                        recog.base.set_state(1229);
                                                        recog.test()?;
                                                    }
                                                }

                                                STAR => {
                                                    {
                                                        /*InvokeRule star_expr*/
                                                        recog.base.set_state(1230);
                                                        recog.star_expr()?;
                                                    }
                                                }

                                                _ => Err(ANTLRError::NoAltError(
                                                    NoViableAltError::new(&mut recog.base),
                                                ))?,
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(1237);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(159, &mut recog.base)?;
                            }
                            recog.base.set_state(1239);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == COMMA {
                                {
                                    recog.base.set_state(1238);
                                    recog.base.match_token(COMMA, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- trailer ----------------
pub type TrailerContextAll<'input> = TrailerContext<'input>;

pub type TrailerContext<'input> = BaseParserRuleContext<'input, TrailerContextExt<'input>>;

#[derive(Clone)]
pub struct TrailerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for TrailerContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for TrailerContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_trailer(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_trailer(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for TrailerContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_trailer(self);
    }
}

impl<'input> CustomRuleContext<'input> for TrailerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_trailer
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_trailer }
}
antlr_rust::tid! {TrailerContextExt<'a>}

impl<'input> TrailerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TrailerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TrailerContextExt { ph: PhantomData },
        ))
    }
}

pub trait TrailerContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<TrailerContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn arglist(&self) -> Option<Rc<ArglistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_BRACK
    /// Returns `None` if there is no child corresponding to token OPEN_BRACK
    fn OPEN_BRACK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_BRACK, 0)
    }
    fn subscriptlist(&self) -> Option<Rc<SubscriptlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_BRACK
    /// Returns `None` if there is no child corresponding to token CLOSE_BRACK
    fn CLOSE_BRACK(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_BRACK, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DOT, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> TrailerContextAttrs<'input> for TrailerContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn trailer(&mut self) -> Result<Rc<TrailerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TrailerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_trailer);
        let mut _localctx: Rc<TrailerContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1254);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                OPEN_PAREN => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1243);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        recog.base.set_state(1245);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (STAR - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (POWER - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule arglist*/
                                recog.base.set_state(1244);
                                recog.arglist()?;
                            }
                        }

                        recog.base.set_state(1247);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }

                OPEN_BRACK => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1248);
                        recog.base.match_token(OPEN_BRACK, &mut recog.err_handler)?;

                        /*InvokeRule subscriptlist*/
                        recog.base.set_state(1249);
                        recog.subscriptlist()?;

                        recog.base.set_state(1250);
                        recog
                            .base
                            .match_token(CLOSE_BRACK, &mut recog.err_handler)?;
                    }
                }

                DOT => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(1252);
                        recog.base.match_token(DOT, &mut recog.err_handler)?;

                        /*InvokeRule name*/
                        recog.base.set_state(1253);
                        recog.name()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- subscriptlist ----------------
pub type SubscriptlistContextAll<'input> = SubscriptlistContext<'input>;

pub type SubscriptlistContext<'input> =
    BaseParserRuleContext<'input, SubscriptlistContextExt<'input>>;

#[derive(Clone)]
pub struct SubscriptlistContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for SubscriptlistContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for SubscriptlistContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_subscriptlist(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_subscriptlist(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for SubscriptlistContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_subscriptlist(self);
    }
}

impl<'input> CustomRuleContext<'input> for SubscriptlistContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_subscriptlist
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_subscriptlist }
}
antlr_rust::tid! {SubscriptlistContextExt<'a>}

impl<'input> SubscriptlistContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SubscriptlistContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SubscriptlistContextExt { ph: PhantomData },
        ))
    }
}

pub trait SubscriptlistContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<SubscriptlistContextExt<'input>>
{
    fn subscript__all(&self) -> Vec<Rc<Subscript_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn subscript_(&self, i: usize) -> Option<Rc<Subscript_ContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> SubscriptlistContextAttrs<'input> for SubscriptlistContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn subscriptlist(&mut self) -> Result<Rc<SubscriptlistContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            SubscriptlistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 206, RULE_subscriptlist);
        let mut _localctx: Rc<SubscriptlistContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule subscript_*/
                recog.base.set_state(1256);
                recog.subscript_()?;

                recog.base.set_state(1261);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(164, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1257);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule subscript_*/
                                recog.base.set_state(1258);
                                recog.subscript_()?;
                            }
                        }
                    }
                    recog.base.set_state(1263);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(164, &mut recog.base)?;
                }
                recog.base.set_state(1265);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(1264);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- subscript_ ----------------
pub type Subscript_ContextAll<'input> = Subscript_Context<'input>;

pub type Subscript_Context<'input> = BaseParserRuleContext<'input, Subscript_ContextExt<'input>>;

#[derive(Clone)]
pub struct Subscript_ContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Subscript_Context<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Subscript_Context<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_subscript_(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_subscript_(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Subscript_Context<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_subscript_(self);
    }
}

impl<'input> CustomRuleContext<'input> for Subscript_ContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_subscript_
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_subscript_ }
}
antlr_rust::tid! {Subscript_ContextExt<'a>}

impl<'input> Subscript_ContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Subscript_ContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Subscript_ContextExt { ph: PhantomData },
        ))
    }
}

pub trait Subscript_ContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Subscript_ContextExt<'input>>
{
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn sliceop(&self) -> Option<Rc<SliceopContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Subscript_ContextAttrs<'input> for Subscript_Context<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn subscript_(&mut self) -> Result<Rc<Subscript_ContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Subscript_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 208, RULE_subscript_);
        let mut _localctx: Rc<Subscript_ContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1278);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(169, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule test*/
                        recog.base.set_state(1267);
                        recog.test()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1269);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule test*/
                                recog.base.set_state(1268);
                                recog.test()?;
                            }
                        }

                        recog.base.set_state(1271);
                        recog.base.match_token(COLON, &mut recog.err_handler)?;

                        recog.base.set_state(1273);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule test*/
                                recog.base.set_state(1272);
                                recog.test()?;
                            }
                        }

                        recog.base.set_state(1276);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == COLON {
                            {
                                /*InvokeRule sliceop*/
                                recog.base.set_state(1275);
                                recog.sliceop()?;
                            }
                        }
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- sliceop ----------------
pub type SliceopContextAll<'input> = SliceopContext<'input>;

pub type SliceopContext<'input> = BaseParserRuleContext<'input, SliceopContextExt<'input>>;

#[derive(Clone)]
pub struct SliceopContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for SliceopContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for SliceopContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_sliceop(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_sliceop(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for SliceopContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_sliceop(self);
    }
}

impl<'input> CustomRuleContext<'input> for SliceopContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_sliceop
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_sliceop }
}
antlr_rust::tid! {SliceopContextExt<'a>}

impl<'input> SliceopContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<SliceopContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            SliceopContextExt { ph: PhantomData },
        ))
    }
}

pub trait SliceopContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<SliceopContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> SliceopContextAttrs<'input> for SliceopContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn sliceop(&mut self) -> Result<Rc<SliceopContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = SliceopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_sliceop);
        let mut _localctx: Rc<SliceopContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1280);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                recog.base.set_state(1282);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << AWAIT)
                            | (1usize << FALSE)
                            | (1usize << LAMBDA)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 33) & !0x3f) == 0
                        && ((1usize << (_la - 33))
                            & ((1usize << (NOT - 33))
                                | (1usize << (TRUE - 33))
                                | (1usize << (UNDERSCORE - 33))
                                | (1usize << (NAME - 33))
                                | (1usize << (ELLIPSIS - 33))
                                | (1usize << (OPEN_PAREN - 33))
                                | (1usize << (OPEN_BRACK - 33))))
                            != 0)
                    || (((_la - 71) & !0x3f) == 0
                        && ((1usize << (_la - 71))
                            & ((1usize << (ADD - 71))
                                | (1usize << (MINUS - 71))
                                | (1usize << (NOT_OP - 71))
                                | (1usize << (OPEN_BRACE - 71))))
                            != 0)
                {
                    {
                        /*InvokeRule test*/
                        recog.base.set_state(1281);
                        recog.test()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- exprlist ----------------
pub type ExprlistContextAll<'input> = ExprlistContext<'input>;

pub type ExprlistContext<'input> = BaseParserRuleContext<'input, ExprlistContextExt<'input>>;

#[derive(Clone)]
pub struct ExprlistContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ExprlistContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ExprlistContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_exprlist(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_exprlist(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ExprlistContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_exprlist(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExprlistContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_exprlist
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_exprlist }
}
antlr_rust::tid! {ExprlistContextExt<'a>}

impl<'input> ExprlistContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExprlistContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExprlistContextExt { ph: PhantomData },
        ))
    }
}

pub trait ExprlistContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ExprlistContextExt<'input>>
{
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn star_expr_all(&self) -> Vec<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn star_expr(&self, i: usize) -> Option<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ExprlistContextAttrs<'input> for ExprlistContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn exprlist(&mut self) -> Result<Rc<ExprlistContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExprlistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 212, RULE_exprlist);
        let mut _localctx: Rc<ExprlistContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1286);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    STRING | NUMBER | AWAIT | FALSE | MATCH | NONE | TRUE | UNDERSCORE | NAME
                    | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                        {
                            /*InvokeRule expr*/
                            recog.base.set_state(1284);
                            recog.expr_rec(0)?;
                        }
                    }

                    STAR => {
                        {
                            /*InvokeRule star_expr*/
                            recog.base.set_state(1285);
                            recog.star_expr()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(1295);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(173, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1288);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                recog.base.set_state(1291);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.base.input.la(1) {
                                    STRING | NUMBER | AWAIT | FALSE | MATCH | NONE | TRUE
                                    | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN | OPEN_BRACK
                                    | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                                        {
                                            /*InvokeRule expr*/
                                            recog.base.set_state(1289);
                                            recog.expr_rec(0)?;
                                        }
                                    }

                                    STAR => {
                                        {
                                            /*InvokeRule star_expr*/
                                            recog.base.set_state(1290);
                                            recog.star_expr()?;
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                            }
                        }
                    }
                    recog.base.set_state(1297);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(173, &mut recog.base)?;
                }
                recog.base.set_state(1299);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(1298);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- testlist ----------------
pub type TestlistContextAll<'input> = TestlistContext<'input>;

pub type TestlistContext<'input> = BaseParserRuleContext<'input, TestlistContextExt<'input>>;

#[derive(Clone)]
pub struct TestlistContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for TestlistContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for TestlistContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_testlist(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_testlist(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for TestlistContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_testlist(self);
    }
}

impl<'input> CustomRuleContext<'input> for TestlistContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_testlist
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_testlist }
}
antlr_rust::tid! {TestlistContextExt<'a>}

impl<'input> TestlistContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<TestlistContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            TestlistContextExt { ph: PhantomData },
        ))
    }
}

pub trait TestlistContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<TestlistContextExt<'input>>
{
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> TestlistContextAttrs<'input> for TestlistContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn testlist(&mut self) -> Result<Rc<TestlistContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = TestlistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_testlist);
        let mut _localctx: Rc<TestlistContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule test*/
                recog.base.set_state(1301);
                recog.test()?;

                recog.base.set_state(1306);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(175, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1302);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule test*/
                                recog.base.set_state(1303);
                                recog.test()?;
                            }
                        }
                    }
                    recog.base.set_state(1308);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(175, &mut recog.base)?;
                }
                recog.base.set_state(1310);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(1309);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- dictorsetmaker ----------------
pub type DictorsetmakerContextAll<'input> = DictorsetmakerContext<'input>;

pub type DictorsetmakerContext<'input> =
    BaseParserRuleContext<'input, DictorsetmakerContextExt<'input>>;

#[derive(Clone)]
pub struct DictorsetmakerContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for DictorsetmakerContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for DictorsetmakerContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dictorsetmaker(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_dictorsetmaker(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a>
    for DictorsetmakerContext<'input>
{
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_dictorsetmaker(self);
    }
}

impl<'input> CustomRuleContext<'input> for DictorsetmakerContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dictorsetmaker
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dictorsetmaker }
}
antlr_rust::tid! {DictorsetmakerContextExt<'a>}

impl<'input> DictorsetmakerContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DictorsetmakerContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DictorsetmakerContextExt { ph: PhantomData },
        ))
    }
}

pub trait DictorsetmakerContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<DictorsetmakerContextExt<'input>>
{
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COLON in current rule
    fn COLON_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COLON, starting from 0.
    /// Returns `None` if number of children corresponding to token COLON is less or equal than `i`.
    fn COLON(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token POWER in current rule
    fn POWER_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token POWER, starting from 0.
    /// Returns `None` if number of children corresponding to token POWER is less or equal than `i`.
    fn POWER(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER, i)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn comp_for(&self) -> Option<Rc<Comp_forContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn star_expr_all(&self) -> Vec<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn star_expr(&self, i: usize) -> Option<Rc<Star_exprContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> DictorsetmakerContextAttrs<'input> for DictorsetmakerContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dictorsetmaker(&mut self) -> Result<Rc<DictorsetmakerContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            DictorsetmakerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 216, RULE_dictorsetmaker);
        let mut _localctx: Rc<DictorsetmakerContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1360);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(187, &mut recog.base)? {
                    1 => {
                        {
                            {
                                recog.base.set_state(1318);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.base.input.la(1) {
                                    STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE
                                    | NOT | TRUE | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN
                                    | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                                        {
                                            /*InvokeRule test*/
                                            recog.base.set_state(1312);
                                            recog.test()?;

                                            recog.base.set_state(1313);
                                            recog
                                                .base
                                                .match_token(COLON, &mut recog.err_handler)?;

                                            /*InvokeRule test*/
                                            recog.base.set_state(1314);
                                            recog.test()?;
                                        }
                                    }

                                    POWER => {
                                        {
                                            recog.base.set_state(1316);
                                            recog
                                                .base
                                                .match_token(POWER, &mut recog.err_handler)?;

                                            /*InvokeRule expr*/
                                            recog.base.set_state(1317);
                                            recog.expr_rec(0)?;
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                                recog.base.set_state(1338);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.base.input.la(1) {
                                    ASYNC | FOR => {
                                        {
                                            /*InvokeRule comp_for*/
                                            recog.base.set_state(1320);
                                            recog.comp_for()?;
                                        }
                                    }

                                    COMMA | CLOSE_BRACE => {
                                        {
                                            recog.base.set_state(1332);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _alt = recog
                                                .interpreter
                                                .adaptive_predict(179, &mut recog.base)?;
                                            while { _alt != 2 && _alt != INVALID_ALT } {
                                                if _alt == 1 {
                                                    {
                                                        {
                                                            recog.base.set_state(1321);
                                                            recog.base.match_token(
                                                                COMMA,
                                                                &mut recog.err_handler,
                                                            )?;

                                                            recog.base.set_state(1328);
                                                            recog
                                                                .err_handler
                                                                .sync(&mut recog.base)?;
                                                            match recog.base.input.la(1) {
                                                                STRING | NUMBER | AWAIT | FALSE
                                                                | LAMBDA | MATCH | NONE | NOT
                                                                | TRUE | UNDERSCORE | NAME
                                                                | ELLIPSIS | OPEN_PAREN
                                                                | OPEN_BRACK | ADD | MINUS
                                                                | NOT_OP | OPEN_BRACE => {
                                                                    {
                                                                        /*InvokeRule test*/
                                                                        recog.base.set_state(1322);
                                                                        recog.test()?;

                                                                        recog.base.set_state(1323);
                                                                        recog.base.match_token(
                                                                            COLON,
                                                                            &mut recog.err_handler,
                                                                        )?;

                                                                        /*InvokeRule test*/
                                                                        recog.base.set_state(1324);
                                                                        recog.test()?;
                                                                    }
                                                                }

                                                                POWER => {
                                                                    {
                                                                        recog.base.set_state(1326);
                                                                        recog.base.match_token(
                                                                            POWER,
                                                                            &mut recog.err_handler,
                                                                        )?;

                                                                        /*InvokeRule expr*/
                                                                        recog.base.set_state(1327);
                                                                        recog.expr_rec(0)?;
                                                                    }
                                                                }

                                                                _ => Err(ANTLRError::NoAltError(
                                                                    NoViableAltError::new(
                                                                        &mut recog.base,
                                                                    ),
                                                                ))?,
                                                            }
                                                        }
                                                    }
                                                }
                                                recog.base.set_state(1334);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _alt = recog
                                                    .interpreter
                                                    .adaptive_predict(179, &mut recog.base)?;
                                            }
                                            recog.base.set_state(1336);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == COMMA {
                                                {
                                                    recog.base.set_state(1335);
                                                    recog.base.match_token(
                                                        COMMA,
                                                        &mut recog.err_handler,
                                                    )?;
                                                }
                                            }
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                            }
                        }
                    }
                    2 => {
                        {
                            {
                                recog.base.set_state(1342);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.base.input.la(1) {
                                    STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE
                                    | NOT | TRUE | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN
                                    | OPEN_BRACK | ADD | MINUS | NOT_OP | OPEN_BRACE => {
                                        {
                                            /*InvokeRule test*/
                                            recog.base.set_state(1340);
                                            recog.test()?;
                                        }
                                    }

                                    STAR => {
                                        {
                                            /*InvokeRule star_expr*/
                                            recog.base.set_state(1341);
                                            recog.star_expr()?;
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                                recog.base.set_state(1358);
                                recog.err_handler.sync(&mut recog.base)?;
                                match recog.base.input.la(1) {
                                    ASYNC | FOR => {
                                        {
                                            /*InvokeRule comp_for*/
                                            recog.base.set_state(1344);
                                            recog.comp_for()?;
                                        }
                                    }

                                    COMMA | CLOSE_BRACE => {
                                        {
                                            recog.base.set_state(1352);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _alt = recog
                                                .interpreter
                                                .adaptive_predict(184, &mut recog.base)?;
                                            while { _alt != 2 && _alt != INVALID_ALT } {
                                                if _alt == 1 {
                                                    {
                                                        {
                                                            recog.base.set_state(1345);
                                                            recog.base.match_token(
                                                                COMMA,
                                                                &mut recog.err_handler,
                                                            )?;

                                                            recog.base.set_state(1348);
                                                            recog
                                                                .err_handler
                                                                .sync(&mut recog.base)?;
                                                            match recog.base.input.la(1) {
                                                                STRING | NUMBER | AWAIT | FALSE
                                                                | LAMBDA | MATCH | NONE | NOT
                                                                | TRUE | UNDERSCORE | NAME
                                                                | ELLIPSIS | OPEN_PAREN
                                                                | OPEN_BRACK | ADD | MINUS
                                                                | NOT_OP | OPEN_BRACE => {
                                                                    {
                                                                        /*InvokeRule test*/
                                                                        recog.base.set_state(1346);
                                                                        recog.test()?;
                                                                    }
                                                                }

                                                                STAR => {
                                                                    {
                                                                        /*InvokeRule star_expr*/
                                                                        recog.base.set_state(1347);
                                                                        recog.star_expr()?;
                                                                    }
                                                                }

                                                                _ => Err(ANTLRError::NoAltError(
                                                                    NoViableAltError::new(
                                                                        &mut recog.base,
                                                                    ),
                                                                ))?,
                                                            }
                                                        }
                                                    }
                                                }
                                                recog.base.set_state(1354);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _alt = recog
                                                    .interpreter
                                                    .adaptive_predict(184, &mut recog.base)?;
                                            }
                                            recog.base.set_state(1356);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == COMMA {
                                                {
                                                    recog.base.set_state(1355);
                                                    recog.base.match_token(
                                                        COMMA,
                                                        &mut recog.err_handler,
                                                    )?;
                                                }
                                            }
                                        }
                                    }

                                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                        &mut recog.base,
                                    )))?,
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- classdef ----------------
pub type ClassdefContextAll<'input> = ClassdefContext<'input>;

pub type ClassdefContext<'input> = BaseParserRuleContext<'input, ClassdefContextExt<'input>>;

#[derive(Clone)]
pub struct ClassdefContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ClassdefContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ClassdefContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_classdef(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_classdef(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ClassdefContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_classdef(self);
    }
}

impl<'input> CustomRuleContext<'input> for ClassdefContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_classdef
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_classdef }
}
antlr_rust::tid! {ClassdefContextExt<'a>}

impl<'input> ClassdefContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ClassdefContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ClassdefContextExt { ph: PhantomData },
        ))
    }
}

pub trait ClassdefContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ClassdefContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CLASS
    /// Returns `None` if there is no child corresponding to token CLASS
    fn CLASS(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLASS, 0)
    }
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COLON, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPEN_PAREN
    /// Returns `None` if there is no child corresponding to token OPEN_PAREN
    fn OPEN_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPEN_PAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CLOSE_PAREN
    /// Returns `None` if there is no child corresponding to token CLOSE_PAREN
    fn CLOSE_PAREN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CLOSE_PAREN, 0)
    }
    fn arglist(&self) -> Option<Rc<ArglistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ClassdefContextAttrs<'input> for ClassdefContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn classdef(&mut self) -> Result<Rc<ClassdefContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ClassdefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_classdef);
        let mut _localctx: Rc<ClassdefContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1362);
                recog.base.match_token(CLASS, &mut recog.err_handler)?;

                /*InvokeRule name*/
                recog.base.set_state(1363);
                recog.name()?;

                recog.base.set_state(1369);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPEN_PAREN {
                    {
                        recog.base.set_state(1364);
                        recog.base.match_token(OPEN_PAREN, &mut recog.err_handler)?;

                        recog.base.set_state(1366);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << STRING)
                                    | (1usize << NUMBER)
                                    | (1usize << AWAIT)
                                    | (1usize << FALSE)
                                    | (1usize << LAMBDA)
                                    | (1usize << MATCH)
                                    | (1usize << NONE)))
                                != 0)
                            || (((_la - 33) & !0x3f) == 0
                                && ((1usize << (_la - 33))
                                    & ((1usize << (NOT - 33))
                                        | (1usize << (TRUE - 33))
                                        | (1usize << (UNDERSCORE - 33))
                                        | (1usize << (NAME - 33))
                                        | (1usize << (ELLIPSIS - 33))
                                        | (1usize << (STAR - 33))
                                        | (1usize << (OPEN_PAREN - 33))
                                        | (1usize << (POWER - 33))
                                        | (1usize << (OPEN_BRACK - 33))))
                                    != 0)
                            || (((_la - 71) & !0x3f) == 0
                                && ((1usize << (_la - 71))
                                    & ((1usize << (ADD - 71))
                                        | (1usize << (MINUS - 71))
                                        | (1usize << (NOT_OP - 71))
                                        | (1usize << (OPEN_BRACE - 71))))
                                    != 0)
                        {
                            {
                                /*InvokeRule arglist*/
                                recog.base.set_state(1365);
                                recog.arglist()?;
                            }
                        }

                        recog.base.set_state(1368);
                        recog
                            .base
                            .match_token(CLOSE_PAREN, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1371);
                recog.base.match_token(COLON, &mut recog.err_handler)?;

                /*InvokeRule block*/
                recog.base.set_state(1372);
                recog.block()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- arglist ----------------
pub type ArglistContextAll<'input> = ArglistContext<'input>;

pub type ArglistContext<'input> = BaseParserRuleContext<'input, ArglistContextExt<'input>>;

#[derive(Clone)]
pub struct ArglistContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ArglistContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ArglistContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_arglist(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_arglist(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ArglistContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_arglist(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArglistContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_arglist
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_arglist }
}
antlr_rust::tid! {ArglistContextExt<'a>}

impl<'input> ArglistContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArglistContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArglistContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArglistContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ArglistContextExt<'input>>
{
    fn argument_all(&self) -> Vec<Rc<ArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn argument(&self, i: usize) -> Option<Rc<ArgumentContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> ArglistContextAttrs<'input> for ArglistContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn arglist(&mut self) -> Result<Rc<ArglistContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArglistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_arglist);
        let mut _localctx: Rc<ArglistContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule argument*/
                recog.base.set_state(1374);
                recog.argument()?;

                recog.base.set_state(1379);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(190, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1375);
                                recog.base.match_token(COMMA, &mut recog.err_handler)?;

                                /*InvokeRule argument*/
                                recog.base.set_state(1376);
                                recog.argument()?;
                            }
                        }
                    }
                    recog.base.set_state(1381);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(190, &mut recog.base)?;
                }
                recog.base.set_state(1383);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMA {
                    {
                        recog.base.set_state(1382);
                        recog.base.match_token(COMMA, &mut recog.err_handler)?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- argument ----------------
pub type ArgumentContextAll<'input> = ArgumentContext<'input>;

pub type ArgumentContext<'input> = BaseParserRuleContext<'input, ArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for ArgumentContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for ArgumentContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_argument(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_argument(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for ArgumentContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_argument(self);
    }
}

impl<'input> CustomRuleContext<'input> for ArgumentContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_argument
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_argument }
}
antlr_rust::tid! {ArgumentContextExt<'a>}

impl<'input> ArgumentContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ArgumentContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ArgumentContextExt { ph: PhantomData },
        ))
    }
}

pub trait ArgumentContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<ArgumentContextExt<'input>>
{
    fn test_all(&self) -> Vec<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn test(&self, i: usize) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASSIGN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token POWER
    /// Returns `None` if there is no child corresponding to token POWER
    fn POWER(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(POWER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STAR, 0)
    }
    fn comp_for(&self) -> Option<Rc<Comp_forContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ArgumentContextAttrs<'input> for ArgumentContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn argument(&mut self) -> Result<Rc<ArgumentContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_argument);
        let mut _localctx: Rc<ArgumentContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1397);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(193, &mut recog.base)? {
                    1 => {
                        {
                            /*InvokeRule test*/
                            recog.base.set_state(1385);
                            recog.test()?;

                            recog.base.set_state(1387);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == ASYNC || _la == FOR {
                                {
                                    /*InvokeRule comp_for*/
                                    recog.base.set_state(1386);
                                    recog.comp_for()?;
                                }
                            }
                        }
                    }
                    2 => {
                        {
                            /*InvokeRule test*/
                            recog.base.set_state(1389);
                            recog.test()?;

                            recog.base.set_state(1390);
                            recog.base.match_token(ASSIGN, &mut recog.err_handler)?;

                            /*InvokeRule test*/
                            recog.base.set_state(1391);
                            recog.test()?;
                        }
                    }
                    3 => {
                        {
                            recog.base.set_state(1393);
                            recog.base.match_token(POWER, &mut recog.err_handler)?;

                            /*InvokeRule test*/
                            recog.base.set_state(1394);
                            recog.test()?;
                        }
                    }
                    4 => {
                        {
                            recog.base.set_state(1395);
                            recog.base.match_token(STAR, &mut recog.err_handler)?;

                            /*InvokeRule test*/
                            recog.base.set_state(1396);
                            recog.test()?;
                        }
                    }

                    _ => {}
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- comp_iter ----------------
pub type Comp_iterContextAll<'input> = Comp_iterContext<'input>;

pub type Comp_iterContext<'input> = BaseParserRuleContext<'input, Comp_iterContextExt<'input>>;

#[derive(Clone)]
pub struct Comp_iterContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Comp_iterContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Comp_iterContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_comp_iter(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_comp_iter(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Comp_iterContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_comp_iter(self);
    }
}

impl<'input> CustomRuleContext<'input> for Comp_iterContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_comp_iter
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_comp_iter }
}
antlr_rust::tid! {Comp_iterContextExt<'a>}

impl<'input> Comp_iterContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Comp_iterContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Comp_iterContextExt { ph: PhantomData },
        ))
    }
}

pub trait Comp_iterContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Comp_iterContextExt<'input>>
{
    fn comp_for(&self) -> Option<Rc<Comp_forContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn comp_if(&self) -> Option<Rc<Comp_ifContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Comp_iterContextAttrs<'input> for Comp_iterContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn comp_iter(&mut self) -> Result<Rc<Comp_iterContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Comp_iterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 224, RULE_comp_iter);
        let mut _localctx: Rc<Comp_iterContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1401);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                ASYNC | FOR => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule comp_for*/
                        recog.base.set_state(1399);
                        recog.comp_for()?;
                    }
                }

                IF => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule comp_if*/
                        recog.base.set_state(1400);
                        recog.comp_if()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- comp_for ----------------
pub type Comp_forContextAll<'input> = Comp_forContext<'input>;

pub type Comp_forContext<'input> = BaseParserRuleContext<'input, Comp_forContextExt<'input>>;

#[derive(Clone)]
pub struct Comp_forContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Comp_forContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Comp_forContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_comp_for(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_comp_for(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Comp_forContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_comp_for(self);
    }
}

impl<'input> CustomRuleContext<'input> for Comp_forContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_comp_for
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_comp_for }
}
antlr_rust::tid! {Comp_forContextExt<'a>}

impl<'input> Comp_forContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Comp_forContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Comp_forContextExt { ph: PhantomData },
        ))
    }
}

pub trait Comp_forContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Comp_forContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    fn exprlist(&self) -> Option<Rc<ExprlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    fn or_test(&self) -> Option<Rc<Or_testContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASYNC
    /// Returns `None` if there is no child corresponding to token ASYNC
    fn ASYNC(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASYNC, 0)
    }
    fn comp_iter(&self) -> Option<Rc<Comp_iterContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Comp_forContextAttrs<'input> for Comp_forContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn comp_for(&mut self) -> Result<Rc<Comp_forContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Comp_forContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_comp_for);
        let mut _localctx: Rc<Comp_forContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1404);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == ASYNC {
                    {
                        recog.base.set_state(1403);
                        recog.base.match_token(ASYNC, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1406);
                recog.base.match_token(FOR, &mut recog.err_handler)?;

                /*InvokeRule exprlist*/
                recog.base.set_state(1407);
                recog.exprlist()?;

                recog.base.set_state(1408);
                recog.base.match_token(IN, &mut recog.err_handler)?;

                /*InvokeRule or_test*/
                recog.base.set_state(1409);
                recog.or_test()?;

                recog.base.set_state(1411);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if ((_la) & !0x3f) == 0
                    && ((1usize << _la) & ((1usize << ASYNC) | (1usize << FOR) | (1usize << IF)))
                        != 0
                {
                    {
                        /*InvokeRule comp_iter*/
                        recog.base.set_state(1410);
                        recog.comp_iter()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- comp_if ----------------
pub type Comp_ifContextAll<'input> = Comp_ifContext<'input>;

pub type Comp_ifContext<'input> = BaseParserRuleContext<'input, Comp_ifContextExt<'input>>;

#[derive(Clone)]
pub struct Comp_ifContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Comp_ifContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Comp_ifContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_comp_if(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_comp_if(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Comp_ifContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_comp_if(self);
    }
}

impl<'input> CustomRuleContext<'input> for Comp_ifContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_comp_if
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_comp_if }
}
antlr_rust::tid! {Comp_ifContextExt<'a>}

impl<'input> Comp_ifContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Comp_ifContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Comp_ifContextExt { ph: PhantomData },
        ))
    }
}

pub trait Comp_ifContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Comp_ifContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    fn test_nocond(&self) -> Option<Rc<Test_nocondContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn comp_iter(&self) -> Option<Rc<Comp_iterContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Comp_ifContextAttrs<'input> for Comp_ifContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn comp_if(&mut self) -> Result<Rc<Comp_ifContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Comp_ifContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_comp_if);
        let mut _localctx: Rc<Comp_ifContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1413);
                recog.base.match_token(IF, &mut recog.err_handler)?;

                /*InvokeRule test_nocond*/
                recog.base.set_state(1414);
                recog.test_nocond()?;

                recog.base.set_state(1416);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if ((_la) & !0x3f) == 0
                    && ((1usize << _la) & ((1usize << ASYNC) | (1usize << FOR) | (1usize << IF)))
                        != 0
                {
                    {
                        /*InvokeRule comp_iter*/
                        recog.base.set_state(1415);
                        recog.comp_iter()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- encoding_decl ----------------
pub type Encoding_declContextAll<'input> = Encoding_declContext<'input>;

pub type Encoding_declContext<'input> =
    BaseParserRuleContext<'input, Encoding_declContextExt<'input>>;

#[derive(Clone)]
pub struct Encoding_declContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Encoding_declContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a>
    for Encoding_declContext<'input>
{
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_encoding_decl(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_encoding_decl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Encoding_declContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_encoding_decl(self);
    }
}

impl<'input> CustomRuleContext<'input> for Encoding_declContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_encoding_decl
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_encoding_decl }
}
antlr_rust::tid! {Encoding_declContextExt<'a>}

impl<'input> Encoding_declContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Encoding_declContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Encoding_declContextExt { ph: PhantomData },
        ))
    }
}

pub trait Encoding_declContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Encoding_declContextExt<'input>>
{
    fn name(&self) -> Option<Rc<NameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Encoding_declContextAttrs<'input> for Encoding_declContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn encoding_decl(&mut self) -> Result<Rc<Encoding_declContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Encoding_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 230, RULE_encoding_decl);
        let mut _localctx: Rc<Encoding_declContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule name*/
                recog.base.set_state(1418);
                recog.name()?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- yield_expr ----------------
pub type Yield_exprContextAll<'input> = Yield_exprContext<'input>;

pub type Yield_exprContext<'input> = BaseParserRuleContext<'input, Yield_exprContextExt<'input>>;

#[derive(Clone)]
pub struct Yield_exprContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Yield_exprContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Yield_exprContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_yield_expr(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_yield_expr(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Yield_exprContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_yield_expr(self);
    }
}

impl<'input> CustomRuleContext<'input> for Yield_exprContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_yield_expr
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_yield_expr }
}
antlr_rust::tid! {Yield_exprContextExt<'a>}

impl<'input> Yield_exprContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Yield_exprContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Yield_exprContextExt { ph: PhantomData },
        ))
    }
}

pub trait Yield_exprContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Yield_exprContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token YIELD
    /// Returns `None` if there is no child corresponding to token YIELD
    fn YIELD(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(YIELD, 0)
    }
    fn yield_arg(&self) -> Option<Rc<Yield_argContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Yield_exprContextAttrs<'input> for Yield_exprContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn yield_expr(&mut self) -> Result<Rc<Yield_exprContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Yield_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 232, RULE_yield_expr);
        let mut _localctx: Rc<Yield_exprContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1420);
                recog.base.match_token(YIELD, &mut recog.err_handler)?;

                recog.base.set_state(1422);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << STRING)
                            | (1usize << NUMBER)
                            | (1usize << AWAIT)
                            | (1usize << FALSE)
                            | (1usize << FROM)
                            | (1usize << LAMBDA)
                            | (1usize << MATCH)
                            | (1usize << NONE)))
                        != 0)
                    || (((_la - 33) & !0x3f) == 0
                        && ((1usize << (_la - 33))
                            & ((1usize << (NOT - 33))
                                | (1usize << (TRUE - 33))
                                | (1usize << (UNDERSCORE - 33))
                                | (1usize << (NAME - 33))
                                | (1usize << (ELLIPSIS - 33))
                                | (1usize << (OPEN_PAREN - 33))
                                | (1usize << (OPEN_BRACK - 33))))
                            != 0)
                    || (((_la - 71) & !0x3f) == 0
                        && ((1usize << (_la - 71))
                            & ((1usize << (ADD - 71))
                                | (1usize << (MINUS - 71))
                                | (1usize << (NOT_OP - 71))
                                | (1usize << (OPEN_BRACE - 71))))
                            != 0)
                {
                    {
                        /*InvokeRule yield_arg*/
                        recog.base.set_state(1421);
                        recog.yield_arg()?;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- yield_arg ----------------
pub type Yield_argContextAll<'input> = Yield_argContext<'input>;

pub type Yield_argContext<'input> = BaseParserRuleContext<'input, Yield_argContextExt<'input>>;

#[derive(Clone)]
pub struct Yield_argContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for Yield_argContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for Yield_argContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_yield_arg(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_yield_arg(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for Yield_argContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_yield_arg(self);
    }
}

impl<'input> CustomRuleContext<'input> for Yield_argContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_yield_arg
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_yield_arg }
}
antlr_rust::tid! {Yield_argContextExt<'a>}

impl<'input> Yield_argContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Yield_argContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Yield_argContextExt { ph: PhantomData },
        ))
    }
}

pub trait Yield_argContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<Yield_argContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FROM, 0)
    }
    fn test(&self) -> Option<Rc<TestContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn testlist(&self) -> Option<Rc<TestlistContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> Yield_argContextAttrs<'input> for Yield_argContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn yield_arg(&mut self) -> Result<Rc<Yield_argContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Yield_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 234, RULE_yield_arg);
        let mut _localctx: Rc<Yield_argContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(1427);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                FROM => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(1424);
                        recog.base.match_token(FROM, &mut recog.err_handler)?;

                        /*InvokeRule test*/
                        recog.base.set_state(1425);
                        recog.test()?;
                    }
                }

                STRING | NUMBER | AWAIT | FALSE | LAMBDA | MATCH | NONE | NOT | TRUE
                | UNDERSCORE | NAME | ELLIPSIS | OPEN_PAREN | OPEN_BRACK | ADD | MINUS | NOT_OP
                | OPEN_BRACE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule testlist*/
                        recog.base.set_state(1426);
                        recog.testlist()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- strings ----------------
pub type StringsContextAll<'input> = StringsContext<'input>;

pub type StringsContext<'input> = BaseParserRuleContext<'input, StringsContextExt<'input>>;

#[derive(Clone)]
pub struct StringsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> Python3ParserContext<'input> for StringsContext<'input> {}

impl<'input, 'a> Listenable<dyn Python3ParserListener<'input> + 'a> for StringsContext<'input> {
    fn enter(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_strings(self);
    }
    fn exit(&self, listener: &mut (dyn Python3ParserListener<'input> + 'a)) {
        listener.exit_strings(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn Python3ParserVisitor<'input> + 'a> for StringsContext<'input> {
    fn accept(&self, visitor: &mut (dyn Python3ParserVisitor<'input> + 'a)) {
        visitor.visit_strings(self);
    }
}

impl<'input> CustomRuleContext<'input> for StringsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = Python3ParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_strings
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_strings }
}
antlr_rust::tid! {StringsContextExt<'a>}

impl<'input> StringsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn Python3ParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StringsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StringsContextExt { ph: PhantomData },
        ))
    }
}

pub trait StringsContextAttrs<'input>:
    Python3ParserContext<'input> + BorrowMut<StringsContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
    fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
    /// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
    fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input, Python3ParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, i)
    }
}

impl<'input> StringsContextAttrs<'input> for StringsContext<'input> {}

impl<'input, I, H> Python3Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn strings(&mut self) -> Result<Rc<StringsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StringsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_strings);
        let mut _localctx: Rc<StringsContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1430);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            recog.base.set_state(1429);
                            recog.base.match_token(STRING, &mut recog.err_handler)?;
                        }
                    }
                    recog.base.set_state(1432);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == STRING) {
                        break;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x68\u{59d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x04\x67\x09\x67\x04\x68\x09\
	\x68\x04\x69\x09\x69\x04\x6a\x09\x6a\x04\x6b\x09\x6b\x04\x6c\x09\x6c\x04\
	\x6d\x09\x6d\x04\x6e\x09\x6e\x04\x6f\x09\x6f\x04\x70\x09\x70\x04\x71\x09\
	\x71\x04\x72\x09\x72\x04\x73\x09\x73\x04\x74\x09\x74\x04\x75\x09\x75\x04\
	\x76\x09\x76\x04\x77\x09\x77\x04\x78\x09\x78\x03\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x05\x02\u{f6}\x0a\x02\x03\x03\x03\x03\x07\x03\u{fa}\x0a\x03\
	\x0c\x03\x0e\x03\u{fd}\x0b\x03\x03\x03\x03\x03\x03\x04\x03\x04\x07\x04\u{103}\
	\x0a\x04\x0c\x04\x0e\x04\u{106}\x0b\x04\x03\x04\x03\x04\x03\x05\x03\x05\
	\x03\x05\x03\x05\x05\x05\u{10e}\x0a\x05\x03\x05\x05\x05\u{111}\x0a\x05\x03\
	\x05\x03\x05\x03\x06\x06\x06\u{116}\x0a\x06\x0d\x06\x0e\x06\u{117}\x03\x07\
	\x03\x07\x03\x07\x03\x07\x05\x07\u{11e}\x0a\x07\x03\x08\x03\x08\x03\x08\
	\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\x09\u{128}\x0a\x09\x03\x09\
	\x03\x09\x03\x09\x03\x0a\x03\x0a\x05\x0a\u{12f}\x0a\x0a\x03\x0a\x03\x0a\
	\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{136}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x05\x0b\u{13c}\x0a\x0b\x07\x0b\u{13e}\x0a\x0b\x0c\x0b\x0e\x0b\u{141}\
	\x0b\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{146}\x0a\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x05\x0b\u{14c}\x0a\x0b\x07\x0b\u{14e}\x0a\x0b\x0c\x0b\x0e\
	\x0b\u{151}\x0b\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{157}\x0a\x0b\
	\x05\x0b\u{159}\x0a\x0b\x05\x0b\u{15b}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x05\
	\x0b\u{160}\x0a\x0b\x05\x0b\u{162}\x0a\x0b\x05\x0b\u{164}\x0a\x0b\x03\x0b\
	\x03\x0b\x05\x0b\u{168}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\
	\u{16e}\x0a\x0b\x07\x0b\u{170}\x0a\x0b\x0c\x0b\x0e\x0b\u{173}\x0b\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{179}\x0a\x0b\x05\x0b\u{17b}\x0a\x0b\
	\x05\x0b\u{17d}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{182}\x0a\x0b\x05\
	\x0b\u{184}\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{189}\x0a\x0c\x03\x0d\
	\x03\x0d\x03\x0d\x05\x0d\u{18e}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x05\x0d\u{194}\x0a\x0d\x07\x0d\u{196}\x0a\x0d\x0c\x0d\x0e\x0d\u{199}\x0b\
	\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{19e}\x0a\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0d\x05\x0d\u{1a4}\x0a\x0d\x07\x0d\u{1a6}\x0a\x0d\x0c\x0d\x0e\x0d\
	\u{1a9}\x0b\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{1af}\x0a\x0d\x05\
	\x0d\u{1b1}\x0a\x0d\x05\x0d\u{1b3}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\
	\u{1b8}\x0a\x0d\x05\x0d\u{1ba}\x0a\x0d\x05\x0d\u{1bc}\x0a\x0d\x03\x0d\x03\
	\x0d\x05\x0d\u{1c0}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{1c6}\
	\x0a\x0d\x07\x0d\u{1c8}\x0a\x0d\x0c\x0d\x0e\x0d\u{1cb}\x0b\x0d\x03\x0d\x03\
	\x0d\x03\x0d\x03\x0d\x05\x0d\u{1d1}\x0a\x0d\x05\x0d\u{1d3}\x0a\x0d\x05\x0d\
	\u{1d5}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{1da}\x0a\x0d\x05\x0d\u{1dc}\
	\x0a\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x05\x0f\u{1e2}\x0a\x0f\x03\x10\
	\x03\x10\x03\x10\x07\x10\u{1e7}\x0a\x10\x0c\x10\x0e\x10\u{1ea}\x0b\x10\x03\
	\x10\x05\x10\u{1ed}\x0a\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x11\x03\x11\x03\x11\x03\x11\x05\x11\u{1f9}\x0a\x11\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{200}\x0a\x12\x03\x12\x03\x12\x03\
	\x12\x05\x12\u{205}\x0a\x12\x07\x12\u{207}\x0a\x12\x0c\x12\x0e\x12\u{20a}\
	\x0b\x12\x05\x12\u{20c}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\
	\u{212}\x0a\x13\x03\x14\x03\x14\x05\x14\u{216}\x0a\x14\x03\x14\x03\x14\x03\
	\x14\x05\x14\u{21b}\x0a\x14\x07\x14\u{21d}\x0a\x14\x0c\x14\x0e\x14\u{220}\
	\x0b\x14\x03\x14\x05\x14\u{223}\x0a\x14\x03\x15\x03\x15\x03\x16\x03\x16\
	\x03\x16\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x05\x18\
	\u{231}\x0a\x18\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x05\x1b\
	\u{239}\x0a\x1b\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x05\x1d\
	\u{241}\x0a\x1d\x05\x1d\u{243}\x0a\x1d\x03\x1e\x03\x1e\x05\x1e\u{247}\x0a\
	\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x20\x03\x20\x07\x20\u{24e}\x0a\x20\x0c\
	\x20\x0e\x20\u{251}\x0b\x20\x03\x20\x03\x20\x06\x20\u{255}\x0a\x20\x0d\x20\
	\x0e\x20\u{256}\x05\x20\u{259}\x0a\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\
	\x20\x03\x20\x03\x20\x05\x20\u{262}\x0a\x20\x03\x21\x03\x21\x03\x21\x05\
	\x21\u{267}\x0a\x21\x03\x22\x03\x22\x03\x22\x05\x22\u{26c}\x0a\x22\x03\x23\
	\x03\x23\x03\x23\x07\x23\u{271}\x0a\x23\x0c\x23\x0e\x23\u{274}\x0b\x23\x03\
	\x23\x05\x23\u{277}\x0a\x23\x03\x24\x03\x24\x03\x24\x07\x24\u{27c}\x0a\x24\
	\x0c\x24\x0e\x24\u{27f}\x0b\x24\x03\x25\x03\x25\x03\x25\x07\x25\u{284}\x0a\
	\x25\x0c\x25\x0e\x25\u{287}\x0b\x25\x03\x26\x03\x26\x03\x26\x03\x26\x07\
	\x26\u{28d}\x0a\x26\x0c\x26\x0e\x26\u{290}\x0b\x26\x03\x27\x03\x27\x03\x27\
	\x03\x27\x07\x27\u{296}\x0a\x27\x0c\x27\x0e\x27\u{299}\x0b\x27\x03\x28\x03\
	\x28\x03\x28\x03\x28\x05\x28\u{29f}\x0a\x28\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x05\x29\u{2ab}\x0a\
	\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{2b1}\x0a\x2a\x03\x2b\x03\
	\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x07\x2b\u{2bc}\
	\x0a\x2b\x0c\x2b\x0e\x2b\u{2bf}\x0b\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\
	\u{2c4}\x0a\x2b\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\
	\x05\x2c\u{2cd}\x0a\x2c\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x03\x2d\x03\x2d\x03\x2d\x05\x2d\u{2d8}\x0a\x2d\x03\x2e\x03\x2e\x03\x2e\
	\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x06\x2e\u{2e1}\x0a\x2e\x0d\x2e\x0e\x2e\
	\u{2e2}\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{2e8}\x0a\x2e\x03\x2e\x03\x2e\x03\
	\x2e\x05\x2e\u{2ed}\x0a\x2e\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{2f2}\x0a\x2e\
	\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x07\x2f\u{2f8}\x0a\x2f\x0c\x2f\x0e\x2f\
	\u{2fb}\x0b\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x30\x03\x30\x03\x30\x05\x30\
	\u{303}\x0a\x30\x03\x31\x03\x31\x03\x31\x03\x31\x05\x31\u{309}\x0a\x31\x05\
	\x31\u{30b}\x0a\x31\x03\x32\x03\x32\x03\x32\x03\x32\x06\x32\u{311}\x0a\x32\
	\x0d\x32\x0e\x32\u{312}\x03\x32\x03\x32\x05\x32\u{317}\x0a\x32\x03\x33\x03\
	\x33\x03\x33\x03\x33\x03\x33\x03\x33\x06\x33\u{31f}\x0a\x33\x0d\x33\x0e\
	\x33\u{320}\x03\x33\x03\x33\x03\x34\x03\x34\x03\x34\x05\x34\u{328}\x0a\x34\
	\x03\x34\x05\x34\u{32b}\x0a\x34\x03\x35\x03\x35\x06\x35\u{32f}\x0a\x35\x0d\
	\x35\x0e\x35\u{330}\x03\x35\x05\x35\u{334}\x0a\x35\x03\x36\x03\x36\x03\x36\
	\x05\x36\u{339}\x0a\x36\x03\x37\x03\x37\x03\x37\x05\x37\u{33e}\x0a\x37\x03\
	\x37\x03\x37\x03\x37\x03\x38\x03\x38\x03\x38\x03\x39\x03\x39\x05\x39\u{348}\
	\x0a\x39\x03\x3a\x03\x3a\x05\x3a\u{34c}\x0a\x3a\x03\x3b\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x07\x3c\u{355}\x0a\x3c\x0c\x3c\x0e\x3c\
	\u{358}\x0b\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x03\x3d\x05\x3d\u{362}\x0a\x3d\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\
	\x03\x3e\x03\x3e\x03\x3e\x05\x3e\u{36c}\x0a\x3e\x03\x3f\x03\x3f\x03\x3f\
	\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{376}\x0a\x3f\x03\x40\
	\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x05\x40\u{380}\
	\x0a\x40\x03\x41\x03\x41\x03\x41\x05\x41\u{385}\x0a\x41\x03\x42\x03\x42\
	\x03\x42\x05\x42\u{38a}\x0a\x42\x03\x43\x03\x43\x03\x44\x03\x44\x03\x45\
	\x03\x45\x03\x46\x03\x46\x03\x46\x03\x47\x03\x47\x03\x48\x03\x48\x03\x48\
	\x03\x49\x03\x49\x03\x49\x06\x49\u{39d}\x0a\x49\x0d\x49\x0e\x49\u{39e}\x03\
	\x4a\x03\x4a\x05\x4a\u{3a3}\x0a\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4c\x03\x4c\x05\x4c\u{3ab}\x0a\x4c\x03\x4c\x03\x4c\x03\x4c\x05\x4c\u{3b0}\
	\x0a\x4c\x03\x4c\x05\x4c\u{3b3}\x0a\x4c\x03\x4d\x03\x4d\x03\x4d\x05\x4d\
	\u{3b8}\x0a\x4d\x03\x4e\x03\x4e\x03\x4e\x07\x4e\u{3bd}\x0a\x4e\x0c\x4e\x0e\
	\x4e\u{3c0}\x0b\x4e\x03\x4e\x05\x4e\u{3c3}\x0a\x4e\x03\x4f\x03\x4f\x05\x4f\
	\u{3c7}\x0a\x4f\x03\x50\x03\x50\x03\x50\x03\x50\x05\x50\u{3cd}\x0a\x50\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x05\x51\u{3d4}\x0a\x51\x03\x51\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x05\x51\u{3dd}\x0a\x51\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x05\x51\u{3e4}\x0a\x51\x03\x51\x03\
	\x51\x05\x51\u{3e8}\x0a\x51\x03\x52\x03\x52\x03\x52\x07\x52\u{3ed}\x0a\x52\
	\x0c\x52\x0e\x52\u{3f0}\x0b\x52\x03\x53\x03\x53\x05\x53\u{3f4}\x0a\x53\x03\
	\x53\x03\x53\x03\x53\x03\x54\x03\x54\x03\x54\x03\x55\x03\x55\x03\x55\x03\
	\x55\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{404}\x0a\x55\x03\x55\x03\
	\x55\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{40c}\x0a\x55\x03\x55\x03\
	\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{416}\x0a\
	\x55\x03\x55\x03\x55\x05\x55\u{41a}\x0a\x55\x03\x56\x03\x56\x03\x56\x07\
	\x56\u{41f}\x0a\x56\x0c\x56\x0e\x56\u{422}\x0b\x56\x03\x57\x03\x57\x03\x57\
	\x07\x57\u{427}\x0a\x57\x0c\x57\x0e\x57\u{42a}\x0b\x57\x03\x58\x03\x58\x03\
	\x58\x03\x58\x03\x59\x03\x59\x03\x59\x03\x59\x03\x59\x03\x59\x05\x59\u{436}\
	\x0a\x59\x03\x59\x05\x59\u{439}\x0a\x59\x03\x5a\x03\x5a\x05\x5a\u{43d}\x0a\
	\x5a\x03\x5b\x03\x5b\x05\x5b\u{441}\x0a\x5b\x03\x5b\x03\x5b\x03\x5b\x03\
	\x5c\x03\x5c\x05\x5c\u{448}\x0a\x5c\x03\x5c\x03\x5c\x03\x5c\x03\x5d\x03\
	\x5d\x03\x5d\x07\x5d\u{450}\x0a\x5d\x0c\x5d\x0e\x5d\u{453}\x0b\x5d\x03\x5e\
	\x03\x5e\x03\x5e\x07\x5e\u{458}\x0a\x5e\x0c\x5e\x0e\x5e\u{45b}\x0b\x5e\x03\
	\x5f\x03\x5f\x03\x5f\x05\x5f\u{460}\x0a\x5f\x03\x60\x03\x60\x03\x60\x03\
	\x60\x07\x60\u{466}\x0a\x60\x0c\x60\x0e\x60\u{469}\x0b\x60\x03\x61\x03\x61\
	\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\x03\x61\
	\x03\x61\x03\x61\x05\x61\u{478}\x0a\x61\x03\x62\x03\x62\x03\x62\x03\x63\
	\x03\x63\x03\x63\x06\x63\u{480}\x0a\x63\x0d\x63\x0e\x63\u{481}\x03\x63\x05\
	\x63\u{485}\x0a\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\
	\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\
	\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x07\x63\u{49c}\x0a\x63\x0c\
	\x63\x0e\x63\u{49f}\x0b\x63\x03\x64\x05\x64\u{4a2}\x0a\x64\x03\x64\x03\x64\
	\x07\x64\u{4a6}\x0a\x64\x0c\x64\x0e\x64\u{4a9}\x0b\x64\x03\x65\x03\x65\x03\
	\x65\x05\x65\u{4ae}\x0a\x65\x03\x65\x03\x65\x03\x65\x05\x65\u{4b3}\x0a\x65\
	\x03\x65\x03\x65\x03\x65\x05\x65\u{4b8}\x0a\x65\x03\x65\x03\x65\x03\x65\
	\x03\x65\x06\x65\u{4be}\x0a\x65\x0d\x65\x0e\x65\u{4bf}\x03\x65\x03\x65\x03\
	\x65\x03\x65\x05\x65\u{4c6}\x0a\x65\x03\x66\x03\x66\x03\x67\x03\x67\x05\
	\x67\u{4cc}\x0a\x67\x03\x67\x03\x67\x03\x67\x03\x67\x05\x67\u{4d2}\x0a\x67\
	\x07\x67\u{4d4}\x0a\x67\x0c\x67\x0e\x67\u{4d7}\x0b\x67\x03\x67\x05\x67\u{4da}\
	\x0a\x67\x05\x67\u{4dc}\x0a\x67\x03\x68\x03\x68\x05\x68\u{4e0}\x0a\x68\x03\
	\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x03\x68\x05\x68\u{4e9}\x0a\
	\x68\x03\x69\x03\x69\x03\x69\x07\x69\u{4ee}\x0a\x69\x0c\x69\x0e\x69\u{4f1}\
	\x0b\x69\x03\x69\x05\x69\u{4f4}\x0a\x69\x03\x6a\x03\x6a\x05\x6a\u{4f8}\x0a\
	\x6a\x03\x6a\x03\x6a\x05\x6a\u{4fc}\x0a\x6a\x03\x6a\x05\x6a\u{4ff}\x0a\x6a\
	\x05\x6a\u{501}\x0a\x6a\x03\x6b\x03\x6b\x05\x6b\u{505}\x0a\x6b\x03\x6c\x03\
	\x6c\x05\x6c\u{509}\x0a\x6c\x03\x6c\x03\x6c\x03\x6c\x05\x6c\u{50e}\x0a\x6c\
	\x07\x6c\u{510}\x0a\x6c\x0c\x6c\x0e\x6c\u{513}\x0b\x6c\x03\x6c\x05\x6c\u{516}\
	\x0a\x6c\x03\x6d\x03\x6d\x03\x6d\x07\x6d\u{51b}\x0a\x6d\x0c\x6d\x0e\x6d\
	\u{51e}\x0b\x6d\x03\x6d\x05\x6d\u{521}\x0a\x6d\x03\x6e\x03\x6e\x03\x6e\x03\
	\x6e\x03\x6e\x03\x6e\x05\x6e\u{529}\x0a\x6e\x03\x6e\x03\x6e\x03\x6e\x03\
	\x6e\x03\x6e\x03\x6e\x03\x6e\x03\x6e\x05\x6e\u{533}\x0a\x6e\x07\x6e\u{535}\
	\x0a\x6e\x0c\x6e\x0e\x6e\u{538}\x0b\x6e\x03\x6e\x05\x6e\u{53b}\x0a\x6e\x05\
	\x6e\u{53d}\x0a\x6e\x03\x6e\x03\x6e\x05\x6e\u{541}\x0a\x6e\x03\x6e\x03\x6e\
	\x03\x6e\x03\x6e\x05\x6e\u{547}\x0a\x6e\x07\x6e\u{549}\x0a\x6e\x0c\x6e\x0e\
	\x6e\u{54c}\x0b\x6e\x03\x6e\x05\x6e\u{54f}\x0a\x6e\x05\x6e\u{551}\x0a\x6e\
	\x05\x6e\u{553}\x0a\x6e\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x05\x6f\u{559}\x0a\
	\x6f\x03\x6f\x05\x6f\u{55c}\x0a\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x70\x03\
	\x70\x03\x70\x07\x70\u{564}\x0a\x70\x0c\x70\x0e\x70\u{567}\x0b\x70\x03\x70\
	\x05\x70\u{56a}\x0a\x70\x03\x71\x03\x71\x05\x71\u{56e}\x0a\x71\x03\x71\x03\
	\x71\x03\x71\x03\x71\x03\x71\x03\x71\x03\x71\x03\x71\x05\x71\u{578}\x0a\
	\x71\x03\x72\x03\x72\x05\x72\u{57c}\x0a\x72\x03\x73\x05\x73\u{57f}\x0a\x73\
	\x03\x73\x03\x73\x03\x73\x03\x73\x03\x73\x05\x73\u{586}\x0a\x73\x03\x74\
	\x03\x74\x03\x74\x05\x74\u{58b}\x0a\x74\x03\x75\x03\x75\x03\x76\x03\x76\
	\x05\x76\u{591}\x0a\x76\x03\x77\x03\x77\x03\x77\x05\x77\u{596}\x0a\x77\x03\
	\x78\x06\x78\u{599}\x0a\x78\x0d\x78\x0e\x78\u{59a}\x03\x78\x02\x03\u{c4}\
	\x79\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\
	\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\
	\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\
	\x6c\x6e\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\
	\u{8c}\u{8e}\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\
	\u{a4}\u{a6}\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\
	\u{bc}\u{be}\u{c0}\u{c2}\u{c4}\u{c6}\u{c8}\u{ca}\u{cc}\u{ce}\u{d0}\u{d2}\
	\u{d4}\u{d6}\u{d8}\u{da}\u{dc}\u{de}\u{e0}\u{e2}\u{e4}\u{e6}\u{e8}\u{ea}\
	\u{ec}\u{ee}\x02\x09\x03\x02\x5a\x66\x03\x02\x38\x39\x04\x02\x49\x4a\x4e\
	\x4e\x05\x02\x3a\x3a\x4b\x4d\x58\x58\x03\x02\x49\x4a\x03\x02\x47\x48\x05\
	\x02\x20\x20\x2a\x2a\x2f\x2f\x02\u{634}\x02\u{f5}\x03\x02\x02\x02\x04\u{fb}\
	\x03\x02\x02\x02\x06\u{100}\x03\x02\x02\x02\x08\u{109}\x03\x02\x02\x02\x0a\
	\u{115}\x03\x02\x02\x02\x0c\u{119}\x03\x02\x02\x02\x0e\u{11f}\x03\x02\x02\
	\x02\x10\u{122}\x03\x02\x02\x02\x12\u{12c}\x03\x02\x02\x02\x14\u{183}\x03\
	\x02\x02\x02\x16\u{185}\x03\x02\x02\x02\x18\u{1db}\x03\x02\x02\x02\x1a\u{1dd}\
	\x03\x02\x02\x02\x1c\u{1e1}\x03\x02\x02\x02\x1e\u{1e3}\x03\x02\x02\x02\x20\
	\u{1f8}\x03\x02\x02\x02\x22\u{1fa}\x03\x02\x02\x02\x24\u{20d}\x03\x02\x02\
	\x02\x26\u{215}\x03\x02\x02\x02\x28\u{224}\x03\x02\x02\x02\x2a\u{226}\x03\
	\x02\x02\x02\x2c\u{229}\x03\x02\x02\x02\x2e\u{230}\x03\x02\x02\x02\x30\u{232}\
	\x03\x02\x02\x02\x32\u{234}\x03\x02\x02\x02\x34\u{236}\x03\x02\x02\x02\x36\
	\u{23a}\x03\x02\x02\x02\x38\u{23c}\x03\x02\x02\x02\x3a\u{246}\x03\x02\x02\
	\x02\x3c\u{248}\x03\x02\x02\x02\x3e\u{24b}\x03\x02\x02\x02\x40\u{263}\x03\
	\x02\x02\x02\x42\u{268}\x03\x02\x02\x02\x44\u{26d}\x03\x02\x02\x02\x46\u{278}\
	\x03\x02\x02\x02\x48\u{280}\x03\x02\x02\x02\x4a\u{288}\x03\x02\x02\x02\x4c\
	\u{291}\x03\x02\x02\x02\x4e\u{29a}\x03\x02\x02\x02\x50\u{2aa}\x03\x02\x02\
	\x02\x52\u{2ac}\x03\x02\x02\x02\x54\u{2b2}\x03\x02\x02\x02\x56\u{2c5}\x03\
	\x02\x02\x02\x58\u{2ce}\x03\x02\x02\x02\x5a\u{2d9}\x03\x02\x02\x02\x5c\u{2f3}\
	\x03\x02\x02\x02\x5e\u{2ff}\x03\x02\x02\x02\x60\u{304}\x03\x02\x02\x02\x62\
	\u{316}\x03\x02\x02\x02\x64\u{318}\x03\x02\x02\x02\x66\u{32a}\x03\x02\x02\
	\x02\x68\u{32c}\x03\x02\x02\x02\x6a\u{338}\x03\x02\x02\x02\x6c\u{33a}\x03\
	\x02\x02\x02\x6e\u{342}\x03\x02\x02\x02\x70\u{347}\x03\x02\x02\x02\x72\u{34b}\
	\x03\x02\x02\x02\x74\u{34d}\x03\x02\x02\x02\x76\u{351}\x03\x02\x02\x02\x78\
	\u{361}\x03\x02\x02\x02\x7a\u{36b}\x03\x02\x02\x02\x7c\u{375}\x03\x02\x02\
	\x02\x7e\u{37f}\x03\x02\x02\x02\u{80}\u{384}\x03\x02\x02\x02\u{82}\u{389}\
	\x03\x02\x02\x02\u{84}\u{38b}\x03\x02\x02\x02\u{86}\u{38d}\x03\x02\x02\x02\
	\u{88}\u{38f}\x03\x02\x02\x02\u{8a}\u{391}\x03\x02\x02\x02\u{8c}\u{394}\
	\x03\x02\x02\x02\u{8e}\u{396}\x03\x02\x02\x02\u{90}\u{399}\x03\x02\x02\x02\
	\u{92}\u{3a2}\x03\x02\x02\x02\u{94}\u{3a4}\x03\x02\x02\x02\u{96}\u{3b2}\
	\x03\x02\x02\x02\u{98}\u{3b4}\x03\x02\x02\x02\u{9a}\u{3b9}\x03\x02\x02\x02\
	\u{9c}\u{3c6}\x03\x02\x02\x02\u{9e}\u{3cc}\x03\x02\x02\x02\u{a0}\u{3e7}\
	\x03\x02\x02\x02\u{a2}\u{3e9}\x03\x02\x02\x02\u{a4}\u{3f3}\x03\x02\x02\x02\
	\u{a6}\u{3f8}\x03\x02\x02\x02\u{a8}\u{419}\x03\x02\x02\x02\u{aa}\u{41b}\
	\x03\x02\x02\x02\u{ac}\u{423}\x03\x02\x02\x02\u{ae}\u{42b}\x03\x02\x02\x02\
	\u{b0}\u{438}\x03\x02\x02\x02\u{b2}\u{43c}\x03\x02\x02\x02\u{b4}\u{43e}\
	\x03\x02\x02\x02\u{b6}\u{445}\x03\x02\x02\x02\u{b8}\u{44c}\x03\x02\x02\x02\
	\u{ba}\u{454}\x03\x02\x02\x02\u{bc}\u{45f}\x03\x02\x02\x02\u{be}\u{461}\
	\x03\x02\x02\x02\u{c0}\u{477}\x03\x02\x02\x02\u{c2}\u{479}\x03\x02\x02\x02\
	\u{c4}\u{484}\x03\x02\x02\x02\u{c6}\u{4a1}\x03\x02\x02\x02\u{c8}\u{4c5}\
	\x03\x02\x02\x02\u{ca}\u{4c7}\x03\x02\x02\x02\u{cc}\u{4cb}\x03\x02\x02\x02\
	\u{ce}\u{4e8}\x03\x02\x02\x02\u{d0}\u{4ea}\x03\x02\x02\x02\u{d2}\u{500}\
	\x03\x02\x02\x02\u{d4}\u{502}\x03\x02\x02\x02\u{d6}\u{508}\x03\x02\x02\x02\
	\u{d8}\u{517}\x03\x02\x02\x02\u{da}\u{552}\x03\x02\x02\x02\u{dc}\u{554}\
	\x03\x02\x02\x02\u{de}\u{560}\x03\x02\x02\x02\u{e0}\u{577}\x03\x02\x02\x02\
	\u{e2}\u{57b}\x03\x02\x02\x02\u{e4}\u{57e}\x03\x02\x02\x02\u{e6}\u{587}\
	\x03\x02\x02\x02\u{e8}\u{58c}\x03\x02\x02\x02\u{ea}\u{58e}\x03\x02\x02\x02\
	\u{ec}\u{595}\x03\x02\x02\x02\u{ee}\u{598}\x03\x02\x02\x02\u{f0}\u{f6}\x07\
	\x2e\x02\x02\u{f1}\u{f6}\x05\x1e\x10\x02\u{f2}\u{f3}\x05\x50\x29\x02\u{f3}\
	\u{f4}\x07\x2e\x02\x02\u{f4}\u{f6}\x03\x02\x02\x02\u{f5}\u{f0}\x03\x02\x02\
	\x02\u{f5}\u{f1}\x03\x02\x02\x02\u{f5}\u{f2}\x03\x02\x02\x02\u{f6}\x03\x03\
	\x02\x02\x02\u{f7}\u{fa}\x07\x2e\x02\x02\u{f8}\u{fa}\x05\x1c\x0f\x02\u{f9}\
	\u{f7}\x03\x02\x02\x02\u{f9}\u{f8}\x03\x02\x02\x02\u{fa}\u{fd}\x03\x02\x02\
	\x02\u{fb}\u{f9}\x03\x02\x02\x02\u{fb}\u{fc}\x03\x02\x02\x02\u{fc}\u{fe}\
	\x03\x02\x02\x02\u{fd}\u{fb}\x03\x02\x02\x02\u{fe}\u{ff}\x07\x02\x02\x03\
	\u{ff}\x05\x03\x02\x02\x02\u{100}\u{104}\x05\u{d8}\x6d\x02\u{101}\u{103}\
	\x07\x2e\x02\x02\u{102}\u{101}\x03\x02\x02\x02\u{103}\u{106}\x03\x02\x02\
	\x02\u{104}\u{102}\x03\x02\x02\x02\u{104}\u{105}\x03\x02\x02\x02\u{105}\
	\u{107}\x03\x02\x02\x02\u{106}\u{104}\x03\x02\x02\x02\u{107}\u{108}\x07\
	\x02\x02\x03\u{108}\x07\x03\x02\x02\x02\u{109}\u{10a}\x07\x58\x02\x02\u{10a}\
	\u{110}\x05\x48\x25\x02\u{10b}\u{10d}\x07\x3b\x02\x02\u{10c}\u{10e}\x05\
	\u{de}\x70\x02\u{10d}\u{10c}\x03\x02\x02\x02\u{10d}\u{10e}\x03\x02\x02\x02\
	\u{10e}\u{10f}\x03\x02\x02\x02\u{10f}\u{111}\x07\x3c\x02\x02\u{110}\u{10b}\
	\x03\x02\x02\x02\u{110}\u{111}\x03\x02\x02\x02\u{111}\u{112}\x03\x02\x02\
	\x02\u{112}\u{113}\x07\x2e\x02\x02\u{113}\x09\x03\x02\x02\x02\u{114}\u{116}\
	\x05\x08\x05\x02\u{115}\u{114}\x03\x02\x02\x02\u{116}\u{117}\x03\x02\x02\
	\x02\u{117}\u{115}\x03\x02\x02\x02\u{117}\u{118}\x03\x02\x02\x02\u{118}\
	\x0b\x03\x02\x02\x02\u{119}\u{11d}\x05\x0a\x06\x02\u{11a}\u{11e}\x05\u{dc}\
	\x6f\x02\u{11b}\u{11e}\x05\x10\x09\x02\u{11c}\u{11e}\x05\x0e\x08\x02\u{11d}\
	\u{11a}\x03\x02\x02\x02\u{11d}\u{11b}\x03\x02\x02\x02\u{11d}\u{11c}\x03\
	\x02\x02\x02\u{11e}\x0d\x03\x02\x02\x02\u{11f}\u{120}\x07\x0b\x02\x02\u{120}\
	\u{121}\x05\x10\x09\x02\u{121}\x0f\x03\x02\x02\x02\u{122}\u{123}\x07\x11\
	\x02\x02\u{123}\u{124}\x05\u{ca}\x66\x02\u{124}\u{127}\x05\x12\x0a\x02\u{125}\
	\u{126}\x07\x59\x02\x02\u{126}\u{128}\x05\u{b0}\x59\x02\u{127}\u{125}\x03\
	\x02\x02\x02\u{127}\u{128}\x03\x02\x02\x02\u{128}\u{129}\x03\x02\x02\x02\
	\u{129}\u{12a}\x07\x3e\x02\x02\u{12a}\u{12b}\x05\x62\x32\x02\u{12b}\x11\
	\x03\x02\x02\x02\u{12c}\u{12e}\x07\x3b\x02\x02\u{12d}\u{12f}\x05\x14\x0b\
	\x02\u{12e}\u{12d}\x03\x02\x02\x02\u{12e}\u{12f}\x03\x02\x02\x02\u{12f}\
	\u{130}\x03\x02\x02\x02\u{130}\u{131}\x07\x3c\x02\x02\u{131}\x13\x03\x02\
	\x02\x02\u{132}\u{135}\x05\x16\x0c\x02\u{133}\u{134}\x07\x41\x02\x02\u{134}\
	\u{136}\x05\u{b0}\x59\x02\u{135}\u{133}\x03\x02\x02\x02\u{135}\u{136}\x03\
	\x02\x02\x02\u{136}\u{13f}\x03\x02\x02\x02\u{137}\u{138}\x07\x3d\x02\x02\
	\u{138}\u{13b}\x05\x16\x0c\x02\u{139}\u{13a}\x07\x41\x02\x02\u{13a}\u{13c}\
	\x05\u{b0}\x59\x02\u{13b}\u{139}\x03\x02\x02\x02\u{13b}\u{13c}\x03\x02\x02\
	\x02\u{13c}\u{13e}\x03\x02\x02\x02\u{13d}\u{137}\x03\x02\x02\x02\u{13e}\
	\u{141}\x03\x02\x02\x02\u{13f}\u{13d}\x03\x02\x02\x02\u{13f}\u{140}\x03\
	\x02\x02\x02\u{140}\u{163}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\x02\
	\u{142}\u{161}\x07\x3d\x02\x02\u{143}\u{145}\x07\x3a\x02\x02\u{144}\u{146}\
	\x05\x16\x0c\x02\u{145}\u{144}\x03\x02\x02\x02\u{145}\u{146}\x03\x02\x02\
	\x02\u{146}\u{14f}\x03\x02\x02\x02\u{147}\u{148}\x07\x3d\x02\x02\u{148}\
	\u{14b}\x05\x16\x0c\x02\u{149}\u{14a}\x07\x41\x02\x02\u{14a}\u{14c}\x05\
	\u{b0}\x59\x02\u{14b}\u{149}\x03\x02\x02\x02\u{14b}\u{14c}\x03\x02\x02\x02\
	\u{14c}\u{14e}\x03\x02\x02\x02\u{14d}\u{147}\x03\x02\x02\x02\u{14e}\u{151}\
	\x03\x02\x02\x02\u{14f}\u{14d}\x03\x02\x02\x02\u{14f}\u{150}\x03\x02\x02\
	\x02\u{150}\u{15a}\x03\x02\x02\x02\u{151}\u{14f}\x03\x02\x02\x02\u{152}\
	\u{158}\x07\x3d\x02\x02\u{153}\u{154}\x07\x40\x02\x02\u{154}\u{156}\x05\
	\x16\x0c\x02\u{155}\u{157}\x07\x3d\x02\x02\u{156}\u{155}\x03\x02\x02\x02\
	\u{156}\u{157}\x03\x02\x02\x02\u{157}\u{159}\x03\x02\x02\x02\u{158}\u{153}\
	\x03\x02\x02\x02\u{158}\u{159}\x03\x02\x02\x02\u{159}\u{15b}\x03\x02\x02\
	\x02\u{15a}\u{152}\x03\x02\x02\x02\u{15a}\u{15b}\x03\x02\x02\x02\u{15b}\
	\u{162}\x03\x02\x02\x02\u{15c}\u{15d}\x07\x40\x02\x02\u{15d}\u{15f}\x05\
	\x16\x0c\x02\u{15e}\u{160}\x07\x3d\x02\x02\u{15f}\u{15e}\x03\x02\x02\x02\
	\u{15f}\u{160}\x03\x02\x02\x02\u{160}\u{162}\x03\x02\x02\x02\u{161}\u{143}\
	\x03\x02\x02\x02\u{161}\u{15c}\x03\x02\x02\x02\u{161}\u{162}\x03\x02\x02\
	\x02\u{162}\u{164}\x03\x02\x02\x02\u{163}\u{142}\x03\x02\x02\x02\u{163}\
	\u{164}\x03\x02\x02\x02\u{164}\u{184}\x03\x02\x02\x02\u{165}\u{167}\x07\
	\x3a\x02\x02\u{166}\u{168}\x05\x16\x0c\x02\u{167}\u{166}\x03\x02\x02\x02\
	\u{167}\u{168}\x03\x02\x02\x02\u{168}\u{171}\x03\x02\x02\x02\u{169}\u{16a}\
	\x07\x3d\x02\x02\u{16a}\u{16d}\x05\x16\x0c\x02\u{16b}\u{16c}\x07\x41\x02\
	\x02\u{16c}\u{16e}\x05\u{b0}\x59\x02\u{16d}\u{16b}\x03\x02\x02\x02\u{16d}\
	\u{16e}\x03\x02\x02\x02\u{16e}\u{170}\x03\x02\x02\x02\u{16f}\u{169}\x03\
	\x02\x02\x02\u{170}\u{173}\x03\x02\x02\x02\u{171}\u{16f}\x03\x02\x02\x02\
	\u{171}\u{172}\x03\x02\x02\x02\u{172}\u{17c}\x03\x02\x02\x02\u{173}\u{171}\
	\x03\x02\x02\x02\u{174}\u{17a}\x07\x3d\x02\x02\u{175}\u{176}\x07\x40\x02\
	\x02\u{176}\u{178}\x05\x16\x0c\x02\u{177}\u{179}\x07\x3d\x02\x02\u{178}\
	\u{177}\x03\x02\x02\x02\u{178}\u{179}\x03\x02\x02\x02\u{179}\u{17b}\x03\
	\x02\x02\x02\u{17a}\u{175}\x03\x02\x02\x02\u{17a}\u{17b}\x03\x02\x02\x02\
	\u{17b}\u{17d}\x03\x02\x02\x02\u{17c}\u{174}\x03\x02\x02\x02\u{17c}\u{17d}\
	\x03\x02\x02\x02\u{17d}\u{184}\x03\x02\x02\x02\u{17e}\u{17f}\x07\x40\x02\
	\x02\u{17f}\u{181}\x05\x16\x0c\x02\u{180}\u{182}\x07\x3d\x02\x02\u{181}\
	\u{180}\x03\x02\x02\x02\u{181}\u{182}\x03\x02\x02\x02\u{182}\u{184}\x03\
	\x02\x02\x02\u{183}\u{132}\x03\x02\x02\x02\u{183}\u{165}\x03\x02\x02\x02\
	\u{183}\u{17e}\x03\x02\x02\x02\u{184}\x15\x03\x02\x02\x02\u{185}\u{188}\
	\x05\u{ca}\x66\x02\u{186}\u{187}\x07\x3e\x02\x02\u{187}\u{189}\x05\u{b0}\
	\x59\x02\u{188}\u{186}\x03\x02\x02\x02\u{188}\u{189}\x03\x02\x02\x02\u{189}\
	\x17\x03\x02\x02\x02\u{18a}\u{18d}\x05\x1a\x0e\x02\u{18b}\u{18c}\x07\x41\
	\x02\x02\u{18c}\u{18e}\x05\u{b0}\x59\x02\u{18d}\u{18b}\x03\x02\x02\x02\u{18d}\
	\u{18e}\x03\x02\x02\x02\u{18e}\u{197}\x03\x02\x02\x02\u{18f}\u{190}\x07\
	\x3d\x02\x02\u{190}\u{193}\x05\x1a\x0e\x02\u{191}\u{192}\x07\x41\x02\x02\
	\u{192}\u{194}\x05\u{b0}\x59\x02\u{193}\u{191}\x03\x02\x02\x02\u{193}\u{194}\
	\x03\x02\x02\x02\u{194}\u{196}\x03\x02\x02\x02\u{195}\u{18f}\x03\x02\x02\
	\x02\u{196}\u{199}\x03\x02\x02\x02\u{197}\u{195}\x03\x02\x02\x02\u{197}\
	\u{198}\x03\x02\x02\x02\u{198}\u{1bb}\x03\x02\x02\x02\u{199}\u{197}\x03\
	\x02\x02\x02\u{19a}\u{1b9}\x07\x3d\x02\x02\u{19b}\u{19d}\x07\x3a\x02\x02\
	\u{19c}\u{19e}\x05\x1a\x0e\x02\u{19d}\u{19c}\x03\x02\x02\x02\u{19d}\u{19e}\
	\x03\x02\x02\x02\u{19e}\u{1a7}\x03\x02\x02\x02\u{19f}\u{1a0}\x07\x3d\x02\
	\x02\u{1a0}\u{1a3}\x05\x1a\x0e\x02\u{1a1}\u{1a2}\x07\x41\x02\x02\u{1a2}\
	\u{1a4}\x05\u{b0}\x59\x02\u{1a3}\u{1a1}\x03\x02\x02\x02\u{1a3}\u{1a4}\x03\
	\x02\x02\x02\u{1a4}\u{1a6}\x03\x02\x02\x02\u{1a5}\u{19f}\x03\x02\x02\x02\
	\u{1a6}\u{1a9}\x03\x02\x02\x02\u{1a7}\u{1a5}\x03\x02\x02\x02\u{1a7}\u{1a8}\
	\x03\x02\x02\x02\u{1a8}\u{1b2}\x03\x02\x02\x02\u{1a9}\u{1a7}\x03\x02\x02\
	\x02\u{1aa}\u{1b0}\x07\x3d\x02\x02\u{1ab}\u{1ac}\x07\x40\x02\x02\u{1ac}\
	\u{1ae}\x05\x1a\x0e\x02\u{1ad}\u{1af}\x07\x3d\x02\x02\u{1ae}\u{1ad}\x03\
	\x02\x02\x02\u{1ae}\u{1af}\x03\x02\x02\x02\u{1af}\u{1b1}\x03\x02\x02\x02\
	\u{1b0}\u{1ab}\x03\x02\x02\x02\u{1b0}\u{1b1}\x03\x02\x02\x02\u{1b1}\u{1b3}\
	\x03\x02\x02\x02\u{1b2}\u{1aa}\x03\x02\x02\x02\u{1b2}\u{1b3}\x03\x02\x02\
	\x02\u{1b3}\u{1ba}\x03\x02\x02\x02\u{1b4}\u{1b5}\x07\x40\x02\x02\u{1b5}\
	\u{1b7}\x05\x1a\x0e\x02\u{1b6}\u{1b8}\x07\x3d\x02\x02\u{1b7}\u{1b6}\x03\
	\x02\x02\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1ba}\x03\x02\x02\x02\
	\u{1b9}\u{19b}\x03\x02\x02\x02\u{1b9}\u{1b4}\x03\x02\x02\x02\u{1b9}\u{1ba}\
	\x03\x02\x02\x02\u{1ba}\u{1bc}\x03\x02\x02\x02\u{1bb}\u{19a}\x03\x02\x02\
	\x02\u{1bb}\u{1bc}\x03\x02\x02\x02\u{1bc}\u{1dc}\x03\x02\x02\x02\u{1bd}\
	\u{1bf}\x07\x3a\x02\x02\u{1be}\u{1c0}\x05\x1a\x0e\x02\u{1bf}\u{1be}\x03\
	\x02\x02\x02\u{1bf}\u{1c0}\x03\x02\x02\x02\u{1c0}\u{1c9}\x03\x02\x02\x02\
	\u{1c1}\u{1c2}\x07\x3d\x02\x02\u{1c2}\u{1c5}\x05\x1a\x0e\x02\u{1c3}\u{1c4}\
	\x07\x41\x02\x02\u{1c4}\u{1c6}\x05\u{b0}\x59\x02\u{1c5}\u{1c3}\x03\x02\x02\
	\x02\u{1c5}\u{1c6}\x03\x02\x02\x02\u{1c6}\u{1c8}\x03\x02\x02\x02\u{1c7}\
	\u{1c1}\x03\x02\x02\x02\u{1c8}\u{1cb}\x03\x02\x02\x02\u{1c9}\u{1c7}\x03\
	\x02\x02\x02\u{1c9}\u{1ca}\x03\x02\x02\x02\u{1ca}\u{1d4}\x03\x02\x02\x02\
	\u{1cb}\u{1c9}\x03\x02\x02\x02\u{1cc}\u{1d2}\x07\x3d\x02\x02\u{1cd}\u{1ce}\
	\x07\x40\x02\x02\u{1ce}\u{1d0}\x05\x1a\x0e\x02\u{1cf}\u{1d1}\x07\x3d\x02\
	\x02\u{1d0}\u{1cf}\x03\x02\x02\x02\u{1d0}\u{1d1}\x03\x02\x02\x02\u{1d1}\
	\u{1d3}\x03\x02\x02\x02\u{1d2}\u{1cd}\x03\x02\x02\x02\u{1d2}\u{1d3}\x03\
	\x02\x02\x02\u{1d3}\u{1d5}\x03\x02\x02\x02\u{1d4}\u{1cc}\x03\x02\x02\x02\
	\u{1d4}\u{1d5}\x03\x02\x02\x02\u{1d5}\u{1dc}\x03\x02\x02\x02\u{1d6}\u{1d7}\
	\x07\x40\x02\x02\u{1d7}\u{1d9}\x05\x1a\x0e\x02\u{1d8}\u{1da}\x07\x3d\x02\
	\x02\u{1d9}\u{1d8}\x03\x02\x02\x02\u{1d9}\u{1da}\x03\x02\x02\x02\u{1da}\
	\u{1dc}\x03\x02\x02\x02\u{1db}\u{18a}\x03\x02\x02\x02\u{1db}\u{1bd}\x03\
	\x02\x02\x02\u{1db}\u{1d6}\x03\x02\x02\x02\u{1dc}\x19\x03\x02\x02\x02\u{1dd}\
	\u{1de}\x05\u{ca}\x66\x02\u{1de}\x1b\x03\x02\x02\x02\u{1df}\u{1e2}\x05\x1e\
	\x10\x02\u{1e0}\u{1e2}\x05\x50\x29\x02\u{1e1}\u{1df}\x03\x02\x02\x02\u{1e1}\
	\u{1e0}\x03\x02\x02\x02\u{1e2}\x1d\x03\x02\x02\x02\u{1e3}\u{1e8}\x05\x20\
	\x11\x02\u{1e4}\u{1e5}\x07\x3f\x02\x02\u{1e5}\u{1e7}\x05\x20\x11\x02\u{1e6}\
	\u{1e4}\x03\x02\x02\x02\u{1e7}\u{1ea}\x03\x02\x02\x02\u{1e8}\u{1e6}\x03\
	\x02\x02\x02\u{1e8}\u{1e9}\x03\x02\x02\x02\u{1e9}\u{1ec}\x03\x02\x02\x02\
	\u{1ea}\u{1e8}\x03\x02\x02\x02\u{1eb}\u{1ed}\x07\x3f\x02\x02\u{1ec}\u{1eb}\
	\x03\x02\x02\x02\u{1ec}\u{1ed}\x03\x02\x02\x02\u{1ed}\u{1ee}\x03\x02\x02\
	\x02\u{1ee}\u{1ef}\x07\x2e\x02\x02\u{1ef}\x1f\x03\x02\x02\x02\u{1f0}\u{1f9}\
	\x05\x22\x12\x02\u{1f1}\u{1f9}\x05\x2a\x16\x02\u{1f2}\u{1f9}\x05\x2c\x17\
	\x02\u{1f3}\u{1f9}\x05\x2e\x18\x02\u{1f4}\u{1f9}\x05\x3a\x1e\x02\u{1f5}\
	\u{1f9}\x05\x4a\x26\x02\u{1f6}\u{1f9}\x05\x4c\x27\x02\u{1f7}\u{1f9}\x05\
	\x4e\x28\x02\u{1f8}\u{1f0}\x03\x02\x02\x02\u{1f8}\u{1f1}\x03\x02\x02\x02\
	\u{1f8}\u{1f2}\x03\x02\x02\x02\u{1f8}\u{1f3}\x03\x02\x02\x02\u{1f8}\u{1f4}\
	\x03\x02\x02\x02\u{1f8}\u{1f5}\x03\x02\x02\x02\u{1f8}\u{1f6}\x03\x02\x02\
	\x02\u{1f8}\u{1f7}\x03\x02\x02\x02\u{1f9}\x21\x03\x02\x02\x02\u{1fa}\u{20b}\
	\x05\x26\x14\x02\u{1fb}\u{20c}\x05\x24\x13\x02\u{1fc}\u{1ff}\x05\x28\x15\
	\x02\u{1fd}\u{200}\x05\u{ea}\x76\x02\u{1fe}\u{200}\x05\u{d8}\x6d\x02\u{1ff}\
	\u{1fd}\x03\x02\x02\x02\u{1ff}\u{1fe}\x03\x02\x02\x02\u{200}\u{20c}\x03\
	\x02\x02\x02\u{201}\u{204}\x07\x41\x02\x02\u{202}\u{205}\x05\u{ea}\x76\x02\
	\u{203}\u{205}\x05\x26\x14\x02\u{204}\u{202}\x03\x02\x02\x02\u{204}\u{203}\
	\x03\x02\x02\x02\u{205}\u{207}\x03\x02\x02\x02\u{206}\u{201}\x03\x02\x02\
	\x02\u{207}\u{20a}\x03\x02\x02\x02\u{208}\u{206}\x03\x02\x02\x02\u{208}\
	\u{209}\x03\x02\x02\x02\u{209}\u{20c}\x03\x02\x02\x02\u{20a}\u{208}\x03\
	\x02\x02\x02\u{20b}\u{1fb}\x03\x02\x02\x02\u{20b}\u{1fc}\x03\x02\x02\x02\
	\u{20b}\u{208}\x03\x02\x02\x02\u{20c}\x23\x03\x02\x02\x02\u{20d}\u{20e}\
	\x07\x3e\x02\x02\u{20e}\u{211}\x05\u{b0}\x59\x02\u{20f}\u{210}\x07\x41\x02\
	\x02\u{210}\u{212}\x05\u{b0}\x59\x02\u{211}\u{20f}\x03\x02\x02\x02\u{211}\
	\u{212}\x03\x02\x02\x02\u{212}\x25\x03\x02\x02\x02\u{213}\u{216}\x05\u{b0}\
	\x59\x02\u{214}\u{216}\x05\u{c2}\x62\x02\u{215}\u{213}\x03\x02\x02\x02\u{215}\
	\u{214}\x03\x02\x02\x02\u{216}\u{21e}\x03\x02\x02\x02\u{217}\u{21a}\x07\
	\x3d\x02\x02\u{218}\u{21b}\x05\u{b0}\x59\x02\u{219}\u{21b}\x05\u{c2}\x62\
	\x02\u{21a}\u{218}\x03\x02\x02\x02\u{21a}\u{219}\x03\x02\x02\x02\u{21b}\
	\u{21d}\x03\x02\x02\x02\u{21c}\u{217}\x03\x02\x02\x02\u{21d}\u{220}\x03\
	\x02\x02\x02\u{21e}\u{21c}\x03\x02\x02\x02\u{21e}\u{21f}\x03\x02\x02\x02\
	\u{21f}\u{222}\x03\x02\x02\x02\u{220}\u{21e}\x03\x02\x02\x02\u{221}\u{223}\
	\x07\x3d\x02\x02\u{222}\u{221}\x03\x02\x02\x02\u{222}\u{223}\x03\x02\x02\
	\x02\u{223}\x27\x03\x02\x02\x02\u{224}\u{225}\x09\x02\x02\x02\u{225}\x29\
	\x03\x02\x02\x02\u{226}\u{227}\x07\x12\x02\x02\u{227}\u{228}\x05\u{d6}\x6c\
	\x02\u{228}\x2b\x03\x02\x02\x02\u{229}\u{22a}\x07\x25\x02\x02\u{22a}\x2d\
	\x03\x02\x02\x02\u{22b}\u{231}\x05\x30\x19\x02\u{22c}\u{231}\x05\x32\x1a\
	\x02\u{22d}\u{231}\x05\x34\x1b\x02\u{22e}\u{231}\x05\x38\x1d\x02\u{22f}\
	\u{231}\x05\x36\x1c\x02\u{230}\u{22b}\x03\x02\x02\x02\u{230}\u{22c}\x03\
	\x02\x02\x02\u{230}\u{22d}\x03\x02\x02\x02\u{230}\u{22e}\x03\x02\x02\x02\
	\u{230}\u{22f}\x03\x02\x02\x02\u{231}\x2f\x03\x02\x02\x02\u{232}\u{233}\
	\x07\x0d\x02\x02\u{233}\x31\x03\x02\x02\x02\u{234}\u{235}\x07\x10\x02\x02\
	\u{235}\x33\x03\x02\x02\x02\u{236}\u{238}\x07\x27\x02\x02\u{237}\u{239}\
	\x05\u{d8}\x6d\x02\u{238}\u{237}\x03\x02\x02\x02\u{238}\u{239}\x03\x02\x02\
	\x02\u{239}\x35\x03\x02\x02\x02\u{23a}\u{23b}\x05\u{ea}\x76\x02\u{23b}\x37\
	\x03\x02\x02\x02\u{23c}\u{242}\x07\x26\x02\x02\u{23d}\u{240}\x05\u{b0}\x59\
	\x02\u{23e}\u{23f}\x07\x19\x02\x02\u{23f}\u{241}\x05\u{b0}\x59\x02\u{240}\
	\u{23e}\x03\x02\x02\x02\u{240}\u{241}\x03\x02\x02\x02\u{241}\u{243}\x03\
	\x02\x02\x02\u{242}\u{23d}\x03\x02\x02\x02\u{242}\u{243}\x03\x02\x02\x02\
	\u{243}\x39\x03\x02\x02\x02\u{244}\u{247}\x05\x3c\x1f\x02\u{245}\u{247}\
	\x05\x3e\x20\x02\u{246}\u{244}\x03\x02\x02\x02\u{246}\u{245}\x03\x02\x02\
	\x02\u{247}\x3b\x03\x02\x02\x02\u{248}\u{249}\x07\x1c\x02\x02\u{249}\u{24a}\
	\x05\x46\x24\x02\u{24a}\x3d\x03\x02\x02\x02\u{24b}\u{258}\x07\x19\x02\x02\
	\u{24c}\u{24e}\x09\x03\x02\x02\u{24d}\u{24c}\x03\x02\x02\x02\u{24e}\u{251}\
	\x03\x02\x02\x02\u{24f}\u{24d}\x03\x02\x02\x02\u{24f}\u{250}\x03\x02\x02\
	\x02\u{250}\u{252}\x03\x02\x02\x02\u{251}\u{24f}\x03\x02\x02\x02\u{252}\
	\u{259}\x05\x48\x25\x02\u{253}\u{255}\x09\x03\x02\x02\u{254}\u{253}\x03\
	\x02\x02\x02\u{255}\u{256}\x03\x02\x02\x02\u{256}\u{254}\x03\x02\x02\x02\
	\u{256}\u{257}\x03\x02\x02\x02\u{257}\u{259}\x03\x02\x02\x02\u{258}\u{24f}\
	\x03\x02\x02\x02\u{258}\u{254}\x03\x02\x02\x02\u{259}\u{25a}\x03\x02\x02\
	\x02\u{25a}\u{261}\x07\x1c\x02\x02\u{25b}\u{262}\x07\x3a\x02\x02\u{25c}\
	\u{25d}\x07\x3b\x02\x02\u{25d}\u{25e}\x05\x44\x23\x02\u{25e}\u{25f}\x07\
	\x3c\x02\x02\u{25f}\u{262}\x03\x02\x02\x02\u{260}\u{262}\x05\x44\x23\x02\
	\u{261}\u{25b}\x03\x02\x02\x02\u{261}\u{25c}\x03\x02\x02\x02\u{261}\u{260}\
	\x03\x02\x02\x02\u{262}\x3f\x03\x02\x02\x02\u{263}\u{266}\x05\u{ca}\x66\
	\x02\u{264}\u{265}\x07\x09\x02\x02\u{265}\u{267}\x05\u{ca}\x66\x02\u{266}\
	\u{264}\x03\x02\x02\x02\u{266}\u{267}\x03\x02\x02\x02\u{267}\x41\x03\x02\
	\x02\x02\u{268}\u{26b}\x05\x48\x25\x02\u{269}\u{26a}\x07\x09\x02\x02\u{26a}\
	\u{26c}\x05\u{ca}\x66\x02\u{26b}\u{269}\x03\x02\x02\x02\u{26b}\u{26c}\x03\
	\x02\x02\x02\u{26c}\x43\x03\x02\x02\x02\u{26d}\u{272}\x05\x40\x21\x02\u{26e}\
	\u{26f}\x07\x3d\x02\x02\u{26f}\u{271}\x05\x40\x21\x02\u{270}\u{26e}\x03\
	\x02\x02\x02\u{271}\u{274}\x03\x02\x02\x02\u{272}\u{270}\x03\x02\x02\x02\
	\u{272}\u{273}\x03\x02\x02\x02\u{273}\u{276}\x03\x02\x02\x02\u{274}\u{272}\
	\x03\x02\x02\x02\u{275}\u{277}\x07\x3d\x02\x02\u{276}\u{275}\x03\x02\x02\
	\x02\u{276}\u{277}\x03\x02\x02\x02\u{277}\x45\x03\x02\x02\x02\u{278}\u{27d}\
	\x05\x42\x22\x02\u{279}\u{27a}\x07\x3d\x02\x02\u{27a}\u{27c}\x05\x42\x22\
	\x02\u{27b}\u{279}\x03\x02\x02\x02\u{27c}\u{27f}\x03\x02\x02\x02\u{27d}\
	\u{27b}\x03\x02\x02\x02\u{27d}\u{27e}\x03\x02\x02\x02\u{27e}\x47\x03\x02\
	\x02\x02\u{27f}\u{27d}\x03\x02\x02\x02\u{280}\u{285}\x05\u{ca}\x66\x02\u{281}\
	\u{282}\x07\x38\x02\x02\u{282}\u{284}\x05\u{ca}\x66\x02\u{283}\u{281}\x03\
	\x02\x02\x02\u{284}\u{287}\x03\x02\x02\x02\u{285}\u{283}\x03\x02\x02\x02\
	\u{285}\u{286}\x03\x02\x02\x02\u{286}\x49\x03\x02\x02\x02\u{287}\u{285}\
	\x03\x02\x02\x02\u{288}\u{289}\x07\x1a\x02\x02\u{289}\u{28e}\x05\u{ca}\x66\
	\x02\u{28a}\u{28b}\x07\x3d\x02\x02\u{28b}\u{28d}\x05\u{ca}\x66\x02\u{28c}\
	\u{28a}\x03\x02\x02\x02\u{28d}\u{290}\x03\x02\x02\x02\u{28e}\u{28c}\x03\
	\x02\x02\x02\u{28e}\u{28f}\x03\x02\x02\x02\u{28f}\x4b\x03\x02\x02\x02\u{290}\
	\u{28e}\x03\x02\x02\x02\u{291}\u{292}\x07\x22\x02\x02\u{292}\u{297}\x05\
	\u{ca}\x66\x02\u{293}\u{294}\x07\x3d\x02\x02\u{294}\u{296}\x05\u{ca}\x66\
	\x02\u{295}\u{293}\x03\x02\x02\x02\u{296}\u{299}\x03\x02\x02\x02\u{297}\
	\u{295}\x03\x02\x02\x02\u{297}\u{298}\x03\x02\x02\x02\u{298}\x4d\x03\x02\
	\x02\x02\u{299}\u{297}\x03\x02\x02\x02\u{29a}\u{29b}\x07\x0a\x02\x02\u{29b}\
	\u{29e}\x05\u{b0}\x59\x02\u{29c}\u{29d}\x07\x3d\x02\x02\u{29d}\u{29f}\x05\
	\u{b0}\x59\x02\u{29e}\u{29c}\x03\x02\x02\x02\u{29e}\u{29f}\x03\x02\x02\x02\
	\u{29f}\x4f\x03\x02\x02\x02\u{2a0}\u{2ab}\x05\x54\x2b\x02\u{2a1}\u{2ab}\
	\x05\x56\x2c\x02\u{2a2}\u{2ab}\x05\x58\x2d\x02\u{2a3}\u{2ab}\x05\x5a\x2e\
	\x02\u{2a4}\u{2ab}\x05\x5c\x2f\x02\u{2a5}\u{2ab}\x05\x10\x09\x02\u{2a6}\
	\u{2ab}\x05\u{dc}\x6f\x02\u{2a7}\u{2ab}\x05\x0c\x07\x02\u{2a8}\u{2ab}\x05\
	\x52\x2a\x02\u{2a9}\u{2ab}\x05\x64\x33\x02\u{2aa}\u{2a0}\x03\x02\x02\x02\
	\u{2aa}\u{2a1}\x03\x02\x02\x02\u{2aa}\u{2a2}\x03\x02\x02\x02\u{2aa}\u{2a3}\
	\x03\x02\x02\x02\u{2aa}\u{2a4}\x03\x02\x02\x02\u{2aa}\u{2a5}\x03\x02\x02\
	\x02\u{2aa}\u{2a6}\x03\x02\x02\x02\u{2aa}\u{2a7}\x03\x02\x02\x02\u{2aa}\
	\u{2a8}\x03\x02\x02\x02\u{2aa}\u{2a9}\x03\x02\x02\x02\u{2ab}\x51\x03\x02\
	\x02\x02\u{2ac}\u{2b0}\x07\x0b\x02\x02\u{2ad}\u{2b1}\x05\x10\x09\x02\u{2ae}\
	\u{2b1}\x05\x5c\x2f\x02\u{2af}\u{2b1}\x05\x58\x2d\x02\u{2b0}\u{2ad}\x03\
	\x02\x02\x02\u{2b0}\u{2ae}\x03\x02\x02\x02\u{2b0}\u{2af}\x03\x02\x02\x02\
	\u{2b1}\x53\x03\x02\x02\x02\u{2b2}\u{2b3}\x07\x1b\x02\x02\u{2b3}\u{2b4}\
	\x05\u{b0}\x59\x02\u{2b4}\u{2b5}\x07\x3e\x02\x02\u{2b5}\u{2bd}\x05\x62\x32\
	\x02\u{2b6}\u{2b7}\x07\x13\x02\x02\u{2b7}\u{2b8}\x05\u{b0}\x59\x02\u{2b8}\
	\u{2b9}\x07\x3e\x02\x02\u{2b9}\u{2ba}\x05\x62\x32\x02\u{2ba}\u{2bc}\x03\
	\x02\x02\x02\u{2bb}\u{2b6}\x03\x02\x02\x02\u{2bc}\u{2bf}\x03\x02\x02\x02\
	\u{2bd}\u{2bb}\x03\x02\x02\x02\u{2bd}\u{2be}\x03\x02\x02\x02\u{2be}\u{2c3}\
	\x03\x02\x02\x02\u{2bf}\u{2bd}\x03\x02\x02\x02\u{2c0}\u{2c1}\x07\x14\x02\
	\x02\u{2c1}\u{2c2}\x07\x3e\x02\x02\u{2c2}\u{2c4}\x05\x62\x32\x02\u{2c3}\
	\u{2c0}\x03\x02\x02\x02\u{2c3}\u{2c4}\x03\x02\x02\x02\u{2c4}\x55\x03\x02\
	\x02\x02\u{2c5}\u{2c6}\x07\x2b\x02\x02\u{2c6}\u{2c7}\x05\u{b0}\x59\x02\u{2c7}\
	\u{2c8}\x07\x3e\x02\x02\u{2c8}\u{2cc}\x05\x62\x32\x02\u{2c9}\u{2ca}\x07\
	\x14\x02\x02\u{2ca}\u{2cb}\x07\x3e\x02\x02\u{2cb}\u{2cd}\x05\x62\x32\x02\
	\u{2cc}\u{2c9}\x03\x02\x02\x02\u{2cc}\u{2cd}\x03\x02\x02\x02\u{2cd}\x57\
	\x03\x02\x02\x02\u{2ce}\u{2cf}\x07\x18\x02\x02\u{2cf}\u{2d0}\x05\u{d6}\x6c\
	\x02\u{2d0}\u{2d1}\x07\x1d\x02\x02\u{2d1}\u{2d2}\x05\u{d8}\x6d\x02\u{2d2}\
	\u{2d3}\x07\x3e\x02\x02\u{2d3}\u{2d7}\x05\x62\x32\x02\u{2d4}\u{2d5}\x07\
	\x14\x02\x02\u{2d5}\u{2d6}\x07\x3e\x02\x02\u{2d6}\u{2d8}\x05\x62\x32\x02\
	\u{2d7}\u{2d4}\x03\x02\x02\x02\u{2d7}\u{2d8}\x03\x02\x02\x02\u{2d8}\x59\
	\x03\x02\x02\x02\u{2d9}\u{2da}\x07\x29\x02\x02\u{2da}\u{2db}\x07\x3e\x02\
	\x02\u{2db}\u{2f1}\x05\x62\x32\x02\u{2dc}\u{2dd}\x05\x60\x31\x02\u{2dd}\
	\u{2de}\x07\x3e\x02\x02\u{2de}\u{2df}\x05\x62\x32\x02\u{2df}\u{2e1}\x03\
	\x02\x02\x02\u{2e0}\u{2dc}\x03\x02\x02\x02\u{2e1}\u{2e2}\x03\x02\x02\x02\
	\u{2e2}\u{2e0}\x03\x02\x02\x02\u{2e2}\u{2e3}\x03\x02\x02\x02\u{2e3}\u{2e7}\
	\x03\x02\x02\x02\u{2e4}\u{2e5}\x07\x14\x02\x02\u{2e5}\u{2e6}\x07\x3e\x02\
	\x02\u{2e6}\u{2e8}\x05\x62\x32\x02\u{2e7}\u{2e4}\x03\x02\x02\x02\u{2e7}\
	\u{2e8}\x03\x02\x02\x02\u{2e8}\u{2ec}\x03\x02\x02\x02\u{2e9}\u{2ea}\x07\
	\x17\x02\x02\u{2ea}\u{2eb}\x07\x3e\x02\x02\u{2eb}\u{2ed}\x05\x62\x32\x02\
	\u{2ec}\u{2e9}\x03\x02\x02\x02\u{2ec}\u{2ed}\x03\x02\x02\x02\u{2ed}\u{2f2}\
	\x03\x02\x02\x02\u{2ee}\u{2ef}\x07\x17\x02\x02\u{2ef}\u{2f0}\x07\x3e\x02\
	\x02\u{2f0}\u{2f2}\x05\x62\x32\x02\u{2f1}\u{2e0}\x03\x02\x02\x02\u{2f1}\
	\u{2ee}\x03\x02\x02\x02\u{2f2}\x5b\x03\x02\x02\x02\u{2f3}\u{2f4}\x07\x2c\
	\x02\x02\u{2f4}\u{2f9}\x05\x5e\x30\x02\u{2f5}\u{2f6}\x07\x3d\x02\x02\u{2f6}\
	\u{2f8}\x05\x5e\x30\x02\u{2f7}\u{2f5}\x03\x02\x02\x02\u{2f8}\u{2fb}\x03\
	\x02\x02\x02\u{2f9}\u{2f7}\x03\x02\x02\x02\u{2f9}\u{2fa}\x03\x02\x02\x02\
	\u{2fa}\u{2fc}\x03\x02\x02\x02\u{2fb}\u{2f9}\x03\x02\x02\x02\u{2fc}\u{2fd}\
	\x07\x3e\x02\x02\u{2fd}\u{2fe}\x05\x62\x32\x02\u{2fe}\x5d\x03\x02\x02\x02\
	\u{2ff}\u{302}\x05\u{b0}\x59\x02\u{300}\u{301}\x07\x09\x02\x02\u{301}\u{303}\
	\x05\u{c4}\x63\x02\u{302}\u{300}\x03\x02\x02\x02\u{302}\u{303}\x03\x02\x02\
	\x02\u{303}\x5f\x03\x02\x02\x02\u{304}\u{30a}\x07\x15\x02\x02\u{305}\u{308}\
	\x05\u{b0}\x59\x02\u{306}\u{307}\x07\x09\x02\x02\u{307}\u{309}\x05\u{ca}\
	\x66\x02\u{308}\u{306}\x03\x02\x02\x02\u{308}\u{309}\x03\x02\x02\x02\u{309}\
	\u{30b}\x03\x02\x02\x02\u{30a}\u{305}\x03\x02\x02\x02\u{30a}\u{30b}\x03\
	\x02\x02\x02\u{30b}\x61\x03\x02\x02\x02\u{30c}\u{317}\x05\x1e\x10\x02\u{30d}\
	\u{30e}\x07\x2e\x02\x02\u{30e}\u{310}\x07\x03\x02\x02\u{30f}\u{311}\x05\
	\x1c\x0f\x02\u{310}\u{30f}\x03\x02\x02\x02\u{311}\u{312}\x03\x02\x02\x02\
	\u{312}\u{310}\x03\x02\x02\x02\u{312}\u{313}\x03\x02\x02\x02\u{313}\u{314}\
	\x03\x02\x02\x02\u{314}\u{315}\x07\x04\x02\x02\u{315}\u{317}\x03\x02\x02\
	\x02\u{316}\u{30c}\x03\x02\x02\x02\u{316}\u{30d}\x03\x02\x02\x02\u{317}\
	\x63\x03\x02\x02\x02\u{318}\u{319}\x07\x20\x02\x02\u{319}\u{31a}\x05\x66\
	\x34\x02\u{31a}\u{31b}\x07\x3e\x02\x02\u{31b}\u{31c}\x07\x2e\x02\x02\u{31c}\
	\u{31e}\x07\x03\x02\x02\u{31d}\u{31f}\x05\x6c\x37\x02\u{31e}\u{31d}\x03\
	\x02\x02\x02\u{31f}\u{320}\x03\x02\x02\x02\u{320}\u{31e}\x03\x02\x02\x02\
	\u{320}\u{321}\x03\x02\x02\x02\u{321}\u{322}\x03\x02\x02\x02\u{322}\u{323}\
	\x07\x04\x02\x02\u{323}\x65\x03\x02\x02\x02\u{324}\u{325}\x05\x6a\x36\x02\
	\u{325}\u{327}\x07\x3d\x02\x02\u{326}\u{328}\x05\x68\x35\x02\u{327}\u{326}\
	\x03\x02\x02\x02\u{327}\u{328}\x03\x02\x02\x02\u{328}\u{32b}\x03\x02\x02\
	\x02\u{329}\u{32b}\x05\u{b0}\x59\x02\u{32a}\u{324}\x03\x02\x02\x02\u{32a}\
	\u{329}\x03\x02\x02\x02\u{32b}\x67\x03\x02\x02\x02\u{32c}\u{32e}\x07\x3d\
	\x02\x02\u{32d}\u{32f}\x05\x6a\x36\x02\u{32e}\u{32d}\x03\x02\x02\x02\u{32f}\
	\u{330}\x03\x02\x02\x02\u{330}\u{32e}\x03\x02\x02\x02\u{330}\u{331}\x03\
	\x02\x02\x02\u{331}\u{333}\x03\x02\x02\x02\u{332}\u{334}\x07\x3d\x02\x02\
	\u{333}\u{332}\x03\x02\x02\x02\u{333}\u{334}\x03\x02\x02\x02\u{334}\x69\
	\x03\x02\x02\x02\u{335}\u{336}\x07\x3a\x02\x02\u{336}\u{339}\x05\u{c4}\x63\
	\x02\u{337}\u{339}\x05\u{b0}\x59\x02\u{338}\u{335}\x03\x02\x02\x02\u{338}\
	\u{337}\x03\x02\x02\x02\u{339}\x6b\x03\x02\x02\x02\u{33a}\u{33b}\x07\x0e\
	\x02\x02\u{33b}\u{33d}\x05\x70\x39\x02\u{33c}\u{33e}\x05\x6e\x38\x02\u{33d}\
	\u{33c}\x03\x02\x02\x02\u{33d}\u{33e}\x03\x02\x02\x02\u{33e}\u{33f}\x03\
	\x02\x02\x02\u{33f}\u{340}\x07\x3e\x02\x02\u{340}\u{341}\x05\x62\x32\x02\
	\u{341}\x6d\x03\x02\x02\x02\u{342}\u{343}\x07\x1b\x02\x02\u{343}\u{344}\
	\x05\u{b0}\x59\x02\u{344}\x6f\x03\x02\x02\x02\u{345}\u{348}\x05\u{98}\x4d\
	\x02\u{346}\u{348}\x05\x72\x3a\x02\u{347}\u{345}\x03\x02\x02\x02\u{347}\
	\u{346}\x03\x02\x02\x02\u{348}\x71\x03\x02\x02\x02\u{349}\u{34c}\x05\x74\
	\x3b\x02\u{34a}\u{34c}\x05\x76\x3c\x02\u{34b}\u{349}\x03\x02\x02\x02\u{34b}\
	\u{34a}\x03\x02\x02\x02\u{34c}\x73\x03\x02\x02\x02\u{34d}\u{34e}\x05\x76\
	\x3c\x02\u{34e}\u{34f}\x07\x09\x02\x02\u{34f}\u{350}\x05\u{8a}\x46\x02\u{350}\
	\x75\x03\x02\x02\x02\u{351}\u{356}\x05\x78\x3d\x02\u{352}\u{353}\x07\x44\
	\x02\x02\u{353}\u{355}\x05\x78\x3d\x02\u{354}\u{352}\x03\x02\x02\x02\u{355}\
	\u{358}\x03\x02\x02\x02\u{356}\u{354}\x03\x02\x02\x02\u{356}\u{357}\x03\
	\x02\x02\x02\u{357}\x77\x03\x02\x02\x02\u{358}\u{356}\x03\x02\x02\x02\u{359}\
	\u{362}\x05\x7a\x3e\x02\u{35a}\u{362}\x05\u{88}\x45\x02\u{35b}\u{362}\x05\
	\u{8c}\x47\x02\u{35c}\u{362}\x05\u{8e}\x48\x02\u{35d}\u{362}\x05\u{94}\x4b\
	\x02\u{35e}\u{362}\x05\u{96}\x4c\x02\u{35f}\u{362}\x05\u{a0}\x51\x02\u{360}\
	\u{362}\x05\u{a8}\x55\x02\u{361}\u{359}\x03\x02\x02\x02\u{361}\u{35a}\x03\
	\x02\x02\x02\u{361}\u{35b}\x03\x02\x02\x02\u{361}\u{35c}\x03\x02\x02\x02\
	\u{361}\u{35d}\x03\x02\x02\x02\u{361}\u{35e}\x03\x02\x02\x02\u{361}\u{35f}\
	\x03\x02\x02\x02\u{361}\u{360}\x03\x02\x02\x02\u{362}\x79\x03\x02\x02\x02\
	\u{363}\u{364}\x05\u{80}\x41\x02\u{364}\u{365}\x06\x3e\x02\x02\u{365}\u{36c}\
	\x03\x02\x02\x02\u{366}\u{36c}\x05\x7e\x40\x02\u{367}\u{36c}\x05\u{ee}\x78\
	\x02\u{368}\u{36c}\x07\x21\x02\x02\u{369}\u{36c}\x07\x28\x02\x02\u{36a}\
	\u{36c}\x07\x16\x02\x02\u{36b}\u{363}\x03\x02\x02\x02\u{36b}\u{366}\x03\
	\x02\x02\x02\u{36b}\u{367}\x03\x02\x02\x02\u{36b}\u{368}\x03\x02\x02\x02\
	\u{36b}\u{369}\x03\x02\x02\x02\u{36b}\u{36a}\x03\x02\x02\x02\u{36c}\x7b\
	\x03\x02\x02\x02\u{36d}\u{36e}\x05\u{80}\x41\x02\u{36e}\u{36f}\x06\x3f\x03\
	\x02\u{36f}\u{376}\x03\x02\x02\x02\u{370}\u{376}\x05\x7e\x40\x02\u{371}\
	\u{376}\x05\u{ee}\x78\x02\u{372}\u{376}\x07\x21\x02\x02\u{373}\u{376}\x07\
	\x28\x02\x02\u{374}\u{376}\x07\x16\x02\x02\u{375}\u{36d}\x03\x02\x02\x02\
	\u{375}\u{370}\x03\x02\x02\x02\u{375}\u{371}\x03\x02\x02\x02\u{375}\u{372}\
	\x03\x02\x02\x02\u{375}\u{373}\x03\x02\x02\x02\u{375}\u{374}\x03\x02\x02\
	\x02\u{376}\x7d\x03\x02\x02\x02\u{377}\u{378}\x05\u{82}\x42\x02\u{378}\u{379}\
	\x07\x49\x02\x02\u{379}\u{37a}\x05\u{86}\x44\x02\u{37a}\u{380}\x03\x02\x02\
	\x02\u{37b}\u{37c}\x05\u{82}\x42\x02\u{37c}\u{37d}\x07\x4a\x02\x02\u{37d}\
	\u{37e}\x05\u{86}\x44\x02\u{37e}\u{380}\x03\x02\x02\x02\u{37f}\u{377}\x03\
	\x02\x02\x02\u{37f}\u{37b}\x03\x02\x02\x02\u{380}\x7f\x03\x02\x02\x02\u{381}\
	\u{385}\x07\x06\x02\x02\u{382}\u{383}\x07\x4a\x02\x02\u{383}\u{385}\x07\
	\x06\x02\x02\u{384}\u{381}\x03\x02\x02\x02\u{384}\u{382}\x03\x02\x02\x02\
	\u{385}\u{81}\x03\x02\x02\x02\u{386}\u{38a}\x05\u{84}\x43\x02\u{387}\u{388}\
	\x07\x4a\x02\x02\u{388}\u{38a}\x05\u{84}\x43\x02\u{389}\u{386}\x03\x02\x02\
	\x02\u{389}\u{387}\x03\x02\x02\x02\u{38a}\u{83}\x03\x02\x02\x02\u{38b}\u{38c}\
	\x07\x06\x02\x02\u{38c}\u{85}\x03\x02\x02\x02\u{38d}\u{38e}\x07\x06\x02\
	\x02\u{38e}\u{87}\x03\x02\x02\x02\u{38f}\u{390}\x05\u{8a}\x46\x02\u{390}\
	\u{89}\x03\x02\x02\x02\u{391}\u{392}\x05\u{ca}\x66\x02\u{392}\u{393}\x06\
	\x46\x04\x02\u{393}\u{8b}\x03\x02\x02\x02\u{394}\u{395}\x07\x2a\x02\x02\
	\u{395}\u{8d}\x03\x02\x02\x02\u{396}\u{397}\x05\u{90}\x49\x02\u{397}\u{398}\
	\x06\x48\x05\x02\u{398}\u{8f}\x03\x02\x02\x02\u{399}\u{39c}\x05\u{ca}\x66\
	\x02\u{39a}\u{39b}\x07\x38\x02\x02\u{39b}\u{39d}\x05\u{ca}\x66\x02\u{39c}\
	\u{39a}\x03\x02\x02\x02\u{39d}\u{39e}\x03\x02\x02\x02\u{39e}\u{39c}\x03\
	\x02\x02\x02\u{39e}\u{39f}\x03\x02\x02\x02\u{39f}\u{91}\x03\x02\x02\x02\
	\u{3a0}\u{3a3}\x05\u{90}\x49\x02\u{3a1}\u{3a3}\x05\u{ca}\x66\x02\u{3a2}\
	\u{3a0}\x03\x02\x02\x02\u{3a2}\u{3a1}\x03\x02\x02\x02\u{3a3}\u{93}\x03\x02\
	\x02\x02\u{3a4}\u{3a5}\x07\x3b\x02\x02\u{3a5}\u{3a6}\x05\x72\x3a\x02\u{3a6}\
	\u{3a7}\x07\x3c\x02\x02\u{3a7}\u{95}\x03\x02\x02\x02\u{3a8}\u{3aa}\x07\x42\
	\x02\x02\u{3a9}\u{3ab}\x05\u{9a}\x4e\x02\u{3aa}\u{3a9}\x03\x02\x02\x02\u{3aa}\
	\u{3ab}\x03\x02\x02\x02\u{3ab}\u{3ac}\x03\x02\x02\x02\u{3ac}\u{3b3}\x07\
	\x43\x02\x02\u{3ad}\u{3af}\x07\x3b\x02\x02\u{3ae}\u{3b0}\x05\u{98}\x4d\x02\
	\u{3af}\u{3ae}\x03\x02\x02\x02\u{3af}\u{3b0}\x03\x02\x02\x02\u{3b0}\u{3b1}\
	\x03\x02\x02\x02\u{3b1}\u{3b3}\x07\x3c\x02\x02\u{3b2}\u{3a8}\x03\x02\x02\
	\x02\u{3b2}\u{3ad}\x03\x02\x02\x02\u{3b3}\u{97}\x03\x02\x02\x02\u{3b4}\u{3b5}\
	\x05\u{9c}\x4f\x02\u{3b5}\u{3b7}\x07\x3d\x02\x02\u{3b6}\u{3b8}\x05\u{9a}\
	\x4e\x02\u{3b7}\u{3b6}\x03\x02\x02\x02\u{3b7}\u{3b8}\x03\x02\x02\x02\u{3b8}\
	\u{99}\x03\x02\x02\x02\u{3b9}\u{3be}\x05\u{9c}\x4f\x02\u{3ba}\u{3bb}\x07\
	\x3d\x02\x02\u{3bb}\u{3bd}\x05\u{9c}\x4f\x02\u{3bc}\u{3ba}\x03\x02\x02\x02\
	\u{3bd}\u{3c0}\x03\x02\x02\x02\u{3be}\u{3bc}\x03\x02\x02\x02\u{3be}\u{3bf}\
	\x03\x02\x02\x02\u{3bf}\u{3c2}\x03\x02\x02\x02\u{3c0}\u{3be}\x03\x02\x02\
	\x02\u{3c1}\u{3c3}\x07\x3d\x02\x02\u{3c2}\u{3c1}\x03\x02\x02\x02\u{3c2}\
	\u{3c3}\x03\x02\x02\x02\u{3c3}\u{9b}\x03\x02\x02\x02\u{3c4}\u{3c7}\x05\u{9e}\
	\x50\x02\u{3c5}\u{3c7}\x05\x72\x3a\x02\u{3c6}\u{3c4}\x03\x02\x02\x02\u{3c6}\
	\u{3c5}\x03\x02\x02\x02\u{3c7}\u{9d}\x03\x02\x02\x02\u{3c8}\u{3c9}\x07\x3a\
	\x02\x02\u{3c9}\u{3cd}\x05\u{8a}\x46\x02\u{3ca}\u{3cb}\x07\x3a\x02\x02\u{3cb}\
	\u{3cd}\x05\u{8c}\x47\x02\u{3cc}\u{3c8}\x03\x02\x02\x02\u{3cc}\u{3ca}\x03\
	\x02\x02\x02\u{3cd}\u{9f}\x03\x02\x02\x02\u{3ce}\u{3cf}\x07\x4f\x02\x02\
	\u{3cf}\u{3e8}\x07\x50\x02\x02\u{3d0}\u{3d1}\x07\x4f\x02\x02\u{3d1}\u{3d3}\
	\x05\u{a6}\x54\x02\u{3d2}\u{3d4}\x07\x3d\x02\x02\u{3d3}\u{3d2}\x03\x02\x02\
	\x02\u{3d3}\u{3d4}\x03\x02\x02\x02\u{3d4}\u{3d5}\x03\x02\x02\x02\u{3d5}\
	\u{3d6}\x07\x50\x02\x02\u{3d6}\u{3e8}\x03\x02\x02\x02\u{3d7}\u{3d8}\x07\
	\x4f\x02\x02\u{3d8}\u{3d9}\x05\u{a2}\x52\x02\u{3d9}\u{3da}\x07\x3d\x02\x02\
	\u{3da}\u{3dc}\x05\u{a6}\x54\x02\u{3db}\u{3dd}\x07\x3d\x02\x02\u{3dc}\u{3db}\
	\x03\x02\x02\x02\u{3dc}\u{3dd}\x03\x02\x02\x02\u{3dd}\u{3de}\x03\x02\x02\
	\x02\u{3de}\u{3df}\x07\x50\x02\x02\u{3df}\u{3e8}\x03\x02\x02\x02\u{3e0}\
	\u{3e1}\x07\x4f\x02\x02\u{3e1}\u{3e3}\x05\u{a2}\x52\x02\u{3e2}\u{3e4}\x07\
	\x3d\x02\x02\u{3e3}\u{3e2}\x03\x02\x02\x02\u{3e3}\u{3e4}\x03\x02\x02\x02\
	\u{3e4}\u{3e5}\x03\x02\x02\x02\u{3e5}\u{3e6}\x07\x50\x02\x02\u{3e6}\u{3e8}\
	\x03\x02\x02\x02\u{3e7}\u{3ce}\x03\x02\x02\x02\u{3e7}\u{3d0}\x03\x02\x02\
	\x02\u{3e7}\u{3d7}\x03\x02\x02\x02\u{3e7}\u{3e0}\x03\x02\x02\x02\u{3e8}\
	\u{a1}\x03\x02\x02\x02\u{3e9}\u{3ee}\x05\u{a4}\x53\x02\u{3ea}\u{3eb}\x07\
	\x3d\x02\x02\u{3eb}\u{3ed}\x05\u{a4}\x53\x02\u{3ec}\u{3ea}\x03\x02\x02\x02\
	\u{3ed}\u{3f0}\x03\x02\x02\x02\u{3ee}\u{3ec}\x03\x02\x02\x02\u{3ee}\u{3ef}\
	\x03\x02\x02\x02\u{3ef}\u{a3}\x03\x02\x02\x02\u{3f0}\u{3ee}\x03\x02\x02\
	\x02\u{3f1}\u{3f4}\x05\x7c\x3f\x02\u{3f2}\u{3f4}\x05\u{90}\x49\x02\u{3f3}\
	\u{3f1}\x03\x02\x02\x02\u{3f3}\u{3f2}\x03\x02\x02\x02\u{3f4}\u{3f5}\x03\
	\x02\x02\x02\u{3f5}\u{3f6}\x07\x3e\x02\x02\u{3f6}\u{3f7}\x05\x72\x3a\x02\
	\u{3f7}\u{a5}\x03\x02\x02\x02\u{3f8}\u{3f9}\x07\x40\x02\x02\u{3f9}\u{3fa}\
	\x05\u{8a}\x46\x02\u{3fa}\u{a7}\x03\x02\x02\x02\u{3fb}\u{3fc}\x05\u{92}\
	\x4a\x02\u{3fc}\u{3fd}\x07\x3b\x02\x02\u{3fd}\u{3fe}\x07\x3c\x02\x02\u{3fe}\
	\u{41a}\x03\x02\x02\x02\u{3ff}\u{400}\x05\u{92}\x4a\x02\u{400}\u{401}\x07\
	\x3b\x02\x02\u{401}\u{403}\x05\u{aa}\x56\x02\u{402}\u{404}\x07\x3d\x02\x02\
	\u{403}\u{402}\x03\x02\x02\x02\u{403}\u{404}\x03\x02\x02\x02\u{404}\u{405}\
	\x03\x02\x02\x02\u{405}\u{406}\x07\x3c\x02\x02\u{406}\u{41a}\x03\x02\x02\
	\x02\u{407}\u{408}\x05\u{92}\x4a\x02\u{408}\u{409}\x07\x3b\x02\x02\u{409}\
	\u{40b}\x05\u{ac}\x57\x02\u{40a}\u{40c}\x07\x3d\x02\x02\u{40b}\u{40a}\x03\
	\x02\x02\x02\u{40b}\u{40c}\x03\x02\x02\x02\u{40c}\u{40d}\x03\x02\x02\x02\
	\u{40d}\u{40e}\x07\x3c\x02\x02\u{40e}\u{41a}\x03\x02\x02\x02\u{40f}\u{410}\
	\x05\u{92}\x4a\x02\u{410}\u{411}\x07\x3b\x02\x02\u{411}\u{412}\x05\u{aa}\
	\x56\x02\u{412}\u{413}\x07\x3d\x02\x02\u{413}\u{415}\x05\u{ac}\x57\x02\u{414}\
	\u{416}\x07\x3d\x02\x02\u{415}\u{414}\x03\x02\x02\x02\u{415}\u{416}\x03\
	\x02\x02\x02\u{416}\u{417}\x03\x02\x02\x02\u{417}\u{418}\x07\x3c\x02\x02\
	\u{418}\u{41a}\x03\x02\x02\x02\u{419}\u{3fb}\x03\x02\x02\x02\u{419}\u{3ff}\
	\x03\x02\x02\x02\u{419}\u{407}\x03\x02\x02\x02\u{419}\u{40f}\x03\x02\x02\
	\x02\u{41a}\u{a9}\x03\x02\x02\x02\u{41b}\u{420}\x05\x72\x3a\x02\u{41c}\u{41d}\
	\x07\x3d\x02\x02\u{41d}\u{41f}\x05\x72\x3a\x02\u{41e}\u{41c}\x03\x02\x02\
	\x02\u{41f}\u{422}\x03\x02\x02\x02\u{420}\u{41e}\x03\x02\x02\x02\u{420}\
	\u{421}\x03\x02\x02\x02\u{421}\u{ab}\x03\x02\x02\x02\u{422}\u{420}\x03\x02\
	\x02\x02\u{423}\u{428}\x05\u{ae}\x58\x02\u{424}\u{425}\x07\x3d\x02\x02\u{425}\
	\u{427}\x05\u{ae}\x58\x02\u{426}\u{424}\x03\x02\x02\x02\u{427}\u{42a}\x03\
	\x02\x02\x02\u{428}\u{426}\x03\x02\x02\x02\u{428}\u{429}\x03\x02\x02\x02\
	\u{429}\u{ad}\x03\x02\x02\x02\u{42a}\u{428}\x03\x02\x02\x02\u{42b}\u{42c}\
	\x05\u{ca}\x66\x02\u{42c}\u{42d}\x07\x41\x02\x02\u{42d}\u{42e}\x05\x72\x3a\
	\x02\u{42e}\u{af}\x03\x02\x02\x02\u{42f}\u{435}\x05\u{b8}\x5d\x02\u{430}\
	\u{431}\x07\x1b\x02\x02\u{431}\u{432}\x05\u{b8}\x5d\x02\u{432}\u{433}\x07\
	\x14\x02\x02\u{433}\u{434}\x05\u{b0}\x59\x02\u{434}\u{436}\x03\x02\x02\x02\
	\u{435}\u{430}\x03\x02\x02\x02\u{435}\u{436}\x03\x02\x02\x02\u{436}\u{439}\
	\x03\x02\x02\x02\u{437}\u{439}\x05\u{b4}\x5b\x02\u{438}\u{42f}\x03\x02\x02\
	\x02\u{438}\u{437}\x03\x02\x02\x02\u{439}\u{b1}\x03\x02\x02\x02\u{43a}\u{43d}\
	\x05\u{b8}\x5d\x02\u{43b}\u{43d}\x05\u{b6}\x5c\x02\u{43c}\u{43a}\x03\x02\
	\x02\x02\u{43c}\u{43b}\x03\x02\x02\x02\u{43d}\u{b3}\x03\x02\x02\x02\u{43e}\
	\u{440}\x07\x1f\x02\x02\u{43f}\u{441}\x05\x18\x0d\x02\u{440}\u{43f}\x03\
	\x02\x02\x02\u{440}\u{441}\x03\x02\x02\x02\u{441}\u{442}\x03\x02\x02\x02\
	\u{442}\u{443}\x07\x3e\x02\x02\u{443}\u{444}\x05\u{b0}\x59\x02\u{444}\u{b5}\
	\x03\x02\x02\x02\u{445}\u{447}\x07\x1f\x02\x02\u{446}\u{448}\x05\x18\x0d\
	\x02\u{447}\u{446}\x03\x02\x02\x02\u{447}\u{448}\x03\x02\x02\x02\u{448}\
	\u{449}\x03\x02\x02\x02\u{449}\u{44a}\x07\x3e\x02\x02\u{44a}\u{44b}\x05\
	\u{b2}\x5a\x02\u{44b}\u{b7}\x03\x02\x02\x02\u{44c}\u{451}\x05\u{ba}\x5e\
	\x02\u{44d}\u{44e}\x07\x24\x02\x02\u{44e}\u{450}\x05\u{ba}\x5e\x02\u{44f}\
	\u{44d}\x03\x02\x02\x02\u{450}\u{453}\x03\x02\x02\x02\u{451}\u{44f}\x03\
	\x02\x02\x02\u{451}\u{452}\x03\x02\x02\x02\u{452}\u{b9}\x03\x02\x02\x02\
	\u{453}\u{451}\x03\x02\x02\x02\u{454}\u{459}\x05\u{bc}\x5f\x02\u{455}\u{456}\
	\x07\x08\x02\x02\u{456}\u{458}\x05\u{bc}\x5f\x02\u{457}\u{455}\x03\x02\x02\
	\x02\u{458}\u{45b}\x03\x02\x02\x02\u{459}\u{457}\x03\x02\x02\x02\u{459}\
	\u{45a}\x03\x02\x02\x02\u{45a}\u{bb}\x03\x02\x02\x02\u{45b}\u{459}\x03\x02\
	\x02\x02\u{45c}\u{45d}\x07\x23\x02\x02\u{45d}\u{460}\x05\u{bc}\x5f\x02\u{45e}\
	\u{460}\x05\u{be}\x60\x02\u{45f}\u{45c}\x03\x02\x02\x02\u{45f}\u{45e}\x03\
	\x02\x02\x02\u{460}\u{bd}\x03\x02\x02\x02\u{461}\u{467}\x05\u{c4}\x63\x02\
	\u{462}\u{463}\x05\u{c0}\x61\x02\u{463}\u{464}\x05\u{c4}\x63\x02\u{464}\
	\u{466}\x03\x02\x02\x02\u{465}\u{462}\x03\x02\x02\x02\u{466}\u{469}\x03\
	\x02\x02\x02\u{467}\u{465}\x03\x02\x02\x02\u{467}\u{468}\x03\x02\x02\x02\
	\u{468}\u{bf}\x03\x02\x02\x02\u{469}\u{467}\x03\x02\x02\x02\u{46a}\u{478}\
	\x07\x51\x02\x02\u{46b}\u{478}\x07\x52\x02\x02\u{46c}\u{478}\x07\x53\x02\
	\x02\u{46d}\u{478}\x07\x54\x02\x02\u{46e}\u{478}\x07\x55\x02\x02\u{46f}\
	\u{478}\x07\x56\x02\x02\u{470}\u{478}\x07\x57\x02\x02\u{471}\u{478}\x07\
	\x1d\x02\x02\u{472}\u{473}\x07\x23\x02\x02\u{473}\u{478}\x07\x1d\x02\x02\
	\u{474}\u{478}\x07\x1e\x02\x02\u{475}\u{476}\x07\x1e\x02\x02\u{476}\u{478}\
	\x07\x23\x02\x02\u{477}\u{46a}\x03\x02\x02\x02\u{477}\u{46b}\x03\x02\x02\
	\x02\u{477}\u{46c}\x03\x02\x02\x02\u{477}\u{46d}\x03\x02\x02\x02\u{477}\
	\u{46e}\x03\x02\x02\x02\u{477}\u{46f}\x03\x02\x02\x02\u{477}\u{470}\x03\
	\x02\x02\x02\u{477}\u{471}\x03\x02\x02\x02\u{477}\u{472}\x03\x02\x02\x02\
	\u{477}\u{474}\x03\x02\x02\x02\u{477}\u{475}\x03\x02\x02\x02\u{478}\u{c1}\
	\x03\x02\x02\x02\u{479}\u{47a}\x07\x3a\x02\x02\u{47a}\u{47b}\x05\u{c4}\x63\
	\x02\u{47b}\u{c3}\x03\x02\x02\x02\u{47c}\u{47d}\x08\x63\x01\x02\u{47d}\u{485}\
	\x05\u{c6}\x64\x02\u{47e}\u{480}\x09\x04\x02\x02\u{47f}\u{47e}\x03\x02\x02\
	\x02\u{480}\u{481}\x03\x02\x02\x02\u{481}\u{47f}\x03\x02\x02\x02\u{481}\
	\u{482}\x03\x02\x02\x02\u{482}\u{483}\x03\x02\x02\x02\u{483}\u{485}\x05\
	\u{c4}\x63\x09\u{484}\u{47c}\x03\x02\x02\x02\u{484}\u{47f}\x03\x02\x02\x02\
	\u{485}\u{49d}\x03\x02\x02\x02\u{486}\u{487}\x0c\x0a\x02\x02\u{487}\u{488}\
	\x07\x40\x02\x02\u{488}\u{49c}\x05\u{c4}\x63\x0b\u{489}\u{48a}\x0c\x08\x02\
	\x02\u{48a}\u{48b}\x09\x05\x02\x02\u{48b}\u{49c}\x05\u{c4}\x63\x09\u{48c}\
	\u{48d}\x0c\x07\x02\x02\u{48d}\u{48e}\x09\x06\x02\x02\u{48e}\u{49c}\x05\
	\u{c4}\x63\x08\u{48f}\u{490}\x0c\x06\x02\x02\u{490}\u{491}\x09\x07\x02\x02\
	\u{491}\u{49c}\x05\u{c4}\x63\x07\u{492}\u{493}\x0c\x05\x02\x02\u{493}\u{494}\
	\x07\x46\x02\x02\u{494}\u{49c}\x05\u{c4}\x63\x06\u{495}\u{496}\x0c\x04\x02\
	\x02\u{496}\u{497}\x07\x45\x02\x02\u{497}\u{49c}\x05\u{c4}\x63\x05\u{498}\
	\u{499}\x0c\x03\x02\x02\u{499}\u{49a}\x07\x44\x02\x02\u{49a}\u{49c}\x05\
	\u{c4}\x63\x04\u{49b}\u{486}\x03\x02\x02\x02\u{49b}\u{489}\x03\x02\x02\x02\
	\u{49b}\u{48c}\x03\x02\x02\x02\u{49b}\u{48f}\x03\x02\x02\x02\u{49b}\u{492}\
	\x03\x02\x02\x02\u{49b}\u{495}\x03\x02\x02\x02\u{49b}\u{498}\x03\x02\x02\
	\x02\u{49c}\u{49f}\x03\x02\x02\x02\u{49d}\u{49b}\x03\x02\x02\x02\u{49d}\
	\u{49e}\x03\x02\x02\x02\u{49e}\u{c5}\x03\x02\x02\x02\u{49f}\u{49d}\x03\x02\
	\x02\x02\u{4a0}\u{4a2}\x07\x0c\x02\x02\u{4a1}\u{4a0}\x03\x02\x02\x02\u{4a1}\
	\u{4a2}\x03\x02\x02\x02\u{4a2}\u{4a3}\x03\x02\x02\x02\u{4a3}\u{4a7}\x05\
	\u{c8}\x65\x02\u{4a4}\u{4a6}\x05\u{ce}\x68\x02\u{4a5}\u{4a4}\x03\x02\x02\
	\x02\u{4a6}\u{4a9}\x03\x02\x02\x02\u{4a7}\u{4a5}\x03\x02\x02\x02\u{4a7}\
	\u{4a8}\x03\x02\x02\x02\u{4a8}\u{c7}\x03\x02\x02\x02\u{4a9}\u{4a7}\x03\x02\
	\x02\x02\u{4aa}\u{4ad}\x07\x3b\x02\x02\u{4ab}\u{4ae}\x05\u{ea}\x76\x02\u{4ac}\
	\u{4ae}\x05\u{cc}\x67\x02\u{4ad}\u{4ab}\x03\x02\x02\x02\u{4ad}\u{4ac}\x03\
	\x02\x02\x02\u{4ad}\u{4ae}\x03\x02\x02\x02\u{4ae}\u{4af}\x03\x02\x02\x02\
	\u{4af}\u{4c6}\x07\x3c\x02\x02\u{4b0}\u{4b2}\x07\x42\x02\x02\u{4b1}\u{4b3}\
	\x05\u{cc}\x67\x02\u{4b2}\u{4b1}\x03\x02\x02\x02\u{4b2}\u{4b3}\x03\x02\x02\
	\x02\u{4b3}\u{4b4}\x03\x02\x02\x02\u{4b4}\u{4c6}\x07\x43\x02\x02\u{4b5}\
	\u{4b7}\x07\x4f\x02\x02\u{4b6}\u{4b8}\x05\u{da}\x6e\x02\u{4b7}\u{4b6}\x03\
	\x02\x02\x02\u{4b7}\u{4b8}\x03\x02\x02\x02\u{4b8}\u{4b9}\x03\x02\x02\x02\
	\u{4b9}\u{4c6}\x07\x50\x02\x02\u{4ba}\u{4c6}\x05\u{ca}\x66\x02\u{4bb}\u{4c6}\
	\x07\x06\x02\x02\u{4bc}\u{4be}\x07\x05\x02\x02\u{4bd}\u{4bc}\x03\x02\x02\
	\x02\u{4be}\u{4bf}\x03\x02\x02\x02\u{4bf}\u{4bd}\x03\x02\x02\x02\u{4bf}\
	\u{4c0}\x03\x02\x02\x02\u{4c0}\u{4c6}\x03\x02\x02\x02\u{4c1}\u{4c6}\x07\
	\x39\x02\x02\u{4c2}\u{4c6}\x07\x21\x02\x02\u{4c3}\u{4c6}\x07\x28\x02\x02\
	\u{4c4}\u{4c6}\x07\x16\x02\x02\u{4c5}\u{4aa}\x03\x02\x02\x02\u{4c5}\u{4b0}\
	\x03\x02\x02\x02\u{4c5}\u{4b5}\x03\x02\x02\x02\u{4c5}\u{4ba}\x03\x02\x02\
	\x02\u{4c5}\u{4bb}\x03\x02\x02\x02\u{4c5}\u{4bd}\x03\x02\x02\x02\u{4c5}\
	\u{4c1}\x03\x02\x02\x02\u{4c5}\u{4c2}\x03\x02\x02\x02\u{4c5}\u{4c3}\x03\
	\x02\x02\x02\u{4c5}\u{4c4}\x03\x02\x02\x02\u{4c6}\u{c9}\x03\x02\x02\x02\
	\u{4c7}\u{4c8}\x09\x08\x02\x02\u{4c8}\u{cb}\x03\x02\x02\x02\u{4c9}\u{4cc}\
	\x05\u{b0}\x59\x02\u{4ca}\u{4cc}\x05\u{c2}\x62\x02\u{4cb}\u{4c9}\x03\x02\
	\x02\x02\u{4cb}\u{4ca}\x03\x02\x02\x02\u{4cc}\u{4db}\x03\x02\x02\x02\u{4cd}\
	\u{4dc}\x05\u{e4}\x73\x02\u{4ce}\u{4d1}\x07\x3d\x02\x02\u{4cf}\u{4d2}\x05\
	\u{b0}\x59\x02\u{4d0}\u{4d2}\x05\u{c2}\x62\x02\u{4d1}\u{4cf}\x03\x02\x02\
	\x02\u{4d1}\u{4d0}\x03\x02\x02\x02\u{4d2}\u{4d4}\x03\x02\x02\x02\u{4d3}\
	\u{4ce}\x03\x02\x02\x02\u{4d4}\u{4d7}\x03\x02\x02\x02\u{4d5}\u{4d3}\x03\
	\x02\x02\x02\u{4d5}\u{4d6}\x03\x02\x02\x02\u{4d6}\u{4d9}\x03\x02\x02\x02\
	\u{4d7}\u{4d5}\x03\x02\x02\x02\u{4d8}\u{4da}\x07\x3d\x02\x02\u{4d9}\u{4d8}\
	\x03\x02\x02\x02\u{4d9}\u{4da}\x03\x02\x02\x02\u{4da}\u{4dc}\x03\x02\x02\
	\x02\u{4db}\u{4cd}\x03\x02\x02\x02\u{4db}\u{4d5}\x03\x02\x02\x02\u{4dc}\
	\u{cd}\x03\x02\x02\x02\u{4dd}\u{4df}\x07\x3b\x02\x02\u{4de}\u{4e0}\x05\u{de}\
	\x70\x02\u{4df}\u{4de}\x03\x02\x02\x02\u{4df}\u{4e0}\x03\x02\x02\x02\u{4e0}\
	\u{4e1}\x03\x02\x02\x02\u{4e1}\u{4e9}\x07\x3c\x02\x02\u{4e2}\u{4e3}\x07\
	\x42\x02\x02\u{4e3}\u{4e4}\x05\u{d0}\x69\x02\u{4e4}\u{4e5}\x07\x43\x02\x02\
	\u{4e5}\u{4e9}\x03\x02\x02\x02\u{4e6}\u{4e7}\x07\x38\x02\x02\u{4e7}\u{4e9}\
	\x05\u{ca}\x66\x02\u{4e8}\u{4dd}\x03\x02\x02\x02\u{4e8}\u{4e2}\x03\x02\x02\
	\x02\u{4e8}\u{4e6}\x03\x02\x02\x02\u{4e9}\u{cf}\x03\x02\x02\x02\u{4ea}\u{4ef}\
	\x05\u{d2}\x6a\x02\u{4eb}\u{4ec}\x07\x3d\x02\x02\u{4ec}\u{4ee}\x05\u{d2}\
	\x6a\x02\u{4ed}\u{4eb}\x03\x02\x02\x02\u{4ee}\u{4f1}\x03\x02\x02\x02\u{4ef}\
	\u{4ed}\x03\x02\x02\x02\u{4ef}\u{4f0}\x03\x02\x02\x02\u{4f0}\u{4f3}\x03\
	\x02\x02\x02\u{4f1}\u{4ef}\x03\x02\x02\x02\u{4f2}\u{4f4}\x07\x3d\x02\x02\
	\u{4f3}\u{4f2}\x03\x02\x02\x02\u{4f3}\u{4f4}\x03\x02\x02\x02\u{4f4}\u{d1}\
	\x03\x02\x02\x02\u{4f5}\u{501}\x05\u{b0}\x59\x02\u{4f6}\u{4f8}\x05\u{b0}\
	\x59\x02\u{4f7}\u{4f6}\x03\x02\x02\x02\u{4f7}\u{4f8}\x03\x02\x02\x02\u{4f8}\
	\u{4f9}\x03\x02\x02\x02\u{4f9}\u{4fb}\x07\x3e\x02\x02\u{4fa}\u{4fc}\x05\
	\u{b0}\x59\x02\u{4fb}\u{4fa}\x03\x02\x02\x02\u{4fb}\u{4fc}\x03\x02\x02\x02\
	\u{4fc}\u{4fe}\x03\x02\x02\x02\u{4fd}\u{4ff}\x05\u{d4}\x6b\x02\u{4fe}\u{4fd}\
	\x03\x02\x02\x02\u{4fe}\u{4ff}\x03\x02\x02\x02\u{4ff}\u{501}\x03\x02\x02\
	\x02\u{500}\u{4f5}\x03\x02\x02\x02\u{500}\u{4f7}\x03\x02\x02\x02\u{501}\
	\u{d3}\x03\x02\x02\x02\u{502}\u{504}\x07\x3e\x02\x02\u{503}\u{505}\x05\u{b0}\
	\x59\x02\u{504}\u{503}\x03\x02\x02\x02\u{504}\u{505}\x03\x02\x02\x02\u{505}\
	\u{d5}\x03\x02\x02\x02\u{506}\u{509}\x05\u{c4}\x63\x02\u{507}\u{509}\x05\
	\u{c2}\x62\x02\u{508}\u{506}\x03\x02\x02\x02\u{508}\u{507}\x03\x02\x02\x02\
	\u{509}\u{511}\x03\x02\x02\x02\u{50a}\u{50d}\x07\x3d\x02\x02\u{50b}\u{50e}\
	\x05\u{c4}\x63\x02\u{50c}\u{50e}\x05\u{c2}\x62\x02\u{50d}\u{50b}\x03\x02\
	\x02\x02\u{50d}\u{50c}\x03\x02\x02\x02\u{50e}\u{510}\x03\x02\x02\x02\u{50f}\
	\u{50a}\x03\x02\x02\x02\u{510}\u{513}\x03\x02\x02\x02\u{511}\u{50f}\x03\
	\x02\x02\x02\u{511}\u{512}\x03\x02\x02\x02\u{512}\u{515}\x03\x02\x02\x02\
	\u{513}\u{511}\x03\x02\x02\x02\u{514}\u{516}\x07\x3d\x02\x02\u{515}\u{514}\
	\x03\x02\x02\x02\u{515}\u{516}\x03\x02\x02\x02\u{516}\u{d7}\x03\x02\x02\
	\x02\u{517}\u{51c}\x05\u{b0}\x59\x02\u{518}\u{519}\x07\x3d\x02\x02\u{519}\
	\u{51b}\x05\u{b0}\x59\x02\u{51a}\u{518}\x03\x02\x02\x02\u{51b}\u{51e}\x03\
	\x02\x02\x02\u{51c}\u{51a}\x03\x02\x02\x02\u{51c}\u{51d}\x03\x02\x02\x02\
	\u{51d}\u{520}\x03\x02\x02\x02\u{51e}\u{51c}\x03\x02\x02\x02\u{51f}\u{521}\
	\x07\x3d\x02\x02\u{520}\u{51f}\x03\x02\x02\x02\u{520}\u{521}\x03\x02\x02\
	\x02\u{521}\u{d9}\x03\x02\x02\x02\u{522}\u{523}\x05\u{b0}\x59\x02\u{523}\
	\u{524}\x07\x3e\x02\x02\u{524}\u{525}\x05\u{b0}\x59\x02\u{525}\u{529}\x03\
	\x02\x02\x02\u{526}\u{527}\x07\x40\x02\x02\u{527}\u{529}\x05\u{c4}\x63\x02\
	\u{528}\u{522}\x03\x02\x02\x02\u{528}\u{526}\x03\x02\x02\x02\u{529}\u{53c}\
	\x03\x02\x02\x02\u{52a}\u{53d}\x05\u{e4}\x73\x02\u{52b}\u{532}\x07\x3d\x02\
	\x02\u{52c}\u{52d}\x05\u{b0}\x59\x02\u{52d}\u{52e}\x07\x3e\x02\x02\u{52e}\
	\u{52f}\x05\u{b0}\x59\x02\u{52f}\u{533}\x03\x02\x02\x02\u{530}\u{531}\x07\
	\x40\x02\x02\u{531}\u{533}\x05\u{c4}\x63\x02\u{532}\u{52c}\x03\x02\x02\x02\
	\u{532}\u{530}\x03\x02\x02\x02\u{533}\u{535}\x03\x02\x02\x02\u{534}\u{52b}\
	\x03\x02\x02\x02\u{535}\u{538}\x03\x02\x02\x02\u{536}\u{534}\x03\x02\x02\
	\x02\u{536}\u{537}\x03\x02\x02\x02\u{537}\u{53a}\x03\x02\x02\x02\u{538}\
	\u{536}\x03\x02\x02\x02\u{539}\u{53b}\x07\x3d\x02\x02\u{53a}\u{539}\x03\
	\x02\x02\x02\u{53a}\u{53b}\x03\x02\x02\x02\u{53b}\u{53d}\x03\x02\x02\x02\
	\u{53c}\u{52a}\x03\x02\x02\x02\u{53c}\u{536}\x03\x02\x02\x02\u{53d}\u{553}\
	\x03\x02\x02\x02\u{53e}\u{541}\x05\u{b0}\x59\x02\u{53f}\u{541}\x05\u{c2}\
	\x62\x02\u{540}\u{53e}\x03\x02\x02\x02\u{540}\u{53f}\x03\x02\x02\x02\u{541}\
	\u{550}\x03\x02\x02\x02\u{542}\u{551}\x05\u{e4}\x73\x02\u{543}\u{546}\x07\
	\x3d\x02\x02\u{544}\u{547}\x05\u{b0}\x59\x02\u{545}\u{547}\x05\u{c2}\x62\
	\x02\u{546}\u{544}\x03\x02\x02\x02\u{546}\u{545}\x03\x02\x02\x02\u{547}\
	\u{549}\x03\x02\x02\x02\u{548}\u{543}\x03\x02\x02\x02\u{549}\u{54c}\x03\
	\x02\x02\x02\u{54a}\u{548}\x03\x02\x02\x02\u{54a}\u{54b}\x03\x02\x02\x02\
	\u{54b}\u{54e}\x03\x02\x02\x02\u{54c}\u{54a}\x03\x02\x02\x02\u{54d}\u{54f}\
	\x07\x3d\x02\x02\u{54e}\u{54d}\x03\x02\x02\x02\u{54e}\u{54f}\x03\x02\x02\
	\x02\u{54f}\u{551}\x03\x02\x02\x02\u{550}\u{542}\x03\x02\x02\x02\u{550}\
	\u{54a}\x03\x02\x02\x02\u{551}\u{553}\x03\x02\x02\x02\u{552}\u{528}\x03\
	\x02\x02\x02\u{552}\u{540}\x03\x02\x02\x02\u{553}\u{db}\x03\x02\x02\x02\
	\u{554}\u{555}\x07\x0f\x02\x02\u{555}\u{55b}\x05\u{ca}\x66\x02\u{556}\u{558}\
	\x07\x3b\x02\x02\u{557}\u{559}\x05\u{de}\x70\x02\u{558}\u{557}\x03\x02\x02\
	\x02\u{558}\u{559}\x03\x02\x02\x02\u{559}\u{55a}\x03\x02\x02\x02\u{55a}\
	\u{55c}\x07\x3c\x02\x02\u{55b}\u{556}\x03\x02\x02\x02\u{55b}\u{55c}\x03\
	\x02\x02\x02\u{55c}\u{55d}\x03\x02\x02\x02\u{55d}\u{55e}\x07\x3e\x02\x02\
	\u{55e}\u{55f}\x05\x62\x32\x02\u{55f}\u{dd}\x03\x02\x02\x02\u{560}\u{565}\
	\x05\u{e0}\x71\x02\u{561}\u{562}\x07\x3d\x02\x02\u{562}\u{564}\x05\u{e0}\
	\x71\x02\u{563}\u{561}\x03\x02\x02\x02\u{564}\u{567}\x03\x02\x02\x02\u{565}\
	\u{563}\x03\x02\x02\x02\u{565}\u{566}\x03\x02\x02\x02\u{566}\u{569}\x03\
	\x02\x02\x02\u{567}\u{565}\x03\x02\x02\x02\u{568}\u{56a}\x07\x3d\x02\x02\
	\u{569}\u{568}\x03\x02\x02\x02\u{569}\u{56a}\x03\x02\x02\x02\u{56a}\u{df}\
	\x03\x02\x02\x02\u{56b}\u{56d}\x05\u{b0}\x59\x02\u{56c}\u{56e}\x05\u{e4}\
	\x73\x02\u{56d}\u{56c}\x03\x02\x02\x02\u{56d}\u{56e}\x03\x02\x02\x02\u{56e}\
	\u{578}\x03\x02\x02\x02\u{56f}\u{570}\x05\u{b0}\x59\x02\u{570}\u{571}\x07\
	\x41\x02\x02\u{571}\u{572}\x05\u{b0}\x59\x02\u{572}\u{578}\x03\x02\x02\x02\
	\u{573}\u{574}\x07\x40\x02\x02\u{574}\u{578}\x05\u{b0}\x59\x02\u{575}\u{576}\
	\x07\x3a\x02\x02\u{576}\u{578}\x05\u{b0}\x59\x02\u{577}\u{56b}\x03\x02\x02\
	\x02\u{577}\u{56f}\x03\x02\x02\x02\u{577}\u{573}\x03\x02\x02\x02\u{577}\
	\u{575}\x03\x02\x02\x02\u{578}\u{e1}\x03\x02\x02\x02\u{579}\u{57c}\x05\u{e4}\
	\x73\x02\u{57a}\u{57c}\x05\u{e6}\x74\x02\u{57b}\u{579}\x03\x02\x02\x02\u{57b}\
	\u{57a}\x03\x02\x02\x02\u{57c}\u{e3}\x03\x02\x02\x02\u{57d}\u{57f}\x07\x0b\
	\x02\x02\u{57e}\u{57d}\x03\x02\x02\x02\u{57e}\u{57f}\x03\x02\x02\x02\u{57f}\
	\u{580}\x03\x02\x02\x02\u{580}\u{581}\x07\x18\x02\x02\u{581}\u{582}\x05\
	\u{d6}\x6c\x02\u{582}\u{583}\x07\x1d\x02\x02\u{583}\u{585}\x05\u{b8}\x5d\
	\x02\u{584}\u{586}\x05\u{e2}\x72\x02\u{585}\u{584}\x03\x02\x02\x02\u{585}\
	\u{586}\x03\x02\x02\x02\u{586}\u{e5}\x03\x02\x02\x02\u{587}\u{588}\x07\x1b\
	\x02\x02\u{588}\u{58a}\x05\u{b2}\x5a\x02\u{589}\u{58b}\x05\u{e2}\x72\x02\
	\u{58a}\u{589}\x03\x02\x02\x02\u{58a}\u{58b}\x03\x02\x02\x02\u{58b}\u{e7}\
	\x03\x02\x02\x02\u{58c}\u{58d}\x05\u{ca}\x66\x02\u{58d}\u{e9}\x03\x02\x02\
	\x02\u{58e}\u{590}\x07\x2d\x02\x02\u{58f}\u{591}\x05\u{ec}\x77\x02\u{590}\
	\u{58f}\x03\x02\x02\x02\u{590}\u{591}\x03\x02\x02\x02\u{591}\u{eb}\x03\x02\
	\x02\x02\u{592}\u{593}\x07\x19\x02\x02\u{593}\u{596}\x05\u{b0}\x59\x02\u{594}\
	\u{596}\x05\u{d8}\x6d\x02\u{595}\u{592}\x03\x02\x02\x02\u{595}\u{594}\x03\
	\x02\x02\x02\u{596}\u{ed}\x03\x02\x02\x02\u{597}\u{599}\x07\x05\x02\x02\
	\u{598}\u{597}\x03\x02\x02\x02\u{599}\u{59a}\x03\x02\x02\x02\u{59a}\u{598}\
	\x03\x02\x02\x02\u{59a}\u{59b}\x03\x02\x02\x02\u{59b}\u{ef}\x03\x02\x02\
	\x02\u{cb}\u{f5}\u{f9}\u{fb}\u{104}\u{10d}\u{110}\u{117}\u{11d}\u{127}\u{12e}\
	\u{135}\u{13b}\u{13f}\u{145}\u{14b}\u{14f}\u{156}\u{158}\u{15a}\u{15f}\u{161}\
	\u{163}\u{167}\u{16d}\u{171}\u{178}\u{17a}\u{17c}\u{181}\u{183}\u{188}\u{18d}\
	\u{193}\u{197}\u{19d}\u{1a3}\u{1a7}\u{1ae}\u{1b0}\u{1b2}\u{1b7}\u{1b9}\u{1bb}\
	\u{1bf}\u{1c5}\u{1c9}\u{1d0}\u{1d2}\u{1d4}\u{1d9}\u{1db}\u{1e1}\u{1e8}\u{1ec}\
	\u{1f8}\u{1ff}\u{204}\u{208}\u{20b}\u{211}\u{215}\u{21a}\u{21e}\u{222}\u{230}\
	\u{238}\u{240}\u{242}\u{246}\u{24f}\u{256}\u{258}\u{261}\u{266}\u{26b}\u{272}\
	\u{276}\u{27d}\u{285}\u{28e}\u{297}\u{29e}\u{2aa}\u{2b0}\u{2bd}\u{2c3}\u{2cc}\
	\u{2d7}\u{2e2}\u{2e7}\u{2ec}\u{2f1}\u{2f9}\u{302}\u{308}\u{30a}\u{312}\u{316}\
	\u{320}\u{327}\u{32a}\u{330}\u{333}\u{338}\u{33d}\u{347}\u{34b}\u{356}\u{361}\
	\u{36b}\u{375}\u{37f}\u{384}\u{389}\u{39e}\u{3a2}\u{3aa}\u{3af}\u{3b2}\u{3b7}\
	\u{3be}\u{3c2}\u{3c6}\u{3cc}\u{3d3}\u{3dc}\u{3e3}\u{3e7}\u{3ee}\u{3f3}\u{403}\
	\u{40b}\u{415}\u{419}\u{420}\u{428}\u{435}\u{438}\u{43c}\u{440}\u{447}\u{451}\
	\u{459}\u{45f}\u{467}\u{477}\u{481}\u{484}\u{49b}\u{49d}\u{4a1}\u{4a7}\u{4ad}\
	\u{4b2}\u{4b7}\u{4bf}\u{4c5}\u{4cb}\u{4d1}\u{4d5}\u{4d9}\u{4db}\u{4df}\u{4e8}\
	\u{4ef}\u{4f3}\u{4f7}\u{4fb}\u{4fe}\u{500}\u{504}\u{508}\u{50d}\u{511}\u{515}\
	\u{51c}\u{520}\u{528}\u{532}\u{536}\u{53a}\u{53c}\u{540}\u{546}\u{54a}\u{54e}\
	\u{550}\u{552}\u{558}\u{55b}\u{565}\u{569}\u{56d}\u{577}\u{57b}\u{57e}\u{585}\
	\u{58a}\u{590}\u{595}\u{59a}";
