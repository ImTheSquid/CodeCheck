#![allow(nonstandard_style)]
// Generated from PythonParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::pythonparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link PythonParser}.
 */
pub trait PythonParserVisitor<'input>: ParseTreeVisitor<'input,PythonParserContextType>{
	/**
	 * Visit a parse tree produced by {@link PythonParser#file_input}.
	 * @param ctx the parse tree
	 */
	fn visit_file_input(&mut self, ctx: &File_inputContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#interactive}.
	 * @param ctx the parse tree
	 */
	fn visit_interactive(&mut self, ctx: &InteractiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#eval}.
	 * @param ctx the parse tree
	 */
	fn visit_eval(&mut self, ctx: &EvalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#func_type}.
	 * @param ctx the parse tree
	 */
	fn visit_func_type(&mut self, ctx: &Func_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#statements}.
	 * @param ctx the parse tree
	 */
	fn visit_statements(&mut self, ctx: &StatementsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#statement_newline}.
	 * @param ctx the parse tree
	 */
	fn visit_statement_newline(&mut self, ctx: &Statement_newlineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#simple_stmts}.
	 * @param ctx the parse tree
	 */
	fn visit_simple_stmts(&mut self, ctx: &Simple_stmtsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#simple_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_simple_stmt(&mut self, ctx: &Simple_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#compound_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_compound_stmt(&mut self, ctx: &Compound_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#annotated_rhs}.
	 * @param ctx the parse tree
	 */
	fn visit_annotated_rhs(&mut self, ctx: &Annotated_rhsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#augassign}.
	 * @param ctx the parse tree
	 */
	fn visit_augassign(&mut self, ctx: &AugassignContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#return_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_return_stmt(&mut self, ctx: &Return_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#raise_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_raise_stmt(&mut self, ctx: &Raise_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#global_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_global_stmt(&mut self, ctx: &Global_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#nonlocal_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_nonlocal_stmt(&mut self, ctx: &Nonlocal_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_del_stmt(&mut self, ctx: &Del_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#yield_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_yield_stmt(&mut self, ctx: &Yield_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#assert_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_assert_stmt(&mut self, ctx: &Assert_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_import_stmt(&mut self, ctx: &Import_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_name}.
	 * @param ctx the parse tree
	 */
	fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from}.
	 * @param ctx the parse tree
	 */
	fn visit_import_from(&mut self, ctx: &Import_fromContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from_targets}.
	 * @param ctx the parse tree
	 */
	fn visit_import_from_targets(&mut self, ctx: &Import_from_targetsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from_as_names}.
	 * @param ctx the parse tree
	 */
	fn visit_import_from_as_names(&mut self, ctx: &Import_from_as_namesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from_as_name}.
	 * @param ctx the parse tree
	 */
	fn visit_import_from_as_name(&mut self, ctx: &Import_from_as_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#dotted_as_names}.
	 * @param ctx the parse tree
	 */
	fn visit_dotted_as_names(&mut self, ctx: &Dotted_as_namesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#dotted_as_name}.
	 * @param ctx the parse tree
	 */
	fn visit_dotted_as_name(&mut self, ctx: &Dotted_as_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#dotted_name}.
	 * @param ctx the parse tree
	 */
	fn visit_dotted_name(&mut self, ctx: &Dotted_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#decorators}.
	 * @param ctx the parse tree
	 */
	fn visit_decorators(&mut self, ctx: &DecoratorsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#class_def}.
	 * @param ctx the parse tree
	 */
	fn visit_class_def(&mut self, ctx: &Class_defContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#class_def_raw}.
	 * @param ctx the parse tree
	 */
	fn visit_class_def_raw(&mut self, ctx: &Class_def_rawContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#function_def}.
	 * @param ctx the parse tree
	 */
	fn visit_function_def(&mut self, ctx: &Function_defContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#function_def_raw}.
	 * @param ctx the parse tree
	 */
	fn visit_function_def_raw(&mut self, ctx: &Function_def_rawContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#params}.
	 * @param ctx the parse tree
	 */
	fn visit_params(&mut self, ctx: &ParamsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#parameters}.
	 * @param ctx the parse tree
	 */
	fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#slash_no_default}.
	 * @param ctx the parse tree
	 */
	fn visit_slash_no_default(&mut self, ctx: &Slash_no_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#slash_with_default}.
	 * @param ctx the parse tree
	 */
	fn visit_slash_with_default(&mut self, ctx: &Slash_with_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_etc}.
	 * @param ctx the parse tree
	 */
	fn visit_star_etc(&mut self, ctx: &Star_etcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwds}.
	 * @param ctx the parse tree
	 */
	fn visit_kwds(&mut self, ctx: &KwdsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_no_default}.
	 * @param ctx the parse tree
	 */
	fn visit_param_no_default(&mut self, ctx: &Param_no_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_no_default_star_annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_param_no_default_star_annotation(&mut self, ctx: &Param_no_default_star_annotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_with_default}.
	 * @param ctx the parse tree
	 */
	fn visit_param_with_default(&mut self, ctx: &Param_with_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_maybe_default}.
	 * @param ctx the parse tree
	 */
	fn visit_param_maybe_default(&mut self, ctx: &Param_maybe_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#param}.
	 * @param ctx the parse tree
	 */
	fn visit_param(&mut self, ctx: &ParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_star_annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_param_star_annotation(&mut self, ctx: &Param_star_annotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_star_annotation(&mut self, ctx: &Star_annotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#default_assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_default_assignment(&mut self, ctx: &Default_assignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#if_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_if_stmt(&mut self, ctx: &If_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#elif_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_elif_stmt(&mut self, ctx: &Elif_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#else_block}.
	 * @param ctx the parse tree
	 */
	fn visit_else_block(&mut self, ctx: &Else_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#while_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_while_stmt(&mut self, ctx: &While_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#for_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_for_stmt(&mut self, ctx: &For_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#with_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_with_stmt(&mut self, ctx: &With_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#with_item}.
	 * @param ctx the parse tree
	 */
	fn visit_with_item(&mut self, ctx: &With_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#try_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_try_stmt(&mut self, ctx: &Try_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#except_block}.
	 * @param ctx the parse tree
	 */
	fn visit_except_block(&mut self, ctx: &Except_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#except_star_block}.
	 * @param ctx the parse tree
	 */
	fn visit_except_star_block(&mut self, ctx: &Except_star_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#finally_block}.
	 * @param ctx the parse tree
	 */
	fn visit_finally_block(&mut self, ctx: &Finally_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#match_stmt}.
	 * @param ctx the parse tree
	 */
	fn visit_match_stmt(&mut self, ctx: &Match_stmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#subject_expr}.
	 * @param ctx the parse tree
	 */
	fn visit_subject_expr(&mut self, ctx: &Subject_exprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#case_block}.
	 * @param ctx the parse tree
	 */
	fn visit_case_block(&mut self, ctx: &Case_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#guard}.
	 * @param ctx the parse tree
	 */
	fn visit_guard(&mut self, ctx: &GuardContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#patterns}.
	 * @param ctx the parse tree
	 */
	fn visit_patterns(&mut self, ctx: &PatternsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern(&mut self, ctx: &PatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#as_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_as_pattern(&mut self, ctx: &As_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#or_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_or_pattern(&mut self, ctx: &Or_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#closed_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_closed_pattern(&mut self, ctx: &Closed_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#literal_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_literal_pattern(&mut self, ctx: &Literal_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#literal_expr}.
	 * @param ctx the parse tree
	 */
	fn visit_literal_expr(&mut self, ctx: &Literal_exprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#complex_number}.
	 * @param ctx the parse tree
	 */
	fn visit_complex_number(&mut self, ctx: &Complex_numberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#signed_number}.
	 * @param ctx the parse tree
	 */
	fn visit_signed_number(&mut self, ctx: &Signed_numberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#signed_real_number}.
	 * @param ctx the parse tree
	 */
	fn visit_signed_real_number(&mut self, ctx: &Signed_real_numberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#real_number}.
	 * @param ctx the parse tree
	 */
	fn visit_real_number(&mut self, ctx: &Real_numberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#imaginary_number}.
	 * @param ctx the parse tree
	 */
	fn visit_imaginary_number(&mut self, ctx: &Imaginary_numberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#capture_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_capture_pattern(&mut self, ctx: &Capture_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#pattern_capture_target}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern_capture_target(&mut self, ctx: &Pattern_capture_targetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#wildcard_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_wildcard_pattern(&mut self, ctx: &Wildcard_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#value_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_value_pattern(&mut self, ctx: &Value_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#attr}.
	 * @param ctx the parse tree
	 */
	fn visit_attr(&mut self, ctx: &AttrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#name_or_attr}.
	 * @param ctx the parse tree
	 */
	fn visit_name_or_attr(&mut self, ctx: &Name_or_attrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#group_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_group_pattern(&mut self, ctx: &Group_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#sequence_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_sequence_pattern(&mut self, ctx: &Sequence_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#open_sequence_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_open_sequence_pattern(&mut self, ctx: &Open_sequence_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#maybe_sequence_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_maybe_sequence_pattern(&mut self, ctx: &Maybe_sequence_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#maybe_star_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_maybe_star_pattern(&mut self, ctx: &Maybe_star_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_star_pattern(&mut self, ctx: &Star_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#mapping_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_mapping_pattern(&mut self, ctx: &Mapping_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#items_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_items_pattern(&mut self, ctx: &Items_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#key_value_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_key_value_pattern(&mut self, ctx: &Key_value_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#double_star_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_double_star_pattern(&mut self, ctx: &Double_star_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#class_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_class_pattern(&mut self, ctx: &Class_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#positional_patterns}.
	 * @param ctx the parse tree
	 */
	fn visit_positional_patterns(&mut self, ctx: &Positional_patternsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#keyword_patterns}.
	 * @param ctx the parse tree
	 */
	fn visit_keyword_patterns(&mut self, ctx: &Keyword_patternsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#keyword_pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_keyword_pattern(&mut self, ctx: &Keyword_patternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_alias}.
	 * @param ctx the parse tree
	 */
	fn visit_type_alias(&mut self, ctx: &Type_aliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_params}.
	 * @param ctx the parse tree
	 */
	fn visit_type_params(&mut self, ctx: &Type_paramsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_seq}.
	 * @param ctx the parse tree
	 */
	fn visit_type_param_seq(&mut self, ctx: &Type_param_seqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param}.
	 * @param ctx the parse tree
	 */
	fn visit_type_param(&mut self, ctx: &Type_paramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_bound}.
	 * @param ctx the parse tree
	 */
	fn visit_type_param_bound(&mut self, ctx: &Type_param_boundContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_default}.
	 * @param ctx the parse tree
	 */
	fn visit_type_param_default(&mut self, ctx: &Type_param_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_starred_default}.
	 * @param ctx the parse tree
	 */
	fn visit_type_param_starred_default(&mut self, ctx: &Type_param_starred_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#expressions}.
	 * @param ctx the parse tree
	 */
	fn visit_expressions(&mut self, ctx: &ExpressionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#yield_expr}.
	 * @param ctx the parse tree
	 */
	fn visit_yield_expr(&mut self, ctx: &Yield_exprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_expressions}.
	 * @param ctx the parse tree
	 */
	fn visit_star_expressions(&mut self, ctx: &Star_expressionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_star_expression(&mut self, ctx: &Star_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_named_expressions}.
	 * @param ctx the parse tree
	 */
	fn visit_star_named_expressions(&mut self, ctx: &Star_named_expressionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_named_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_star_named_expression(&mut self, ctx: &Star_named_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#assignment_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment_expression(&mut self, ctx: &Assignment_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#named_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_named_expression(&mut self, ctx: &Named_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#disjunction}.
	 * @param ctx the parse tree
	 */
	fn visit_disjunction(&mut self, ctx: &DisjunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#conjunction}.
	 * @param ctx the parse tree
	 */
	fn visit_conjunction(&mut self, ctx: &ConjunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#inversion}.
	 * @param ctx the parse tree
	 */
	fn visit_inversion(&mut self, ctx: &InversionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#comparison}.
	 * @param ctx the parse tree
	 */
	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#compare_op_bitwise_or_pair}.
	 * @param ctx the parse tree
	 */
	fn visit_compare_op_bitwise_or_pair(&mut self, ctx: &Compare_op_bitwise_or_pairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#eq_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_eq_bitwise_or(&mut self, ctx: &Eq_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#noteq_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_noteq_bitwise_or(&mut self, ctx: &Noteq_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lte_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_lte_bitwise_or(&mut self, ctx: &Lte_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lt_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_lt_bitwise_or(&mut self, ctx: &Lt_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#gte_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_gte_bitwise_or(&mut self, ctx: &Gte_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#gt_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_gt_bitwise_or(&mut self, ctx: &Gt_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#notin_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_notin_bitwise_or(&mut self, ctx: &Notin_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#in_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_in_bitwise_or(&mut self, ctx: &In_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#isnot_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_isnot_bitwise_or(&mut self, ctx: &Isnot_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#is_bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_is_bitwise_or(&mut self, ctx: &Is_bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#bitwise_or}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwise_or(&mut self, ctx: &Bitwise_orContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#bitwise_xor}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwise_xor(&mut self, ctx: &Bitwise_xorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#bitwise_and}.
	 * @param ctx the parse tree
	 */
	fn visit_bitwise_and(&mut self, ctx: &Bitwise_andContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#shift_expr}.
	 * @param ctx the parse tree
	 */
	fn visit_shift_expr(&mut self, ctx: &Shift_exprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#sum}.
	 * @param ctx the parse tree
	 */
	fn visit_sum(&mut self, ctx: &SumContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_term(&mut self, ctx: &TermContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#factor}.
	 * @param ctx the parse tree
	 */
	fn visit_factor(&mut self, ctx: &FactorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#power}.
	 * @param ctx the parse tree
	 */
	fn visit_power(&mut self, ctx: &PowerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#await_primary}.
	 * @param ctx the parse tree
	 */
	fn visit_await_primary(&mut self, ctx: &Await_primaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#primary}.
	 * @param ctx the parse tree
	 */
	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#slices}.
	 * @param ctx the parse tree
	 */
	fn visit_slices(&mut self, ctx: &SlicesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#slice}.
	 * @param ctx the parse tree
	 */
	fn visit_slice(&mut self, ctx: &SliceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#group}.
	 * @param ctx the parse tree
	 */
	fn visit_group(&mut self, ctx: &GroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambdef}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdef(&mut self, ctx: &LambdefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_params}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_params(&mut self, ctx: &Lambda_paramsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_parameters}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_parameters(&mut self, ctx: &Lambda_parametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_slash_no_default}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_slash_no_default(&mut self, ctx: &Lambda_slash_no_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_slash_with_default}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_slash_with_default(&mut self, ctx: &Lambda_slash_with_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_star_etc}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_star_etc(&mut self, ctx: &Lambda_star_etcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_kwds}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_kwds(&mut self, ctx: &Lambda_kwdsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param_no_default}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_param_no_default(&mut self, ctx: &Lambda_param_no_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param_with_default}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_param_with_default(&mut self, ctx: &Lambda_param_with_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param_maybe_default}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_param_maybe_default(&mut self, ctx: &Lambda_param_maybe_defaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda_param(&mut self, ctx: &Lambda_paramContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_middle}.
	 * @param ctx the parse tree
	 */
	fn visit_fstring_middle(&mut self, ctx: &Fstring_middleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_replacement_field}.
	 * @param ctx the parse tree
	 */
	fn visit_fstring_replacement_field(&mut self, ctx: &Fstring_replacement_fieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_conversion}.
	 * @param ctx the parse tree
	 */
	fn visit_fstring_conversion(&mut self, ctx: &Fstring_conversionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_full_format_spec}.
	 * @param ctx the parse tree
	 */
	fn visit_fstring_full_format_spec(&mut self, ctx: &Fstring_full_format_specContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_format_spec}.
	 * @param ctx the parse tree
	 */
	fn visit_fstring_format_spec(&mut self, ctx: &Fstring_format_specContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring}.
	 * @param ctx the parse tree
	 */
	fn visit_fstring(&mut self, ctx: &FstringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#string}.
	 * @param ctx the parse tree
	 */
	fn visit_string(&mut self, ctx: &StringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#strings}.
	 * @param ctx the parse tree
	 */
	fn visit_strings(&mut self, ctx: &StringsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#list}.
	 * @param ctx the parse tree
	 */
	fn visit_list(&mut self, ctx: &ListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#tuple}.
	 * @param ctx the parse tree
	 */
	fn visit_tuple(&mut self, ctx: &TupleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#set}.
	 * @param ctx the parse tree
	 */
	fn visit_set(&mut self, ctx: &SetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#dict}.
	 * @param ctx the parse tree
	 */
	fn visit_dict(&mut self, ctx: &DictContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#double_starred_kvpairs}.
	 * @param ctx the parse tree
	 */
	fn visit_double_starred_kvpairs(&mut self, ctx: &Double_starred_kvpairsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#double_starred_kvpair}.
	 * @param ctx the parse tree
	 */
	fn visit_double_starred_kvpair(&mut self, ctx: &Double_starred_kvpairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#kvpair}.
	 * @param ctx the parse tree
	 */
	fn visit_kvpair(&mut self, ctx: &KvpairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#for_if_clauses}.
	 * @param ctx the parse tree
	 */
	fn visit_for_if_clauses(&mut self, ctx: &For_if_clausesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#for_if_clause}.
	 * @param ctx the parse tree
	 */
	fn visit_for_if_clause(&mut self, ctx: &For_if_clauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#listcomp}.
	 * @param ctx the parse tree
	 */
	fn visit_listcomp(&mut self, ctx: &ListcompContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#setcomp}.
	 * @param ctx the parse tree
	 */
	fn visit_setcomp(&mut self, ctx: &SetcompContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#genexp}.
	 * @param ctx the parse tree
	 */
	fn visit_genexp(&mut self, ctx: &GenexpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#dictcomp}.
	 * @param ctx the parse tree
	 */
	fn visit_dictcomp(&mut self, ctx: &DictcompContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#args}.
	 * @param ctx the parse tree
	 */
	fn visit_args(&mut self, ctx: &ArgsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwargs}.
	 * @param ctx the parse tree
	 */
	fn visit_kwargs(&mut self, ctx: &KwargsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#starred_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_starred_expression(&mut self, ctx: &Starred_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwarg_or_starred}.
	 * @param ctx the parse tree
	 */
	fn visit_kwarg_or_starred(&mut self, ctx: &Kwarg_or_starredContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwarg_or_double_starred}.
	 * @param ctx the parse tree
	 */
	fn visit_kwarg_or_double_starred(&mut self, ctx: &Kwarg_or_double_starredContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_targets}.
	 * @param ctx the parse tree
	 */
	fn visit_star_targets(&mut self, ctx: &Star_targetsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_targets_list_seq}.
	 * @param ctx the parse tree
	 */
	fn visit_star_targets_list_seq(&mut self, ctx: &Star_targets_list_seqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_targets_tuple_seq}.
	 * @param ctx the parse tree
	 */
	fn visit_star_targets_tuple_seq(&mut self, ctx: &Star_targets_tuple_seqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_target}.
	 * @param ctx the parse tree
	 */
	fn visit_star_target(&mut self, ctx: &Star_targetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#target_with_star_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_target_with_star_atom(&mut self, ctx: &Target_with_star_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_star_atom(&mut self, ctx: &Star_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#single_target}.
	 * @param ctx the parse tree
	 */
	fn visit_single_target(&mut self, ctx: &Single_targetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#single_subscript_attribute_target}.
	 * @param ctx the parse tree
	 */
	fn visit_single_subscript_attribute_target(&mut self, ctx: &Single_subscript_attribute_targetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#t_primary}.
	 * @param ctx the parse tree
	 */
	fn visit_t_primary(&mut self, ctx: &T_primaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_targets}.
	 * @param ctx the parse tree
	 */
	fn visit_del_targets(&mut self, ctx: &Del_targetsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_target}.
	 * @param ctx the parse tree
	 */
	fn visit_del_target(&mut self, ctx: &Del_targetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_t_atom}.
	 * @param ctx the parse tree
	 */
	fn visit_del_t_atom(&mut self, ctx: &Del_t_atomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_expressions}.
	 * @param ctx the parse tree
	 */
	fn visit_type_expressions(&mut self, ctx: &Type_expressionsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#func_type_comment}.
	 * @param ctx the parse tree
	 */
	fn visit_func_type_comment(&mut self, ctx: &Func_type_commentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#name_except_underscore}.
	 * @param ctx the parse tree
	 */
	fn visit_name_except_underscore(&mut self, ctx: &Name_except_underscoreContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link PythonParser#name}.
	 * @param ctx the parse tree
	 */
	fn visit_name(&mut self, ctx: &NameContext<'input>) { self.visit_children(ctx) }

}

pub trait PythonParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= PythonParserContextType>{
	/**
	 * Visit a parse tree produced by {@link PythonParser#file_input}.
	 * @param ctx the parse tree
	 */
		fn visit_file_input(&mut self, ctx: &File_inputContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#interactive}.
	 * @param ctx the parse tree
	 */
		fn visit_interactive(&mut self, ctx: &InteractiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#eval}.
	 * @param ctx the parse tree
	 */
		fn visit_eval(&mut self, ctx: &EvalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#func_type}.
	 * @param ctx the parse tree
	 */
		fn visit_func_type(&mut self, ctx: &Func_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#statements}.
	 * @param ctx the parse tree
	 */
		fn visit_statements(&mut self, ctx: &StatementsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#statement_newline}.
	 * @param ctx the parse tree
	 */
		fn visit_statement_newline(&mut self, ctx: &Statement_newlineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#simple_stmts}.
	 * @param ctx the parse tree
	 */
		fn visit_simple_stmts(&mut self, ctx: &Simple_stmtsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#simple_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_simple_stmt(&mut self, ctx: &Simple_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#compound_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_compound_stmt(&mut self, ctx: &Compound_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#annotated_rhs}.
	 * @param ctx the parse tree
	 */
		fn visit_annotated_rhs(&mut self, ctx: &Annotated_rhsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#augassign}.
	 * @param ctx the parse tree
	 */
		fn visit_augassign(&mut self, ctx: &AugassignContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#return_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_return_stmt(&mut self, ctx: &Return_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#raise_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_raise_stmt(&mut self, ctx: &Raise_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#global_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_global_stmt(&mut self, ctx: &Global_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#nonlocal_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_nonlocal_stmt(&mut self, ctx: &Nonlocal_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_del_stmt(&mut self, ctx: &Del_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#yield_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_yield_stmt(&mut self, ctx: &Yield_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#assert_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_assert_stmt(&mut self, ctx: &Assert_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_import_stmt(&mut self, ctx: &Import_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_name}.
	 * @param ctx the parse tree
	 */
		fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from}.
	 * @param ctx the parse tree
	 */
		fn visit_import_from(&mut self, ctx: &Import_fromContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from_targets}.
	 * @param ctx the parse tree
	 */
		fn visit_import_from_targets(&mut self, ctx: &Import_from_targetsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from_as_names}.
	 * @param ctx the parse tree
	 */
		fn visit_import_from_as_names(&mut self, ctx: &Import_from_as_namesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#import_from_as_name}.
	 * @param ctx the parse tree
	 */
		fn visit_import_from_as_name(&mut self, ctx: &Import_from_as_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#dotted_as_names}.
	 * @param ctx the parse tree
	 */
		fn visit_dotted_as_names(&mut self, ctx: &Dotted_as_namesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#dotted_as_name}.
	 * @param ctx the parse tree
	 */
		fn visit_dotted_as_name(&mut self, ctx: &Dotted_as_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#dotted_name}.
	 * @param ctx the parse tree
	 */
		fn visit_dotted_name(&mut self, ctx: &Dotted_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#decorators}.
	 * @param ctx the parse tree
	 */
		fn visit_decorators(&mut self, ctx: &DecoratorsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#class_def}.
	 * @param ctx the parse tree
	 */
		fn visit_class_def(&mut self, ctx: &Class_defContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#class_def_raw}.
	 * @param ctx the parse tree
	 */
		fn visit_class_def_raw(&mut self, ctx: &Class_def_rawContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#function_def}.
	 * @param ctx the parse tree
	 */
		fn visit_function_def(&mut self, ctx: &Function_defContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#function_def_raw}.
	 * @param ctx the parse tree
	 */
		fn visit_function_def_raw(&mut self, ctx: &Function_def_rawContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#params}.
	 * @param ctx the parse tree
	 */
		fn visit_params(&mut self, ctx: &ParamsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#parameters}.
	 * @param ctx the parse tree
	 */
		fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#slash_no_default}.
	 * @param ctx the parse tree
	 */
		fn visit_slash_no_default(&mut self, ctx: &Slash_no_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#slash_with_default}.
	 * @param ctx the parse tree
	 */
		fn visit_slash_with_default(&mut self, ctx: &Slash_with_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_etc}.
	 * @param ctx the parse tree
	 */
		fn visit_star_etc(&mut self, ctx: &Star_etcContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwds}.
	 * @param ctx the parse tree
	 */
		fn visit_kwds(&mut self, ctx: &KwdsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_no_default}.
	 * @param ctx the parse tree
	 */
		fn visit_param_no_default(&mut self, ctx: &Param_no_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_no_default_star_annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_param_no_default_star_annotation(&mut self, ctx: &Param_no_default_star_annotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_with_default}.
	 * @param ctx the parse tree
	 */
		fn visit_param_with_default(&mut self, ctx: &Param_with_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_maybe_default}.
	 * @param ctx the parse tree
	 */
		fn visit_param_maybe_default(&mut self, ctx: &Param_maybe_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#param}.
	 * @param ctx the parse tree
	 */
		fn visit_param(&mut self, ctx: &ParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#param_star_annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_param_star_annotation(&mut self, ctx: &Param_star_annotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_star_annotation(&mut self, ctx: &Star_annotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#default_assignment}.
	 * @param ctx the parse tree
	 */
		fn visit_default_assignment(&mut self, ctx: &Default_assignmentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#if_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_if_stmt(&mut self, ctx: &If_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#elif_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_elif_stmt(&mut self, ctx: &Elif_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#else_block}.
	 * @param ctx the parse tree
	 */
		fn visit_else_block(&mut self, ctx: &Else_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#while_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_while_stmt(&mut self, ctx: &While_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#for_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_for_stmt(&mut self, ctx: &For_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#with_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_with_stmt(&mut self, ctx: &With_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#with_item}.
	 * @param ctx the parse tree
	 */
		fn visit_with_item(&mut self, ctx: &With_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#try_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_try_stmt(&mut self, ctx: &Try_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#except_block}.
	 * @param ctx the parse tree
	 */
		fn visit_except_block(&mut self, ctx: &Except_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#except_star_block}.
	 * @param ctx the parse tree
	 */
		fn visit_except_star_block(&mut self, ctx: &Except_star_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#finally_block}.
	 * @param ctx the parse tree
	 */
		fn visit_finally_block(&mut self, ctx: &Finally_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#match_stmt}.
	 * @param ctx the parse tree
	 */
		fn visit_match_stmt(&mut self, ctx: &Match_stmtContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#subject_expr}.
	 * @param ctx the parse tree
	 */
		fn visit_subject_expr(&mut self, ctx: &Subject_exprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#case_block}.
	 * @param ctx the parse tree
	 */
		fn visit_case_block(&mut self, ctx: &Case_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#guard}.
	 * @param ctx the parse tree
	 */
		fn visit_guard(&mut self, ctx: &GuardContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#patterns}.
	 * @param ctx the parse tree
	 */
		fn visit_patterns(&mut self, ctx: &PatternsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_pattern(&mut self, ctx: &PatternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#as_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_as_pattern(&mut self, ctx: &As_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#or_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_or_pattern(&mut self, ctx: &Or_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#closed_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_closed_pattern(&mut self, ctx: &Closed_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#literal_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_literal_pattern(&mut self, ctx: &Literal_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#literal_expr}.
	 * @param ctx the parse tree
	 */
		fn visit_literal_expr(&mut self, ctx: &Literal_exprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#complex_number}.
	 * @param ctx the parse tree
	 */
		fn visit_complex_number(&mut self, ctx: &Complex_numberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#signed_number}.
	 * @param ctx the parse tree
	 */
		fn visit_signed_number(&mut self, ctx: &Signed_numberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#signed_real_number}.
	 * @param ctx the parse tree
	 */
		fn visit_signed_real_number(&mut self, ctx: &Signed_real_numberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#real_number}.
	 * @param ctx the parse tree
	 */
		fn visit_real_number(&mut self, ctx: &Real_numberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#imaginary_number}.
	 * @param ctx the parse tree
	 */
		fn visit_imaginary_number(&mut self, ctx: &Imaginary_numberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#capture_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_capture_pattern(&mut self, ctx: &Capture_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#pattern_capture_target}.
	 * @param ctx the parse tree
	 */
		fn visit_pattern_capture_target(&mut self, ctx: &Pattern_capture_targetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#wildcard_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_wildcard_pattern(&mut self, ctx: &Wildcard_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#value_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_value_pattern(&mut self, ctx: &Value_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#attr}.
	 * @param ctx the parse tree
	 */
		fn visit_attr(&mut self, ctx: &AttrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#name_or_attr}.
	 * @param ctx the parse tree
	 */
		fn visit_name_or_attr(&mut self, ctx: &Name_or_attrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#group_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_group_pattern(&mut self, ctx: &Group_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#sequence_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_sequence_pattern(&mut self, ctx: &Sequence_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#open_sequence_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_open_sequence_pattern(&mut self, ctx: &Open_sequence_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#maybe_sequence_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_maybe_sequence_pattern(&mut self, ctx: &Maybe_sequence_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#maybe_star_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_maybe_star_pattern(&mut self, ctx: &Maybe_star_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_star_pattern(&mut self, ctx: &Star_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#mapping_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_mapping_pattern(&mut self, ctx: &Mapping_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#items_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_items_pattern(&mut self, ctx: &Items_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#key_value_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_key_value_pattern(&mut self, ctx: &Key_value_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#double_star_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_double_star_pattern(&mut self, ctx: &Double_star_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#class_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_class_pattern(&mut self, ctx: &Class_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#positional_patterns}.
	 * @param ctx the parse tree
	 */
		fn visit_positional_patterns(&mut self, ctx: &Positional_patternsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#keyword_patterns}.
	 * @param ctx the parse tree
	 */
		fn visit_keyword_patterns(&mut self, ctx: &Keyword_patternsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#keyword_pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_keyword_pattern(&mut self, ctx: &Keyword_patternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_alias}.
	 * @param ctx the parse tree
	 */
		fn visit_type_alias(&mut self, ctx: &Type_aliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_params}.
	 * @param ctx the parse tree
	 */
		fn visit_type_params(&mut self, ctx: &Type_paramsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_seq}.
	 * @param ctx the parse tree
	 */
		fn visit_type_param_seq(&mut self, ctx: &Type_param_seqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param}.
	 * @param ctx the parse tree
	 */
		fn visit_type_param(&mut self, ctx: &Type_paramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_bound}.
	 * @param ctx the parse tree
	 */
		fn visit_type_param_bound(&mut self, ctx: &Type_param_boundContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_default}.
	 * @param ctx the parse tree
	 */
		fn visit_type_param_default(&mut self, ctx: &Type_param_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_param_starred_default}.
	 * @param ctx the parse tree
	 */
		fn visit_type_param_starred_default(&mut self, ctx: &Type_param_starred_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#expressions}.
	 * @param ctx the parse tree
	 */
		fn visit_expressions(&mut self, ctx: &ExpressionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#yield_expr}.
	 * @param ctx the parse tree
	 */
		fn visit_yield_expr(&mut self, ctx: &Yield_exprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_expressions}.
	 * @param ctx the parse tree
	 */
		fn visit_star_expressions(&mut self, ctx: &Star_expressionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_star_expression(&mut self, ctx: &Star_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_named_expressions}.
	 * @param ctx the parse tree
	 */
		fn visit_star_named_expressions(&mut self, ctx: &Star_named_expressionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_named_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_star_named_expression(&mut self, ctx: &Star_named_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#assignment_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_assignment_expression(&mut self, ctx: &Assignment_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#named_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_named_expression(&mut self, ctx: &Named_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#disjunction}.
	 * @param ctx the parse tree
	 */
		fn visit_disjunction(&mut self, ctx: &DisjunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#conjunction}.
	 * @param ctx the parse tree
	 */
		fn visit_conjunction(&mut self, ctx: &ConjunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#inversion}.
	 * @param ctx the parse tree
	 */
		fn visit_inversion(&mut self, ctx: &InversionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#comparison}.
	 * @param ctx the parse tree
	 */
		fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#compare_op_bitwise_or_pair}.
	 * @param ctx the parse tree
	 */
		fn visit_compare_op_bitwise_or_pair(&mut self, ctx: &Compare_op_bitwise_or_pairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#eq_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_eq_bitwise_or(&mut self, ctx: &Eq_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#noteq_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_noteq_bitwise_or(&mut self, ctx: &Noteq_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lte_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_lte_bitwise_or(&mut self, ctx: &Lte_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lt_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_lt_bitwise_or(&mut self, ctx: &Lt_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#gte_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_gte_bitwise_or(&mut self, ctx: &Gte_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#gt_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_gt_bitwise_or(&mut self, ctx: &Gt_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#notin_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_notin_bitwise_or(&mut self, ctx: &Notin_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#in_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_in_bitwise_or(&mut self, ctx: &In_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#isnot_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_isnot_bitwise_or(&mut self, ctx: &Isnot_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#is_bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_is_bitwise_or(&mut self, ctx: &Is_bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#bitwise_or}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwise_or(&mut self, ctx: &Bitwise_orContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#bitwise_xor}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwise_xor(&mut self, ctx: &Bitwise_xorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#bitwise_and}.
	 * @param ctx the parse tree
	 */
		fn visit_bitwise_and(&mut self, ctx: &Bitwise_andContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#shift_expr}.
	 * @param ctx the parse tree
	 */
		fn visit_shift_expr(&mut self, ctx: &Shift_exprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#sum}.
	 * @param ctx the parse tree
	 */
		fn visit_sum(&mut self, ctx: &SumContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#term}.
	 * @param ctx the parse tree
	 */
		fn visit_term(&mut self, ctx: &TermContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#factor}.
	 * @param ctx the parse tree
	 */
		fn visit_factor(&mut self, ctx: &FactorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#power}.
	 * @param ctx the parse tree
	 */
		fn visit_power(&mut self, ctx: &PowerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#await_primary}.
	 * @param ctx the parse tree
	 */
		fn visit_await_primary(&mut self, ctx: &Await_primaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#primary}.
	 * @param ctx the parse tree
	 */
		fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#slices}.
	 * @param ctx the parse tree
	 */
		fn visit_slices(&mut self, ctx: &SlicesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#slice}.
	 * @param ctx the parse tree
	 */
		fn visit_slice(&mut self, ctx: &SliceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#group}.
	 * @param ctx the parse tree
	 */
		fn visit_group(&mut self, ctx: &GroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambdef}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdef(&mut self, ctx: &LambdefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_params}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_params(&mut self, ctx: &Lambda_paramsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_parameters}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_parameters(&mut self, ctx: &Lambda_parametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_slash_no_default}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_slash_no_default(&mut self, ctx: &Lambda_slash_no_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_slash_with_default}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_slash_with_default(&mut self, ctx: &Lambda_slash_with_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_star_etc}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_star_etc(&mut self, ctx: &Lambda_star_etcContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_kwds}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_kwds(&mut self, ctx: &Lambda_kwdsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param_no_default}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_param_no_default(&mut self, ctx: &Lambda_param_no_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param_with_default}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_param_with_default(&mut self, ctx: &Lambda_param_with_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param_maybe_default}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_param_maybe_default(&mut self, ctx: &Lambda_param_maybe_defaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#lambda_param}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda_param(&mut self, ctx: &Lambda_paramContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_middle}.
	 * @param ctx the parse tree
	 */
		fn visit_fstring_middle(&mut self, ctx: &Fstring_middleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_replacement_field}.
	 * @param ctx the parse tree
	 */
		fn visit_fstring_replacement_field(&mut self, ctx: &Fstring_replacement_fieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_conversion}.
	 * @param ctx the parse tree
	 */
		fn visit_fstring_conversion(&mut self, ctx: &Fstring_conversionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_full_format_spec}.
	 * @param ctx the parse tree
	 */
		fn visit_fstring_full_format_spec(&mut self, ctx: &Fstring_full_format_specContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring_format_spec}.
	 * @param ctx the parse tree
	 */
		fn visit_fstring_format_spec(&mut self, ctx: &Fstring_format_specContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#fstring}.
	 * @param ctx the parse tree
	 */
		fn visit_fstring(&mut self, ctx: &FstringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#string}.
	 * @param ctx the parse tree
	 */
		fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#strings}.
	 * @param ctx the parse tree
	 */
		fn visit_strings(&mut self, ctx: &StringsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#list}.
	 * @param ctx the parse tree
	 */
		fn visit_list(&mut self, ctx: &ListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#tuple}.
	 * @param ctx the parse tree
	 */
		fn visit_tuple(&mut self, ctx: &TupleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#set}.
	 * @param ctx the parse tree
	 */
		fn visit_set(&mut self, ctx: &SetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#dict}.
	 * @param ctx the parse tree
	 */
		fn visit_dict(&mut self, ctx: &DictContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#double_starred_kvpairs}.
	 * @param ctx the parse tree
	 */
		fn visit_double_starred_kvpairs(&mut self, ctx: &Double_starred_kvpairsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#double_starred_kvpair}.
	 * @param ctx the parse tree
	 */
		fn visit_double_starred_kvpair(&mut self, ctx: &Double_starred_kvpairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#kvpair}.
	 * @param ctx the parse tree
	 */
		fn visit_kvpair(&mut self, ctx: &KvpairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#for_if_clauses}.
	 * @param ctx the parse tree
	 */
		fn visit_for_if_clauses(&mut self, ctx: &For_if_clausesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#for_if_clause}.
	 * @param ctx the parse tree
	 */
		fn visit_for_if_clause(&mut self, ctx: &For_if_clauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#listcomp}.
	 * @param ctx the parse tree
	 */
		fn visit_listcomp(&mut self, ctx: &ListcompContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#setcomp}.
	 * @param ctx the parse tree
	 */
		fn visit_setcomp(&mut self, ctx: &SetcompContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#genexp}.
	 * @param ctx the parse tree
	 */
		fn visit_genexp(&mut self, ctx: &GenexpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#dictcomp}.
	 * @param ctx the parse tree
	 */
		fn visit_dictcomp(&mut self, ctx: &DictcompContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#args}.
	 * @param ctx the parse tree
	 */
		fn visit_args(&mut self, ctx: &ArgsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwargs}.
	 * @param ctx the parse tree
	 */
		fn visit_kwargs(&mut self, ctx: &KwargsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#starred_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_starred_expression(&mut self, ctx: &Starred_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwarg_or_starred}.
	 * @param ctx the parse tree
	 */
		fn visit_kwarg_or_starred(&mut self, ctx: &Kwarg_or_starredContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#kwarg_or_double_starred}.
	 * @param ctx the parse tree
	 */
		fn visit_kwarg_or_double_starred(&mut self, ctx: &Kwarg_or_double_starredContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_targets}.
	 * @param ctx the parse tree
	 */
		fn visit_star_targets(&mut self, ctx: &Star_targetsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_targets_list_seq}.
	 * @param ctx the parse tree
	 */
		fn visit_star_targets_list_seq(&mut self, ctx: &Star_targets_list_seqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_targets_tuple_seq}.
	 * @param ctx the parse tree
	 */
		fn visit_star_targets_tuple_seq(&mut self, ctx: &Star_targets_tuple_seqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_target}.
	 * @param ctx the parse tree
	 */
		fn visit_star_target(&mut self, ctx: &Star_targetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#target_with_star_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_target_with_star_atom(&mut self, ctx: &Target_with_star_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#star_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_star_atom(&mut self, ctx: &Star_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#single_target}.
	 * @param ctx the parse tree
	 */
		fn visit_single_target(&mut self, ctx: &Single_targetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#single_subscript_attribute_target}.
	 * @param ctx the parse tree
	 */
		fn visit_single_subscript_attribute_target(&mut self, ctx: &Single_subscript_attribute_targetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#t_primary}.
	 * @param ctx the parse tree
	 */
		fn visit_t_primary(&mut self, ctx: &T_primaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_targets}.
	 * @param ctx the parse tree
	 */
		fn visit_del_targets(&mut self, ctx: &Del_targetsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_target}.
	 * @param ctx the parse tree
	 */
		fn visit_del_target(&mut self, ctx: &Del_targetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#del_t_atom}.
	 * @param ctx the parse tree
	 */
		fn visit_del_t_atom(&mut self, ctx: &Del_t_atomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#type_expressions}.
	 * @param ctx the parse tree
	 */
		fn visit_type_expressions(&mut self, ctx: &Type_expressionsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#func_type_comment}.
	 * @param ctx the parse tree
	 */
		fn visit_func_type_comment(&mut self, ctx: &Func_type_commentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#name_except_underscore}.
	 * @param ctx the parse tree
	 */
		fn visit_name_except_underscore(&mut self, ctx: &Name_except_underscoreContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link PythonParser#name}.
	 * @param ctx the parse tree
	 */
		fn visit_name(&mut self, ctx: &NameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> PythonParserVisitor<'input> for T
where
	T: PythonParserVisitorCompat<'input>
{
	fn visit_file_input(&mut self, ctx: &File_inputContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_file_input(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interactive(&mut self, ctx: &InteractiveContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_interactive(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eval(&mut self, ctx: &EvalContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_eval(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_type(&mut self, ctx: &Func_typeContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_func_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statements(&mut self, ctx: &StatementsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_statements(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement_newline(&mut self, ctx: &Statement_newlineContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_statement_newline(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simple_stmts(&mut self, ctx: &Simple_stmtsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_simple_stmts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simple_stmt(&mut self, ctx: &Simple_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_simple_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compound_stmt(&mut self, ctx: &Compound_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_compound_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotated_rhs(&mut self, ctx: &Annotated_rhsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_annotated_rhs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_augassign(&mut self, ctx: &AugassignContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_augassign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_return_stmt(&mut self, ctx: &Return_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_return_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_raise_stmt(&mut self, ctx: &Raise_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_raise_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_global_stmt(&mut self, ctx: &Global_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_global_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonlocal_stmt(&mut self, ctx: &Nonlocal_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_nonlocal_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_del_stmt(&mut self, ctx: &Del_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_del_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_yield_stmt(&mut self, ctx: &Yield_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_yield_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assert_stmt(&mut self, ctx: &Assert_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_assert_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_import_stmt(&mut self, ctx: &Import_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_import_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_import_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_import_from(&mut self, ctx: &Import_fromContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_import_from(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_import_from_targets(&mut self, ctx: &Import_from_targetsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_import_from_targets(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_import_from_as_names(&mut self, ctx: &Import_from_as_namesContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_import_from_as_names(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_import_from_as_name(&mut self, ctx: &Import_from_as_nameContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_import_from_as_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dotted_as_names(&mut self, ctx: &Dotted_as_namesContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_dotted_as_names(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dotted_as_name(&mut self, ctx: &Dotted_as_nameContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_dotted_as_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dotted_name(&mut self, ctx: &Dotted_nameContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_dotted_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decorators(&mut self, ctx: &DecoratorsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_decorators(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_class_def(&mut self, ctx: &Class_defContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_class_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_class_def_raw(&mut self, ctx: &Class_def_rawContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_class_def_raw(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_def(&mut self, ctx: &Function_defContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_function_def(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_def_raw(&mut self, ctx: &Function_def_rawContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_function_def_raw(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_params(&mut self, ctx: &ParamsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_params(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameters(&mut self, ctx: &ParametersContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_parameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_slash_no_default(&mut self, ctx: &Slash_no_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_slash_no_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_slash_with_default(&mut self, ctx: &Slash_with_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_slash_with_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_etc(&mut self, ctx: &Star_etcContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_etc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_kwds(&mut self, ctx: &KwdsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_kwds(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param_no_default(&mut self, ctx: &Param_no_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_param_no_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param_no_default_star_annotation(&mut self, ctx: &Param_no_default_star_annotationContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_param_no_default_star_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param_with_default(&mut self, ctx: &Param_with_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_param_with_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param_maybe_default(&mut self, ctx: &Param_maybe_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_param_maybe_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param(&mut self, ctx: &ParamContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_param(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param_star_annotation(&mut self, ctx: &Param_star_annotationContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_param_star_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_annotation(&mut self, ctx: &Star_annotationContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_default_assignment(&mut self, ctx: &Default_assignmentContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_default_assignment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_if_stmt(&mut self, ctx: &If_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_if_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elif_stmt(&mut self, ctx: &Elif_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_elif_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_else_block(&mut self, ctx: &Else_blockContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_else_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_while_stmt(&mut self, ctx: &While_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_while_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_for_stmt(&mut self, ctx: &For_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_for_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_with_stmt(&mut self, ctx: &With_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_with_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_with_item(&mut self, ctx: &With_itemContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_with_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_try_stmt(&mut self, ctx: &Try_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_try_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_except_block(&mut self, ctx: &Except_blockContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_except_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_except_star_block(&mut self, ctx: &Except_star_blockContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_except_star_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_finally_block(&mut self, ctx: &Finally_blockContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_finally_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_match_stmt(&mut self, ctx: &Match_stmtContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_match_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subject_expr(&mut self, ctx: &Subject_exprContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_subject_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_case_block(&mut self, ctx: &Case_blockContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_case_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_guard(&mut self, ctx: &GuardContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_guard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_patterns(&mut self, ctx: &PatternsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_patterns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pattern(&mut self, ctx: &PatternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_as_pattern(&mut self, ctx: &As_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_as_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_or_pattern(&mut self, ctx: &Or_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_or_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_closed_pattern(&mut self, ctx: &Closed_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_closed_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal_pattern(&mut self, ctx: &Literal_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_literal_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal_expr(&mut self, ctx: &Literal_exprContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_literal_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_complex_number(&mut self, ctx: &Complex_numberContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_complex_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_signed_number(&mut self, ctx: &Signed_numberContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_signed_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_signed_real_number(&mut self, ctx: &Signed_real_numberContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_signed_real_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_real_number(&mut self, ctx: &Real_numberContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_real_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_imaginary_number(&mut self, ctx: &Imaginary_numberContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_imaginary_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_capture_pattern(&mut self, ctx: &Capture_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_capture_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pattern_capture_target(&mut self, ctx: &Pattern_capture_targetContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_pattern_capture_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_wildcard_pattern(&mut self, ctx: &Wildcard_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_wildcard_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_value_pattern(&mut self, ctx: &Value_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_value_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attr(&mut self, ctx: &AttrContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_attr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name_or_attr(&mut self, ctx: &Name_or_attrContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_name_or_attr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_group_pattern(&mut self, ctx: &Group_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_group_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sequence_pattern(&mut self, ctx: &Sequence_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_sequence_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_open_sequence_pattern(&mut self, ctx: &Open_sequence_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_open_sequence_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_maybe_sequence_pattern(&mut self, ctx: &Maybe_sequence_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_maybe_sequence_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_maybe_star_pattern(&mut self, ctx: &Maybe_star_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_maybe_star_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_pattern(&mut self, ctx: &Star_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mapping_pattern(&mut self, ctx: &Mapping_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_mapping_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_items_pattern(&mut self, ctx: &Items_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_items_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_key_value_pattern(&mut self, ctx: &Key_value_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_key_value_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_double_star_pattern(&mut self, ctx: &Double_star_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_double_star_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_class_pattern(&mut self, ctx: &Class_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_class_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_positional_patterns(&mut self, ctx: &Positional_patternsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_positional_patterns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keyword_patterns(&mut self, ctx: &Keyword_patternsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_keyword_patterns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keyword_pattern(&mut self, ctx: &Keyword_patternContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_keyword_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_alias(&mut self, ctx: &Type_aliasContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_alias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_params(&mut self, ctx: &Type_paramsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_params(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_param_seq(&mut self, ctx: &Type_param_seqContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_param_seq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_param(&mut self, ctx: &Type_paramContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_param(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_param_bound(&mut self, ctx: &Type_param_boundContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_param_bound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_param_default(&mut self, ctx: &Type_param_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_param_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_param_starred_default(&mut self, ctx: &Type_param_starred_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_param_starred_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressions(&mut self, ctx: &ExpressionsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_expressions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_yield_expr(&mut self, ctx: &Yield_exprContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_yield_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_expressions(&mut self, ctx: &Star_expressionsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_expressions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_expression(&mut self, ctx: &Star_expressionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_named_expressions(&mut self, ctx: &Star_named_expressionsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_named_expressions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_named_expression(&mut self, ctx: &Star_named_expressionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_named_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignment_expression(&mut self, ctx: &Assignment_expressionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_assignment_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_named_expression(&mut self, ctx: &Named_expressionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_named_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_disjunction(&mut self, ctx: &DisjunctionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_disjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conjunction(&mut self, ctx: &ConjunctionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_conjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inversion(&mut self, ctx: &InversionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_inversion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_comparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compare_op_bitwise_or_pair(&mut self, ctx: &Compare_op_bitwise_or_pairContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_compare_op_bitwise_or_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eq_bitwise_or(&mut self, ctx: &Eq_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_eq_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noteq_bitwise_or(&mut self, ctx: &Noteq_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_noteq_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lte_bitwise_or(&mut self, ctx: &Lte_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lte_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lt_bitwise_or(&mut self, ctx: &Lt_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lt_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gte_bitwise_or(&mut self, ctx: &Gte_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_gte_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_gt_bitwise_or(&mut self, ctx: &Gt_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_gt_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_notin_bitwise_or(&mut self, ctx: &Notin_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_notin_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_in_bitwise_or(&mut self, ctx: &In_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_in_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_isnot_bitwise_or(&mut self, ctx: &Isnot_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_isnot_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_is_bitwise_or(&mut self, ctx: &Is_bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_is_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwise_or(&mut self, ctx: &Bitwise_orContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_bitwise_or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwise_xor(&mut self, ctx: &Bitwise_xorContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_bitwise_xor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bitwise_and(&mut self, ctx: &Bitwise_andContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_bitwise_and(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shift_expr(&mut self, ctx: &Shift_exprContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_shift_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sum(&mut self, ctx: &SumContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_sum(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_term(&mut self, ctx: &TermContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_term(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_factor(&mut self, ctx: &FactorContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_factor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_power(&mut self, ctx: &PowerContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_power(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_await_primary(&mut self, ctx: &Await_primaryContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_await_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_slices(&mut self, ctx: &SlicesContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_slices(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_slice(&mut self, ctx: &SliceContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_slice(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atom(&mut self, ctx: &AtomContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_group(&mut self, ctx: &GroupContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_group(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdef(&mut self, ctx: &LambdefContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_params(&mut self, ctx: &Lambda_paramsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_params(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_parameters(&mut self, ctx: &Lambda_parametersContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_parameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_slash_no_default(&mut self, ctx: &Lambda_slash_no_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_slash_no_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_slash_with_default(&mut self, ctx: &Lambda_slash_with_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_slash_with_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_star_etc(&mut self, ctx: &Lambda_star_etcContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_star_etc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_kwds(&mut self, ctx: &Lambda_kwdsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_kwds(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_param_no_default(&mut self, ctx: &Lambda_param_no_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_param_no_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_param_with_default(&mut self, ctx: &Lambda_param_with_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_param_with_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_param_maybe_default(&mut self, ctx: &Lambda_param_maybe_defaultContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_param_maybe_default(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda_param(&mut self, ctx: &Lambda_paramContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_lambda_param(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fstring_middle(&mut self, ctx: &Fstring_middleContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_fstring_middle(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fstring_replacement_field(&mut self, ctx: &Fstring_replacement_fieldContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_fstring_replacement_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fstring_conversion(&mut self, ctx: &Fstring_conversionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_fstring_conversion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fstring_full_format_spec(&mut self, ctx: &Fstring_full_format_specContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_fstring_full_format_spec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fstring_format_spec(&mut self, ctx: &Fstring_format_specContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_fstring_format_spec(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fstring(&mut self, ctx: &FstringContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_fstring(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_string(&mut self, ctx: &StringContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_strings(&mut self, ctx: &StringsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_strings(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_list(&mut self, ctx: &ListContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tuple(&mut self, ctx: &TupleContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_tuple(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_set(&mut self, ctx: &SetContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_set(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dict(&mut self, ctx: &DictContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_dict(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_double_starred_kvpairs(&mut self, ctx: &Double_starred_kvpairsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_double_starred_kvpairs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_double_starred_kvpair(&mut self, ctx: &Double_starred_kvpairContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_double_starred_kvpair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_kvpair(&mut self, ctx: &KvpairContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_kvpair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_for_if_clauses(&mut self, ctx: &For_if_clausesContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_for_if_clauses(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_for_if_clause(&mut self, ctx: &For_if_clauseContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_for_if_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_listcomp(&mut self, ctx: &ListcompContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_listcomp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_setcomp(&mut self, ctx: &SetcompContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_setcomp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_genexp(&mut self, ctx: &GenexpContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_genexp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dictcomp(&mut self, ctx: &DictcompContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_dictcomp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_args(&mut self, ctx: &ArgsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_args(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_kwargs(&mut self, ctx: &KwargsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_kwargs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_starred_expression(&mut self, ctx: &Starred_expressionContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_starred_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_kwarg_or_starred(&mut self, ctx: &Kwarg_or_starredContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_kwarg_or_starred(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_kwarg_or_double_starred(&mut self, ctx: &Kwarg_or_double_starredContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_kwarg_or_double_starred(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_targets(&mut self, ctx: &Star_targetsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_targets(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_targets_list_seq(&mut self, ctx: &Star_targets_list_seqContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_targets_list_seq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_targets_tuple_seq(&mut self, ctx: &Star_targets_tuple_seqContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_targets_tuple_seq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_target(&mut self, ctx: &Star_targetContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_target_with_star_atom(&mut self, ctx: &Target_with_star_atomContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_target_with_star_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_star_atom(&mut self, ctx: &Star_atomContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_star_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_single_target(&mut self, ctx: &Single_targetContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_single_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_single_subscript_attribute_target(&mut self, ctx: &Single_subscript_attribute_targetContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_single_subscript_attribute_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_t_primary(&mut self, ctx: &T_primaryContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_t_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_del_targets(&mut self, ctx: &Del_targetsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_del_targets(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_del_target(&mut self, ctx: &Del_targetContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_del_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_del_t_atom(&mut self, ctx: &Del_t_atomContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_del_t_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_type_expressions(&mut self, ctx: &Type_expressionsContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_type_expressions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_type_comment(&mut self, ctx: &Func_type_commentContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_func_type_comment(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name_except_underscore(&mut self, ctx: &Name_except_underscoreContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_name_except_underscore(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name(&mut self, ctx: &NameContext<'input>){
		let result = <Self as PythonParserVisitorCompat>::visit_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}