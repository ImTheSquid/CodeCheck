use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};

use antlr_rust::char_stream::CharStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::int_stream::EOF;
use antlr_rust::lexer_atn_simulator::LexerATNSimulator;
use antlr_rust::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF};
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::tree::{ErrorNode, ParseTreeVisitorCompat, TerminalNode};
use antlr_rust::{BaseLexer, InputStream, Lexer, TokenSource};
use syntree::{Span, Tree};

use crate::gen::pythonlexer::{PythonLexer, PythonLexerActions};
use crate::gen::pythonparser::*;
use crate::gen::pythonparservisitor::PythonParserVisitorCompat;
use crate::{visitor_result, SyntaxTree, TreeParseError, VisitorReturn};

use macros::auto_visitor;

#[derive(Debug)]
pub struct PythonTree {
    /// Contains all necessary indices to reconstruct the source code from the original with
    /// symbols. This tree also contains whitespace and variable names, so it may not work as
    /// well for comparisons.
    /// TODO: Figure if non-token structure tree is needed
    pub symbol_tree: syntree::Builder<PythonTreeItem, usize, usize>,
    /// Temporary variable for visitor
    tmp: VisitorReturn<()>,
}

impl Clone for PythonTree {
    fn clone(&self) -> Self {
        Self {
            symbol_tree: self.symbol_tree.clone(),
            tmp: Default::default(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for PythonTree {
    type Node = PythonParserContextType;
    type Return = VisitorReturn<()>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp
    }

    fn visit_terminal(&mut self, node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        visitor_result!(self.symbol_tree.token_with(
            PythonTreeItem::Terminal,
            Span::new(node.symbol.start as usize, node.symbol.stop as usize + 1)
        ));

        VisitorReturn(Ok(()))
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        VisitorReturn(Err(TreeParseError::InvalidNode))
    }
}

auto_visitor!(pythonparser, PythonTree, PythonTreeItem);

impl SyntaxTree for PythonTree {
    type Item = PythonTreeItem;
    fn symbol_tree(self) -> anyhow::Result<Tree<Self::Item, usize, usize>, TreeParseError> {
        Ok(self.symbol_tree.build()?)
    }
}

impl TryFrom<String> for PythonTree {
    type Error = TreeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lexer = BasePythonLexer::new(InputStream::new(value.as_str()));
        let mut parser = PythonParser::new(CommonTokenStream::new(lexer));

        let root = parser.file_input()?;

        let mut tree = PythonTree {
            symbol_tree: Default::default(),
            tmp: Default::default(),
        };

        tree.visit(&*root).0?;

        Ok(tree)
    }
}

use crate::gen::pythonlexer as pythonlex;

type Token<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

struct BasePythonLexer<'input, Input>
where
    Input: CharStream<pythonlex::From<'input>>,
{
    base: BaseLexer<'input, PythonLexerActions, Input, LocalTokenFactory<'input>>,
    indent_length_stack: Vec<usize>,
    pending_tokens: VecDeque<Token<'input>>,
    prev_pending_token_type: isize,
    last_pending_token_type_from_default_channel: isize,
    opened_parentheses: usize,
    paren_or_bracket_open_stack: Vec<usize>,
    brace_expression_stack: Vec<String>,
    prev_brace_expression: Option<String>,
    current_mode: usize,
    mode_stack: Vec<usize>,
    was_space_indentation: bool,
    was_tab_indentation: bool,
    was_mixed_indentation: bool,
    current_token: Option<Token<'input>>,
    ffg_token: Option<Token<'input>>,
    error_occurred: bool,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for BasePythonLexer<'input,Input> where Input:CharStream<pythonlex::From<'input> > }

impl<'input, Input: CharStream<pythonlex::From<'input>>> Deref for BasePythonLexer<'input, Input> {
    type Target = BaseLexer<'input, PythonLexerActions, Input, LocalTokenFactory<'input>>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<pythonlex::From<'input>>> DerefMut
    for BasePythonLexer<'input, Input>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

const TOKEN_HIDDEN_CHANNEL: isize = 1;
const LEXER_DEFAULT_MODE: usize = 0;

fn make_base_lexer<'input, Input>(
    input: Input,
) -> BaseLexer<'input, PythonLexerActions, Input, LocalTokenFactory<'input>>
where
    Input: CharStream<pythonlex::From<'input>>,
{
    BaseLexer::new_base_lexer(
        input,
        LexerATNSimulator::new_lexer_atnsimulator(
            pythonlex::_ATN.clone(),
            pythonlex::_decision_to_DFA.clone(),
            pythonlex::_shared_context_cache.clone(),
        ),
        pythonlex::PythonLexerActions {},
        <&LocalTokenFactory<'input> as Default>::default(),
    )
}

impl<'input, Input> BasePythonLexer<'input, Input>
where
    Input: CharStream<pythonlex::From<'input>>,
{
    fn new(input: Input) -> Self {
        Self {
            base: make_base_lexer(input),
            indent_length_stack: Default::default(),
            pending_tokens: Default::default(),
            prev_pending_token_type: Default::default(),
            last_pending_token_type_from_default_channel: Default::default(),
            opened_parentheses: Default::default(),
            paren_or_bracket_open_stack: Default::default(),
            brace_expression_stack: Default::default(),
            prev_brace_expression: Default::default(),
            current_mode: Default::default(),
            mode_stack: Default::default(),
            was_space_indentation: Default::default(),
            was_tab_indentation: Default::default(),
            was_mixed_indentation: Default::default(),
            current_token: Default::default(),
            ffg_token: Default::default(),
            error_occurred: false,
        }
    }
}

#[derive(Debug)]
enum IndentationLength {
    Valid(usize),
    Invalid,
}

impl<'input, Input: CharStream<pythonlex::From<'input>>> BasePythonLexer<'input, Input> {
    fn check_next_token(&mut self) {
        if self.prev_pending_token_type == TOKEN_EOF {
            return;
        }

        if self.indent_length_stack.is_empty() {
            // Disable this, it's useless
            // self.insert_encoding_token();
            self.set_current_and_following_tokens();
            self.handle_start_of_input();
        } else {
            self.set_current_and_following_tokens();
        }

        if let Some(current_token) = &mut self.current_token {
            match current_token.token_type {
                pythonlex::NEWLINE => self.handle_newline_token(),
                pythonlex::LPAR | pythonlex::LSQB | pythonlex::LBRACE => {
                    self.opened_parentheses += 1;
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        current_token.clone(),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
                pythonlex::RPAR | pythonlex::RSQB | pythonlex::RBRACE => {
                    self.opened_parentheses -= 1;
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        current_token.clone(),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
                pythonlex::FSTRING_MIDDLE => {
                    Self::handle_fstring_middle_with_double_brace(
                        current_token,
                        &mut self.pending_tokens,
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        current_token.clone(),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
                pythonlex::COLONEQUAL => self.handle_colon_equal_token_in_fstring(),
                pythonlex::ERRORTOKEN => {
                    eprintln!("Token error at {}", current_token.text);
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        current_token.clone(),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
                TOKEN_EOF => self.handle_eof_token(),
                _ => {
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        current_token.clone(),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
            }

            self.handle_format_specification_mode();
        }
    }

    fn handle_newline_token(&mut self) {
        let current_token = self.current_token.clone().expect("cur tok");
        if !self.mode_stack.is_empty() {
            Self::add_pending_token(
                &mut self.pending_tokens,
                current_token,
                &mut self.prev_pending_token_type,
                &mut self.last_pending_token_type_from_default_channel,
            );
        } else if self.opened_parentheses > 0 {
            Self::hide_and_add_pending_token(
                &mut self.pending_tokens,
                current_token,
                &mut self.prev_pending_token_type,
                &mut self.last_pending_token_type_from_default_channel,
            );
        } else {
            let tok = current_token;
            let is_looking_ahead = self
                .ffg_token
                .as_ref()
                .map(|f| f.token_type == pythonlex::WS)
                .expect("ffg");
            if is_looking_ahead {
                self.set_current_and_following_tokens();
            }

            if matches!(
                self.ffg_token.as_ref().expect("ffg tok").token_type,
                pythonlex::NEWLINE | pythonlex::COMMENT
            ) {
                Self::hide_and_add_pending_token(
                    &mut self.pending_tokens,
                    tok,
                    &mut self.prev_pending_token_type,
                    &mut self.last_pending_token_type_from_default_channel,
                );

                if is_looking_ahead {
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        self.current_token.clone().expect("cur tok"),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
            } else {
                Self::add_pending_token(
                    &mut self.pending_tokens,
                    tok,
                    &mut self.prev_pending_token_type,
                    &mut self.last_pending_token_type_from_default_channel,
                );

                if is_looking_ahead {
                    let indent = if self
                        .ffg_token
                        .as_ref()
                        .map(|ffg| ffg.token_type == EOF)
                        .expect("ffg tok")
                    {
                        IndentationLength::Valid(0)
                    } else {
                        Self::indentation_length(
                            &self.current_token.as_ref().expect("cur tok").text,
                            &mut self.was_space_indentation,
                            &mut self.was_tab_indentation,
                            &mut self.was_mixed_indentation,
                        )
                    };

                    match indent {
                        IndentationLength::Valid(il) => {
                            Self::add_pending_token(
                                &mut self.pending_tokens,
                                self.current_token.clone().expect("cur tok"),
                                &mut self.prev_pending_token_type,
                                &mut self.last_pending_token_type_from_default_channel,
                            );
                            self.insert_indent_or_dedent_token(il);
                        }
                        IndentationLength::Invalid => {
                            eprintln!("inconsistent use of tabs and spaces in indentation");
                            self.error_occurred = true;
                        }
                    }
                } else {
                    self.insert_indent_or_dedent_token(0);
                }
            }
        }
    }

    fn handle_format_specification_mode(&mut self) {
        let ffg = self.ffg_token.clone().expect("ffg tok");
        if !self.mode_stack.is_empty() && ffg.token_type == pythonlex::RBRACE {
            match self.current_token.as_ref().expect("cur tok").token_type {
                pythonlex::COLON => Self::create_and_add_pending_token(
                    &mut self.pending_tokens,
                    ffg,
                    &mut self.prev_pending_token_type,
                    &mut self.last_pending_token_type_from_default_channel,
                    pythonlex::FSTRING_MIDDLE,
                    TOKEN_DEFAULT_CHANNEL,
                    Some(Cow::Borrowed("")),
                ),
                pythonlex::RBRACE => {
                    if !Self::is_dictionary_comprehension_or_set_comprehension(
                        self.prev_brace_expression.as_ref().expect("prev brace"),
                    ) {
                        Self::create_and_add_pending_token(
                            &mut self.pending_tokens,
                            self.ffg_token.clone().expect("ffg"),
                            &mut self.prev_pending_token_type,
                            &mut self.last_pending_token_type_from_default_channel,
                            pythonlex::FSTRING_MIDDLE,
                            TOKEN_DEFAULT_CHANNEL,
                            Some(Cow::Borrowed("")),
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn is_dictionary_comprehension_or_set_comprehension(code: &str) -> bool {
        let mut parser = {
            let is = InputStream::new(code);
            let lex = PythonLexer::new(is);
            let ts = CommonTokenStream::new(lex);
            PythonParser::new(ts)
        };

        parser.dictcomp().is_ok() || parser.setcomp().is_ok()
    }

    fn handle_colon_equal_token_in_fstring(&mut self) {
        if !self.mode_stack.is_empty()
            && self
                .paren_or_bracket_open_stack
                .first()
                .is_some_and(|s| *s == 0)
        {
            {
                let tok = self.current_token.as_mut().expect("cur tok");
                tok.token_type = pythonlex::COLON;
                tok.text = Cow::Borrowed(":");
                tok.stop = tok.start;
            }
            let ffg = self.ffg_token.as_mut().expect("ffg tok");
            if ffg.token_type == pythonlex::FSTRING_MIDDLE {
                ffg.text = Cow::Owned(format!("={}", ffg.text));
                ffg.start -= 1;
                ffg.column -= 1;
            } else {
                Self::add_pending_token(
                    &mut self.pending_tokens,
                    self.current_token.clone().expect("cur tok"),
                    &mut self.prev_pending_token_type,
                    &mut self.last_pending_token_type_from_default_channel,
                );
                Self::update_current_token(
                    self.current_token.as_mut().expect("cur tok"),
                    pythonlex::FSTRING_MIDDLE,
                    "=",
                    TOKEN_DEFAULT_CHANNEL,
                );
            }
        }
        Self::add_pending_token(
            &mut self.pending_tokens,
            self.current_token.clone().expect("cur tok"),
            &mut self.prev_pending_token_type,
            &mut self.last_pending_token_type_from_default_channel,
        );
    }

    // fn insert_encoding_token(&mut self) {
    //     let stream = self.base.get_input_stream().expect("input stream");
    //     let size = stream.size();
    //     let mut lines_traversed = 0;
    //     const WS_COMMENT_PATTERN: &str = r#"^[\t\f]*(#.*)?$"#;
    //     let ws_comment_pat = regex::Regex::new(WS_COMMENT_PATTERN).unwrap();
    //     const ENC_COMMENT_PATTERN: &str = r#"^[ \t\f]*#.*?coding[:=][ \t]*([-_.a-zA-Z0-9]+)"#;
    //     let enc_comment_pat = regex::Regex::new(ENC_COMMENT_PATTERN).unwrap();
    //     let mut collector = Vec::with_capacity(size as usize);
    //     stream.seek(0);
    //     let mut encoding_name = "utf-8".to_string();

    //     for i in 0..size {
    //         let c = stream.la(i + 1) as u8 as char;
    //         collector.push(c);
    //         if c == '\n' {
    //             let line: String = collector.iter().collect();
    //             if ws_comment_pat.is_match(&line) {
    //                 if let Some(Some(encoding)) = enc_comment_pat.captures(&line).map(|c| c.get(1))
    //                 {
    //                     encoding_name = encoding.as_str().to_string();
    //                 }
    //             }

    //             lines_traversed += 1;
    //             if lines_traversed >= 2 {
    //                 break;
    //             }

    //             collector = Vec::with_capacity(size as usize);
    //         }
    //     }

    //     let token = CommonToken {
    //         token_type: pythonlex::ENCODING,
    //         channel: TOKEN_HIDDEN_CHANNEL,
    //         start: 0,
    //         stop: 0,
    //         line: 0,
    //         column: 0,
    //         text: Cow::Owned(encoding_name),
    //         token_index: AtomicIsize::new(-1),
    //         read_only: false,
    //     };

    //     Self::add_pending_token(
    //         &mut self.pending_tokens,
    //         Box::new(token),
    //         &mut self.prev_pending_token_type,
    //         &mut self.last_pending_token_type_from_default_channel,
    //     );
    // }

    fn handle_start_of_input(&mut self) {
        self.indent_length_stack.push(0);
        while self
            .current_token
            .as_ref()
            .is_some_and(|ct| ct.token_type != TOKEN_EOF)
        {
            if let Some(current_token) = &mut self.current_token {
                if current_token.channel == TOKEN_DEFAULT_CHANNEL {
                    if current_token.token_type == pythonlex::NEWLINE {
                        Self::hide_and_add_pending_token(
                            &mut self.pending_tokens,
                            current_token.clone(),
                            &mut self.prev_pending_token_type,
                            &mut self.last_pending_token_type_from_default_channel,
                        );
                    } else {
                        self.insert_leading_indent_token();
                        return;
                    }
                } else {
                    Self::add_pending_token(
                        &mut self.pending_tokens,
                        current_token.clone(),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                    );
                }
            }

            self.set_current_and_following_tokens();
        }
    }

    fn insert_leading_indent_token(&mut self) {
        if self.prev_pending_token_type == pythonlex::WS {
            let prev = self.pending_tokens.back().cloned();
            if let Some(true) = prev.as_ref().map(|p| {
                match Self::indentation_length(
                    p.text.as_ref(),
                    &mut self.was_space_indentation,
                    &mut self.was_tab_indentation,
                    &mut self.was_mixed_indentation,
                ) {
                    IndentationLength::Invalid => true,
                    IndentationLength::Valid(il) => il > 0,
                }
            }) {
                eprintln!("First statement indented");
                self.error_occurred = true;
                Self::create_and_add_pending_token(
                    &mut self.pending_tokens,
                    prev.clone().expect("prev tok"),
                    &mut self.prev_pending_token_type,
                    &mut self.last_pending_token_type_from_default_channel,
                    pythonlex::INDENT,
                    TOKEN_DEFAULT_CHANNEL,
                    Some(Cow::Borrowed("ERROR: First statement indented")),
                );
            }
        }
    }

    fn create_and_add_pending_token(
        pending_tokens: &mut VecDeque<Token<'input>>,
        mut token: Token<'input>,
        prev_pending_token_type: &mut isize,
        last_tt_from_def_channel: &mut isize,
        token_type: isize,
        channel: isize,
        text: Option<Cow<'input, str>>,
    ) {
        token.token_type = token_type;
        token.channel = channel;
        if let Some(text) = text {
            token.text = text;
        }
        Self::add_pending_token(
            pending_tokens,
            token,
            prev_pending_token_type,
            last_tt_from_def_channel,
        );
    }

    fn hide_and_add_pending_token(
        pending_tokens: &mut VecDeque<Token<'input>>,
        mut token: Token<'input>,
        prev_pending_token_type: &mut isize,
        last_pending_token_type_from_default_channel: &mut isize,
    ) {
        token.channel = TOKEN_HIDDEN_CHANNEL;
        Self::add_pending_token(
            pending_tokens,
            token,
            prev_pending_token_type,
            last_pending_token_type_from_default_channel,
        );
    }

    fn set_current_and_following_tokens(&mut self) {
        self.current_token = Some(
            self.ffg_token
                .take()
                .unwrap_or_else(|| self.base.next_token()),
        );
        self.check_current_token();
        self.ffg_token = Some(
            self.current_token
                .take_if(|ct| ct.token_type == TOKEN_EOF)
                .unwrap_or_else(|| self.base.next_token()),
        );
    }

    fn check_current_token(&mut self) {
        let current_token = self.current_token.as_mut().expect("cur tok");
        match current_token.token_type {
            pythonlex::FSTRING_START => {
                self.set_lexer_mode_by_fstring_start();
                return;
            }
            pythonlex::FSTRING_MIDDLE => {
                Self::handle_fstring_middle_with_quote_and_lbrace(
                    current_token,
                    &mut self.pending_tokens,
                    &mut self.prev_pending_token_type,
                    &mut self.last_pending_token_type_from_default_channel,
                );
                if current_token.token_type == pythonlex::FSTRING_MIDDLE {
                    return;
                }
            }
            pythonlex::FSTRING_END => {
                self.pop_lexer_mode();
                return;
            }
            _ => {
                if self.mode_stack.is_empty() {
                    return;
                }
            }
        }

        let ct_text = current_token.text.clone();
        let ct_ty = current_token.token_type;
        let _ = current_token;

        match ct_ty {
            pythonlex::NEWLINE => {
                Self::append_to_brace_expression(
                    &mut self.brace_expression_stack,
                    ct_text.as_ref(),
                );
                self.current_token.as_mut().expect("ct").channel = TOKEN_HIDDEN_CHANNEL;
            }
            pythonlex::LBRACE => {
                self.brace_expression_stack.push("{".into());
                self.paren_or_bracket_open_stack.push(0);
                self.push_lexer_mode(LEXER_DEFAULT_MODE);
            }
            pythonlex::LPAR | pythonlex::LSQB => {
                Self::append_to_brace_expression(
                    &mut self.brace_expression_stack,
                    ct_text.as_ref(),
                );
                Self::incr_brace_stack(&mut self.paren_or_bracket_open_stack);
            }
            pythonlex::RPAR | pythonlex::RSQB => {
                Self::append_to_brace_expression(
                    &mut self.brace_expression_stack,
                    ct_text.as_ref(),
                );
                Self::decr_brace_stack(&mut self.paren_or_bracket_open_stack);
            }
            pythonlex::COLON | pythonlex::COLONEQUAL => {
                Self::append_to_brace_expression(
                    &mut self.brace_expression_stack,
                    ct_text.as_ref(),
                );
                self.set_lexer_mode_by_colon_ce_token();
            }
            pythonlex::RBRACE => self.set_lexer_mode_after_rbrace_token(),
            _ => {
                Self::append_to_brace_expression(&mut self.brace_expression_stack, ct_text.as_ref())
            }
        }
    }

    fn set_lexer_mode_by_colon_ce_token(&mut self) {
        if self
            .paren_or_bracket_open_stack
            .last()
            .is_some_and(|&itm| itm != 0)
        {
            return;
        }

        let Some(&top) = self.mode_stack.last() else {
            return;
        };

        let mode = if top <= 8 { top + 8 } else { top };

        self.push_lexer_mode(mode);
    }

    fn set_lexer_mode_after_rbrace_token(&mut self) {
        match self.current_mode {
            LEXER_DEFAULT_MODE => {
                self.pop_lexer_mode();
                self.pop_by_brace();
            }
            9..=16 => {
                self.pop_lexer_mode();
                self.pop_lexer_mode();
                self.pop_by_brace();
            }
            _ => {
                eprintln!("fstring: single }} not allowed");
                self.error_occurred = true;
            }
        }
    }

    fn pop_by_brace(&mut self) {
        self.paren_or_bracket_open_stack.pop();
        self.prev_brace_expression = self
            .brace_expression_stack
            .pop()
            .map(|itm| format!("{itm}}}"));

        if let Some(top) = self.brace_expression_stack.last_mut() {
            if let Some(prev) = &self.prev_brace_expression {
                *top = format!("{top}{prev}");
            }
        };
    }

    fn incr_brace_stack(paren_or_bracket_open_stack: &mut [usize]) {
        *paren_or_bracket_open_stack.last_mut().expect("stack item") += 1;
    }

    fn decr_brace_stack(paren_or_bracket_open_stack: &mut [usize]) {
        *paren_or_bracket_open_stack.last_mut().expect("stack item") -= 1;
    }

    fn append_to_brace_expression(brace_stack: &mut [String], to_append: &str) {
        let text = brace_stack.last().unwrap();
        *brace_stack.last_mut().unwrap() = format!("{text}{to_append}");
    }

    fn pop_lexer_mode(&mut self) {
        self.base.pop_mode();
        self.current_mode = self.mode_stack.pop().expect("lexer mode stack pop");
    }

    fn handle_fstring_middle_with_double_brace(
        current_token: &mut Token<'input>,
        pending_tokens: &mut VecDeque<Token<'input>>,
        prev_pending_token_type: &mut isize,
        last_tt_from_def_channel: &mut isize,
    ) {
        match Self::get_last_two_chars_of_token(current_token) {
            "{{" => Self::trim_last_char_add_pending_token_set_current_token(
                pending_tokens,
                current_token,
                prev_pending_token_type,
                last_tt_from_def_channel,
                pythonlex::LBRACE,
                "{",
                TOKEN_HIDDEN_CHANNEL,
            ),
            "}}" => Self::trim_last_char_add_pending_token_set_current_token(
                pending_tokens,
                current_token,
                prev_pending_token_type,
                last_tt_from_def_channel,
                pythonlex::RBRACE,
                "}",
                TOKEN_HIDDEN_CHANNEL,
            ),
            _ => {}
        }
    }

    fn indentation_length(
        txt: &str,
        was_space_indented: &mut bool,
        was_tab_indented: &mut bool,
        was_space_or_tab_indented: &mut bool,
    ) -> IndentationLength {
        const TAB_SIZE: usize = 8;
        let mut length = 0usize;
        for c in txt.chars() {
            match c {
                ' ' => {
                    *was_space_indented = true;
                    length += 1;
                }
                '\t' => {
                    *was_tab_indented = true;
                    length += TAB_SIZE - (length % TAB_SIZE);
                }
                _ => {}
            }
        }

        if *was_space_indented && *was_tab_indented && !*was_space_or_tab_indented {
            *was_space_or_tab_indented = *was_space_indented || *was_tab_indented;
            IndentationLength::Invalid
        } else {
            IndentationLength::Valid(length)
        }
    }

    fn insert_indent_or_dedent_token(&mut self, length: usize) {
        let mut prev_indent_length = *self.indent_length_stack.last().expect("some indent");
        if length > prev_indent_length {
            Self::create_and_add_pending_token(
                &mut self.pending_tokens,
                self.ffg_token.clone().expect("ffg token"),
                &mut self.prev_pending_token_type,
                &mut self.last_pending_token_type_from_default_channel,
                pythonlex::INDENT,
                TOKEN_DEFAULT_CHANNEL,
                None,
            );
            self.indent_length_stack.push(length);
        } else {
            while length < prev_indent_length {
                self.indent_length_stack.pop();
                prev_indent_length = self.indent_length_stack.last().cloned().expect("dedent");
                if length <= prev_indent_length {
                    Self::create_and_add_pending_token(
                        &mut self.pending_tokens,
                        self.ffg_token.clone().expect("ffg token"),
                        &mut self.prev_pending_token_type,
                        &mut self.last_pending_token_type_from_default_channel,
                        pythonlex::DEDENT,
                        TOKEN_DEFAULT_CHANNEL,
                        None,
                    );
                } else {
                    eprintln!("Inconsistent dedent");
                    self.error_occurred = true;
                }
            }
        }
    }

    fn handle_fstring_middle_with_quote_and_lbrace(
        current_token: &mut Token<'input>,
        pending_tokens: &mut VecDeque<Token<'input>>,
        prev_pending_token_type: &mut isize,
        last_tt_from_def_channel: &mut isize,
    ) {
        if [r#""{"#, "'{", r#"\{"#].contains(&Self::get_last_two_chars_of_token(current_token)) {
            Self::trim_last_char_add_pending_token_set_current_token(
                pending_tokens,
                current_token,
                prev_pending_token_type,
                last_tt_from_def_channel,
                pythonlex::LBRACE,
                "{",
                TOKEN_DEFAULT_CHANNEL,
            );
        }
    }

    fn trim_last_char_add_pending_token_set_current_token(
        pending_tokens: &mut VecDeque<Token<'input>>,
        current_token: &mut Token<'input>,
        prev_pending_token_type: &mut isize,
        last_pending_token_type_from_default_channel: &mut isize,
        ty: isize,
        text: &'input str,
        channel: isize,
    ) {
        let mut token = current_token.clone();
        let trimmed_text = token.text[..token.text.len() - 1].to_string();
        token.text = Cow::Owned(trimmed_text);
        token.stop -= 1;
        Self::add_pending_token(
            pending_tokens,
            token,
            prev_pending_token_type,
            last_pending_token_type_from_default_channel,
        );

        Self::update_current_token(current_token, ty, text, channel);
    }

    fn update_current_token(
        current_token: &mut Token<'input>,
        ty: isize,
        text: &'input str,
        channel: isize,
    ) {
        current_token.token_type = ty;
        current_token.text = Cow::Borrowed(text);
        current_token.channel = channel;
        current_token.column += 1;
        current_token.start += 1;
        current_token.stop = current_token.start;
    }

    fn add_pending_token(
        pending_tokens: &mut VecDeque<Token<'input>>,
        token: Token<'input>,
        prev_pending_token_type: &mut isize,
        last_tt_from_def_channel: &mut isize,
    ) {
        *prev_pending_token_type = token.token_type;
        if token.channel == TOKEN_DEFAULT_CHANNEL {
            *last_tt_from_def_channel = token.token_type;
        }
        pending_tokens.push_back(token);
    }

    fn get_last_two_chars_of_token<'a>(token: &'a Token<'input>) -> &'a str {
        let token_text = &token.text;
        let len = token_text.len();
        if len < 2 {
            ""
        } else {
            &token_text[len - 2..]
        }
    }

    fn set_lexer_mode_by_fstring_start(&mut self) {
        let token_text = self.current_token.as_ref().unwrap().text.to_lowercase();
        // Convert the text to lowercase for case-insensitive comparison
        let token_text_lower = token_text.to_lowercase();

        // Create a map where keys are string slices of prefixes and values are mode constants as isize
        let mode_map: HashMap<&str, usize> = [
            ("f'", pythonlex::SQ1__FSTRING_MODE),
            ("rf'", pythonlex::SQ1R_FSTRING_MODE),
            ("fr'", pythonlex::SQ1R_FSTRING_MODE),
            ("f\"", pythonlex::DQ1__FSTRING_MODE),
            ("rf\"", pythonlex::DQ1R_FSTRING_MODE),
            ("fr\"", pythonlex::DQ1R_FSTRING_MODE),
            ("f'''", pythonlex::SQ3__FSTRING_MODE),
            ("rf'''", pythonlex::SQ3R_FSTRING_MODE),
            ("fr'''", pythonlex::SQ3R_FSTRING_MODE),
            ("f\"\"\"", pythonlex::DQ3__FSTRING_MODE),
            ("rf\"\"\"", pythonlex::DQ3R_FSTRING_MODE),
            ("fr\"\"\"", pythonlex::DQ3R_FSTRING_MODE),
        ]
        .iter()
        .cloned()
        .collect();

        // Retrieve the mode based on the token text
        let mode = mode_map.get(token_text_lower.as_str()).expect("mode");
        self.push_lexer_mode(*mode);
    }

    fn push_lexer_mode(&mut self, mode: usize) {
        self.base.push_mode(mode);
        self.mode_stack.push(self.current_mode);
        self.current_mode = mode;
    }

    fn insert_trailing_tokens(&mut self) {
        if !matches!(
            self.last_pending_token_type_from_default_channel,
            pythonlex::NEWLINE | pythonlex::DEDENT
        ) {
            Self::create_and_add_pending_token(
                &mut self.pending_tokens,
                self.ffg_token.clone().expect("ffg token"),
                &mut self.prev_pending_token_type,
                &mut self.last_pending_token_type_from_default_channel,
                pythonlex::NEWLINE,
                TOKEN_DEFAULT_CHANNEL,
                None,
            );
        }
        self.insert_indent_or_dedent_token(0);
    }

    fn handle_eof_token(&mut self) {
        if self.last_pending_token_type_from_default_channel > 0 {
            self.insert_trailing_tokens();
        }
        Self::add_pending_token(
            &mut self.pending_tokens,
            self.current_token.clone().expect("current token"),
            &mut self.prev_pending_token_type,
            &mut self.last_pending_token_type_from_default_channel,
        );
    }
}

impl<'input, Input: CharStream<pythonlex::From<'input>>> TokenSource<'input>
    for BasePythonLexer<'input, Input>
{
    type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> Token<'input> {
        self.check_next_token();

        self.pending_tokens.pop_front().unwrap_or_else(|| {
            self.base.get_token_factory().create(
                None::<&mut Input>,
                antlr_rust::int_stream::EOF,
                None,
                TOKEN_DEFAULT_CHANNEL,
                self.get_char_index(),
                self.get_char_index() - 1,
                self.get_line(),
                self.get_char_position_in_line(),
            )
        })
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn antlr_rust::int_stream::IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }
}
