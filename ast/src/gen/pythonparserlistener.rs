#![allow(nonstandard_style)]
// Generated from PythonParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::pythonparser::*;

pub trait PythonParserListener<'input> : ParseTreeListener<'input,PythonParserContextType>{
/**
 * Enter a parse tree produced by {@link PythonParser#file_input}.
 * @param ctx the parse tree
 */
fn enter_file_input(&mut self, _ctx: &File_inputContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#file_input}.
 * @param ctx the parse tree
 */
fn exit_file_input(&mut self, _ctx: &File_inputContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#interactive}.
 * @param ctx the parse tree
 */
fn enter_interactive(&mut self, _ctx: &InteractiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#interactive}.
 * @param ctx the parse tree
 */
fn exit_interactive(&mut self, _ctx: &InteractiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#eval}.
 * @param ctx the parse tree
 */
fn enter_eval(&mut self, _ctx: &EvalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#eval}.
 * @param ctx the parse tree
 */
fn exit_eval(&mut self, _ctx: &EvalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#func_type}.
 * @param ctx the parse tree
 */
fn enter_func_type(&mut self, _ctx: &Func_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#func_type}.
 * @param ctx the parse tree
 */
fn exit_func_type(&mut self, _ctx: &Func_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#statements}.
 * @param ctx the parse tree
 */
fn enter_statements(&mut self, _ctx: &StatementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#statements}.
 * @param ctx the parse tree
 */
fn exit_statements(&mut self, _ctx: &StatementsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#statement_newline}.
 * @param ctx the parse tree
 */
fn enter_statement_newline(&mut self, _ctx: &Statement_newlineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#statement_newline}.
 * @param ctx the parse tree
 */
fn exit_statement_newline(&mut self, _ctx: &Statement_newlineContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#simple_stmts}.
 * @param ctx the parse tree
 */
fn enter_simple_stmts(&mut self, _ctx: &Simple_stmtsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#simple_stmts}.
 * @param ctx the parse tree
 */
fn exit_simple_stmts(&mut self, _ctx: &Simple_stmtsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#simple_stmt}.
 * @param ctx the parse tree
 */
fn enter_simple_stmt(&mut self, _ctx: &Simple_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#simple_stmt}.
 * @param ctx the parse tree
 */
fn exit_simple_stmt(&mut self, _ctx: &Simple_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#compound_stmt}.
 * @param ctx the parse tree
 */
fn enter_compound_stmt(&mut self, _ctx: &Compound_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#compound_stmt}.
 * @param ctx the parse tree
 */
fn exit_compound_stmt(&mut self, _ctx: &Compound_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#annotated_rhs}.
 * @param ctx the parse tree
 */
fn enter_annotated_rhs(&mut self, _ctx: &Annotated_rhsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#annotated_rhs}.
 * @param ctx the parse tree
 */
fn exit_annotated_rhs(&mut self, _ctx: &Annotated_rhsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#augassign}.
 * @param ctx the parse tree
 */
fn enter_augassign(&mut self, _ctx: &AugassignContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#augassign}.
 * @param ctx the parse tree
 */
fn exit_augassign(&mut self, _ctx: &AugassignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#return_stmt}.
 * @param ctx the parse tree
 */
fn enter_return_stmt(&mut self, _ctx: &Return_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#return_stmt}.
 * @param ctx the parse tree
 */
fn exit_return_stmt(&mut self, _ctx: &Return_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#raise_stmt}.
 * @param ctx the parse tree
 */
fn enter_raise_stmt(&mut self, _ctx: &Raise_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#raise_stmt}.
 * @param ctx the parse tree
 */
fn exit_raise_stmt(&mut self, _ctx: &Raise_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#global_stmt}.
 * @param ctx the parse tree
 */
fn enter_global_stmt(&mut self, _ctx: &Global_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#global_stmt}.
 * @param ctx the parse tree
 */
fn exit_global_stmt(&mut self, _ctx: &Global_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#nonlocal_stmt}.
 * @param ctx the parse tree
 */
fn enter_nonlocal_stmt(&mut self, _ctx: &Nonlocal_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#nonlocal_stmt}.
 * @param ctx the parse tree
 */
fn exit_nonlocal_stmt(&mut self, _ctx: &Nonlocal_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#del_stmt}.
 * @param ctx the parse tree
 */
fn enter_del_stmt(&mut self, _ctx: &Del_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#del_stmt}.
 * @param ctx the parse tree
 */
fn exit_del_stmt(&mut self, _ctx: &Del_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#yield_stmt}.
 * @param ctx the parse tree
 */
fn enter_yield_stmt(&mut self, _ctx: &Yield_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#yield_stmt}.
 * @param ctx the parse tree
 */
fn exit_yield_stmt(&mut self, _ctx: &Yield_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#assert_stmt}.
 * @param ctx the parse tree
 */
fn enter_assert_stmt(&mut self, _ctx: &Assert_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#assert_stmt}.
 * @param ctx the parse tree
 */
fn exit_assert_stmt(&mut self, _ctx: &Assert_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#import_stmt}.
 * @param ctx the parse tree
 */
fn enter_import_stmt(&mut self, _ctx: &Import_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#import_stmt}.
 * @param ctx the parse tree
 */
fn exit_import_stmt(&mut self, _ctx: &Import_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#import_name}.
 * @param ctx the parse tree
 */
fn enter_import_name(&mut self, _ctx: &Import_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#import_name}.
 * @param ctx the parse tree
 */
fn exit_import_name(&mut self, _ctx: &Import_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#import_from}.
 * @param ctx the parse tree
 */
fn enter_import_from(&mut self, _ctx: &Import_fromContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#import_from}.
 * @param ctx the parse tree
 */
fn exit_import_from(&mut self, _ctx: &Import_fromContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#import_from_targets}.
 * @param ctx the parse tree
 */
fn enter_import_from_targets(&mut self, _ctx: &Import_from_targetsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#import_from_targets}.
 * @param ctx the parse tree
 */
fn exit_import_from_targets(&mut self, _ctx: &Import_from_targetsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#import_from_as_names}.
 * @param ctx the parse tree
 */
fn enter_import_from_as_names(&mut self, _ctx: &Import_from_as_namesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#import_from_as_names}.
 * @param ctx the parse tree
 */
fn exit_import_from_as_names(&mut self, _ctx: &Import_from_as_namesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#import_from_as_name}.
 * @param ctx the parse tree
 */
fn enter_import_from_as_name(&mut self, _ctx: &Import_from_as_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#import_from_as_name}.
 * @param ctx the parse tree
 */
fn exit_import_from_as_name(&mut self, _ctx: &Import_from_as_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#dotted_as_names}.
 * @param ctx the parse tree
 */
fn enter_dotted_as_names(&mut self, _ctx: &Dotted_as_namesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#dotted_as_names}.
 * @param ctx the parse tree
 */
fn exit_dotted_as_names(&mut self, _ctx: &Dotted_as_namesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#dotted_as_name}.
 * @param ctx the parse tree
 */
fn enter_dotted_as_name(&mut self, _ctx: &Dotted_as_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#dotted_as_name}.
 * @param ctx the parse tree
 */
fn exit_dotted_as_name(&mut self, _ctx: &Dotted_as_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#dotted_name}.
 * @param ctx the parse tree
 */
fn enter_dotted_name(&mut self, _ctx: &Dotted_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#dotted_name}.
 * @param ctx the parse tree
 */
fn exit_dotted_name(&mut self, _ctx: &Dotted_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#decorators}.
 * @param ctx the parse tree
 */
fn enter_decorators(&mut self, _ctx: &DecoratorsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#decorators}.
 * @param ctx the parse tree
 */
fn exit_decorators(&mut self, _ctx: &DecoratorsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#class_def}.
 * @param ctx the parse tree
 */
fn enter_class_def(&mut self, _ctx: &Class_defContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#class_def}.
 * @param ctx the parse tree
 */
fn exit_class_def(&mut self, _ctx: &Class_defContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#class_def_raw}.
 * @param ctx the parse tree
 */
fn enter_class_def_raw(&mut self, _ctx: &Class_def_rawContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#class_def_raw}.
 * @param ctx the parse tree
 */
fn exit_class_def_raw(&mut self, _ctx: &Class_def_rawContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#function_def}.
 * @param ctx the parse tree
 */
fn enter_function_def(&mut self, _ctx: &Function_defContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#function_def}.
 * @param ctx the parse tree
 */
fn exit_function_def(&mut self, _ctx: &Function_defContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#function_def_raw}.
 * @param ctx the parse tree
 */
fn enter_function_def_raw(&mut self, _ctx: &Function_def_rawContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#function_def_raw}.
 * @param ctx the parse tree
 */
fn exit_function_def_raw(&mut self, _ctx: &Function_def_rawContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#params}.
 * @param ctx the parse tree
 */
fn enter_params(&mut self, _ctx: &ParamsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#params}.
 * @param ctx the parse tree
 */
fn exit_params(&mut self, _ctx: &ParamsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#parameters}.
 * @param ctx the parse tree
 */
fn enter_parameters(&mut self, _ctx: &ParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#parameters}.
 * @param ctx the parse tree
 */
fn exit_parameters(&mut self, _ctx: &ParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#slash_no_default}.
 * @param ctx the parse tree
 */
fn enter_slash_no_default(&mut self, _ctx: &Slash_no_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#slash_no_default}.
 * @param ctx the parse tree
 */
fn exit_slash_no_default(&mut self, _ctx: &Slash_no_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#slash_with_default}.
 * @param ctx the parse tree
 */
fn enter_slash_with_default(&mut self, _ctx: &Slash_with_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#slash_with_default}.
 * @param ctx the parse tree
 */
fn exit_slash_with_default(&mut self, _ctx: &Slash_with_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_etc}.
 * @param ctx the parse tree
 */
fn enter_star_etc(&mut self, _ctx: &Star_etcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_etc}.
 * @param ctx the parse tree
 */
fn exit_star_etc(&mut self, _ctx: &Star_etcContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#kwds}.
 * @param ctx the parse tree
 */
fn enter_kwds(&mut self, _ctx: &KwdsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#kwds}.
 * @param ctx the parse tree
 */
fn exit_kwds(&mut self, _ctx: &KwdsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#param_no_default}.
 * @param ctx the parse tree
 */
fn enter_param_no_default(&mut self, _ctx: &Param_no_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#param_no_default}.
 * @param ctx the parse tree
 */
fn exit_param_no_default(&mut self, _ctx: &Param_no_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#param_no_default_star_annotation}.
 * @param ctx the parse tree
 */
fn enter_param_no_default_star_annotation(&mut self, _ctx: &Param_no_default_star_annotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#param_no_default_star_annotation}.
 * @param ctx the parse tree
 */
fn exit_param_no_default_star_annotation(&mut self, _ctx: &Param_no_default_star_annotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#param_with_default}.
 * @param ctx the parse tree
 */
fn enter_param_with_default(&mut self, _ctx: &Param_with_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#param_with_default}.
 * @param ctx the parse tree
 */
fn exit_param_with_default(&mut self, _ctx: &Param_with_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#param_maybe_default}.
 * @param ctx the parse tree
 */
fn enter_param_maybe_default(&mut self, _ctx: &Param_maybe_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#param_maybe_default}.
 * @param ctx the parse tree
 */
fn exit_param_maybe_default(&mut self, _ctx: &Param_maybe_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#param}.
 * @param ctx the parse tree
 */
fn enter_param(&mut self, _ctx: &ParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#param}.
 * @param ctx the parse tree
 */
fn exit_param(&mut self, _ctx: &ParamContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#param_star_annotation}.
 * @param ctx the parse tree
 */
fn enter_param_star_annotation(&mut self, _ctx: &Param_star_annotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#param_star_annotation}.
 * @param ctx the parse tree
 */
fn exit_param_star_annotation(&mut self, _ctx: &Param_star_annotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#annotation}.
 * @param ctx the parse tree
 */
fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#annotation}.
 * @param ctx the parse tree
 */
fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_annotation}.
 * @param ctx the parse tree
 */
fn enter_star_annotation(&mut self, _ctx: &Star_annotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_annotation}.
 * @param ctx the parse tree
 */
fn exit_star_annotation(&mut self, _ctx: &Star_annotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#default_assignment}.
 * @param ctx the parse tree
 */
fn enter_default_assignment(&mut self, _ctx: &Default_assignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#default_assignment}.
 * @param ctx the parse tree
 */
fn exit_default_assignment(&mut self, _ctx: &Default_assignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#if_stmt}.
 * @param ctx the parse tree
 */
fn enter_if_stmt(&mut self, _ctx: &If_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#if_stmt}.
 * @param ctx the parse tree
 */
fn exit_if_stmt(&mut self, _ctx: &If_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#elif_stmt}.
 * @param ctx the parse tree
 */
fn enter_elif_stmt(&mut self, _ctx: &Elif_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#elif_stmt}.
 * @param ctx the parse tree
 */
fn exit_elif_stmt(&mut self, _ctx: &Elif_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#else_block}.
 * @param ctx the parse tree
 */
fn enter_else_block(&mut self, _ctx: &Else_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#else_block}.
 * @param ctx the parse tree
 */
fn exit_else_block(&mut self, _ctx: &Else_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#while_stmt}.
 * @param ctx the parse tree
 */
fn enter_while_stmt(&mut self, _ctx: &While_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#while_stmt}.
 * @param ctx the parse tree
 */
fn exit_while_stmt(&mut self, _ctx: &While_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#for_stmt}.
 * @param ctx the parse tree
 */
fn enter_for_stmt(&mut self, _ctx: &For_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#for_stmt}.
 * @param ctx the parse tree
 */
fn exit_for_stmt(&mut self, _ctx: &For_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#with_stmt}.
 * @param ctx the parse tree
 */
fn enter_with_stmt(&mut self, _ctx: &With_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#with_stmt}.
 * @param ctx the parse tree
 */
fn exit_with_stmt(&mut self, _ctx: &With_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#with_item}.
 * @param ctx the parse tree
 */
fn enter_with_item(&mut self, _ctx: &With_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#with_item}.
 * @param ctx the parse tree
 */
fn exit_with_item(&mut self, _ctx: &With_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#try_stmt}.
 * @param ctx the parse tree
 */
fn enter_try_stmt(&mut self, _ctx: &Try_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#try_stmt}.
 * @param ctx the parse tree
 */
fn exit_try_stmt(&mut self, _ctx: &Try_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#except_block}.
 * @param ctx the parse tree
 */
fn enter_except_block(&mut self, _ctx: &Except_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#except_block}.
 * @param ctx the parse tree
 */
fn exit_except_block(&mut self, _ctx: &Except_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#except_star_block}.
 * @param ctx the parse tree
 */
fn enter_except_star_block(&mut self, _ctx: &Except_star_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#except_star_block}.
 * @param ctx the parse tree
 */
fn exit_except_star_block(&mut self, _ctx: &Except_star_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#finally_block}.
 * @param ctx the parse tree
 */
fn enter_finally_block(&mut self, _ctx: &Finally_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#finally_block}.
 * @param ctx the parse tree
 */
fn exit_finally_block(&mut self, _ctx: &Finally_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#match_stmt}.
 * @param ctx the parse tree
 */
fn enter_match_stmt(&mut self, _ctx: &Match_stmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#match_stmt}.
 * @param ctx the parse tree
 */
fn exit_match_stmt(&mut self, _ctx: &Match_stmtContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#subject_expr}.
 * @param ctx the parse tree
 */
fn enter_subject_expr(&mut self, _ctx: &Subject_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#subject_expr}.
 * @param ctx the parse tree
 */
fn exit_subject_expr(&mut self, _ctx: &Subject_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#case_block}.
 * @param ctx the parse tree
 */
fn enter_case_block(&mut self, _ctx: &Case_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#case_block}.
 * @param ctx the parse tree
 */
fn exit_case_block(&mut self, _ctx: &Case_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#guard}.
 * @param ctx the parse tree
 */
fn enter_guard(&mut self, _ctx: &GuardContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#guard}.
 * @param ctx the parse tree
 */
fn exit_guard(&mut self, _ctx: &GuardContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#patterns}.
 * @param ctx the parse tree
 */
fn enter_patterns(&mut self, _ctx: &PatternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#patterns}.
 * @param ctx the parse tree
 */
fn exit_patterns(&mut self, _ctx: &PatternsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#as_pattern}.
 * @param ctx the parse tree
 */
fn enter_as_pattern(&mut self, _ctx: &As_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#as_pattern}.
 * @param ctx the parse tree
 */
fn exit_as_pattern(&mut self, _ctx: &As_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#or_pattern}.
 * @param ctx the parse tree
 */
fn enter_or_pattern(&mut self, _ctx: &Or_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#or_pattern}.
 * @param ctx the parse tree
 */
fn exit_or_pattern(&mut self, _ctx: &Or_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#closed_pattern}.
 * @param ctx the parse tree
 */
fn enter_closed_pattern(&mut self, _ctx: &Closed_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#closed_pattern}.
 * @param ctx the parse tree
 */
fn exit_closed_pattern(&mut self, _ctx: &Closed_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#literal_pattern}.
 * @param ctx the parse tree
 */
fn enter_literal_pattern(&mut self, _ctx: &Literal_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#literal_pattern}.
 * @param ctx the parse tree
 */
fn exit_literal_pattern(&mut self, _ctx: &Literal_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#literal_expr}.
 * @param ctx the parse tree
 */
fn enter_literal_expr(&mut self, _ctx: &Literal_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#literal_expr}.
 * @param ctx the parse tree
 */
fn exit_literal_expr(&mut self, _ctx: &Literal_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#complex_number}.
 * @param ctx the parse tree
 */
fn enter_complex_number(&mut self, _ctx: &Complex_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#complex_number}.
 * @param ctx the parse tree
 */
fn exit_complex_number(&mut self, _ctx: &Complex_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#signed_number}.
 * @param ctx the parse tree
 */
fn enter_signed_number(&mut self, _ctx: &Signed_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#signed_number}.
 * @param ctx the parse tree
 */
fn exit_signed_number(&mut self, _ctx: &Signed_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#signed_real_number}.
 * @param ctx the parse tree
 */
fn enter_signed_real_number(&mut self, _ctx: &Signed_real_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#signed_real_number}.
 * @param ctx the parse tree
 */
fn exit_signed_real_number(&mut self, _ctx: &Signed_real_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#real_number}.
 * @param ctx the parse tree
 */
fn enter_real_number(&mut self, _ctx: &Real_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#real_number}.
 * @param ctx the parse tree
 */
fn exit_real_number(&mut self, _ctx: &Real_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#imaginary_number}.
 * @param ctx the parse tree
 */
fn enter_imaginary_number(&mut self, _ctx: &Imaginary_numberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#imaginary_number}.
 * @param ctx the parse tree
 */
fn exit_imaginary_number(&mut self, _ctx: &Imaginary_numberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#capture_pattern}.
 * @param ctx the parse tree
 */
fn enter_capture_pattern(&mut self, _ctx: &Capture_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#capture_pattern}.
 * @param ctx the parse tree
 */
fn exit_capture_pattern(&mut self, _ctx: &Capture_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#pattern_capture_target}.
 * @param ctx the parse tree
 */
fn enter_pattern_capture_target(&mut self, _ctx: &Pattern_capture_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#pattern_capture_target}.
 * @param ctx the parse tree
 */
fn exit_pattern_capture_target(&mut self, _ctx: &Pattern_capture_targetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#wildcard_pattern}.
 * @param ctx the parse tree
 */
fn enter_wildcard_pattern(&mut self, _ctx: &Wildcard_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#wildcard_pattern}.
 * @param ctx the parse tree
 */
fn exit_wildcard_pattern(&mut self, _ctx: &Wildcard_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#value_pattern}.
 * @param ctx the parse tree
 */
fn enter_value_pattern(&mut self, _ctx: &Value_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#value_pattern}.
 * @param ctx the parse tree
 */
fn exit_value_pattern(&mut self, _ctx: &Value_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#attr}.
 * @param ctx the parse tree
 */
fn enter_attr(&mut self, _ctx: &AttrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#attr}.
 * @param ctx the parse tree
 */
fn exit_attr(&mut self, _ctx: &AttrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#name_or_attr}.
 * @param ctx the parse tree
 */
fn enter_name_or_attr(&mut self, _ctx: &Name_or_attrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#name_or_attr}.
 * @param ctx the parse tree
 */
fn exit_name_or_attr(&mut self, _ctx: &Name_or_attrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#group_pattern}.
 * @param ctx the parse tree
 */
fn enter_group_pattern(&mut self, _ctx: &Group_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#group_pattern}.
 * @param ctx the parse tree
 */
fn exit_group_pattern(&mut self, _ctx: &Group_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#sequence_pattern}.
 * @param ctx the parse tree
 */
fn enter_sequence_pattern(&mut self, _ctx: &Sequence_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#sequence_pattern}.
 * @param ctx the parse tree
 */
fn exit_sequence_pattern(&mut self, _ctx: &Sequence_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#open_sequence_pattern}.
 * @param ctx the parse tree
 */
fn enter_open_sequence_pattern(&mut self, _ctx: &Open_sequence_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#open_sequence_pattern}.
 * @param ctx the parse tree
 */
fn exit_open_sequence_pattern(&mut self, _ctx: &Open_sequence_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#maybe_sequence_pattern}.
 * @param ctx the parse tree
 */
fn enter_maybe_sequence_pattern(&mut self, _ctx: &Maybe_sequence_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#maybe_sequence_pattern}.
 * @param ctx the parse tree
 */
fn exit_maybe_sequence_pattern(&mut self, _ctx: &Maybe_sequence_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#maybe_star_pattern}.
 * @param ctx the parse tree
 */
fn enter_maybe_star_pattern(&mut self, _ctx: &Maybe_star_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#maybe_star_pattern}.
 * @param ctx the parse tree
 */
fn exit_maybe_star_pattern(&mut self, _ctx: &Maybe_star_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_pattern}.
 * @param ctx the parse tree
 */
fn enter_star_pattern(&mut self, _ctx: &Star_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_pattern}.
 * @param ctx the parse tree
 */
fn exit_star_pattern(&mut self, _ctx: &Star_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#mapping_pattern}.
 * @param ctx the parse tree
 */
fn enter_mapping_pattern(&mut self, _ctx: &Mapping_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#mapping_pattern}.
 * @param ctx the parse tree
 */
fn exit_mapping_pattern(&mut self, _ctx: &Mapping_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#items_pattern}.
 * @param ctx the parse tree
 */
fn enter_items_pattern(&mut self, _ctx: &Items_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#items_pattern}.
 * @param ctx the parse tree
 */
fn exit_items_pattern(&mut self, _ctx: &Items_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#key_value_pattern}.
 * @param ctx the parse tree
 */
fn enter_key_value_pattern(&mut self, _ctx: &Key_value_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#key_value_pattern}.
 * @param ctx the parse tree
 */
fn exit_key_value_pattern(&mut self, _ctx: &Key_value_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#double_star_pattern}.
 * @param ctx the parse tree
 */
fn enter_double_star_pattern(&mut self, _ctx: &Double_star_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#double_star_pattern}.
 * @param ctx the parse tree
 */
fn exit_double_star_pattern(&mut self, _ctx: &Double_star_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#class_pattern}.
 * @param ctx the parse tree
 */
fn enter_class_pattern(&mut self, _ctx: &Class_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#class_pattern}.
 * @param ctx the parse tree
 */
fn exit_class_pattern(&mut self, _ctx: &Class_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#positional_patterns}.
 * @param ctx the parse tree
 */
fn enter_positional_patterns(&mut self, _ctx: &Positional_patternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#positional_patterns}.
 * @param ctx the parse tree
 */
fn exit_positional_patterns(&mut self, _ctx: &Positional_patternsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#keyword_patterns}.
 * @param ctx the parse tree
 */
fn enter_keyword_patterns(&mut self, _ctx: &Keyword_patternsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#keyword_patterns}.
 * @param ctx the parse tree
 */
fn exit_keyword_patterns(&mut self, _ctx: &Keyword_patternsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#keyword_pattern}.
 * @param ctx the parse tree
 */
fn enter_keyword_pattern(&mut self, _ctx: &Keyword_patternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#keyword_pattern}.
 * @param ctx the parse tree
 */
fn exit_keyword_pattern(&mut self, _ctx: &Keyword_patternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_alias}.
 * @param ctx the parse tree
 */
fn enter_type_alias(&mut self, _ctx: &Type_aliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_alias}.
 * @param ctx the parse tree
 */
fn exit_type_alias(&mut self, _ctx: &Type_aliasContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_params}.
 * @param ctx the parse tree
 */
fn enter_type_params(&mut self, _ctx: &Type_paramsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_params}.
 * @param ctx the parse tree
 */
fn exit_type_params(&mut self, _ctx: &Type_paramsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_param_seq}.
 * @param ctx the parse tree
 */
fn enter_type_param_seq(&mut self, _ctx: &Type_param_seqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_param_seq}.
 * @param ctx the parse tree
 */
fn exit_type_param_seq(&mut self, _ctx: &Type_param_seqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_param}.
 * @param ctx the parse tree
 */
fn enter_type_param(&mut self, _ctx: &Type_paramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_param}.
 * @param ctx the parse tree
 */
fn exit_type_param(&mut self, _ctx: &Type_paramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_param_bound}.
 * @param ctx the parse tree
 */
fn enter_type_param_bound(&mut self, _ctx: &Type_param_boundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_param_bound}.
 * @param ctx the parse tree
 */
fn exit_type_param_bound(&mut self, _ctx: &Type_param_boundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_param_default}.
 * @param ctx the parse tree
 */
fn enter_type_param_default(&mut self, _ctx: &Type_param_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_param_default}.
 * @param ctx the parse tree
 */
fn exit_type_param_default(&mut self, _ctx: &Type_param_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_param_starred_default}.
 * @param ctx the parse tree
 */
fn enter_type_param_starred_default(&mut self, _ctx: &Type_param_starred_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_param_starred_default}.
 * @param ctx the parse tree
 */
fn exit_type_param_starred_default(&mut self, _ctx: &Type_param_starred_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#expressions}.
 * @param ctx the parse tree
 */
fn enter_expressions(&mut self, _ctx: &ExpressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#expressions}.
 * @param ctx the parse tree
 */
fn exit_expressions(&mut self, _ctx: &ExpressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#yield_expr}.
 * @param ctx the parse tree
 */
fn enter_yield_expr(&mut self, _ctx: &Yield_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#yield_expr}.
 * @param ctx the parse tree
 */
fn exit_yield_expr(&mut self, _ctx: &Yield_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_expressions}.
 * @param ctx the parse tree
 */
fn enter_star_expressions(&mut self, _ctx: &Star_expressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_expressions}.
 * @param ctx the parse tree
 */
fn exit_star_expressions(&mut self, _ctx: &Star_expressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_expression}.
 * @param ctx the parse tree
 */
fn enter_star_expression(&mut self, _ctx: &Star_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_expression}.
 * @param ctx the parse tree
 */
fn exit_star_expression(&mut self, _ctx: &Star_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_named_expressions}.
 * @param ctx the parse tree
 */
fn enter_star_named_expressions(&mut self, _ctx: &Star_named_expressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_named_expressions}.
 * @param ctx the parse tree
 */
fn exit_star_named_expressions(&mut self, _ctx: &Star_named_expressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_named_expression}.
 * @param ctx the parse tree
 */
fn enter_star_named_expression(&mut self, _ctx: &Star_named_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_named_expression}.
 * @param ctx the parse tree
 */
fn exit_star_named_expression(&mut self, _ctx: &Star_named_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#assignment_expression}.
 * @param ctx the parse tree
 */
fn enter_assignment_expression(&mut self, _ctx: &Assignment_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#assignment_expression}.
 * @param ctx the parse tree
 */
fn exit_assignment_expression(&mut self, _ctx: &Assignment_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#named_expression}.
 * @param ctx the parse tree
 */
fn enter_named_expression(&mut self, _ctx: &Named_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#named_expression}.
 * @param ctx the parse tree
 */
fn exit_named_expression(&mut self, _ctx: &Named_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#disjunction}.
 * @param ctx the parse tree
 */
fn enter_disjunction(&mut self, _ctx: &DisjunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#disjunction}.
 * @param ctx the parse tree
 */
fn exit_disjunction(&mut self, _ctx: &DisjunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#conjunction}.
 * @param ctx the parse tree
 */
fn enter_conjunction(&mut self, _ctx: &ConjunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#conjunction}.
 * @param ctx the parse tree
 */
fn exit_conjunction(&mut self, _ctx: &ConjunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#inversion}.
 * @param ctx the parse tree
 */
fn enter_inversion(&mut self, _ctx: &InversionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#inversion}.
 * @param ctx the parse tree
 */
fn exit_inversion(&mut self, _ctx: &InversionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#comparison}.
 * @param ctx the parse tree
 */
fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#comparison}.
 * @param ctx the parse tree
 */
fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#compare_op_bitwise_or_pair}.
 * @param ctx the parse tree
 */
fn enter_compare_op_bitwise_or_pair(&mut self, _ctx: &Compare_op_bitwise_or_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#compare_op_bitwise_or_pair}.
 * @param ctx the parse tree
 */
fn exit_compare_op_bitwise_or_pair(&mut self, _ctx: &Compare_op_bitwise_or_pairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#eq_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_eq_bitwise_or(&mut self, _ctx: &Eq_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#eq_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_eq_bitwise_or(&mut self, _ctx: &Eq_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#noteq_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_noteq_bitwise_or(&mut self, _ctx: &Noteq_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#noteq_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_noteq_bitwise_or(&mut self, _ctx: &Noteq_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lte_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_lte_bitwise_or(&mut self, _ctx: &Lte_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lte_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_lte_bitwise_or(&mut self, _ctx: &Lte_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lt_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_lt_bitwise_or(&mut self, _ctx: &Lt_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lt_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_lt_bitwise_or(&mut self, _ctx: &Lt_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#gte_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_gte_bitwise_or(&mut self, _ctx: &Gte_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#gte_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_gte_bitwise_or(&mut self, _ctx: &Gte_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#gt_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_gt_bitwise_or(&mut self, _ctx: &Gt_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#gt_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_gt_bitwise_or(&mut self, _ctx: &Gt_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#notin_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_notin_bitwise_or(&mut self, _ctx: &Notin_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#notin_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_notin_bitwise_or(&mut self, _ctx: &Notin_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#in_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_in_bitwise_or(&mut self, _ctx: &In_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#in_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_in_bitwise_or(&mut self, _ctx: &In_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#isnot_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_isnot_bitwise_or(&mut self, _ctx: &Isnot_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#isnot_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_isnot_bitwise_or(&mut self, _ctx: &Isnot_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#is_bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_is_bitwise_or(&mut self, _ctx: &Is_bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#is_bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_is_bitwise_or(&mut self, _ctx: &Is_bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#bitwise_or}.
 * @param ctx the parse tree
 */
fn enter_bitwise_or(&mut self, _ctx: &Bitwise_orContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#bitwise_or}.
 * @param ctx the parse tree
 */
fn exit_bitwise_or(&mut self, _ctx: &Bitwise_orContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#bitwise_xor}.
 * @param ctx the parse tree
 */
fn enter_bitwise_xor(&mut self, _ctx: &Bitwise_xorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#bitwise_xor}.
 * @param ctx the parse tree
 */
fn exit_bitwise_xor(&mut self, _ctx: &Bitwise_xorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#bitwise_and}.
 * @param ctx the parse tree
 */
fn enter_bitwise_and(&mut self, _ctx: &Bitwise_andContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#bitwise_and}.
 * @param ctx the parse tree
 */
fn exit_bitwise_and(&mut self, _ctx: &Bitwise_andContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#shift_expr}.
 * @param ctx the parse tree
 */
fn enter_shift_expr(&mut self, _ctx: &Shift_exprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#shift_expr}.
 * @param ctx the parse tree
 */
fn exit_shift_expr(&mut self, _ctx: &Shift_exprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#sum}.
 * @param ctx the parse tree
 */
fn enter_sum(&mut self, _ctx: &SumContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#sum}.
 * @param ctx the parse tree
 */
fn exit_sum(&mut self, _ctx: &SumContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#term}.
 * @param ctx the parse tree
 */
fn enter_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#term}.
 * @param ctx the parse tree
 */
fn exit_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#factor}.
 * @param ctx the parse tree
 */
fn enter_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#factor}.
 * @param ctx the parse tree
 */
fn exit_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#power}.
 * @param ctx the parse tree
 */
fn enter_power(&mut self, _ctx: &PowerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#power}.
 * @param ctx the parse tree
 */
fn exit_power(&mut self, _ctx: &PowerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#await_primary}.
 * @param ctx the parse tree
 */
fn enter_await_primary(&mut self, _ctx: &Await_primaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#await_primary}.
 * @param ctx the parse tree
 */
fn exit_await_primary(&mut self, _ctx: &Await_primaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#slices}.
 * @param ctx the parse tree
 */
fn enter_slices(&mut self, _ctx: &SlicesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#slices}.
 * @param ctx the parse tree
 */
fn exit_slices(&mut self, _ctx: &SlicesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#slice}.
 * @param ctx the parse tree
 */
fn enter_slice(&mut self, _ctx: &SliceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#slice}.
 * @param ctx the parse tree
 */
fn exit_slice(&mut self, _ctx: &SliceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#group}.
 * @param ctx the parse tree
 */
fn enter_group(&mut self, _ctx: &GroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#group}.
 * @param ctx the parse tree
 */
fn exit_group(&mut self, _ctx: &GroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambdef}.
 * @param ctx the parse tree
 */
fn enter_lambdef(&mut self, _ctx: &LambdefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambdef}.
 * @param ctx the parse tree
 */
fn exit_lambdef(&mut self, _ctx: &LambdefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_params}.
 * @param ctx the parse tree
 */
fn enter_lambda_params(&mut self, _ctx: &Lambda_paramsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_params}.
 * @param ctx the parse tree
 */
fn exit_lambda_params(&mut self, _ctx: &Lambda_paramsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_parameters}.
 * @param ctx the parse tree
 */
fn enter_lambda_parameters(&mut self, _ctx: &Lambda_parametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_parameters}.
 * @param ctx the parse tree
 */
fn exit_lambda_parameters(&mut self, _ctx: &Lambda_parametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_slash_no_default}.
 * @param ctx the parse tree
 */
fn enter_lambda_slash_no_default(&mut self, _ctx: &Lambda_slash_no_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_slash_no_default}.
 * @param ctx the parse tree
 */
fn exit_lambda_slash_no_default(&mut self, _ctx: &Lambda_slash_no_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_slash_with_default}.
 * @param ctx the parse tree
 */
fn enter_lambda_slash_with_default(&mut self, _ctx: &Lambda_slash_with_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_slash_with_default}.
 * @param ctx the parse tree
 */
fn exit_lambda_slash_with_default(&mut self, _ctx: &Lambda_slash_with_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_star_etc}.
 * @param ctx the parse tree
 */
fn enter_lambda_star_etc(&mut self, _ctx: &Lambda_star_etcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_star_etc}.
 * @param ctx the parse tree
 */
fn exit_lambda_star_etc(&mut self, _ctx: &Lambda_star_etcContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_kwds}.
 * @param ctx the parse tree
 */
fn enter_lambda_kwds(&mut self, _ctx: &Lambda_kwdsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_kwds}.
 * @param ctx the parse tree
 */
fn exit_lambda_kwds(&mut self, _ctx: &Lambda_kwdsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_param_no_default}.
 * @param ctx the parse tree
 */
fn enter_lambda_param_no_default(&mut self, _ctx: &Lambda_param_no_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_param_no_default}.
 * @param ctx the parse tree
 */
fn exit_lambda_param_no_default(&mut self, _ctx: &Lambda_param_no_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_param_with_default}.
 * @param ctx the parse tree
 */
fn enter_lambda_param_with_default(&mut self, _ctx: &Lambda_param_with_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_param_with_default}.
 * @param ctx the parse tree
 */
fn exit_lambda_param_with_default(&mut self, _ctx: &Lambda_param_with_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_param_maybe_default}.
 * @param ctx the parse tree
 */
fn enter_lambda_param_maybe_default(&mut self, _ctx: &Lambda_param_maybe_defaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_param_maybe_default}.
 * @param ctx the parse tree
 */
fn exit_lambda_param_maybe_default(&mut self, _ctx: &Lambda_param_maybe_defaultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#lambda_param}.
 * @param ctx the parse tree
 */
fn enter_lambda_param(&mut self, _ctx: &Lambda_paramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#lambda_param}.
 * @param ctx the parse tree
 */
fn exit_lambda_param(&mut self, _ctx: &Lambda_paramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#fstring_middle}.
 * @param ctx the parse tree
 */
fn enter_fstring_middle(&mut self, _ctx: &Fstring_middleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#fstring_middle}.
 * @param ctx the parse tree
 */
fn exit_fstring_middle(&mut self, _ctx: &Fstring_middleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#fstring_replacement_field}.
 * @param ctx the parse tree
 */
fn enter_fstring_replacement_field(&mut self, _ctx: &Fstring_replacement_fieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#fstring_replacement_field}.
 * @param ctx the parse tree
 */
fn exit_fstring_replacement_field(&mut self, _ctx: &Fstring_replacement_fieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#fstring_conversion}.
 * @param ctx the parse tree
 */
fn enter_fstring_conversion(&mut self, _ctx: &Fstring_conversionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#fstring_conversion}.
 * @param ctx the parse tree
 */
fn exit_fstring_conversion(&mut self, _ctx: &Fstring_conversionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#fstring_full_format_spec}.
 * @param ctx the parse tree
 */
fn enter_fstring_full_format_spec(&mut self, _ctx: &Fstring_full_format_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#fstring_full_format_spec}.
 * @param ctx the parse tree
 */
fn exit_fstring_full_format_spec(&mut self, _ctx: &Fstring_full_format_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#fstring_format_spec}.
 * @param ctx the parse tree
 */
fn enter_fstring_format_spec(&mut self, _ctx: &Fstring_format_specContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#fstring_format_spec}.
 * @param ctx the parse tree
 */
fn exit_fstring_format_spec(&mut self, _ctx: &Fstring_format_specContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#fstring}.
 * @param ctx the parse tree
 */
fn enter_fstring(&mut self, _ctx: &FstringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#fstring}.
 * @param ctx the parse tree
 */
fn exit_fstring(&mut self, _ctx: &FstringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#string}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#string}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#strings}.
 * @param ctx the parse tree
 */
fn enter_strings(&mut self, _ctx: &StringsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#strings}.
 * @param ctx the parse tree
 */
fn exit_strings(&mut self, _ctx: &StringsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#list}.
 * @param ctx the parse tree
 */
fn enter_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#list}.
 * @param ctx the parse tree
 */
fn exit_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#tuple}.
 * @param ctx the parse tree
 */
fn enter_tuple(&mut self, _ctx: &TupleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#tuple}.
 * @param ctx the parse tree
 */
fn exit_tuple(&mut self, _ctx: &TupleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#set}.
 * @param ctx the parse tree
 */
fn enter_set(&mut self, _ctx: &SetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#set}.
 * @param ctx the parse tree
 */
fn exit_set(&mut self, _ctx: &SetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#dict}.
 * @param ctx the parse tree
 */
fn enter_dict(&mut self, _ctx: &DictContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#dict}.
 * @param ctx the parse tree
 */
fn exit_dict(&mut self, _ctx: &DictContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#double_starred_kvpairs}.
 * @param ctx the parse tree
 */
fn enter_double_starred_kvpairs(&mut self, _ctx: &Double_starred_kvpairsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#double_starred_kvpairs}.
 * @param ctx the parse tree
 */
fn exit_double_starred_kvpairs(&mut self, _ctx: &Double_starred_kvpairsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#double_starred_kvpair}.
 * @param ctx the parse tree
 */
fn enter_double_starred_kvpair(&mut self, _ctx: &Double_starred_kvpairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#double_starred_kvpair}.
 * @param ctx the parse tree
 */
fn exit_double_starred_kvpair(&mut self, _ctx: &Double_starred_kvpairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#kvpair}.
 * @param ctx the parse tree
 */
fn enter_kvpair(&mut self, _ctx: &KvpairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#kvpair}.
 * @param ctx the parse tree
 */
fn exit_kvpair(&mut self, _ctx: &KvpairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#for_if_clauses}.
 * @param ctx the parse tree
 */
fn enter_for_if_clauses(&mut self, _ctx: &For_if_clausesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#for_if_clauses}.
 * @param ctx the parse tree
 */
fn exit_for_if_clauses(&mut self, _ctx: &For_if_clausesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#for_if_clause}.
 * @param ctx the parse tree
 */
fn enter_for_if_clause(&mut self, _ctx: &For_if_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#for_if_clause}.
 * @param ctx the parse tree
 */
fn exit_for_if_clause(&mut self, _ctx: &For_if_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#listcomp}.
 * @param ctx the parse tree
 */
fn enter_listcomp(&mut self, _ctx: &ListcompContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#listcomp}.
 * @param ctx the parse tree
 */
fn exit_listcomp(&mut self, _ctx: &ListcompContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#setcomp}.
 * @param ctx the parse tree
 */
fn enter_setcomp(&mut self, _ctx: &SetcompContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#setcomp}.
 * @param ctx the parse tree
 */
fn exit_setcomp(&mut self, _ctx: &SetcompContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#genexp}.
 * @param ctx the parse tree
 */
fn enter_genexp(&mut self, _ctx: &GenexpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#genexp}.
 * @param ctx the parse tree
 */
fn exit_genexp(&mut self, _ctx: &GenexpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#dictcomp}.
 * @param ctx the parse tree
 */
fn enter_dictcomp(&mut self, _ctx: &DictcompContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#dictcomp}.
 * @param ctx the parse tree
 */
fn exit_dictcomp(&mut self, _ctx: &DictcompContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#arguments}.
 * @param ctx the parse tree
 */
fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#arguments}.
 * @param ctx the parse tree
 */
fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#args}.
 * @param ctx the parse tree
 */
fn enter_args(&mut self, _ctx: &ArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#args}.
 * @param ctx the parse tree
 */
fn exit_args(&mut self, _ctx: &ArgsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#kwargs}.
 * @param ctx the parse tree
 */
fn enter_kwargs(&mut self, _ctx: &KwargsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#kwargs}.
 * @param ctx the parse tree
 */
fn exit_kwargs(&mut self, _ctx: &KwargsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#starred_expression}.
 * @param ctx the parse tree
 */
fn enter_starred_expression(&mut self, _ctx: &Starred_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#starred_expression}.
 * @param ctx the parse tree
 */
fn exit_starred_expression(&mut self, _ctx: &Starred_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#kwarg_or_starred}.
 * @param ctx the parse tree
 */
fn enter_kwarg_or_starred(&mut self, _ctx: &Kwarg_or_starredContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#kwarg_or_starred}.
 * @param ctx the parse tree
 */
fn exit_kwarg_or_starred(&mut self, _ctx: &Kwarg_or_starredContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#kwarg_or_double_starred}.
 * @param ctx the parse tree
 */
fn enter_kwarg_or_double_starred(&mut self, _ctx: &Kwarg_or_double_starredContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#kwarg_or_double_starred}.
 * @param ctx the parse tree
 */
fn exit_kwarg_or_double_starred(&mut self, _ctx: &Kwarg_or_double_starredContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_targets}.
 * @param ctx the parse tree
 */
fn enter_star_targets(&mut self, _ctx: &Star_targetsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_targets}.
 * @param ctx the parse tree
 */
fn exit_star_targets(&mut self, _ctx: &Star_targetsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_targets_list_seq}.
 * @param ctx the parse tree
 */
fn enter_star_targets_list_seq(&mut self, _ctx: &Star_targets_list_seqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_targets_list_seq}.
 * @param ctx the parse tree
 */
fn exit_star_targets_list_seq(&mut self, _ctx: &Star_targets_list_seqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_targets_tuple_seq}.
 * @param ctx the parse tree
 */
fn enter_star_targets_tuple_seq(&mut self, _ctx: &Star_targets_tuple_seqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_targets_tuple_seq}.
 * @param ctx the parse tree
 */
fn exit_star_targets_tuple_seq(&mut self, _ctx: &Star_targets_tuple_seqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_target}.
 * @param ctx the parse tree
 */
fn enter_star_target(&mut self, _ctx: &Star_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_target}.
 * @param ctx the parse tree
 */
fn exit_star_target(&mut self, _ctx: &Star_targetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#target_with_star_atom}.
 * @param ctx the parse tree
 */
fn enter_target_with_star_atom(&mut self, _ctx: &Target_with_star_atomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#target_with_star_atom}.
 * @param ctx the parse tree
 */
fn exit_target_with_star_atom(&mut self, _ctx: &Target_with_star_atomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#star_atom}.
 * @param ctx the parse tree
 */
fn enter_star_atom(&mut self, _ctx: &Star_atomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#star_atom}.
 * @param ctx the parse tree
 */
fn exit_star_atom(&mut self, _ctx: &Star_atomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#single_target}.
 * @param ctx the parse tree
 */
fn enter_single_target(&mut self, _ctx: &Single_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#single_target}.
 * @param ctx the parse tree
 */
fn exit_single_target(&mut self, _ctx: &Single_targetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#single_subscript_attribute_target}.
 * @param ctx the parse tree
 */
fn enter_single_subscript_attribute_target(&mut self, _ctx: &Single_subscript_attribute_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#single_subscript_attribute_target}.
 * @param ctx the parse tree
 */
fn exit_single_subscript_attribute_target(&mut self, _ctx: &Single_subscript_attribute_targetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#t_primary}.
 * @param ctx the parse tree
 */
fn enter_t_primary(&mut self, _ctx: &T_primaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#t_primary}.
 * @param ctx the parse tree
 */
fn exit_t_primary(&mut self, _ctx: &T_primaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#del_targets}.
 * @param ctx the parse tree
 */
fn enter_del_targets(&mut self, _ctx: &Del_targetsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#del_targets}.
 * @param ctx the parse tree
 */
fn exit_del_targets(&mut self, _ctx: &Del_targetsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#del_target}.
 * @param ctx the parse tree
 */
fn enter_del_target(&mut self, _ctx: &Del_targetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#del_target}.
 * @param ctx the parse tree
 */
fn exit_del_target(&mut self, _ctx: &Del_targetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#del_t_atom}.
 * @param ctx the parse tree
 */
fn enter_del_t_atom(&mut self, _ctx: &Del_t_atomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#del_t_atom}.
 * @param ctx the parse tree
 */
fn exit_del_t_atom(&mut self, _ctx: &Del_t_atomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#type_expressions}.
 * @param ctx the parse tree
 */
fn enter_type_expressions(&mut self, _ctx: &Type_expressionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#type_expressions}.
 * @param ctx the parse tree
 */
fn exit_type_expressions(&mut self, _ctx: &Type_expressionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#func_type_comment}.
 * @param ctx the parse tree
 */
fn enter_func_type_comment(&mut self, _ctx: &Func_type_commentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#func_type_comment}.
 * @param ctx the parse tree
 */
fn exit_func_type_comment(&mut self, _ctx: &Func_type_commentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#name_except_underscore}.
 * @param ctx the parse tree
 */
fn enter_name_except_underscore(&mut self, _ctx: &Name_except_underscoreContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#name_except_underscore}.
 * @param ctx the parse tree
 */
fn exit_name_except_underscore(&mut self, _ctx: &Name_except_underscoreContext<'input>) { }
/**
 * Enter a parse tree produced by {@link PythonParser#name}.
 * @param ctx the parse tree
 */
fn enter_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link PythonParser#name}.
 * @param ctx the parse tree
 */
fn exit_name(&mut self, _ctx: &NameContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : PythonParserListener<'input> }


