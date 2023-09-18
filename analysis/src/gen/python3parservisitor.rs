#![allow(nonstandard_style)]
// Generated from Python3Parser.g4 by ANTLR 4.8
use super::python3parser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link Python3Parser}.
 */
pub trait Python3ParserVisitor<'input>: ParseTreeVisitor<'input, Python3ParserContextType> {
    /**
     * Visit a parse tree produced by {@link Python3Parser#single_input}.
     * @param ctx the parse tree
     */
    fn visit_single_input(&mut self, ctx: &Single_inputContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#file_input}.
     * @param ctx the parse tree
     */
    fn visit_file_input(&mut self, ctx: &File_inputContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#eval_input}.
     * @param ctx the parse tree
     */
    fn visit_eval_input(&mut self, ctx: &Eval_inputContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#decorator}.
     * @param ctx the parse tree
     */
    fn visit_decorator(&mut self, ctx: &DecoratorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#decorators}.
     * @param ctx the parse tree
     */
    fn visit_decorators(&mut self, ctx: &DecoratorsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#decorated}.
     * @param ctx the parse tree
     */
    fn visit_decorated(&mut self, ctx: &DecoratedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#async_funcdef}.
     * @param ctx the parse tree
     */
    fn visit_async_funcdef(&mut self, ctx: &Async_funcdefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#funcdef}.
     * @param ctx the parse tree
     */
    fn visit_funcdef(&mut self, ctx: &FuncdefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#parameters}.
     * @param ctx the parse tree
     */
    fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#typedargslist}.
     * @param ctx the parse tree
     */
    fn visit_typedargslist(&mut self, ctx: &TypedargslistContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#tfpdef}.
     * @param ctx the parse tree
     */
    fn visit_tfpdef(&mut self, ctx: &TfpdefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#varargslist}.
     * @param ctx the parse tree
     */
    fn visit_varargslist(&mut self, ctx: &VarargslistContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#vfpdef}.
     * @param ctx the parse tree
     */
    fn visit_vfpdef(&mut self, ctx: &VfpdefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#stmt}.
     * @param ctx the parse tree
     */
    fn visit_stmt(&mut self, ctx: &StmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#simple_stmts}.
     * @param ctx the parse tree
     */
    fn visit_simple_stmts(&mut self, ctx: &Simple_stmtsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#simple_stmt}.
     * @param ctx the parse tree
     */
    fn visit_simple_stmt(&mut self, ctx: &Simple_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#expr_stmt}.
     * @param ctx the parse tree
     */
    fn visit_expr_stmt(&mut self, ctx: &Expr_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#annassign}.
     * @param ctx the parse tree
     */
    fn visit_annassign(&mut self, ctx: &AnnassignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#testlist_star_expr}.
     * @param ctx the parse tree
     */
    fn visit_testlist_star_expr(&mut self, ctx: &Testlist_star_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#augassign}.
     * @param ctx the parse tree
     */
    fn visit_augassign(&mut self, ctx: &AugassignContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#del_stmt}.
     * @param ctx the parse tree
     */
    fn visit_del_stmt(&mut self, ctx: &Del_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#pass_stmt}.
     * @param ctx the parse tree
     */
    fn visit_pass_stmt(&mut self, ctx: &Pass_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#flow_stmt}.
     * @param ctx the parse tree
     */
    fn visit_flow_stmt(&mut self, ctx: &Flow_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#break_stmt}.
     * @param ctx the parse tree
     */
    fn visit_break_stmt(&mut self, ctx: &Break_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#continue_stmt}.
     * @param ctx the parse tree
     */
    fn visit_continue_stmt(&mut self, ctx: &Continue_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#return_stmt}.
     * @param ctx the parse tree
     */
    fn visit_return_stmt(&mut self, ctx: &Return_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#yield_stmt}.
     * @param ctx the parse tree
     */
    fn visit_yield_stmt(&mut self, ctx: &Yield_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#raise_stmt}.
     * @param ctx the parse tree
     */
    fn visit_raise_stmt(&mut self, ctx: &Raise_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_stmt}.
     * @param ctx the parse tree
     */
    fn visit_import_stmt(&mut self, ctx: &Import_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_name}.
     * @param ctx the parse tree
     */
    fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_from}.
     * @param ctx the parse tree
     */
    fn visit_import_from(&mut self, ctx: &Import_fromContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_as_name}.
     * @param ctx the parse tree
     */
    fn visit_import_as_name(&mut self, ctx: &Import_as_nameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dotted_as_name}.
     * @param ctx the parse tree
     */
    fn visit_dotted_as_name(&mut self, ctx: &Dotted_as_nameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_as_names}.
     * @param ctx the parse tree
     */
    fn visit_import_as_names(&mut self, ctx: &Import_as_namesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dotted_as_names}.
     * @param ctx the parse tree
     */
    fn visit_dotted_as_names(&mut self, ctx: &Dotted_as_namesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dotted_name}.
     * @param ctx the parse tree
     */
    fn visit_dotted_name(&mut self, ctx: &Dotted_nameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#global_stmt}.
     * @param ctx the parse tree
     */
    fn visit_global_stmt(&mut self, ctx: &Global_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#nonlocal_stmt}.
     * @param ctx the parse tree
     */
    fn visit_nonlocal_stmt(&mut self, ctx: &Nonlocal_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#assert_stmt}.
     * @param ctx the parse tree
     */
    fn visit_assert_stmt(&mut self, ctx: &Assert_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#compound_stmt}.
     * @param ctx the parse tree
     */
    fn visit_compound_stmt(&mut self, ctx: &Compound_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#async_stmt}.
     * @param ctx the parse tree
     */
    fn visit_async_stmt(&mut self, ctx: &Async_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#if_stmt}.
     * @param ctx the parse tree
     */
    fn visit_if_stmt(&mut self, ctx: &If_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#while_stmt}.
     * @param ctx the parse tree
     */
    fn visit_while_stmt(&mut self, ctx: &While_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#for_stmt}.
     * @param ctx the parse tree
     */
    fn visit_for_stmt(&mut self, ctx: &For_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#try_stmt}.
     * @param ctx the parse tree
     */
    fn visit_try_stmt(&mut self, ctx: &Try_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#with_stmt}.
     * @param ctx the parse tree
     */
    fn visit_with_stmt(&mut self, ctx: &With_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#with_item}.
     * @param ctx the parse tree
     */
    fn visit_with_item(&mut self, ctx: &With_itemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#except_clause}.
     * @param ctx the parse tree
     */
    fn visit_except_clause(&mut self, ctx: &Except_clauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#block}.
     * @param ctx the parse tree
     */
    fn visit_block(&mut self, ctx: &BlockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#match_stmt}.
     * @param ctx the parse tree
     */
    fn visit_match_stmt(&mut self, ctx: &Match_stmtContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#subject_expr}.
     * @param ctx the parse tree
     */
    fn visit_subject_expr(&mut self, ctx: &Subject_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_named_expressions}.
     * @param ctx the parse tree
     */
    fn visit_star_named_expressions(&mut self, ctx: &Star_named_expressionsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_named_expression}.
     * @param ctx the parse tree
     */
    fn visit_star_named_expression(&mut self, ctx: &Star_named_expressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#case_block}.
     * @param ctx the parse tree
     */
    fn visit_case_block(&mut self, ctx: &Case_blockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#guard}.
     * @param ctx the parse tree
     */
    fn visit_guard(&mut self, ctx: &GuardContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#patterns}.
     * @param ctx the parse tree
     */
    fn visit_patterns(&mut self, ctx: &PatternsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#pattern}.
     * @param ctx the parse tree
     */
    fn visit_pattern(&mut self, ctx: &PatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#as_pattern}.
     * @param ctx the parse tree
     */
    fn visit_as_pattern(&mut self, ctx: &As_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#or_pattern}.
     * @param ctx the parse tree
     */
    fn visit_or_pattern(&mut self, ctx: &Or_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#closed_pattern}.
     * @param ctx the parse tree
     */
    fn visit_closed_pattern(&mut self, ctx: &Closed_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#literal_pattern}.
     * @param ctx the parse tree
     */
    fn visit_literal_pattern(&mut self, ctx: &Literal_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#literal_expr}.
     * @param ctx the parse tree
     */
    fn visit_literal_expr(&mut self, ctx: &Literal_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#complex_number}.
     * @param ctx the parse tree
     */
    fn visit_complex_number(&mut self, ctx: &Complex_numberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#signed_number}.
     * @param ctx the parse tree
     */
    fn visit_signed_number(&mut self, ctx: &Signed_numberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#signed_real_number}.
     * @param ctx the parse tree
     */
    fn visit_signed_real_number(&mut self, ctx: &Signed_real_numberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#real_number}.
     * @param ctx the parse tree
     */
    fn visit_real_number(&mut self, ctx: &Real_numberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#imaginary_number}.
     * @param ctx the parse tree
     */
    fn visit_imaginary_number(&mut self, ctx: &Imaginary_numberContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#capture_pattern}.
     * @param ctx the parse tree
     */
    fn visit_capture_pattern(&mut self, ctx: &Capture_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#pattern_capture_target}.
     * @param ctx the parse tree
     */
    fn visit_pattern_capture_target(&mut self, ctx: &Pattern_capture_targetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#wildcard_pattern}.
     * @param ctx the parse tree
     */
    fn visit_wildcard_pattern(&mut self, ctx: &Wildcard_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#value_pattern}.
     * @param ctx the parse tree
     */
    fn visit_value_pattern(&mut self, ctx: &Value_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#attr}.
     * @param ctx the parse tree
     */
    fn visit_attr(&mut self, ctx: &AttrContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#name_or_attr}.
     * @param ctx the parse tree
     */
    fn visit_name_or_attr(&mut self, ctx: &Name_or_attrContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#group_pattern}.
     * @param ctx the parse tree
     */
    fn visit_group_pattern(&mut self, ctx: &Group_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#sequence_pattern}.
     * @param ctx the parse tree
     */
    fn visit_sequence_pattern(&mut self, ctx: &Sequence_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#open_sequence_pattern}.
     * @param ctx the parse tree
     */
    fn visit_open_sequence_pattern(&mut self, ctx: &Open_sequence_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#maybe_sequence_pattern}.
     * @param ctx the parse tree
     */
    fn visit_maybe_sequence_pattern(&mut self, ctx: &Maybe_sequence_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#maybe_star_pattern}.
     * @param ctx the parse tree
     */
    fn visit_maybe_star_pattern(&mut self, ctx: &Maybe_star_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_pattern}.
     * @param ctx the parse tree
     */
    fn visit_star_pattern(&mut self, ctx: &Star_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#mapping_pattern}.
     * @param ctx the parse tree
     */
    fn visit_mapping_pattern(&mut self, ctx: &Mapping_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#items_pattern}.
     * @param ctx the parse tree
     */
    fn visit_items_pattern(&mut self, ctx: &Items_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#key_value_pattern}.
     * @param ctx the parse tree
     */
    fn visit_key_value_pattern(&mut self, ctx: &Key_value_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#double_star_pattern}.
     * @param ctx the parse tree
     */
    fn visit_double_star_pattern(&mut self, ctx: &Double_star_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#class_pattern}.
     * @param ctx the parse tree
     */
    fn visit_class_pattern(&mut self, ctx: &Class_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#positional_patterns}.
     * @param ctx the parse tree
     */
    fn visit_positional_patterns(&mut self, ctx: &Positional_patternsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#keyword_patterns}.
     * @param ctx the parse tree
     */
    fn visit_keyword_patterns(&mut self, ctx: &Keyword_patternsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#keyword_pattern}.
     * @param ctx the parse tree
     */
    fn visit_keyword_pattern(&mut self, ctx: &Keyword_patternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#test}.
     * @param ctx the parse tree
     */
    fn visit_test(&mut self, ctx: &TestContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#test_nocond}.
     * @param ctx the parse tree
     */
    fn visit_test_nocond(&mut self, ctx: &Test_nocondContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#lambdef}.
     * @param ctx the parse tree
     */
    fn visit_lambdef(&mut self, ctx: &LambdefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#lambdef_nocond}.
     * @param ctx the parse tree
     */
    fn visit_lambdef_nocond(&mut self, ctx: &Lambdef_nocondContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#or_test}.
     * @param ctx the parse tree
     */
    fn visit_or_test(&mut self, ctx: &Or_testContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#and_test}.
     * @param ctx the parse tree
     */
    fn visit_and_test(&mut self, ctx: &And_testContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#not_test}.
     * @param ctx the parse tree
     */
    fn visit_not_test(&mut self, ctx: &Not_testContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comparison}.
     * @param ctx the parse tree
     */
    fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_op}.
     * @param ctx the parse tree
     */
    fn visit_comp_op(&mut self, ctx: &Comp_opContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_expr}.
     * @param ctx the parse tree
     */
    fn visit_star_expr(&mut self, ctx: &Star_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#expr}.
     * @param ctx the parse tree
     */
    fn visit_expr(&mut self, ctx: &ExprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#atom_expr}.
     * @param ctx the parse tree
     */
    fn visit_atom_expr(&mut self, ctx: &Atom_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#atom}.
     * @param ctx the parse tree
     */
    fn visit_atom(&mut self, ctx: &AtomContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#name}.
     * @param ctx the parse tree
     */
    fn visit_name(&mut self, ctx: &NameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#testlist_comp}.
     * @param ctx the parse tree
     */
    fn visit_testlist_comp(&mut self, ctx: &Testlist_compContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#trailer}.
     * @param ctx the parse tree
     */
    fn visit_trailer(&mut self, ctx: &TrailerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#subscriptlist}.
     * @param ctx the parse tree
     */
    fn visit_subscriptlist(&mut self, ctx: &SubscriptlistContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#subscript_}.
     * @param ctx the parse tree
     */
    fn visit_subscript_(&mut self, ctx: &Subscript_Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#sliceop}.
     * @param ctx the parse tree
     */
    fn visit_sliceop(&mut self, ctx: &SliceopContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#exprlist}.
     * @param ctx the parse tree
     */
    fn visit_exprlist(&mut self, ctx: &ExprlistContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#testlist}.
     * @param ctx the parse tree
     */
    fn visit_testlist(&mut self, ctx: &TestlistContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dictorsetmaker}.
     * @param ctx the parse tree
     */
    fn visit_dictorsetmaker(&mut self, ctx: &DictorsetmakerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#classdef}.
     * @param ctx the parse tree
     */
    fn visit_classdef(&mut self, ctx: &ClassdefContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#arglist}.
     * @param ctx the parse tree
     */
    fn visit_arglist(&mut self, ctx: &ArglistContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#argument}.
     * @param ctx the parse tree
     */
    fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_iter}.
     * @param ctx the parse tree
     */
    fn visit_comp_iter(&mut self, ctx: &Comp_iterContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_for}.
     * @param ctx the parse tree
     */
    fn visit_comp_for(&mut self, ctx: &Comp_forContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_if}.
     * @param ctx the parse tree
     */
    fn visit_comp_if(&mut self, ctx: &Comp_ifContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#encoding_decl}.
     * @param ctx the parse tree
     */
    fn visit_encoding_decl(&mut self, ctx: &Encoding_declContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#yield_expr}.
     * @param ctx the parse tree
     */
    fn visit_yield_expr(&mut self, ctx: &Yield_exprContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#yield_arg}.
     * @param ctx the parse tree
     */
    fn visit_yield_arg(&mut self, ctx: &Yield_argContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#strings}.
     * @param ctx the parse tree
     */
    fn visit_strings(&mut self, ctx: &StringsContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait Python3ParserVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = Python3ParserContextType>
{
    /**
     * Visit a parse tree produced by {@link Python3Parser#single_input}.
     * @param ctx the parse tree
     */
    fn visit_single_input(&mut self, ctx: &Single_inputContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#file_input}.
     * @param ctx the parse tree
     */
    fn visit_file_input(&mut self, ctx: &File_inputContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#eval_input}.
     * @param ctx the parse tree
     */
    fn visit_eval_input(&mut self, ctx: &Eval_inputContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#decorator}.
     * @param ctx the parse tree
     */
    fn visit_decorator(&mut self, ctx: &DecoratorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#decorators}.
     * @param ctx the parse tree
     */
    fn visit_decorators(&mut self, ctx: &DecoratorsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#decorated}.
     * @param ctx the parse tree
     */
    fn visit_decorated(&mut self, ctx: &DecoratedContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#async_funcdef}.
     * @param ctx the parse tree
     */
    fn visit_async_funcdef(&mut self, ctx: &Async_funcdefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#funcdef}.
     * @param ctx the parse tree
     */
    fn visit_funcdef(&mut self, ctx: &FuncdefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#parameters}.
     * @param ctx the parse tree
     */
    fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#typedargslist}.
     * @param ctx the parse tree
     */
    fn visit_typedargslist(&mut self, ctx: &TypedargslistContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#tfpdef}.
     * @param ctx the parse tree
     */
    fn visit_tfpdef(&mut self, ctx: &TfpdefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#varargslist}.
     * @param ctx the parse tree
     */
    fn visit_varargslist(&mut self, ctx: &VarargslistContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#vfpdef}.
     * @param ctx the parse tree
     */
    fn visit_vfpdef(&mut self, ctx: &VfpdefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#stmt}.
     * @param ctx the parse tree
     */
    fn visit_stmt(&mut self, ctx: &StmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#simple_stmts}.
     * @param ctx the parse tree
     */
    fn visit_simple_stmts(&mut self, ctx: &Simple_stmtsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#simple_stmt}.
     * @param ctx the parse tree
     */
    fn visit_simple_stmt(&mut self, ctx: &Simple_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#expr_stmt}.
     * @param ctx the parse tree
     */
    fn visit_expr_stmt(&mut self, ctx: &Expr_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#annassign}.
     * @param ctx the parse tree
     */
    fn visit_annassign(&mut self, ctx: &AnnassignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#testlist_star_expr}.
     * @param ctx the parse tree
     */
    fn visit_testlist_star_expr(
        &mut self,
        ctx: &Testlist_star_exprContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#augassign}.
     * @param ctx the parse tree
     */
    fn visit_augassign(&mut self, ctx: &AugassignContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#del_stmt}.
     * @param ctx the parse tree
     */
    fn visit_del_stmt(&mut self, ctx: &Del_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#pass_stmt}.
     * @param ctx the parse tree
     */
    fn visit_pass_stmt(&mut self, ctx: &Pass_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#flow_stmt}.
     * @param ctx the parse tree
     */
    fn visit_flow_stmt(&mut self, ctx: &Flow_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#break_stmt}.
     * @param ctx the parse tree
     */
    fn visit_break_stmt(&mut self, ctx: &Break_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#continue_stmt}.
     * @param ctx the parse tree
     */
    fn visit_continue_stmt(&mut self, ctx: &Continue_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#return_stmt}.
     * @param ctx the parse tree
     */
    fn visit_return_stmt(&mut self, ctx: &Return_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#yield_stmt}.
     * @param ctx the parse tree
     */
    fn visit_yield_stmt(&mut self, ctx: &Yield_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#raise_stmt}.
     * @param ctx the parse tree
     */
    fn visit_raise_stmt(&mut self, ctx: &Raise_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_stmt}.
     * @param ctx the parse tree
     */
    fn visit_import_stmt(&mut self, ctx: &Import_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_name}.
     * @param ctx the parse tree
     */
    fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_from}.
     * @param ctx the parse tree
     */
    fn visit_import_from(&mut self, ctx: &Import_fromContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_as_name}.
     * @param ctx the parse tree
     */
    fn visit_import_as_name(&mut self, ctx: &Import_as_nameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dotted_as_name}.
     * @param ctx the parse tree
     */
    fn visit_dotted_as_name(&mut self, ctx: &Dotted_as_nameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#import_as_names}.
     * @param ctx the parse tree
     */
    fn visit_import_as_names(&mut self, ctx: &Import_as_namesContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dotted_as_names}.
     * @param ctx the parse tree
     */
    fn visit_dotted_as_names(&mut self, ctx: &Dotted_as_namesContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dotted_name}.
     * @param ctx the parse tree
     */
    fn visit_dotted_name(&mut self, ctx: &Dotted_nameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#global_stmt}.
     * @param ctx the parse tree
     */
    fn visit_global_stmt(&mut self, ctx: &Global_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#nonlocal_stmt}.
     * @param ctx the parse tree
     */
    fn visit_nonlocal_stmt(&mut self, ctx: &Nonlocal_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#assert_stmt}.
     * @param ctx the parse tree
     */
    fn visit_assert_stmt(&mut self, ctx: &Assert_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#compound_stmt}.
     * @param ctx the parse tree
     */
    fn visit_compound_stmt(&mut self, ctx: &Compound_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#async_stmt}.
     * @param ctx the parse tree
     */
    fn visit_async_stmt(&mut self, ctx: &Async_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#if_stmt}.
     * @param ctx the parse tree
     */
    fn visit_if_stmt(&mut self, ctx: &If_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#while_stmt}.
     * @param ctx the parse tree
     */
    fn visit_while_stmt(&mut self, ctx: &While_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#for_stmt}.
     * @param ctx the parse tree
     */
    fn visit_for_stmt(&mut self, ctx: &For_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#try_stmt}.
     * @param ctx the parse tree
     */
    fn visit_try_stmt(&mut self, ctx: &Try_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#with_stmt}.
     * @param ctx the parse tree
     */
    fn visit_with_stmt(&mut self, ctx: &With_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#with_item}.
     * @param ctx the parse tree
     */
    fn visit_with_item(&mut self, ctx: &With_itemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#except_clause}.
     * @param ctx the parse tree
     */
    fn visit_except_clause(&mut self, ctx: &Except_clauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#block}.
     * @param ctx the parse tree
     */
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#match_stmt}.
     * @param ctx the parse tree
     */
    fn visit_match_stmt(&mut self, ctx: &Match_stmtContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#subject_expr}.
     * @param ctx the parse tree
     */
    fn visit_subject_expr(&mut self, ctx: &Subject_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_named_expressions}.
     * @param ctx the parse tree
     */
    fn visit_star_named_expressions(
        &mut self,
        ctx: &Star_named_expressionsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_named_expression}.
     * @param ctx the parse tree
     */
    fn visit_star_named_expression(
        &mut self,
        ctx: &Star_named_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#case_block}.
     * @param ctx the parse tree
     */
    fn visit_case_block(&mut self, ctx: &Case_blockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#guard}.
     * @param ctx the parse tree
     */
    fn visit_guard(&mut self, ctx: &GuardContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#patterns}.
     * @param ctx the parse tree
     */
    fn visit_patterns(&mut self, ctx: &PatternsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#pattern}.
     * @param ctx the parse tree
     */
    fn visit_pattern(&mut self, ctx: &PatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#as_pattern}.
     * @param ctx the parse tree
     */
    fn visit_as_pattern(&mut self, ctx: &As_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#or_pattern}.
     * @param ctx the parse tree
     */
    fn visit_or_pattern(&mut self, ctx: &Or_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#closed_pattern}.
     * @param ctx the parse tree
     */
    fn visit_closed_pattern(&mut self, ctx: &Closed_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#literal_pattern}.
     * @param ctx the parse tree
     */
    fn visit_literal_pattern(&mut self, ctx: &Literal_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#literal_expr}.
     * @param ctx the parse tree
     */
    fn visit_literal_expr(&mut self, ctx: &Literal_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#complex_number}.
     * @param ctx the parse tree
     */
    fn visit_complex_number(&mut self, ctx: &Complex_numberContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#signed_number}.
     * @param ctx the parse tree
     */
    fn visit_signed_number(&mut self, ctx: &Signed_numberContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#signed_real_number}.
     * @param ctx the parse tree
     */
    fn visit_signed_real_number(
        &mut self,
        ctx: &Signed_real_numberContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#real_number}.
     * @param ctx the parse tree
     */
    fn visit_real_number(&mut self, ctx: &Real_numberContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#imaginary_number}.
     * @param ctx the parse tree
     */
    fn visit_imaginary_number(&mut self, ctx: &Imaginary_numberContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#capture_pattern}.
     * @param ctx the parse tree
     */
    fn visit_capture_pattern(&mut self, ctx: &Capture_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#pattern_capture_target}.
     * @param ctx the parse tree
     */
    fn visit_pattern_capture_target(
        &mut self,
        ctx: &Pattern_capture_targetContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#wildcard_pattern}.
     * @param ctx the parse tree
     */
    fn visit_wildcard_pattern(&mut self, ctx: &Wildcard_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#value_pattern}.
     * @param ctx the parse tree
     */
    fn visit_value_pattern(&mut self, ctx: &Value_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#attr}.
     * @param ctx the parse tree
     */
    fn visit_attr(&mut self, ctx: &AttrContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#name_or_attr}.
     * @param ctx the parse tree
     */
    fn visit_name_or_attr(&mut self, ctx: &Name_or_attrContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#group_pattern}.
     * @param ctx the parse tree
     */
    fn visit_group_pattern(&mut self, ctx: &Group_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#sequence_pattern}.
     * @param ctx the parse tree
     */
    fn visit_sequence_pattern(&mut self, ctx: &Sequence_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#open_sequence_pattern}.
     * @param ctx the parse tree
     */
    fn visit_open_sequence_pattern(
        &mut self,
        ctx: &Open_sequence_patternContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#maybe_sequence_pattern}.
     * @param ctx the parse tree
     */
    fn visit_maybe_sequence_pattern(
        &mut self,
        ctx: &Maybe_sequence_patternContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#maybe_star_pattern}.
     * @param ctx the parse tree
     */
    fn visit_maybe_star_pattern(
        &mut self,
        ctx: &Maybe_star_patternContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_pattern}.
     * @param ctx the parse tree
     */
    fn visit_star_pattern(&mut self, ctx: &Star_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#mapping_pattern}.
     * @param ctx the parse tree
     */
    fn visit_mapping_pattern(&mut self, ctx: &Mapping_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#items_pattern}.
     * @param ctx the parse tree
     */
    fn visit_items_pattern(&mut self, ctx: &Items_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#key_value_pattern}.
     * @param ctx the parse tree
     */
    fn visit_key_value_pattern(&mut self, ctx: &Key_value_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#double_star_pattern}.
     * @param ctx the parse tree
     */
    fn visit_double_star_pattern(
        &mut self,
        ctx: &Double_star_patternContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#class_pattern}.
     * @param ctx the parse tree
     */
    fn visit_class_pattern(&mut self, ctx: &Class_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#positional_patterns}.
     * @param ctx the parse tree
     */
    fn visit_positional_patterns(
        &mut self,
        ctx: &Positional_patternsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#keyword_patterns}.
     * @param ctx the parse tree
     */
    fn visit_keyword_patterns(&mut self, ctx: &Keyword_patternsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#keyword_pattern}.
     * @param ctx the parse tree
     */
    fn visit_keyword_pattern(&mut self, ctx: &Keyword_patternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#test}.
     * @param ctx the parse tree
     */
    fn visit_test(&mut self, ctx: &TestContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#test_nocond}.
     * @param ctx the parse tree
     */
    fn visit_test_nocond(&mut self, ctx: &Test_nocondContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#lambdef}.
     * @param ctx the parse tree
     */
    fn visit_lambdef(&mut self, ctx: &LambdefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#lambdef_nocond}.
     * @param ctx the parse tree
     */
    fn visit_lambdef_nocond(&mut self, ctx: &Lambdef_nocondContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#or_test}.
     * @param ctx the parse tree
     */
    fn visit_or_test(&mut self, ctx: &Or_testContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#and_test}.
     * @param ctx the parse tree
     */
    fn visit_and_test(&mut self, ctx: &And_testContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#not_test}.
     * @param ctx the parse tree
     */
    fn visit_not_test(&mut self, ctx: &Not_testContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comparison}.
     * @param ctx the parse tree
     */
    fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_op}.
     * @param ctx the parse tree
     */
    fn visit_comp_op(&mut self, ctx: &Comp_opContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#star_expr}.
     * @param ctx the parse tree
     */
    fn visit_star_expr(&mut self, ctx: &Star_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#expr}.
     * @param ctx the parse tree
     */
    fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#atom_expr}.
     * @param ctx the parse tree
     */
    fn visit_atom_expr(&mut self, ctx: &Atom_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#atom}.
     * @param ctx the parse tree
     */
    fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#name}.
     * @param ctx the parse tree
     */
    fn visit_name(&mut self, ctx: &NameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#testlist_comp}.
     * @param ctx the parse tree
     */
    fn visit_testlist_comp(&mut self, ctx: &Testlist_compContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#trailer}.
     * @param ctx the parse tree
     */
    fn visit_trailer(&mut self, ctx: &TrailerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#subscriptlist}.
     * @param ctx the parse tree
     */
    fn visit_subscriptlist(&mut self, ctx: &SubscriptlistContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#subscript_}.
     * @param ctx the parse tree
     */
    fn visit_subscript_(&mut self, ctx: &Subscript_Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#sliceop}.
     * @param ctx the parse tree
     */
    fn visit_sliceop(&mut self, ctx: &SliceopContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#exprlist}.
     * @param ctx the parse tree
     */
    fn visit_exprlist(&mut self, ctx: &ExprlistContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#testlist}.
     * @param ctx the parse tree
     */
    fn visit_testlist(&mut self, ctx: &TestlistContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#dictorsetmaker}.
     * @param ctx the parse tree
     */
    fn visit_dictorsetmaker(&mut self, ctx: &DictorsetmakerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#classdef}.
     * @param ctx the parse tree
     */
    fn visit_classdef(&mut self, ctx: &ClassdefContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#arglist}.
     * @param ctx the parse tree
     */
    fn visit_arglist(&mut self, ctx: &ArglistContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#argument}.
     * @param ctx the parse tree
     */
    fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_iter}.
     * @param ctx the parse tree
     */
    fn visit_comp_iter(&mut self, ctx: &Comp_iterContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_for}.
     * @param ctx the parse tree
     */
    fn visit_comp_for(&mut self, ctx: &Comp_forContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#comp_if}.
     * @param ctx the parse tree
     */
    fn visit_comp_if(&mut self, ctx: &Comp_ifContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#encoding_decl}.
     * @param ctx the parse tree
     */
    fn visit_encoding_decl(&mut self, ctx: &Encoding_declContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#yield_expr}.
     * @param ctx the parse tree
     */
    fn visit_yield_expr(&mut self, ctx: &Yield_exprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#yield_arg}.
     * @param ctx the parse tree
     */
    fn visit_yield_arg(&mut self, ctx: &Yield_argContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link Python3Parser#strings}.
     * @param ctx the parse tree
     */
    fn visit_strings(&mut self, ctx: &StringsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> Python3ParserVisitor<'input> for T
where
    T: Python3ParserVisitorCompat<'input>,
{
    fn visit_single_input(&mut self, ctx: &Single_inputContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_single_input(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_file_input(&mut self, ctx: &File_inputContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_file_input(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_eval_input(&mut self, ctx: &Eval_inputContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_eval_input(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_decorator(&mut self, ctx: &DecoratorContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_decorator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_decorators(&mut self, ctx: &DecoratorsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_decorators(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_decorated(&mut self, ctx: &DecoratedContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_decorated(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_async_funcdef(&mut self, ctx: &Async_funcdefContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_async_funcdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_funcdef(&mut self, ctx: &FuncdefContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_funcdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parameters(&mut self, ctx: &ParametersContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_parameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_typedargslist(&mut self, ctx: &TypedargslistContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_typedargslist(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_tfpdef(&mut self, ctx: &TfpdefContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_tfpdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_varargslist(&mut self, ctx: &VarargslistContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_varargslist(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_vfpdef(&mut self, ctx: &VfpdefContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_vfpdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_stmt(&mut self, ctx: &StmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simple_stmts(&mut self, ctx: &Simple_stmtsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_simple_stmts(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simple_stmt(&mut self, ctx: &Simple_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_simple_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expr_stmt(&mut self, ctx: &Expr_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_expr_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_annassign(&mut self, ctx: &AnnassignContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_annassign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_testlist_star_expr(&mut self, ctx: &Testlist_star_exprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_testlist_star_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_augassign(&mut self, ctx: &AugassignContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_augassign(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_del_stmt(&mut self, ctx: &Del_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_del_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pass_stmt(&mut self, ctx: &Pass_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_pass_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_flow_stmt(&mut self, ctx: &Flow_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_flow_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_break_stmt(&mut self, ctx: &Break_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_break_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_continue_stmt(&mut self, ctx: &Continue_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_continue_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_return_stmt(&mut self, ctx: &Return_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_return_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yield_stmt(&mut self, ctx: &Yield_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_yield_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_raise_stmt(&mut self, ctx: &Raise_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_raise_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_stmt(&mut self, ctx: &Import_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_import_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_name(&mut self, ctx: &Import_nameContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_import_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_from(&mut self, ctx: &Import_fromContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_import_from(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_as_name(&mut self, ctx: &Import_as_nameContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_import_as_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dotted_as_name(&mut self, ctx: &Dotted_as_nameContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_dotted_as_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_import_as_names(&mut self, ctx: &Import_as_namesContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_import_as_names(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dotted_as_names(&mut self, ctx: &Dotted_as_namesContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_dotted_as_names(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dotted_name(&mut self, ctx: &Dotted_nameContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_dotted_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_global_stmt(&mut self, ctx: &Global_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_global_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nonlocal_stmt(&mut self, ctx: &Nonlocal_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_nonlocal_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_assert_stmt(&mut self, ctx: &Assert_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_assert_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_compound_stmt(&mut self, ctx: &Compound_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_compound_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_async_stmt(&mut self, ctx: &Async_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_async_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_if_stmt(&mut self, ctx: &If_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_if_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_while_stmt(&mut self, ctx: &While_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_while_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_for_stmt(&mut self, ctx: &For_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_for_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_try_stmt(&mut self, ctx: &Try_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_try_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_with_stmt(&mut self, ctx: &With_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_with_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_with_item(&mut self, ctx: &With_itemContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_with_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_except_clause(&mut self, ctx: &Except_clauseContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_except_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_block(&mut self, ctx: &BlockContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_match_stmt(&mut self, ctx: &Match_stmtContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_match_stmt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_subject_expr(&mut self, ctx: &Subject_exprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_subject_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_star_named_expressions(&mut self, ctx: &Star_named_expressionsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_star_named_expressions(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_star_named_expression(&mut self, ctx: &Star_named_expressionContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_star_named_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_case_block(&mut self, ctx: &Case_blockContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_case_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_guard(&mut self, ctx: &GuardContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_guard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_patterns(&mut self, ctx: &PatternsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_patterns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pattern(&mut self, ctx: &PatternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_as_pattern(&mut self, ctx: &As_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_as_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_or_pattern(&mut self, ctx: &Or_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_or_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_closed_pattern(&mut self, ctx: &Closed_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_closed_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_literal_pattern(&mut self, ctx: &Literal_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_literal_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_literal_expr(&mut self, ctx: &Literal_exprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_literal_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_complex_number(&mut self, ctx: &Complex_numberContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_complex_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_signed_number(&mut self, ctx: &Signed_numberContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_signed_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_signed_real_number(&mut self, ctx: &Signed_real_numberContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_signed_real_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_real_number(&mut self, ctx: &Real_numberContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_real_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_imaginary_number(&mut self, ctx: &Imaginary_numberContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_imaginary_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_capture_pattern(&mut self, ctx: &Capture_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_capture_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pattern_capture_target(&mut self, ctx: &Pattern_capture_targetContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_pattern_capture_target(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_wildcard_pattern(&mut self, ctx: &Wildcard_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_wildcard_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_value_pattern(&mut self, ctx: &Value_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_value_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_attr(&mut self, ctx: &AttrContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_attr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_name_or_attr(&mut self, ctx: &Name_or_attrContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_name_or_attr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_group_pattern(&mut self, ctx: &Group_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_group_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sequence_pattern(&mut self, ctx: &Sequence_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_sequence_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_open_sequence_pattern(&mut self, ctx: &Open_sequence_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_open_sequence_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_maybe_sequence_pattern(&mut self, ctx: &Maybe_sequence_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_maybe_sequence_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_maybe_star_pattern(&mut self, ctx: &Maybe_star_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_maybe_star_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_star_pattern(&mut self, ctx: &Star_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_star_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_mapping_pattern(&mut self, ctx: &Mapping_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_mapping_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_items_pattern(&mut self, ctx: &Items_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_items_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_key_value_pattern(&mut self, ctx: &Key_value_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_key_value_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_double_star_pattern(&mut self, ctx: &Double_star_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_double_star_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_class_pattern(&mut self, ctx: &Class_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_class_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_positional_patterns(&mut self, ctx: &Positional_patternsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_positional_patterns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_keyword_patterns(&mut self, ctx: &Keyword_patternsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_keyword_patterns(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_keyword_pattern(&mut self, ctx: &Keyword_patternContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_keyword_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_test(&mut self, ctx: &TestContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_test(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_test_nocond(&mut self, ctx: &Test_nocondContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_test_nocond(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lambdef(&mut self, ctx: &LambdefContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_lambdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lambdef_nocond(&mut self, ctx: &Lambdef_nocondContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_lambdef_nocond(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_or_test(&mut self, ctx: &Or_testContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_or_test(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_and_test(&mut self, ctx: &And_testContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_and_test(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_not_test(&mut self, ctx: &Not_testContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_not_test(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_comparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comp_op(&mut self, ctx: &Comp_opContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_comp_op(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_star_expr(&mut self, ctx: &Star_exprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_star_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_expr(&mut self, ctx: &ExprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_atom_expr(&mut self, ctx: &Atom_exprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_atom_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_atom(&mut self, ctx: &AtomContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_name(&mut self, ctx: &NameContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_testlist_comp(&mut self, ctx: &Testlist_compContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_testlist_comp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trailer(&mut self, ctx: &TrailerContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_trailer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_subscriptlist(&mut self, ctx: &SubscriptlistContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_subscriptlist(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_subscript_(&mut self, ctx: &Subscript_Context<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_subscript_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sliceop(&mut self, ctx: &SliceopContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_sliceop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_exprlist(&mut self, ctx: &ExprlistContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_exprlist(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_testlist(&mut self, ctx: &TestlistContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_testlist(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dictorsetmaker(&mut self, ctx: &DictorsetmakerContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_dictorsetmaker(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_classdef(&mut self, ctx: &ClassdefContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_classdef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_arglist(&mut self, ctx: &ArglistContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_arglist(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_argument(&mut self, ctx: &ArgumentContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_argument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comp_iter(&mut self, ctx: &Comp_iterContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_comp_iter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comp_for(&mut self, ctx: &Comp_forContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_comp_for(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comp_if(&mut self, ctx: &Comp_ifContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_comp_if(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_encoding_decl(&mut self, ctx: &Encoding_declContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_encoding_decl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yield_expr(&mut self, ctx: &Yield_exprContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_yield_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yield_arg(&mut self, ctx: &Yield_argContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_yield_arg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_strings(&mut self, ctx: &StringsContext<'input>) {
        let result = <Self as Python3ParserVisitorCompat>::visit_strings(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
