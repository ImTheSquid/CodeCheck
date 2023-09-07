#![allow(nonstandard_style)]
// Generated from CPP14Parser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::cpp14parser::*;

pub trait CPP14ParserListener<'input> : ParseTreeListener<'input,CPP14ParserContextType>{

/**
 * Enter a parse tree produced by {@link CPP14Parser#translationUnit}.
 * @param ctx the parse tree
 */
fn enter_translationUnit(&mut self, _ctx: &TranslationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#translationUnit}.
 * @param ctx the parse tree
 */
fn exit_translationUnit(&mut self, _ctx: &TranslationUnitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#idExpression}.
 * @param ctx the parse tree
 */
fn enter_idExpression(&mut self, _ctx: &IdExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#idExpression}.
 * @param ctx the parse tree
 */
fn exit_idExpression(&mut self, _ctx: &IdExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#unqualifiedId}.
 * @param ctx the parse tree
 */
fn enter_unqualifiedId(&mut self, _ctx: &UnqualifiedIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#unqualifiedId}.
 * @param ctx the parse tree
 */
fn exit_unqualifiedId(&mut self, _ctx: &UnqualifiedIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#qualifiedId}.
 * @param ctx the parse tree
 */
fn enter_qualifiedId(&mut self, _ctx: &QualifiedIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#qualifiedId}.
 * @param ctx the parse tree
 */
fn exit_qualifiedId(&mut self, _ctx: &QualifiedIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#nestedNameSpecifier}.
 * @param ctx the parse tree
 */
fn enter_nestedNameSpecifier(&mut self, _ctx: &NestedNameSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#nestedNameSpecifier}.
 * @param ctx the parse tree
 */
fn exit_nestedNameSpecifier(&mut self, _ctx: &NestedNameSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#lambdaIntroducer}.
 * @param ctx the parse tree
 */
fn enter_lambdaIntroducer(&mut self, _ctx: &LambdaIntroducerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#lambdaIntroducer}.
 * @param ctx the parse tree
 */
fn exit_lambdaIntroducer(&mut self, _ctx: &LambdaIntroducerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#lambdaCapture}.
 * @param ctx the parse tree
 */
fn enter_lambdaCapture(&mut self, _ctx: &LambdaCaptureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#lambdaCapture}.
 * @param ctx the parse tree
 */
fn exit_lambdaCapture(&mut self, _ctx: &LambdaCaptureContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#captureDefault}.
 * @param ctx the parse tree
 */
fn enter_captureDefault(&mut self, _ctx: &CaptureDefaultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#captureDefault}.
 * @param ctx the parse tree
 */
fn exit_captureDefault(&mut self, _ctx: &CaptureDefaultContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#captureList}.
 * @param ctx the parse tree
 */
fn enter_captureList(&mut self, _ctx: &CaptureListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#captureList}.
 * @param ctx the parse tree
 */
fn exit_captureList(&mut self, _ctx: &CaptureListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#capture}.
 * @param ctx the parse tree
 */
fn enter_capture(&mut self, _ctx: &CaptureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#capture}.
 * @param ctx the parse tree
 */
fn exit_capture(&mut self, _ctx: &CaptureContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#simpleCapture}.
 * @param ctx the parse tree
 */
fn enter_simpleCapture(&mut self, _ctx: &SimpleCaptureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#simpleCapture}.
 * @param ctx the parse tree
 */
fn exit_simpleCapture(&mut self, _ctx: &SimpleCaptureContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#initcapture}.
 * @param ctx the parse tree
 */
fn enter_initcapture(&mut self, _ctx: &InitcaptureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#initcapture}.
 * @param ctx the parse tree
 */
fn exit_initcapture(&mut self, _ctx: &InitcaptureContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#lambdaDeclarator}.
 * @param ctx the parse tree
 */
fn enter_lambdaDeclarator(&mut self, _ctx: &LambdaDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#lambdaDeclarator}.
 * @param ctx the parse tree
 */
fn exit_lambdaDeclarator(&mut self, _ctx: &LambdaDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn enter_postfixExpression(&mut self, _ctx: &PostfixExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn exit_postfixExpression(&mut self, _ctx: &PostfixExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typeIdOfTheTypeId}.
 * @param ctx the parse tree
 */
fn enter_typeIdOfTheTypeId(&mut self, _ctx: &TypeIdOfTheTypeIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typeIdOfTheTypeId}.
 * @param ctx the parse tree
 */
fn exit_typeIdOfTheTypeId(&mut self, _ctx: &TypeIdOfTheTypeIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#expressionList}.
 * @param ctx the parse tree
 */
fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#expressionList}.
 * @param ctx the parse tree
 */
fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#pseudoDestructorName}.
 * @param ctx the parse tree
 */
fn enter_pseudoDestructorName(&mut self, _ctx: &PseudoDestructorNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#pseudoDestructorName}.
 * @param ctx the parse tree
 */
fn exit_pseudoDestructorName(&mut self, _ctx: &PseudoDestructorNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#unaryOperator}.
 * @param ctx the parse tree
 */
fn enter_unaryOperator(&mut self, _ctx: &UnaryOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#unaryOperator}.
 * @param ctx the parse tree
 */
fn exit_unaryOperator(&mut self, _ctx: &UnaryOperatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#newExpression_}.
 * @param ctx the parse tree
 */
fn enter_newExpression_(&mut self, _ctx: &NewExpression_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#newExpression_}.
 * @param ctx the parse tree
 */
fn exit_newExpression_(&mut self, _ctx: &NewExpression_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#newPlacement}.
 * @param ctx the parse tree
 */
fn enter_newPlacement(&mut self, _ctx: &NewPlacementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#newPlacement}.
 * @param ctx the parse tree
 */
fn exit_newPlacement(&mut self, _ctx: &NewPlacementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#newTypeId}.
 * @param ctx the parse tree
 */
fn enter_newTypeId(&mut self, _ctx: &NewTypeIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#newTypeId}.
 * @param ctx the parse tree
 */
fn exit_newTypeId(&mut self, _ctx: &NewTypeIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#newDeclarator_}.
 * @param ctx the parse tree
 */
fn enter_newDeclarator_(&mut self, _ctx: &NewDeclarator_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#newDeclarator_}.
 * @param ctx the parse tree
 */
fn exit_newDeclarator_(&mut self, _ctx: &NewDeclarator_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#noPointerNewDeclarator}.
 * @param ctx the parse tree
 */
fn enter_noPointerNewDeclarator(&mut self, _ctx: &NoPointerNewDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#noPointerNewDeclarator}.
 * @param ctx the parse tree
 */
fn exit_noPointerNewDeclarator(&mut self, _ctx: &NoPointerNewDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#newInitializer_}.
 * @param ctx the parse tree
 */
fn enter_newInitializer_(&mut self, _ctx: &NewInitializer_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#newInitializer_}.
 * @param ctx the parse tree
 */
fn exit_newInitializer_(&mut self, _ctx: &NewInitializer_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#deleteExpression}.
 * @param ctx the parse tree
 */
fn enter_deleteExpression(&mut self, _ctx: &DeleteExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#deleteExpression}.
 * @param ctx the parse tree
 */
fn exit_deleteExpression(&mut self, _ctx: &DeleteExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#noExceptExpression}.
 * @param ctx the parse tree
 */
fn enter_noExceptExpression(&mut self, _ctx: &NoExceptExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#noExceptExpression}.
 * @param ctx the parse tree
 */
fn exit_noExceptExpression(&mut self, _ctx: &NoExceptExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#castExpression}.
 * @param ctx the parse tree
 */
fn enter_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#castExpression}.
 * @param ctx the parse tree
 */
fn exit_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#pointerMemberExpression}.
 * @param ctx the parse tree
 */
fn enter_pointerMemberExpression(&mut self, _ctx: &PointerMemberExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#pointerMemberExpression}.
 * @param ctx the parse tree
 */
fn exit_pointerMemberExpression(&mut self, _ctx: &PointerMemberExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#shiftExpression}.
 * @param ctx the parse tree
 */
fn enter_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#shiftExpression}.
 * @param ctx the parse tree
 */
fn exit_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#shiftOperator}.
 * @param ctx the parse tree
 */
fn enter_shiftOperator(&mut self, _ctx: &ShiftOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#shiftOperator}.
 * @param ctx the parse tree
 */
fn exit_shiftOperator(&mut self, _ctx: &ShiftOperatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#relationalExpression}.
 * @param ctx the parse tree
 */
fn enter_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#relationalExpression}.
 * @param ctx the parse tree
 */
fn exit_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn enter_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn exit_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#andExpression}.
 * @param ctx the parse tree
 */
fn enter_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#andExpression}.
 * @param ctx the parse tree
 */
fn exit_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#exclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn enter_exclusiveOrExpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#exclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn exit_exclusiveOrExpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#inclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn enter_inclusiveOrExpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#inclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn exit_inclusiveOrExpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#logicalAndExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn enter_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#logicalOrExpression}.
 * @param ctx the parse tree
 */
fn exit_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#conditionalExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalExpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#conditionalExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalExpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#assignmentExpression}.
 * @param ctx the parse tree
 */
fn enter_assignmentExpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#assignmentExpression}.
 * @param ctx the parse tree
 */
fn exit_assignmentExpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#assignmentOperator}.
 * @param ctx the parse tree
 */
fn enter_assignmentOperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#assignmentOperator}.
 * @param ctx the parse tree
 */
fn exit_assignmentOperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#constantExpression}.
 * @param ctx the parse tree
 */
fn enter_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#constantExpression}.
 * @param ctx the parse tree
 */
fn exit_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#labeledStatement}.
 * @param ctx the parse tree
 */
fn enter_labeledStatement(&mut self, _ctx: &LabeledStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#labeledStatement}.
 * @param ctx the parse tree
 */
fn exit_labeledStatement(&mut self, _ctx: &LabeledStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#expressionStatement}.
 * @param ctx the parse tree
 */
fn enter_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#expressionStatement}.
 * @param ctx the parse tree
 */
fn exit_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#compoundStatement}.
 * @param ctx the parse tree
 */
fn enter_compoundStatement(&mut self, _ctx: &CompoundStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#compoundStatement}.
 * @param ctx the parse tree
 */
fn exit_compoundStatement(&mut self, _ctx: &CompoundStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#statementSeq}.
 * @param ctx the parse tree
 */
fn enter_statementSeq(&mut self, _ctx: &StatementSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#statementSeq}.
 * @param ctx the parse tree
 */
fn exit_statementSeq(&mut self, _ctx: &StatementSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#selectionStatement}.
 * @param ctx the parse tree
 */
fn enter_selectionStatement(&mut self, _ctx: &SelectionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#selectionStatement}.
 * @param ctx the parse tree
 */
fn exit_selectionStatement(&mut self, _ctx: &SelectionStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#condition}.
 * @param ctx the parse tree
 */
fn enter_condition(&mut self, _ctx: &ConditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#condition}.
 * @param ctx the parse tree
 */
fn exit_condition(&mut self, _ctx: &ConditionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#iterationStatement}.
 * @param ctx the parse tree
 */
fn enter_iterationStatement(&mut self, _ctx: &IterationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#iterationStatement}.
 * @param ctx the parse tree
 */
fn exit_iterationStatement(&mut self, _ctx: &IterationStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#forInitStatement}.
 * @param ctx the parse tree
 */
fn enter_forInitStatement(&mut self, _ctx: &ForInitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#forInitStatement}.
 * @param ctx the parse tree
 */
fn exit_forInitStatement(&mut self, _ctx: &ForInitStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#forRangeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_forRangeDeclaration(&mut self, _ctx: &ForRangeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#forRangeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_forRangeDeclaration(&mut self, _ctx: &ForRangeDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#forRangeInitializer}.
 * @param ctx the parse tree
 */
fn enter_forRangeInitializer(&mut self, _ctx: &ForRangeInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#forRangeInitializer}.
 * @param ctx the parse tree
 */
fn exit_forRangeInitializer(&mut self, _ctx: &ForRangeInitializerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#jumpStatement}.
 * @param ctx the parse tree
 */
fn enter_jumpStatement(&mut self, _ctx: &JumpStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#jumpStatement}.
 * @param ctx the parse tree
 */
fn exit_jumpStatement(&mut self, _ctx: &JumpStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declarationStatement}.
 * @param ctx the parse tree
 */
fn enter_declarationStatement(&mut self, _ctx: &DeclarationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declarationStatement}.
 * @param ctx the parse tree
 */
fn exit_declarationStatement(&mut self, _ctx: &DeclarationStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declarationseq}.
 * @param ctx the parse tree
 */
fn enter_declarationseq(&mut self, _ctx: &DeclarationseqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declarationseq}.
 * @param ctx the parse tree
 */
fn exit_declarationseq(&mut self, _ctx: &DeclarationseqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declaration}.
 * @param ctx the parse tree
 */
fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declaration}.
 * @param ctx the parse tree
 */
fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#blockDeclaration}.
 * @param ctx the parse tree
 */
fn enter_blockDeclaration(&mut self, _ctx: &BlockDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#blockDeclaration}.
 * @param ctx the parse tree
 */
fn exit_blockDeclaration(&mut self, _ctx: &BlockDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#aliasDeclaration}.
 * @param ctx the parse tree
 */
fn enter_aliasDeclaration(&mut self, _ctx: &AliasDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#aliasDeclaration}.
 * @param ctx the parse tree
 */
fn exit_aliasDeclaration(&mut self, _ctx: &AliasDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#simpleDeclaration}.
 * @param ctx the parse tree
 */
fn enter_simpleDeclaration(&mut self, _ctx: &SimpleDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#simpleDeclaration}.
 * @param ctx the parse tree
 */
fn exit_simpleDeclaration(&mut self, _ctx: &SimpleDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#staticAssertDeclaration}.
 * @param ctx the parse tree
 */
fn enter_staticAssertDeclaration(&mut self, _ctx: &StaticAssertDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#staticAssertDeclaration}.
 * @param ctx the parse tree
 */
fn exit_staticAssertDeclaration(&mut self, _ctx: &StaticAssertDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#emptyDeclaration_}.
 * @param ctx the parse tree
 */
fn enter_emptyDeclaration_(&mut self, _ctx: &EmptyDeclaration_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#emptyDeclaration_}.
 * @param ctx the parse tree
 */
fn exit_emptyDeclaration_(&mut self, _ctx: &EmptyDeclaration_Context<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attributeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_attributeDeclaration(&mut self, _ctx: &AttributeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attributeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_attributeDeclaration(&mut self, _ctx: &AttributeDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declSpecifier}.
 * @param ctx the parse tree
 */
fn enter_declSpecifier(&mut self, _ctx: &DeclSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declSpecifier}.
 * @param ctx the parse tree
 */
fn exit_declSpecifier(&mut self, _ctx: &DeclSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declSpecifierSeq}.
 * @param ctx the parse tree
 */
fn enter_declSpecifierSeq(&mut self, _ctx: &DeclSpecifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declSpecifierSeq}.
 * @param ctx the parse tree
 */
fn exit_declSpecifierSeq(&mut self, _ctx: &DeclSpecifierSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#storageClassSpecifier}.
 * @param ctx the parse tree
 */
fn enter_storageClassSpecifier(&mut self, _ctx: &StorageClassSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#storageClassSpecifier}.
 * @param ctx the parse tree
 */
fn exit_storageClassSpecifier(&mut self, _ctx: &StorageClassSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#functionSpecifier}.
 * @param ctx the parse tree
 */
fn enter_functionSpecifier(&mut self, _ctx: &FunctionSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#functionSpecifier}.
 * @param ctx the parse tree
 */
fn exit_functionSpecifier(&mut self, _ctx: &FunctionSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typedefName}.
 * @param ctx the parse tree
 */
fn enter_typedefName(&mut self, _ctx: &TypedefNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typedefName}.
 * @param ctx the parse tree
 */
fn exit_typedefName(&mut self, _ctx: &TypedefNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_typeSpecifier(&mut self, _ctx: &TypeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_typeSpecifier(&mut self, _ctx: &TypeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#trailingTypeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_trailingTypeSpecifier(&mut self, _ctx: &TrailingTypeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#trailingTypeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_trailingTypeSpecifier(&mut self, _ctx: &TrailingTypeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typeSpecifierSeq}.
 * @param ctx the parse tree
 */
fn enter_typeSpecifierSeq(&mut self, _ctx: &TypeSpecifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typeSpecifierSeq}.
 * @param ctx the parse tree
 */
fn exit_typeSpecifierSeq(&mut self, _ctx: &TypeSpecifierSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#trailingTypeSpecifierSeq}.
 * @param ctx the parse tree
 */
fn enter_trailingTypeSpecifierSeq(&mut self, _ctx: &TrailingTypeSpecifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#trailingTypeSpecifierSeq}.
 * @param ctx the parse tree
 */
fn exit_trailingTypeSpecifierSeq(&mut self, _ctx: &TrailingTypeSpecifierSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#simpleTypeLengthModifier}.
 * @param ctx the parse tree
 */
fn enter_simpleTypeLengthModifier(&mut self, _ctx: &SimpleTypeLengthModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#simpleTypeLengthModifier}.
 * @param ctx the parse tree
 */
fn exit_simpleTypeLengthModifier(&mut self, _ctx: &SimpleTypeLengthModifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#simpleTypeSignednessModifier}.
 * @param ctx the parse tree
 */
fn enter_simpleTypeSignednessModifier(&mut self, _ctx: &SimpleTypeSignednessModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#simpleTypeSignednessModifier}.
 * @param ctx the parse tree
 */
fn exit_simpleTypeSignednessModifier(&mut self, _ctx: &SimpleTypeSignednessModifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#simpleTypeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_simpleTypeSpecifier(&mut self, _ctx: &SimpleTypeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#simpleTypeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_simpleTypeSpecifier(&mut self, _ctx: &SimpleTypeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#theTypeName}.
 * @param ctx the parse tree
 */
fn enter_theTypeName(&mut self, _ctx: &TheTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#theTypeName}.
 * @param ctx the parse tree
 */
fn exit_theTypeName(&mut self, _ctx: &TheTypeNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#decltypeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_decltypeSpecifier(&mut self, _ctx: &DecltypeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#decltypeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_decltypeSpecifier(&mut self, _ctx: &DecltypeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#elaboratedTypeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_elaboratedTypeSpecifier(&mut self, _ctx: &ElaboratedTypeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#elaboratedTypeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_elaboratedTypeSpecifier(&mut self, _ctx: &ElaboratedTypeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumName}.
 * @param ctx the parse tree
 */
fn enter_enumName(&mut self, _ctx: &EnumNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumName}.
 * @param ctx the parse tree
 */
fn exit_enumName(&mut self, _ctx: &EnumNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumSpecifier}.
 * @param ctx the parse tree
 */
fn enter_enumSpecifier(&mut self, _ctx: &EnumSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumSpecifier}.
 * @param ctx the parse tree
 */
fn exit_enumSpecifier(&mut self, _ctx: &EnumSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumHead}.
 * @param ctx the parse tree
 */
fn enter_enumHead(&mut self, _ctx: &EnumHeadContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumHead}.
 * @param ctx the parse tree
 */
fn exit_enumHead(&mut self, _ctx: &EnumHeadContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#opaqueEnumDeclaration}.
 * @param ctx the parse tree
 */
fn enter_opaqueEnumDeclaration(&mut self, _ctx: &OpaqueEnumDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#opaqueEnumDeclaration}.
 * @param ctx the parse tree
 */
fn exit_opaqueEnumDeclaration(&mut self, _ctx: &OpaqueEnumDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumkey}.
 * @param ctx the parse tree
 */
fn enter_enumkey(&mut self, _ctx: &EnumkeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumkey}.
 * @param ctx the parse tree
 */
fn exit_enumkey(&mut self, _ctx: &EnumkeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumbase}.
 * @param ctx the parse tree
 */
fn enter_enumbase(&mut self, _ctx: &EnumbaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumbase}.
 * @param ctx the parse tree
 */
fn exit_enumbase(&mut self, _ctx: &EnumbaseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumeratorList}.
 * @param ctx the parse tree
 */
fn enter_enumeratorList(&mut self, _ctx: &EnumeratorListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumeratorList}.
 * @param ctx the parse tree
 */
fn exit_enumeratorList(&mut self, _ctx: &EnumeratorListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumeratorDefinition}.
 * @param ctx the parse tree
 */
fn enter_enumeratorDefinition(&mut self, _ctx: &EnumeratorDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumeratorDefinition}.
 * @param ctx the parse tree
 */
fn exit_enumeratorDefinition(&mut self, _ctx: &EnumeratorDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#enumerator}.
 * @param ctx the parse tree
 */
fn enter_enumerator(&mut self, _ctx: &EnumeratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#enumerator}.
 * @param ctx the parse tree
 */
fn exit_enumerator(&mut self, _ctx: &EnumeratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#namespaceName}.
 * @param ctx the parse tree
 */
fn enter_namespaceName(&mut self, _ctx: &NamespaceNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#namespaceName}.
 * @param ctx the parse tree
 */
fn exit_namespaceName(&mut self, _ctx: &NamespaceNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#originalNamespaceName}.
 * @param ctx the parse tree
 */
fn enter_originalNamespaceName(&mut self, _ctx: &OriginalNamespaceNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#originalNamespaceName}.
 * @param ctx the parse tree
 */
fn exit_originalNamespaceName(&mut self, _ctx: &OriginalNamespaceNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#namespaceDefinition}.
 * @param ctx the parse tree
 */
fn enter_namespaceDefinition(&mut self, _ctx: &NamespaceDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#namespaceDefinition}.
 * @param ctx the parse tree
 */
fn exit_namespaceDefinition(&mut self, _ctx: &NamespaceDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#namespaceAlias}.
 * @param ctx the parse tree
 */
fn enter_namespaceAlias(&mut self, _ctx: &NamespaceAliasContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#namespaceAlias}.
 * @param ctx the parse tree
 */
fn exit_namespaceAlias(&mut self, _ctx: &NamespaceAliasContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#namespaceAliasDefinition}.
 * @param ctx the parse tree
 */
fn enter_namespaceAliasDefinition(&mut self, _ctx: &NamespaceAliasDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#namespaceAliasDefinition}.
 * @param ctx the parse tree
 */
fn exit_namespaceAliasDefinition(&mut self, _ctx: &NamespaceAliasDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#qualifiednamespacespecifier}.
 * @param ctx the parse tree
 */
fn enter_qualifiednamespacespecifier(&mut self, _ctx: &QualifiednamespacespecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#qualifiednamespacespecifier}.
 * @param ctx the parse tree
 */
fn exit_qualifiednamespacespecifier(&mut self, _ctx: &QualifiednamespacespecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#usingDeclaration}.
 * @param ctx the parse tree
 */
fn enter_usingDeclaration(&mut self, _ctx: &UsingDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#usingDeclaration}.
 * @param ctx the parse tree
 */
fn exit_usingDeclaration(&mut self, _ctx: &UsingDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#usingDirective}.
 * @param ctx the parse tree
 */
fn enter_usingDirective(&mut self, _ctx: &UsingDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#usingDirective}.
 * @param ctx the parse tree
 */
fn exit_usingDirective(&mut self, _ctx: &UsingDirectiveContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#asmDefinition}.
 * @param ctx the parse tree
 */
fn enter_asmDefinition(&mut self, _ctx: &AsmDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#asmDefinition}.
 * @param ctx the parse tree
 */
fn exit_asmDefinition(&mut self, _ctx: &AsmDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#linkageSpecification}.
 * @param ctx the parse tree
 */
fn enter_linkageSpecification(&mut self, _ctx: &LinkageSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#linkageSpecification}.
 * @param ctx the parse tree
 */
fn exit_linkageSpecification(&mut self, _ctx: &LinkageSpecificationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attributeSpecifierSeq}.
 * @param ctx the parse tree
 */
fn enter_attributeSpecifierSeq(&mut self, _ctx: &AttributeSpecifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attributeSpecifierSeq}.
 * @param ctx the parse tree
 */
fn exit_attributeSpecifierSeq(&mut self, _ctx: &AttributeSpecifierSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attributeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_attributeSpecifier(&mut self, _ctx: &AttributeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attributeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_attributeSpecifier(&mut self, _ctx: &AttributeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#alignmentspecifier}.
 * @param ctx the parse tree
 */
fn enter_alignmentspecifier(&mut self, _ctx: &AlignmentspecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#alignmentspecifier}.
 * @param ctx the parse tree
 */
fn exit_alignmentspecifier(&mut self, _ctx: &AlignmentspecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attributeList}.
 * @param ctx the parse tree
 */
fn enter_attributeList(&mut self, _ctx: &AttributeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attributeList}.
 * @param ctx the parse tree
 */
fn exit_attributeList(&mut self, _ctx: &AttributeListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attribute}.
 * @param ctx the parse tree
 */
fn enter_attribute(&mut self, _ctx: &AttributeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attribute}.
 * @param ctx the parse tree
 */
fn exit_attribute(&mut self, _ctx: &AttributeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attributeNamespace}.
 * @param ctx the parse tree
 */
fn enter_attributeNamespace(&mut self, _ctx: &AttributeNamespaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attributeNamespace}.
 * @param ctx the parse tree
 */
fn exit_attributeNamespace(&mut self, _ctx: &AttributeNamespaceContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#attributeArgumentClause}.
 * @param ctx the parse tree
 */
fn enter_attributeArgumentClause(&mut self, _ctx: &AttributeArgumentClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#attributeArgumentClause}.
 * @param ctx the parse tree
 */
fn exit_attributeArgumentClause(&mut self, _ctx: &AttributeArgumentClauseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#balancedTokenSeq}.
 * @param ctx the parse tree
 */
fn enter_balancedTokenSeq(&mut self, _ctx: &BalancedTokenSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#balancedTokenSeq}.
 * @param ctx the parse tree
 */
fn exit_balancedTokenSeq(&mut self, _ctx: &BalancedTokenSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#balancedtoken}.
 * @param ctx the parse tree
 */
fn enter_balancedtoken(&mut self, _ctx: &BalancedtokenContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#balancedtoken}.
 * @param ctx the parse tree
 */
fn exit_balancedtoken(&mut self, _ctx: &BalancedtokenContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#initDeclaratorList}.
 * @param ctx the parse tree
 */
fn enter_initDeclaratorList(&mut self, _ctx: &InitDeclaratorListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#initDeclaratorList}.
 * @param ctx the parse tree
 */
fn exit_initDeclaratorList(&mut self, _ctx: &InitDeclaratorListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#initDeclarator}.
 * @param ctx the parse tree
 */
fn enter_initDeclarator(&mut self, _ctx: &InitDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#initDeclarator}.
 * @param ctx the parse tree
 */
fn exit_initDeclarator(&mut self, _ctx: &InitDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declarator}.
 * @param ctx the parse tree
 */
fn enter_declarator(&mut self, _ctx: &DeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declarator}.
 * @param ctx the parse tree
 */
fn exit_declarator(&mut self, _ctx: &DeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#pointerDeclarator}.
 * @param ctx the parse tree
 */
fn enter_pointerDeclarator(&mut self, _ctx: &PointerDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#pointerDeclarator}.
 * @param ctx the parse tree
 */
fn exit_pointerDeclarator(&mut self, _ctx: &PointerDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#noPointerDeclarator}.
 * @param ctx the parse tree
 */
fn enter_noPointerDeclarator(&mut self, _ctx: &NoPointerDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#noPointerDeclarator}.
 * @param ctx the parse tree
 */
fn exit_noPointerDeclarator(&mut self, _ctx: &NoPointerDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#parametersAndQualifiers}.
 * @param ctx the parse tree
 */
fn enter_parametersAndQualifiers(&mut self, _ctx: &ParametersAndQualifiersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#parametersAndQualifiers}.
 * @param ctx the parse tree
 */
fn exit_parametersAndQualifiers(&mut self, _ctx: &ParametersAndQualifiersContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#trailingReturnType}.
 * @param ctx the parse tree
 */
fn enter_trailingReturnType(&mut self, _ctx: &TrailingReturnTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#trailingReturnType}.
 * @param ctx the parse tree
 */
fn exit_trailingReturnType(&mut self, _ctx: &TrailingReturnTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#pointerOperator}.
 * @param ctx the parse tree
 */
fn enter_pointerOperator(&mut self, _ctx: &PointerOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#pointerOperator}.
 * @param ctx the parse tree
 */
fn exit_pointerOperator(&mut self, _ctx: &PointerOperatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#cvqualifierseq}.
 * @param ctx the parse tree
 */
fn enter_cvqualifierseq(&mut self, _ctx: &CvqualifierseqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#cvqualifierseq}.
 * @param ctx the parse tree
 */
fn exit_cvqualifierseq(&mut self, _ctx: &CvqualifierseqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#cvQualifier}.
 * @param ctx the parse tree
 */
fn enter_cvQualifier(&mut self, _ctx: &CvQualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#cvQualifier}.
 * @param ctx the parse tree
 */
fn exit_cvQualifier(&mut self, _ctx: &CvQualifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#refqualifier}.
 * @param ctx the parse tree
 */
fn enter_refqualifier(&mut self, _ctx: &RefqualifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#refqualifier}.
 * @param ctx the parse tree
 */
fn exit_refqualifier(&mut self, _ctx: &RefqualifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#declaratorid}.
 * @param ctx the parse tree
 */
fn enter_declaratorid(&mut self, _ctx: &DeclaratoridContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#declaratorid}.
 * @param ctx the parse tree
 */
fn exit_declaratorid(&mut self, _ctx: &DeclaratoridContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#theTypeId}.
 * @param ctx the parse tree
 */
fn enter_theTypeId(&mut self, _ctx: &TheTypeIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#theTypeId}.
 * @param ctx the parse tree
 */
fn exit_theTypeId(&mut self, _ctx: &TheTypeIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#abstractDeclarator}.
 * @param ctx the parse tree
 */
fn enter_abstractDeclarator(&mut self, _ctx: &AbstractDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#abstractDeclarator}.
 * @param ctx the parse tree
 */
fn exit_abstractDeclarator(&mut self, _ctx: &AbstractDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#pointerAbstractDeclarator}.
 * @param ctx the parse tree
 */
fn enter_pointerAbstractDeclarator(&mut self, _ctx: &PointerAbstractDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#pointerAbstractDeclarator}.
 * @param ctx the parse tree
 */
fn exit_pointerAbstractDeclarator(&mut self, _ctx: &PointerAbstractDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#noPointerAbstractDeclarator}.
 * @param ctx the parse tree
 */
fn enter_noPointerAbstractDeclarator(&mut self, _ctx: &NoPointerAbstractDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#noPointerAbstractDeclarator}.
 * @param ctx the parse tree
 */
fn exit_noPointerAbstractDeclarator(&mut self, _ctx: &NoPointerAbstractDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#abstractPackDeclarator}.
 * @param ctx the parse tree
 */
fn enter_abstractPackDeclarator(&mut self, _ctx: &AbstractPackDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#abstractPackDeclarator}.
 * @param ctx the parse tree
 */
fn exit_abstractPackDeclarator(&mut self, _ctx: &AbstractPackDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#noPointerAbstractPackDeclarator}.
 * @param ctx the parse tree
 */
fn enter_noPointerAbstractPackDeclarator(&mut self, _ctx: &NoPointerAbstractPackDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#noPointerAbstractPackDeclarator}.
 * @param ctx the parse tree
 */
fn exit_noPointerAbstractPackDeclarator(&mut self, _ctx: &NoPointerAbstractPackDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#parameterDeclarationClause}.
 * @param ctx the parse tree
 */
fn enter_parameterDeclarationClause(&mut self, _ctx: &ParameterDeclarationClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#parameterDeclarationClause}.
 * @param ctx the parse tree
 */
fn exit_parameterDeclarationClause(&mut self, _ctx: &ParameterDeclarationClauseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#parameterDeclarationList}.
 * @param ctx the parse tree
 */
fn enter_parameterDeclarationList(&mut self, _ctx: &ParameterDeclarationListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#parameterDeclarationList}.
 * @param ctx the parse tree
 */
fn exit_parameterDeclarationList(&mut self, _ctx: &ParameterDeclarationListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#parameterDeclaration}.
 * @param ctx the parse tree
 */
fn enter_parameterDeclaration(&mut self, _ctx: &ParameterDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#parameterDeclaration}.
 * @param ctx the parse tree
 */
fn exit_parameterDeclaration(&mut self, _ctx: &ParameterDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#functionDefinition}.
 * @param ctx the parse tree
 */
fn enter_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#functionDefinition}.
 * @param ctx the parse tree
 */
fn exit_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#functionBody}.
 * @param ctx the parse tree
 */
fn enter_functionBody(&mut self, _ctx: &FunctionBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#functionBody}.
 * @param ctx the parse tree
 */
fn exit_functionBody(&mut self, _ctx: &FunctionBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#initializer}.
 * @param ctx the parse tree
 */
fn enter_initializer(&mut self, _ctx: &InitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#initializer}.
 * @param ctx the parse tree
 */
fn exit_initializer(&mut self, _ctx: &InitializerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#braceOrEqualInitializer}.
 * @param ctx the parse tree
 */
fn enter_braceOrEqualInitializer(&mut self, _ctx: &BraceOrEqualInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#braceOrEqualInitializer}.
 * @param ctx the parse tree
 */
fn exit_braceOrEqualInitializer(&mut self, _ctx: &BraceOrEqualInitializerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#initializerClause}.
 * @param ctx the parse tree
 */
fn enter_initializerClause(&mut self, _ctx: &InitializerClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#initializerClause}.
 * @param ctx the parse tree
 */
fn exit_initializerClause(&mut self, _ctx: &InitializerClauseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#initializerList}.
 * @param ctx the parse tree
 */
fn enter_initializerList(&mut self, _ctx: &InitializerListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#initializerList}.
 * @param ctx the parse tree
 */
fn exit_initializerList(&mut self, _ctx: &InitializerListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#bracedInitList}.
 * @param ctx the parse tree
 */
fn enter_bracedInitList(&mut self, _ctx: &BracedInitListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#bracedInitList}.
 * @param ctx the parse tree
 */
fn exit_bracedInitList(&mut self, _ctx: &BracedInitListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#className}.
 * @param ctx the parse tree
 */
fn enter_className(&mut self, _ctx: &ClassNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#className}.
 * @param ctx the parse tree
 */
fn exit_className(&mut self, _ctx: &ClassNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#classSpecifier}.
 * @param ctx the parse tree
 */
fn enter_classSpecifier(&mut self, _ctx: &ClassSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#classSpecifier}.
 * @param ctx the parse tree
 */
fn exit_classSpecifier(&mut self, _ctx: &ClassSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#classHead}.
 * @param ctx the parse tree
 */
fn enter_classHead(&mut self, _ctx: &ClassHeadContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#classHead}.
 * @param ctx the parse tree
 */
fn exit_classHead(&mut self, _ctx: &ClassHeadContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#classHeadName}.
 * @param ctx the parse tree
 */
fn enter_classHeadName(&mut self, _ctx: &ClassHeadNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#classHeadName}.
 * @param ctx the parse tree
 */
fn exit_classHeadName(&mut self, _ctx: &ClassHeadNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#classVirtSpecifier}.
 * @param ctx the parse tree
 */
fn enter_classVirtSpecifier(&mut self, _ctx: &ClassVirtSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#classVirtSpecifier}.
 * @param ctx the parse tree
 */
fn exit_classVirtSpecifier(&mut self, _ctx: &ClassVirtSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#classKey}.
 * @param ctx the parse tree
 */
fn enter_classKey(&mut self, _ctx: &ClassKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#classKey}.
 * @param ctx the parse tree
 */
fn exit_classKey(&mut self, _ctx: &ClassKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#memberSpecification}.
 * @param ctx the parse tree
 */
fn enter_memberSpecification(&mut self, _ctx: &MemberSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#memberSpecification}.
 * @param ctx the parse tree
 */
fn exit_memberSpecification(&mut self, _ctx: &MemberSpecificationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#memberdeclaration}.
 * @param ctx the parse tree
 */
fn enter_memberdeclaration(&mut self, _ctx: &MemberdeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#memberdeclaration}.
 * @param ctx the parse tree
 */
fn exit_memberdeclaration(&mut self, _ctx: &MemberdeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#memberDeclaratorList}.
 * @param ctx the parse tree
 */
fn enter_memberDeclaratorList(&mut self, _ctx: &MemberDeclaratorListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#memberDeclaratorList}.
 * @param ctx the parse tree
 */
fn exit_memberDeclaratorList(&mut self, _ctx: &MemberDeclaratorListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#memberDeclarator}.
 * @param ctx the parse tree
 */
fn enter_memberDeclarator(&mut self, _ctx: &MemberDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#memberDeclarator}.
 * @param ctx the parse tree
 */
fn exit_memberDeclarator(&mut self, _ctx: &MemberDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#virtualSpecifierSeq}.
 * @param ctx the parse tree
 */
fn enter_virtualSpecifierSeq(&mut self, _ctx: &VirtualSpecifierSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#virtualSpecifierSeq}.
 * @param ctx the parse tree
 */
fn exit_virtualSpecifierSeq(&mut self, _ctx: &VirtualSpecifierSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#virtualSpecifier}.
 * @param ctx the parse tree
 */
fn enter_virtualSpecifier(&mut self, _ctx: &VirtualSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#virtualSpecifier}.
 * @param ctx the parse tree
 */
fn exit_virtualSpecifier(&mut self, _ctx: &VirtualSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#pureSpecifier}.
 * @param ctx the parse tree
 */
fn enter_pureSpecifier(&mut self, _ctx: &PureSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#pureSpecifier}.
 * @param ctx the parse tree
 */
fn exit_pureSpecifier(&mut self, _ctx: &PureSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#baseClause}.
 * @param ctx the parse tree
 */
fn enter_baseClause(&mut self, _ctx: &BaseClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#baseClause}.
 * @param ctx the parse tree
 */
fn exit_baseClause(&mut self, _ctx: &BaseClauseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#baseSpecifierList}.
 * @param ctx the parse tree
 */
fn enter_baseSpecifierList(&mut self, _ctx: &BaseSpecifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#baseSpecifierList}.
 * @param ctx the parse tree
 */
fn exit_baseSpecifierList(&mut self, _ctx: &BaseSpecifierListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#baseSpecifier}.
 * @param ctx the parse tree
 */
fn enter_baseSpecifier(&mut self, _ctx: &BaseSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#baseSpecifier}.
 * @param ctx the parse tree
 */
fn exit_baseSpecifier(&mut self, _ctx: &BaseSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#classOrDeclType}.
 * @param ctx the parse tree
 */
fn enter_classOrDeclType(&mut self, _ctx: &ClassOrDeclTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#classOrDeclType}.
 * @param ctx the parse tree
 */
fn exit_classOrDeclType(&mut self, _ctx: &ClassOrDeclTypeContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#baseTypeSpecifier}.
 * @param ctx the parse tree
 */
fn enter_baseTypeSpecifier(&mut self, _ctx: &BaseTypeSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#baseTypeSpecifier}.
 * @param ctx the parse tree
 */
fn exit_baseTypeSpecifier(&mut self, _ctx: &BaseTypeSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#accessSpecifier}.
 * @param ctx the parse tree
 */
fn enter_accessSpecifier(&mut self, _ctx: &AccessSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#accessSpecifier}.
 * @param ctx the parse tree
 */
fn exit_accessSpecifier(&mut self, _ctx: &AccessSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#conversionFunctionId}.
 * @param ctx the parse tree
 */
fn enter_conversionFunctionId(&mut self, _ctx: &ConversionFunctionIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#conversionFunctionId}.
 * @param ctx the parse tree
 */
fn exit_conversionFunctionId(&mut self, _ctx: &ConversionFunctionIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#conversionTypeId}.
 * @param ctx the parse tree
 */
fn enter_conversionTypeId(&mut self, _ctx: &ConversionTypeIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#conversionTypeId}.
 * @param ctx the parse tree
 */
fn exit_conversionTypeId(&mut self, _ctx: &ConversionTypeIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#conversionDeclarator}.
 * @param ctx the parse tree
 */
fn enter_conversionDeclarator(&mut self, _ctx: &ConversionDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#conversionDeclarator}.
 * @param ctx the parse tree
 */
fn exit_conversionDeclarator(&mut self, _ctx: &ConversionDeclaratorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#constructorInitializer}.
 * @param ctx the parse tree
 */
fn enter_constructorInitializer(&mut self, _ctx: &ConstructorInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#constructorInitializer}.
 * @param ctx the parse tree
 */
fn exit_constructorInitializer(&mut self, _ctx: &ConstructorInitializerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#memInitializerList}.
 * @param ctx the parse tree
 */
fn enter_memInitializerList(&mut self, _ctx: &MemInitializerListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#memInitializerList}.
 * @param ctx the parse tree
 */
fn exit_memInitializerList(&mut self, _ctx: &MemInitializerListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#memInitializer}.
 * @param ctx the parse tree
 */
fn enter_memInitializer(&mut self, _ctx: &MemInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#memInitializer}.
 * @param ctx the parse tree
 */
fn exit_memInitializer(&mut self, _ctx: &MemInitializerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#meminitializerid}.
 * @param ctx the parse tree
 */
fn enter_meminitializerid(&mut self, _ctx: &MeminitializeridContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#meminitializerid}.
 * @param ctx the parse tree
 */
fn exit_meminitializerid(&mut self, _ctx: &MeminitializeridContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#operatorFunctionId}.
 * @param ctx the parse tree
 */
fn enter_operatorFunctionId(&mut self, _ctx: &OperatorFunctionIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#operatorFunctionId}.
 * @param ctx the parse tree
 */
fn exit_operatorFunctionId(&mut self, _ctx: &OperatorFunctionIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#literalOperatorId}.
 * @param ctx the parse tree
 */
fn enter_literalOperatorId(&mut self, _ctx: &LiteralOperatorIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#literalOperatorId}.
 * @param ctx the parse tree
 */
fn exit_literalOperatorId(&mut self, _ctx: &LiteralOperatorIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateDeclaration}.
 * @param ctx the parse tree
 */
fn enter_templateDeclaration(&mut self, _ctx: &TemplateDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateDeclaration}.
 * @param ctx the parse tree
 */
fn exit_templateDeclaration(&mut self, _ctx: &TemplateDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateparameterList}.
 * @param ctx the parse tree
 */
fn enter_templateparameterList(&mut self, _ctx: &TemplateparameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateparameterList}.
 * @param ctx the parse tree
 */
fn exit_templateparameterList(&mut self, _ctx: &TemplateparameterListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateParameter}.
 * @param ctx the parse tree
 */
fn enter_templateParameter(&mut self, _ctx: &TemplateParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateParameter}.
 * @param ctx the parse tree
 */
fn exit_templateParameter(&mut self, _ctx: &TemplateParameterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typeParameter}.
 * @param ctx the parse tree
 */
fn enter_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typeParameter}.
 * @param ctx the parse tree
 */
fn exit_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#simpleTemplateId}.
 * @param ctx the parse tree
 */
fn enter_simpleTemplateId(&mut self, _ctx: &SimpleTemplateIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#simpleTemplateId}.
 * @param ctx the parse tree
 */
fn exit_simpleTemplateId(&mut self, _ctx: &SimpleTemplateIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateId}.
 * @param ctx the parse tree
 */
fn enter_templateId(&mut self, _ctx: &TemplateIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateId}.
 * @param ctx the parse tree
 */
fn exit_templateId(&mut self, _ctx: &TemplateIdContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateName}.
 * @param ctx the parse tree
 */
fn enter_templateName(&mut self, _ctx: &TemplateNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateName}.
 * @param ctx the parse tree
 */
fn exit_templateName(&mut self, _ctx: &TemplateNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateArgumentList}.
 * @param ctx the parse tree
 */
fn enter_templateArgumentList(&mut self, _ctx: &TemplateArgumentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateArgumentList}.
 * @param ctx the parse tree
 */
fn exit_templateArgumentList(&mut self, _ctx: &TemplateArgumentListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#templateArgument}.
 * @param ctx the parse tree
 */
fn enter_templateArgument(&mut self, _ctx: &TemplateArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#templateArgument}.
 * @param ctx the parse tree
 */
fn exit_templateArgument(&mut self, _ctx: &TemplateArgumentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typeNameSpecifier}.
 * @param ctx the parse tree
 */
fn enter_typeNameSpecifier(&mut self, _ctx: &TypeNameSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typeNameSpecifier}.
 * @param ctx the parse tree
 */
fn exit_typeNameSpecifier(&mut self, _ctx: &TypeNameSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#explicitInstantiation}.
 * @param ctx the parse tree
 */
fn enter_explicitInstantiation(&mut self, _ctx: &ExplicitInstantiationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#explicitInstantiation}.
 * @param ctx the parse tree
 */
fn exit_explicitInstantiation(&mut self, _ctx: &ExplicitInstantiationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#explicitSpecialization}.
 * @param ctx the parse tree
 */
fn enter_explicitSpecialization(&mut self, _ctx: &ExplicitSpecializationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#explicitSpecialization}.
 * @param ctx the parse tree
 */
fn exit_explicitSpecialization(&mut self, _ctx: &ExplicitSpecializationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#tryBlock}.
 * @param ctx the parse tree
 */
fn enter_tryBlock(&mut self, _ctx: &TryBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#tryBlock}.
 * @param ctx the parse tree
 */
fn exit_tryBlock(&mut self, _ctx: &TryBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#functionTryBlock}.
 * @param ctx the parse tree
 */
fn enter_functionTryBlock(&mut self, _ctx: &FunctionTryBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#functionTryBlock}.
 * @param ctx the parse tree
 */
fn exit_functionTryBlock(&mut self, _ctx: &FunctionTryBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#handlerSeq}.
 * @param ctx the parse tree
 */
fn enter_handlerSeq(&mut self, _ctx: &HandlerSeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#handlerSeq}.
 * @param ctx the parse tree
 */
fn exit_handlerSeq(&mut self, _ctx: &HandlerSeqContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#handler}.
 * @param ctx the parse tree
 */
fn enter_handler(&mut self, _ctx: &HandlerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#handler}.
 * @param ctx the parse tree
 */
fn exit_handler(&mut self, _ctx: &HandlerContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#exceptionDeclaration}.
 * @param ctx the parse tree
 */
fn enter_exceptionDeclaration(&mut self, _ctx: &ExceptionDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#exceptionDeclaration}.
 * @param ctx the parse tree
 */
fn exit_exceptionDeclaration(&mut self, _ctx: &ExceptionDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#throwExpression}.
 * @param ctx the parse tree
 */
fn enter_throwExpression(&mut self, _ctx: &ThrowExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#throwExpression}.
 * @param ctx the parse tree
 */
fn exit_throwExpression(&mut self, _ctx: &ThrowExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#exceptionSpecification}.
 * @param ctx the parse tree
 */
fn enter_exceptionSpecification(&mut self, _ctx: &ExceptionSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#exceptionSpecification}.
 * @param ctx the parse tree
 */
fn exit_exceptionSpecification(&mut self, _ctx: &ExceptionSpecificationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#dynamicExceptionSpecification}.
 * @param ctx the parse tree
 */
fn enter_dynamicExceptionSpecification(&mut self, _ctx: &DynamicExceptionSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#dynamicExceptionSpecification}.
 * @param ctx the parse tree
 */
fn exit_dynamicExceptionSpecification(&mut self, _ctx: &DynamicExceptionSpecificationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#typeIdList}.
 * @param ctx the parse tree
 */
fn enter_typeIdList(&mut self, _ctx: &TypeIdListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#typeIdList}.
 * @param ctx the parse tree
 */
fn exit_typeIdList(&mut self, _ctx: &TypeIdListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#noeExceptSpecification}.
 * @param ctx the parse tree
 */
fn enter_noeExceptSpecification(&mut self, _ctx: &NoeExceptSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#noeExceptSpecification}.
 * @param ctx the parse tree
 */
fn exit_noeExceptSpecification(&mut self, _ctx: &NoeExceptSpecificationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#theOperator}.
 * @param ctx the parse tree
 */
fn enter_theOperator(&mut self, _ctx: &TheOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#theOperator}.
 * @param ctx the parse tree
 */
fn exit_theOperator(&mut self, _ctx: &TheOperatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link CPP14Parser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link CPP14Parser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }

}
