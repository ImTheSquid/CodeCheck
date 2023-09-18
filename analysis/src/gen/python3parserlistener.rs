#![allow(nonstandard_style)]
// Generated from Python3Parser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::python3parser::*;

pub trait Python3ParserListener<'input> : ParseTreeListener<'input,Python3ParserContextType>{
/**
 * Enter a parse tree produced by {@link Python3Parser#single_input}.
 * @param ctx the parse tree
 */
fn enter_single_input(&mut self, _ctx: &Single_inputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#single_input}.
 * @param ctx the parse tree
 */
fn exit_single_input(&mut self, _ctx: &Single_inputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#file_input}.
 * @param ctx the parse tree
 */
fn enter_file_input(&mut self, _ctx: &File_inputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#file_input}.
 * @param ctx the parse tree
 */
fn exit_file_input(&mut self, _ctx: &File_inputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#eval_input}.
 * @param ctx the parse tree
 */
fn enter_eval_input(&mut self, _ctx: &Eval_inputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#eval_input}.
 * @param ctx the parse tree
 */
fn exit_eval_input(&mut self, _ctx: &Eval_inputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#decorator}.
 * @param ctx the parse tree
 */
fn enter_decorator(&mut self, _ctx: &DecoratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#decorator}.
 * @param ctx the parse tree
 */
fn exit_decorator(&mut self, _ctx: &DecoratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#decorators}.
 * @param ctx the parse tree
 */
fn enter_decorators(&mut self, _ctx: &DecoratorsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#decorators}.
 * @param ctx the parse tree
 */
fn exit_decorators(&mut self, _ctx: &DecoratorsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#decorated}.
 * @param ctx the parse tree
 */
fn enter_decorated(&mut self, _ctx: &DecoratedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#decorated}.
 * @param ctx the parse tree
 */
fn exit_decorated(&mut self, _ctx: &DecoratedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#async_funcdef}.
 * @param ctx the parse tree
 */
fn enter_async_funcdef(&mut self, _ctx: &Async_funcdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#async_funcdef}.
 * @param ctx the parse tree
 */
fn exit_async_funcdef(&mut self, _ctx: &Async_funcdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#funcdef}.
 * @param ctx the parse tree
 */
fn enter_funcdef(&mut self, _ctx: &FuncdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#funcdef}.
 * @param ctx the parse tree
 */
fn exit_funcdef(&mut self, _ctx: &FuncdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#parameters}.
 * @param ctx the parse tree
 */
fn enter_parameters(&mut self, _ctx: &ParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#parameters}.
 * @param ctx the parse tree
 */
fn exit_parameters(&mut self, _ctx: &ParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#typedargslist}.
 * @param ctx the parse tree
 */
fn enter_typedargslist(&mut self, _ctx: &TypedargslistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#typedargslist}.
 * @param ctx the parse tree
 */
fn exit_typedargslist(&mut self, _ctx: &TypedargslistContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#tfpdef}.
 * @param ctx the parse tree
 */
fn enter_tfpdef(&mut self, _ctx: &TfpdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#tfpdef}.
 * @param ctx the parse tree
 */
fn exit_tfpdef(&mut self, _ctx: &TfpdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#varargslist}.
 * @param ctx the parse tree
 */
fn enter_varargslist(&mut self, _ctx: &VarargslistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#varargslist}.
 * @param ctx the parse tree
 */
fn exit_varargslist(&mut self, _ctx: &VarargslistContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#vfpdef}.
 * @param ctx the parse tree
 */
fn enter_vfpdef(&mut self, _ctx: &VfpdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#vfpdef}.
 * @param ctx the parse tree
 */
fn exit_vfpdef(&mut self, _ctx: &VfpdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#stmt}.
 * @param ctx the parse tree
 */
fn enter_stmt(&mut self, _ctx: &StmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#stmt}.
 * @param ctx the parse tree
 */
fn exit_stmt(&mut self, _ctx: &StmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#simple_stmts}.
 * @param ctx the parse tree
 */
fn enter_simple_stmts(&mut self, _ctx: &Simple_stmtsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#simple_stmts}.
 * @param ctx the parse tree
 */
fn exit_simple_stmts(&mut self, _ctx: &Simple_stmtsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#simple_stmt}.
 * @param ctx the parse tree
 */
fn enter_simple_stmt(&mut self, _ctx: &Simple_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#simple_stmt}.
 * @param ctx the parse tree
 */
fn exit_simple_stmt(&mut self, _ctx: &Simple_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#expr_stmt}.
 * @param ctx the parse tree
 */
fn enter_expr_stmt(&mut self, _ctx: &Expr_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#expr_stmt}.
 * @param ctx the parse tree
 */
fn exit_expr_stmt(&mut self, _ctx: &Expr_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#annassign}.
 * @param ctx the parse tree
 */
fn enter_annassign(&mut self, _ctx: &AnnassignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#annassign}.
 * @param ctx the parse tree
 */
fn exit_annassign(&mut self, _ctx: &AnnassignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#testlist_star_expr}.
 * @param ctx the parse tree
 */
fn enter_testlist_star_expr(&mut self, _ctx: &Testlist_star_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#testlist_star_expr}.
 * @param ctx the parse tree
 */
fn exit_testlist_star_expr(&mut self, _ctx: &Testlist_star_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#augassign}.
 * @param ctx the parse tree
 */
fn enter_augassign(&mut self, _ctx: &AugassignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#augassign}.
 * @param ctx the parse tree
 */
fn exit_augassign(&mut self, _ctx: &AugassignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#del_stmt}.
 * @param ctx the parse tree
 */
fn enter_del_stmt(&mut self, _ctx: &Del_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#del_stmt}.
 * @param ctx the parse tree
 */
fn exit_del_stmt(&mut self, _ctx: &Del_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#pass_stmt}.
 * @param ctx the parse tree
 */
fn enter_pass_stmt(&mut self, _ctx: &Pass_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#pass_stmt}.
 * @param ctx the parse tree
 */
fn exit_pass_stmt(&mut self, _ctx: &Pass_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#flow_stmt}.
 * @param ctx the parse tree
 */
fn enter_flow_stmt(&mut self, _ctx: &Flow_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#flow_stmt}.
 * @param ctx the parse tree
 */
fn exit_flow_stmt(&mut self, _ctx: &Flow_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#break_stmt}.
 * @param ctx the parse tree
 */
fn enter_break_stmt(&mut self, _ctx: &Break_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#break_stmt}.
 * @param ctx the parse tree
 */
fn exit_break_stmt(&mut self, _ctx: &Break_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#continue_stmt}.
 * @param ctx the parse tree
 */
fn enter_continue_stmt(&mut self, _ctx: &Continue_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#continue_stmt}.
 * @param ctx the parse tree
 */
fn exit_continue_stmt(&mut self, _ctx: &Continue_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#return_stmt}.
 * @param ctx the parse tree
 */
fn enter_return_stmt(&mut self, _ctx: &Return_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#return_stmt}.
 * @param ctx the parse tree
 */
fn exit_return_stmt(&mut self, _ctx: &Return_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#yield_stmt}.
 * @param ctx the parse tree
 */
fn enter_yield_stmt(&mut self, _ctx: &Yield_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#yield_stmt}.
 * @param ctx the parse tree
 */
fn exit_yield_stmt(&mut self, _ctx: &Yield_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#raise_stmt}.
 * @param ctx the parse tree
 */
fn enter_raise_stmt(&mut self, _ctx: &Raise_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#raise_stmt}.
 * @param ctx the parse tree
 */
fn exit_raise_stmt(&mut self, _ctx: &Raise_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#import_stmt}.
 * @param ctx the parse tree
 */
fn enter_import_stmt(&mut self, _ctx: &Import_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#import_stmt}.
 * @param ctx the parse tree
 */
fn exit_import_stmt(&mut self, _ctx: &Import_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#import_name}.
 * @param ctx the parse tree
 */
fn enter_import_name(&mut self, _ctx: &Import_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#import_name}.
 * @param ctx the parse tree
 */
fn exit_import_name(&mut self, _ctx: &Import_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#import_from}.
 * @param ctx the parse tree
 */
fn enter_import_from(&mut self, _ctx: &Import_fromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#import_from}.
 * @param ctx the parse tree
 */
fn exit_import_from(&mut self, _ctx: &Import_fromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#import_as_name}.
 * @param ctx the parse tree
 */
fn enter_import_as_name(&mut self, _ctx: &Import_as_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#import_as_name}.
 * @param ctx the parse tree
 */
fn exit_import_as_name(&mut self, _ctx: &Import_as_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#dotted_as_name}.
 * @param ctx the parse tree
 */
fn enter_dotted_as_name(&mut self, _ctx: &Dotted_as_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#dotted_as_name}.
 * @param ctx the parse tree
 */
fn exit_dotted_as_name(&mut self, _ctx: &Dotted_as_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#import_as_names}.
 * @param ctx the parse tree
 */
fn enter_import_as_names(&mut self, _ctx: &Import_as_namesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#import_as_names}.
 * @param ctx the parse tree
 */
fn exit_import_as_names(&mut self, _ctx: &Import_as_namesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#dotted_as_names}.
 * @param ctx the parse tree
 */
fn enter_dotted_as_names(&mut self, _ctx: &Dotted_as_namesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#dotted_as_names}.
 * @param ctx the parse tree
 */
fn exit_dotted_as_names(&mut self, _ctx: &Dotted_as_namesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#dotted_name}.
 * @param ctx the parse tree
 */
fn enter_dotted_name(&mut self, _ctx: &Dotted_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#dotted_name}.
 * @param ctx the parse tree
 */
fn exit_dotted_name(&mut self, _ctx: &Dotted_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#global_stmt}.
 * @param ctx the parse tree
 */
fn enter_global_stmt(&mut self, _ctx: &Global_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#global_stmt}.
 * @param ctx the parse tree
 */
fn exit_global_stmt(&mut self, _ctx: &Global_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#nonlocal_stmt}.
 * @param ctx the parse tree
 */
fn enter_nonlocal_stmt(&mut self, _ctx: &Nonlocal_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#nonlocal_stmt}.
 * @param ctx the parse tree
 */
fn exit_nonlocal_stmt(&mut self, _ctx: &Nonlocal_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#assert_stmt}.
 * @param ctx the parse tree
 */
fn enter_assert_stmt(&mut self, _ctx: &Assert_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#assert_stmt}.
 * @param ctx the parse tree
 */
fn exit_assert_stmt(&mut self, _ctx: &Assert_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#compound_stmt}.
 * @param ctx the parse tree
 */
fn enter_compound_stmt(&mut self, _ctx: &Compound_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#compound_stmt}.
 * @param ctx the parse tree
 */
fn exit_compound_stmt(&mut self, _ctx: &Compound_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#async_stmt}.
 * @param ctx the parse tree
 */
fn enter_async_stmt(&mut self, _ctx: &Async_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#async_stmt}.
 * @param ctx the parse tree
 */
fn exit_async_stmt(&mut self, _ctx: &Async_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#if_stmt}.
 * @param ctx the parse tree
 */
fn enter_if_stmt(&mut self, _ctx: &If_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#if_stmt}.
 * @param ctx the parse tree
 */
fn exit_if_stmt(&mut self, _ctx: &If_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#while_stmt}.
 * @param ctx the parse tree
 */
fn enter_while_stmt(&mut self, _ctx: &While_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#while_stmt}.
 * @param ctx the parse tree
 */
fn exit_while_stmt(&mut self, _ctx: &While_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#for_stmt}.
 * @param ctx the parse tree
 */
fn enter_for_stmt(&mut self, _ctx: &For_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#for_stmt}.
 * @param ctx the parse tree
 */
fn exit_for_stmt(&mut self, _ctx: &For_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#try_stmt}.
 * @param ctx the parse tree
 */
fn enter_try_stmt(&mut self, _ctx: &Try_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#try_stmt}.
 * @param ctx the parse tree
 */
fn exit_try_stmt(&mut self, _ctx: &Try_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#with_stmt}.
 * @param ctx the parse tree
 */
fn enter_with_stmt(&mut self, _ctx: &With_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#with_stmt}.
 * @param ctx the parse tree
 */
fn exit_with_stmt(&mut self, _ctx: &With_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#with_item}.
 * @param ctx the parse tree
 */
fn enter_with_item(&mut self, _ctx: &With_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#with_item}.
 * @param ctx the parse tree
 */
fn exit_with_item(&mut self, _ctx: &With_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#except_clause}.
 * @param ctx the parse tree
 */
fn enter_except_clause(&mut self, _ctx: &Except_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#except_clause}.
 * @param ctx the parse tree
 */
fn exit_except_clause(&mut self, _ctx: &Except_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#match_stmt}.
 * @param ctx the parse tree
 */
fn enter_match_stmt(&mut self, _ctx: &Match_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#match_stmt}.
 * @param ctx the parse tree
 */
fn exit_match_stmt(&mut self, _ctx: &Match_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#subject_expr}.
 * @param ctx the parse tree
 */
fn enter_subject_expr(&mut self, _ctx: &Subject_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#subject_expr}.
 * @param ctx the parse tree
 */
fn exit_subject_expr(&mut self, _ctx: &Subject_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#star_named_expressions}.
 * @param ctx the parse tree
 */
fn enter_star_named_expressions(&mut self, _ctx: &Star_named_expressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#star_named_expressions}.
 * @param ctx the parse tree
 */
fn exit_star_named_expressions(&mut self, _ctx: &Star_named_expressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#star_named_expression}.
 * @param ctx the parse tree
 */
fn enter_star_named_expression(&mut self, _ctx: &Star_named_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#star_named_expression}.
 * @param ctx the parse tree
 */
fn exit_star_named_expression(&mut self, _ctx: &Star_named_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#case_block}.
 * @param ctx the parse tree
 */
fn enter_case_block(&mut self, _ctx: &Case_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#case_block}.
 * @param ctx the parse tree
 */
fn exit_case_block(&mut self, _ctx: &Case_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#guard}.
 * @param ctx the parse tree
 */
fn enter_guard(&mut self, _ctx: &GuardContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#guard}.
 * @param ctx the parse tree
 */
fn exit_guard(&mut self, _ctx: &GuardContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#patterns}.
 * @param ctx the parse tree
 */
fn enter_patterns(&mut self, _ctx: &PatternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#patterns}.
 * @param ctx the parse tree
 */
fn exit_patterns(&mut self, _ctx: &PatternsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#as_pattern}.
 * @param ctx the parse tree
 */
fn enter_as_pattern(&mut self, _ctx: &As_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#as_pattern}.
 * @param ctx the parse tree
 */
fn exit_as_pattern(&mut self, _ctx: &As_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#or_pattern}.
 * @param ctx the parse tree
 */
fn enter_or_pattern(&mut self, _ctx: &Or_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#or_pattern}.
 * @param ctx the parse tree
 */
fn exit_or_pattern(&mut self, _ctx: &Or_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#closed_pattern}.
 * @param ctx the parse tree
 */
fn enter_closed_pattern(&mut self, _ctx: &Closed_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#closed_pattern}.
 * @param ctx the parse tree
 */
fn exit_closed_pattern(&mut self, _ctx: &Closed_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#literal_pattern}.
 * @param ctx the parse tree
 */
fn enter_literal_pattern(&mut self, _ctx: &Literal_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#literal_pattern}.
 * @param ctx the parse tree
 */
fn exit_literal_pattern(&mut self, _ctx: &Literal_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#literal_expr}.
 * @param ctx the parse tree
 */
fn enter_literal_expr(&mut self, _ctx: &Literal_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#literal_expr}.
 * @param ctx the parse tree
 */
fn exit_literal_expr(&mut self, _ctx: &Literal_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#complex_number}.
 * @param ctx the parse tree
 */
fn enter_complex_number(&mut self, _ctx: &Complex_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#complex_number}.
 * @param ctx the parse tree
 */
fn exit_complex_number(&mut self, _ctx: &Complex_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#signed_number}.
 * @param ctx the parse tree
 */
fn enter_signed_number(&mut self, _ctx: &Signed_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#signed_number}.
 * @param ctx the parse tree
 */
fn exit_signed_number(&mut self, _ctx: &Signed_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#signed_real_number}.
 * @param ctx the parse tree
 */
fn enter_signed_real_number(&mut self, _ctx: &Signed_real_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#signed_real_number}.
 * @param ctx the parse tree
 */
fn exit_signed_real_number(&mut self, _ctx: &Signed_real_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#real_number}.
 * @param ctx the parse tree
 */
fn enter_real_number(&mut self, _ctx: &Real_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#real_number}.
 * @param ctx the parse tree
 */
fn exit_real_number(&mut self, _ctx: &Real_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#imaginary_number}.
 * @param ctx the parse tree
 */
fn enter_imaginary_number(&mut self, _ctx: &Imaginary_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#imaginary_number}.
 * @param ctx the parse tree
 */
fn exit_imaginary_number(&mut self, _ctx: &Imaginary_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#capture_pattern}.
 * @param ctx the parse tree
 */
fn enter_capture_pattern(&mut self, _ctx: &Capture_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#capture_pattern}.
 * @param ctx the parse tree
 */
fn exit_capture_pattern(&mut self, _ctx: &Capture_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#pattern_capture_target}.
 * @param ctx the parse tree
 */
fn enter_pattern_capture_target(&mut self, _ctx: &Pattern_capture_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#pattern_capture_target}.
 * @param ctx the parse tree
 */
fn exit_pattern_capture_target(&mut self, _ctx: &Pattern_capture_targetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#wildcard_pattern}.
 * @param ctx the parse tree
 */
fn enter_wildcard_pattern(&mut self, _ctx: &Wildcard_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#wildcard_pattern}.
 * @param ctx the parse tree
 */
fn exit_wildcard_pattern(&mut self, _ctx: &Wildcard_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#value_pattern}.
 * @param ctx the parse tree
 */
fn enter_value_pattern(&mut self, _ctx: &Value_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#value_pattern}.
 * @param ctx the parse tree
 */
fn exit_value_pattern(&mut self, _ctx: &Value_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#attr}.
 * @param ctx the parse tree
 */
fn enter_attr(&mut self, _ctx: &AttrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#attr}.
 * @param ctx the parse tree
 */
fn exit_attr(&mut self, _ctx: &AttrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#name_or_attr}.
 * @param ctx the parse tree
 */
fn enter_name_or_attr(&mut self, _ctx: &Name_or_attrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#name_or_attr}.
 * @param ctx the parse tree
 */
fn exit_name_or_attr(&mut self, _ctx: &Name_or_attrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#group_pattern}.
 * @param ctx the parse tree
 */
fn enter_group_pattern(&mut self, _ctx: &Group_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#group_pattern}.
 * @param ctx the parse tree
 */
fn exit_group_pattern(&mut self, _ctx: &Group_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#sequence_pattern}.
 * @param ctx the parse tree
 */
fn enter_sequence_pattern(&mut self, _ctx: &Sequence_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#sequence_pattern}.
 * @param ctx the parse tree
 */
fn exit_sequence_pattern(&mut self, _ctx: &Sequence_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#open_sequence_pattern}.
 * @param ctx the parse tree
 */
fn enter_open_sequence_pattern(&mut self, _ctx: &Open_sequence_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#open_sequence_pattern}.
 * @param ctx the parse tree
 */
fn exit_open_sequence_pattern(&mut self, _ctx: &Open_sequence_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#maybe_sequence_pattern}.
 * @param ctx the parse tree
 */
fn enter_maybe_sequence_pattern(&mut self, _ctx: &Maybe_sequence_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#maybe_sequence_pattern}.
 * @param ctx the parse tree
 */
fn exit_maybe_sequence_pattern(&mut self, _ctx: &Maybe_sequence_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#maybe_star_pattern}.
 * @param ctx the parse tree
 */
fn enter_maybe_star_pattern(&mut self, _ctx: &Maybe_star_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#maybe_star_pattern}.
 * @param ctx the parse tree
 */
fn exit_maybe_star_pattern(&mut self, _ctx: &Maybe_star_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#star_pattern}.
 * @param ctx the parse tree
 */
fn enter_star_pattern(&mut self, _ctx: &Star_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#star_pattern}.
 * @param ctx the parse tree
 */
fn exit_star_pattern(&mut self, _ctx: &Star_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#mapping_pattern}.
 * @param ctx the parse tree
 */
fn enter_mapping_pattern(&mut self, _ctx: &Mapping_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#mapping_pattern}.
 * @param ctx the parse tree
 */
fn exit_mapping_pattern(&mut self, _ctx: &Mapping_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#items_pattern}.
 * @param ctx the parse tree
 */
fn enter_items_pattern(&mut self, _ctx: &Items_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#items_pattern}.
 * @param ctx the parse tree
 */
fn exit_items_pattern(&mut self, _ctx: &Items_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#key_value_pattern}.
 * @param ctx the parse tree
 */
fn enter_key_value_pattern(&mut self, _ctx: &Key_value_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#key_value_pattern}.
 * @param ctx the parse tree
 */
fn exit_key_value_pattern(&mut self, _ctx: &Key_value_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#double_star_pattern}.
 * @param ctx the parse tree
 */
fn enter_double_star_pattern(&mut self, _ctx: &Double_star_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#double_star_pattern}.
 * @param ctx the parse tree
 */
fn exit_double_star_pattern(&mut self, _ctx: &Double_star_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#class_pattern}.
 * @param ctx the parse tree
 */
fn enter_class_pattern(&mut self, _ctx: &Class_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#class_pattern}.
 * @param ctx the parse tree
 */
fn exit_class_pattern(&mut self, _ctx: &Class_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#positional_patterns}.
 * @param ctx the parse tree
 */
fn enter_positional_patterns(&mut self, _ctx: &Positional_patternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#positional_patterns}.
 * @param ctx the parse tree
 */
fn exit_positional_patterns(&mut self, _ctx: &Positional_patternsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#keyword_patterns}.
 * @param ctx the parse tree
 */
fn enter_keyword_patterns(&mut self, _ctx: &Keyword_patternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#keyword_patterns}.
 * @param ctx the parse tree
 */
fn exit_keyword_patterns(&mut self, _ctx: &Keyword_patternsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#keyword_pattern}.
 * @param ctx the parse tree
 */
fn enter_keyword_pattern(&mut self, _ctx: &Keyword_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#keyword_pattern}.
 * @param ctx the parse tree
 */
fn exit_keyword_pattern(&mut self, _ctx: &Keyword_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#test}.
 * @param ctx the parse tree
 */
fn enter_test(&mut self, _ctx: &TestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#test}.
 * @param ctx the parse tree
 */
fn exit_test(&mut self, _ctx: &TestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#test_nocond}.
 * @param ctx the parse tree
 */
fn enter_test_nocond(&mut self, _ctx: &Test_nocondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#test_nocond}.
 * @param ctx the parse tree
 */
fn exit_test_nocond(&mut self, _ctx: &Test_nocondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#lambdef}.
 * @param ctx the parse tree
 */
fn enter_lambdef(&mut self, _ctx: &LambdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#lambdef}.
 * @param ctx the parse tree
 */
fn exit_lambdef(&mut self, _ctx: &LambdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#lambdef_nocond}.
 * @param ctx the parse tree
 */
fn enter_lambdef_nocond(&mut self, _ctx: &Lambdef_nocondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#lambdef_nocond}.
 * @param ctx the parse tree
 */
fn exit_lambdef_nocond(&mut self, _ctx: &Lambdef_nocondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#or_test}.
 * @param ctx the parse tree
 */
fn enter_or_test(&mut self, _ctx: &Or_testContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#or_test}.
 * @param ctx the parse tree
 */
fn exit_or_test(&mut self, _ctx: &Or_testContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#and_test}.
 * @param ctx the parse tree
 */
fn enter_and_test(&mut self, _ctx: &And_testContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#and_test}.
 * @param ctx the parse tree
 */
fn exit_and_test(&mut self, _ctx: &And_testContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#not_test}.
 * @param ctx the parse tree
 */
fn enter_not_test(&mut self, _ctx: &Not_testContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#not_test}.
 * @param ctx the parse tree
 */
fn exit_not_test(&mut self, _ctx: &Not_testContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#comparison}.
 * @param ctx the parse tree
 */
fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#comparison}.
 * @param ctx the parse tree
 */
fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#comp_op}.
 * @param ctx the parse tree
 */
fn enter_comp_op(&mut self, _ctx: &Comp_opContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#comp_op}.
 * @param ctx the parse tree
 */
fn exit_comp_op(&mut self, _ctx: &Comp_opContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#star_expr}.
 * @param ctx the parse tree
 */
fn enter_star_expr(&mut self, _ctx: &Star_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#star_expr}.
 * @param ctx the parse tree
 */
fn exit_star_expr(&mut self, _ctx: &Star_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#atom_expr}.
 * @param ctx the parse tree
 */
fn enter_atom_expr(&mut self, _ctx: &Atom_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#atom_expr}.
 * @param ctx the parse tree
 */
fn exit_atom_expr(&mut self, _ctx: &Atom_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#atom}.
 * @param ctx the parse tree
 */
fn enter_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#atom}.
 * @param ctx the parse tree
 */
fn exit_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#name}.
 * @param ctx the parse tree
 */
fn enter_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#name}.
 * @param ctx the parse tree
 */
fn exit_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#testlist_comp}.
 * @param ctx the parse tree
 */
fn enter_testlist_comp(&mut self, _ctx: &Testlist_compContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#testlist_comp}.
 * @param ctx the parse tree
 */
fn exit_testlist_comp(&mut self, _ctx: &Testlist_compContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#trailer}.
 * @param ctx the parse tree
 */
fn enter_trailer(&mut self, _ctx: &TrailerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#trailer}.
 * @param ctx the parse tree
 */
fn exit_trailer(&mut self, _ctx: &TrailerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#subscriptlist}.
 * @param ctx the parse tree
 */
fn enter_subscriptlist(&mut self, _ctx: &SubscriptlistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#subscriptlist}.
 * @param ctx the parse tree
 */
fn exit_subscriptlist(&mut self, _ctx: &SubscriptlistContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#subscript_}.
 * @param ctx the parse tree
 */
fn enter_subscript_(&mut self, _ctx: &Subscript_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#subscript_}.
 * @param ctx the parse tree
 */
fn exit_subscript_(&mut self, _ctx: &Subscript_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#sliceop}.
 * @param ctx the parse tree
 */
fn enter_sliceop(&mut self, _ctx: &SliceopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#sliceop}.
 * @param ctx the parse tree
 */
fn exit_sliceop(&mut self, _ctx: &SliceopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#exprlist}.
 * @param ctx the parse tree
 */
fn enter_exprlist(&mut self, _ctx: &ExprlistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#exprlist}.
 * @param ctx the parse tree
 */
fn exit_exprlist(&mut self, _ctx: &ExprlistContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#testlist}.
 * @param ctx the parse tree
 */
fn enter_testlist(&mut self, _ctx: &TestlistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#testlist}.
 * @param ctx the parse tree
 */
fn exit_testlist(&mut self, _ctx: &TestlistContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#dictorsetmaker}.
 * @param ctx the parse tree
 */
fn enter_dictorsetmaker(&mut self, _ctx: &DictorsetmakerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#dictorsetmaker}.
 * @param ctx the parse tree
 */
fn exit_dictorsetmaker(&mut self, _ctx: &DictorsetmakerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#classdef}.
 * @param ctx the parse tree
 */
fn enter_classdef(&mut self, _ctx: &ClassdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#classdef}.
 * @param ctx the parse tree
 */
fn exit_classdef(&mut self, _ctx: &ClassdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#arglist}.
 * @param ctx the parse tree
 */
fn enter_arglist(&mut self, _ctx: &ArglistContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#arglist}.
 * @param ctx the parse tree
 */
fn exit_arglist(&mut self, _ctx: &ArglistContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#argument}.
 * @param ctx the parse tree
 */
fn enter_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#argument}.
 * @param ctx the parse tree
 */
fn exit_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#comp_iter}.
 * @param ctx the parse tree
 */
fn enter_comp_iter(&mut self, _ctx: &Comp_iterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#comp_iter}.
 * @param ctx the parse tree
 */
fn exit_comp_iter(&mut self, _ctx: &Comp_iterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#comp_for}.
 * @param ctx the parse tree
 */
fn enter_comp_for(&mut self, _ctx: &Comp_forContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#comp_for}.
 * @param ctx the parse tree
 */
fn exit_comp_for(&mut self, _ctx: &Comp_forContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#comp_if}.
 * @param ctx the parse tree
 */
fn enter_comp_if(&mut self, _ctx: &Comp_ifContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#comp_if}.
 * @param ctx the parse tree
 */
fn exit_comp_if(&mut self, _ctx: &Comp_ifContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#encoding_decl}.
 * @param ctx the parse tree
 */
fn enter_encoding_decl(&mut self, _ctx: &Encoding_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#encoding_decl}.
 * @param ctx the parse tree
 */
fn exit_encoding_decl(&mut self, _ctx: &Encoding_declContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#yield_expr}.
 * @param ctx the parse tree
 */
fn enter_yield_expr(&mut self, _ctx: &Yield_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#yield_expr}.
 * @param ctx the parse tree
 */
fn exit_yield_expr(&mut self, _ctx: &Yield_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#yield_arg}.
 * @param ctx the parse tree
 */
fn enter_yield_arg(&mut self, _ctx: &Yield_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#yield_arg}.
 * @param ctx the parse tree
 */
fn exit_yield_arg(&mut self, _ctx: &Yield_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Python3Parser#strings}.
 * @param ctx the parse tree
 */
fn enter_strings(&mut self, _ctx: &StringsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Python3Parser#strings}.
 * @param ctx the parse tree
 */
fn exit_strings(&mut self, _ctx: &StringsContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : Python3ParserListener<'input> }


