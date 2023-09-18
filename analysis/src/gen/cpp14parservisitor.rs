#![allow(nonstandard_style)]
// Generated from CPP14Parser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::cpp14parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CPP14Parser}.
 */
pub trait CPP14ParserVisitor<'input>: ParseTreeVisitor<'input,CPP14ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link CPP14Parser#translationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#idExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_idExpression(&mut self, ctx: &IdExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#unqualifiedId}.
	 * @param ctx the parse tree
	 */
	fn visit_unqualifiedId(&mut self, ctx: &UnqualifiedIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#qualifiedId}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedId(&mut self, ctx: &QualifiedIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#nestedNameSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_nestedNameSpecifier(&mut self, ctx: &NestedNameSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaIntroducer}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaIntroducer(&mut self, ctx: &LambdaIntroducerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaCapture}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaCapture(&mut self, ctx: &LambdaCaptureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#captureDefault}.
	 * @param ctx the parse tree
	 */
	fn visit_captureDefault(&mut self, ctx: &CaptureDefaultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#captureList}.
	 * @param ctx the parse tree
	 */
	fn visit_captureList(&mut self, ctx: &CaptureListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#capture}.
	 * @param ctx the parse tree
	 */
	fn visit_capture(&mut self, ctx: &CaptureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleCapture}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleCapture(&mut self, ctx: &SimpleCaptureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initcapture}.
	 * @param ctx the parse tree
	 */
	fn visit_initcapture(&mut self, ctx: &InitcaptureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaDeclarator(&mut self, ctx: &LambdaDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeIdOfTheTypeId}.
	 * @param ctx the parse tree
	 */
	fn visit_typeIdOfTheTypeId(&mut self, ctx: &TypeIdOfTheTypeIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#expressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pseudoDestructorName}.
	 * @param ctx the parse tree
	 */
	fn visit_pseudoDestructorName(&mut self, ctx: &PseudoDestructorNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#unaryOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newExpression_}.
	 * @param ctx the parse tree
	 */
	fn visit_newExpression_(&mut self, ctx: &NewExpression_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newPlacement}.
	 * @param ctx the parse tree
	 */
	fn visit_newPlacement(&mut self, ctx: &NewPlacementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newTypeId}.
	 * @param ctx the parse tree
	 */
	fn visit_newTypeId(&mut self, ctx: &NewTypeIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newDeclarator_}.
	 * @param ctx the parse tree
	 */
	fn visit_newDeclarator_(&mut self, ctx: &NewDeclarator_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerNewDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_noPointerNewDeclarator(&mut self, ctx: &NoPointerNewDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newInitializer_}.
	 * @param ctx the parse tree
	 */
	fn visit_newInitializer_(&mut self, ctx: &NewInitializer_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#deleteExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_deleteExpression(&mut self, ctx: &DeleteExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noExceptExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_noExceptExpression(&mut self, ctx: &NoExceptExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#castExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerMemberExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_pointerMemberExpression(&mut self, ctx: &PointerMemberExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#shiftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#shiftOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#relationalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#andExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#exclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#inclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conditionalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#assignmentExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#assignmentOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#constantExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#labeledStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#expressionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#compoundStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#statementSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_statementSeq(&mut self, ctx: &StatementSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#selectionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#condition}.
	 * @param ctx the parse tree
	 */
	fn visit_condition(&mut self, ctx: &ConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#iterationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#forInitStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_forInitStatement(&mut self, ctx: &ForInitStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#forRangeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_forRangeDeclaration(&mut self, ctx: &ForRangeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#forRangeInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_forRangeInitializer(&mut self, ctx: &ForRangeInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#jumpStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationStatement(&mut self, ctx: &DeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declarationseq}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationseq(&mut self, ctx: &DeclarationseqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declaration}.
	 * @param ctx the parse tree
	 */
	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#blockDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_blockDeclaration(&mut self, ctx: &BlockDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#aliasDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_aliasDeclaration(&mut self, ctx: &AliasDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleDeclaration(&mut self, ctx: &SimpleDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#staticAssertDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_staticAssertDeclaration(&mut self, ctx: &StaticAssertDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#emptyDeclaration_}.
	 * @param ctx the parse tree
	 */
	fn visit_emptyDeclaration_(&mut self, ctx: &EmptyDeclaration_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeDeclaration(&mut self, ctx: &AttributeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_declSpecifier(&mut self, ctx: &DeclSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declSpecifierSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_declSpecifierSeq(&mut self, ctx: &DeclSpecifierSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#storageClassSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_storageClassSpecifier(&mut self, ctx: &StorageClassSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_functionSpecifier(&mut self, ctx: &FunctionSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typedefName}.
	 * @param ctx the parse tree
	 */
	fn visit_typedefName(&mut self, ctx: &TypedefNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#trailingTypeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_trailingTypeSpecifier(&mut self, ctx: &TrailingTypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeSpecifierSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_typeSpecifierSeq(&mut self, ctx: &TypeSpecifierSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#trailingTypeSpecifierSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_trailingTypeSpecifierSeq(&mut self, ctx: &TrailingTypeSpecifierSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTypeLengthModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleTypeLengthModifier(&mut self, ctx: &SimpleTypeLengthModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTypeSignednessModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleTypeSignednessModifier(&mut self, ctx: &SimpleTypeSignednessModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTypeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleTypeSpecifier(&mut self, ctx: &SimpleTypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#theTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_theTypeName(&mut self, ctx: &TheTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#decltypeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_decltypeSpecifier(&mut self, ctx: &DecltypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#elaboratedTypeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_elaboratedTypeSpecifier(&mut self, ctx: &ElaboratedTypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumName}.
	 * @param ctx the parse tree
	 */
	fn visit_enumName(&mut self, ctx: &EnumNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumHead}.
	 * @param ctx the parse tree
	 */
	fn visit_enumHead(&mut self, ctx: &EnumHeadContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#opaqueEnumDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_opaqueEnumDeclaration(&mut self, ctx: &OpaqueEnumDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumkey}.
	 * @param ctx the parse tree
	 */
	fn visit_enumkey(&mut self, ctx: &EnumkeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumbase}.
	 * @param ctx the parse tree
	 */
	fn visit_enumbase(&mut self, ctx: &EnumbaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumeratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumeratorDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_enumeratorDefinition(&mut self, ctx: &EnumeratorDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumerator}.
	 * @param ctx the parse tree
	 */
	fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceName}.
	 * @param ctx the parse tree
	 */
	fn visit_namespaceName(&mut self, ctx: &NamespaceNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#originalNamespaceName}.
	 * @param ctx the parse tree
	 */
	fn visit_originalNamespaceName(&mut self, ctx: &OriginalNamespaceNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_namespaceDefinition(&mut self, ctx: &NamespaceDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceAlias}.
	 * @param ctx the parse tree
	 */
	fn visit_namespaceAlias(&mut self, ctx: &NamespaceAliasContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceAliasDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_namespaceAliasDefinition(&mut self, ctx: &NamespaceAliasDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#qualifiednamespacespecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiednamespacespecifier(&mut self, ctx: &QualifiednamespacespecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#usingDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_usingDeclaration(&mut self, ctx: &UsingDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#usingDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_usingDirective(&mut self, ctx: &UsingDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#asmDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_asmDefinition(&mut self, ctx: &AsmDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#linkageSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_linkageSpecification(&mut self, ctx: &LinkageSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeSpecifierSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeSpecifierSeq(&mut self, ctx: &AttributeSpecifierSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeSpecifier(&mut self, ctx: &AttributeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#alignmentspecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_alignmentspecifier(&mut self, ctx: &AlignmentspecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeList}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeList(&mut self, ctx: &AttributeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attribute}.
	 * @param ctx the parse tree
	 */
	fn visit_attribute(&mut self, ctx: &AttributeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeNamespace}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeNamespace(&mut self, ctx: &AttributeNamespaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeArgumentClause}.
	 * @param ctx the parse tree
	 */
	fn visit_attributeArgumentClause(&mut self, ctx: &AttributeArgumentClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#balancedTokenSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_balancedTokenSeq(&mut self, ctx: &BalancedTokenSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#balancedtoken}.
	 * @param ctx the parse tree
	 */
	fn visit_balancedtoken(&mut self, ctx: &BalancedtokenContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initDeclaratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declarator}.
	 * @param ctx the parse tree
	 */
	fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_pointerDeclarator(&mut self, ctx: &PointerDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_noPointerDeclarator(&mut self, ctx: &NoPointerDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parametersAndQualifiers}.
	 * @param ctx the parse tree
	 */
	fn visit_parametersAndQualifiers(&mut self, ctx: &ParametersAndQualifiersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#trailingReturnType}.
	 * @param ctx the parse tree
	 */
	fn visit_trailingReturnType(&mut self, ctx: &TrailingReturnTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_pointerOperator(&mut self, ctx: &PointerOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#cvqualifierseq}.
	 * @param ctx the parse tree
	 */
	fn visit_cvqualifierseq(&mut self, ctx: &CvqualifierseqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#cvQualifier}.
	 * @param ctx the parse tree
	 */
	fn visit_cvQualifier(&mut self, ctx: &CvQualifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#refqualifier}.
	 * @param ctx the parse tree
	 */
	fn visit_refqualifier(&mut self, ctx: &RefqualifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declaratorid}.
	 * @param ctx the parse tree
	 */
	fn visit_declaratorid(&mut self, ctx: &DeclaratoridContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#theTypeId}.
	 * @param ctx the parse tree
	 */
	fn visit_theTypeId(&mut self, ctx: &TheTypeIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#abstractDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_abstractDeclarator(&mut self, ctx: &AbstractDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_pointerAbstractDeclarator(&mut self, ctx: &PointerAbstractDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_noPointerAbstractDeclarator(&mut self, ctx: &NoPointerAbstractDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#abstractPackDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_abstractPackDeclarator(&mut self, ctx: &AbstractPackDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerAbstractPackDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_noPointerAbstractPackDeclarator(&mut self, ctx: &NoPointerAbstractPackDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parameterDeclarationClause}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDeclarationClause(&mut self, ctx: &ParameterDeclarationClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parameterDeclarationList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDeclarationList(&mut self, ctx: &ParameterDeclarationListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parameterDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionBody}.
	 * @param ctx the parse tree
	 */
	fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initializer}.
	 * @param ctx the parse tree
	 */
	fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#braceOrEqualInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_braceOrEqualInitializer(&mut self, ctx: &BraceOrEqualInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initializerClause}.
	 * @param ctx the parse tree
	 */
	fn visit_initializerClause(&mut self, ctx: &InitializerClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initializerList}.
	 * @param ctx the parse tree
	 */
	fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#bracedInitList}.
	 * @param ctx the parse tree
	 */
	fn visit_bracedInitList(&mut self, ctx: &BracedInitListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#className}.
	 * @param ctx the parse tree
	 */
	fn visit_className(&mut self, ctx: &ClassNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_classSpecifier(&mut self, ctx: &ClassSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classHead}.
	 * @param ctx the parse tree
	 */
	fn visit_classHead(&mut self, ctx: &ClassHeadContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classHeadName}.
	 * @param ctx the parse tree
	 */
	fn visit_classHeadName(&mut self, ctx: &ClassHeadNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classVirtSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_classVirtSpecifier(&mut self, ctx: &ClassVirtSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classKey}.
	 * @param ctx the parse tree
	 */
	fn visit_classKey(&mut self, ctx: &ClassKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_memberSpecification(&mut self, ctx: &MemberSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberdeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_memberdeclaration(&mut self, ctx: &MemberdeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberDeclaratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_memberDeclaratorList(&mut self, ctx: &MemberDeclaratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_memberDeclarator(&mut self, ctx: &MemberDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#virtualSpecifierSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_virtualSpecifierSeq(&mut self, ctx: &VirtualSpecifierSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#virtualSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_virtualSpecifier(&mut self, ctx: &VirtualSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pureSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_pureSpecifier(&mut self, ctx: &PureSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseClause}.
	 * @param ctx the parse tree
	 */
	fn visit_baseClause(&mut self, ctx: &BaseClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseSpecifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_baseSpecifierList(&mut self, ctx: &BaseSpecifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_baseSpecifier(&mut self, ctx: &BaseSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classOrDeclType}.
	 * @param ctx the parse tree
	 */
	fn visit_classOrDeclType(&mut self, ctx: &ClassOrDeclTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseTypeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_baseTypeSpecifier(&mut self, ctx: &BaseTypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#accessSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_accessSpecifier(&mut self, ctx: &AccessSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conversionFunctionId}.
	 * @param ctx the parse tree
	 */
	fn visit_conversionFunctionId(&mut self, ctx: &ConversionFunctionIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conversionTypeId}.
	 * @param ctx the parse tree
	 */
	fn visit_conversionTypeId(&mut self, ctx: &ConversionTypeIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conversionDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_conversionDeclarator(&mut self, ctx: &ConversionDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#constructorInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_constructorInitializer(&mut self, ctx: &ConstructorInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memInitializerList}.
	 * @param ctx the parse tree
	 */
	fn visit_memInitializerList(&mut self, ctx: &MemInitializerListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_memInitializer(&mut self, ctx: &MemInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#meminitializerid}.
	 * @param ctx the parse tree
	 */
	fn visit_meminitializerid(&mut self, ctx: &MeminitializeridContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#operatorFunctionId}.
	 * @param ctx the parse tree
	 */
	fn visit_operatorFunctionId(&mut self, ctx: &OperatorFunctionIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#literalOperatorId}.
	 * @param ctx the parse tree
	 */
	fn visit_literalOperatorId(&mut self, ctx: &LiteralOperatorIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_templateDeclaration(&mut self, ctx: &TemplateDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateparameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_templateparameterList(&mut self, ctx: &TemplateparameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_templateParameter(&mut self, ctx: &TemplateParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTemplateId}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleTemplateId(&mut self, ctx: &SimpleTemplateIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateId}.
	 * @param ctx the parse tree
	 */
	fn visit_templateId(&mut self, ctx: &TemplateIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateName}.
	 * @param ctx the parse tree
	 */
	fn visit_templateName(&mut self, ctx: &TemplateNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateArgumentList}.
	 * @param ctx the parse tree
	 */
	fn visit_templateArgumentList(&mut self, ctx: &TemplateArgumentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeNameSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeNameSpecifier(&mut self, ctx: &TypeNameSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#explicitInstantiation}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitInstantiation(&mut self, ctx: &ExplicitInstantiationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#explicitSpecialization}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitSpecialization(&mut self, ctx: &ExplicitSpecializationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#tryBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_tryBlock(&mut self, ctx: &TryBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionTryBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTryBlock(&mut self, ctx: &FunctionTryBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#handlerSeq}.
	 * @param ctx the parse tree
	 */
	fn visit_handlerSeq(&mut self, ctx: &HandlerSeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#handler}.
	 * @param ctx the parse tree
	 */
	fn visit_handler(&mut self, ctx: &HandlerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#exceptionDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_exceptionDeclaration(&mut self, ctx: &ExceptionDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#throwExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_throwExpression(&mut self, ctx: &ThrowExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#exceptionSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_exceptionSpecification(&mut self, ctx: &ExceptionSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#dynamicExceptionSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_dynamicExceptionSpecification(&mut self, ctx: &DynamicExceptionSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeIdList}.
	 * @param ctx the parse tree
	 */
	fn visit_typeIdList(&mut self, ctx: &TypeIdListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noeExceptSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_noeExceptSpecification(&mut self, ctx: &NoeExceptSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#theOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_theOperator(&mut self, ctx: &TheOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

}

pub trait CPP14ParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= CPP14ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link CPP14Parser#translationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#primaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#idExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_idExpression(&mut self, ctx: &IdExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#unqualifiedId}.
	 * @param ctx the parse tree
	 */
		fn visit_unqualifiedId(&mut self, ctx: &UnqualifiedIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#qualifiedId}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedId(&mut self, ctx: &QualifiedIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#nestedNameSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_nestedNameSpecifier(&mut self, ctx: &NestedNameSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaIntroducer}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaIntroducer(&mut self, ctx: &LambdaIntroducerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaCapture}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaCapture(&mut self, ctx: &LambdaCaptureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#captureDefault}.
	 * @param ctx the parse tree
	 */
		fn visit_captureDefault(&mut self, ctx: &CaptureDefaultContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#captureList}.
	 * @param ctx the parse tree
	 */
		fn visit_captureList(&mut self, ctx: &CaptureListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#capture}.
	 * @param ctx the parse tree
	 */
		fn visit_capture(&mut self, ctx: &CaptureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleCapture}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleCapture(&mut self, ctx: &SimpleCaptureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initcapture}.
	 * @param ctx the parse tree
	 */
		fn visit_initcapture(&mut self, ctx: &InitcaptureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#lambdaDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaDeclarator(&mut self, ctx: &LambdaDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeIdOfTheTypeId}.
	 * @param ctx the parse tree
	 */
		fn visit_typeIdOfTheTypeId(&mut self, ctx: &TypeIdOfTheTypeIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#expressionList}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pseudoDestructorName}.
	 * @param ctx the parse tree
	 */
		fn visit_pseudoDestructorName(&mut self, ctx: &PseudoDestructorNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#unaryOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newExpression_}.
	 * @param ctx the parse tree
	 */
		fn visit_newExpression_(&mut self, ctx: &NewExpression_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newPlacement}.
	 * @param ctx the parse tree
	 */
		fn visit_newPlacement(&mut self, ctx: &NewPlacementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newTypeId}.
	 * @param ctx the parse tree
	 */
		fn visit_newTypeId(&mut self, ctx: &NewTypeIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newDeclarator_}.
	 * @param ctx the parse tree
	 */
		fn visit_newDeclarator_(&mut self, ctx: &NewDeclarator_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerNewDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_noPointerNewDeclarator(&mut self, ctx: &NoPointerNewDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#newInitializer_}.
	 * @param ctx the parse tree
	 */
		fn visit_newInitializer_(&mut self, ctx: &NewInitializer_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#deleteExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_deleteExpression(&mut self, ctx: &DeleteExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noExceptExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_noExceptExpression(&mut self, ctx: &NoExceptExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#castExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerMemberExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_pointerMemberExpression(&mut self, ctx: &PointerMemberExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#shiftExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#shiftOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#relationalExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#andExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#exclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#inclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conditionalExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#assignmentExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#assignmentOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#constantExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#labeledStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#expressionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#compoundStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#statementSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_statementSeq(&mut self, ctx: &StatementSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#selectionStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#condition}.
	 * @param ctx the parse tree
	 */
		fn visit_condition(&mut self, ctx: &ConditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#iterationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#forInitStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_forInitStatement(&mut self, ctx: &ForInitStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#forRangeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_forRangeDeclaration(&mut self, ctx: &ForRangeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#forRangeInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_forRangeInitializer(&mut self, ctx: &ForRangeInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#jumpStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declarationStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_declarationStatement(&mut self, ctx: &DeclarationStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declarationseq}.
	 * @param ctx the parse tree
	 */
		fn visit_declarationseq(&mut self, ctx: &DeclarationseqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declaration}.
	 * @param ctx the parse tree
	 */
		fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#blockDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_blockDeclaration(&mut self, ctx: &BlockDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#aliasDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_aliasDeclaration(&mut self, ctx: &AliasDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleDeclaration(&mut self, ctx: &SimpleDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#staticAssertDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_staticAssertDeclaration(&mut self, ctx: &StaticAssertDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#emptyDeclaration_}.
	 * @param ctx the parse tree
	 */
		fn visit_emptyDeclaration_(&mut self, ctx: &EmptyDeclaration_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeDeclaration(&mut self, ctx: &AttributeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_declSpecifier(&mut self, ctx: &DeclSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declSpecifierSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_declSpecifierSeq(&mut self, ctx: &DeclSpecifierSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#storageClassSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_storageClassSpecifier(&mut self, ctx: &StorageClassSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_functionSpecifier(&mut self, ctx: &FunctionSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typedefName}.
	 * @param ctx the parse tree
	 */
		fn visit_typedefName(&mut self, ctx: &TypedefNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#trailingTypeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_trailingTypeSpecifier(&mut self, ctx: &TrailingTypeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeSpecifierSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_typeSpecifierSeq(&mut self, ctx: &TypeSpecifierSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#trailingTypeSpecifierSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_trailingTypeSpecifierSeq(&mut self, ctx: &TrailingTypeSpecifierSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTypeLengthModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleTypeLengthModifier(&mut self, ctx: &SimpleTypeLengthModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTypeSignednessModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleTypeSignednessModifier(&mut self, ctx: &SimpleTypeSignednessModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTypeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleTypeSpecifier(&mut self, ctx: &SimpleTypeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#theTypeName}.
	 * @param ctx the parse tree
	 */
		fn visit_theTypeName(&mut self, ctx: &TheTypeNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#decltypeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_decltypeSpecifier(&mut self, ctx: &DecltypeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#elaboratedTypeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_elaboratedTypeSpecifier(&mut self, ctx: &ElaboratedTypeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumName}.
	 * @param ctx the parse tree
	 */
		fn visit_enumName(&mut self, ctx: &EnumNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumHead}.
	 * @param ctx the parse tree
	 */
		fn visit_enumHead(&mut self, ctx: &EnumHeadContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#opaqueEnumDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_opaqueEnumDeclaration(&mut self, ctx: &OpaqueEnumDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumkey}.
	 * @param ctx the parse tree
	 */
		fn visit_enumkey(&mut self, ctx: &EnumkeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumbase}.
	 * @param ctx the parse tree
	 */
		fn visit_enumbase(&mut self, ctx: &EnumbaseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumeratorList}.
	 * @param ctx the parse tree
	 */
		fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumeratorDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_enumeratorDefinition(&mut self, ctx: &EnumeratorDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#enumerator}.
	 * @param ctx the parse tree
	 */
		fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceName}.
	 * @param ctx the parse tree
	 */
		fn visit_namespaceName(&mut self, ctx: &NamespaceNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#originalNamespaceName}.
	 * @param ctx the parse tree
	 */
		fn visit_originalNamespaceName(&mut self, ctx: &OriginalNamespaceNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_namespaceDefinition(&mut self, ctx: &NamespaceDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceAlias}.
	 * @param ctx the parse tree
	 */
		fn visit_namespaceAlias(&mut self, ctx: &NamespaceAliasContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#namespaceAliasDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_namespaceAliasDefinition(&mut self, ctx: &NamespaceAliasDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#qualifiednamespacespecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiednamespacespecifier(&mut self, ctx: &QualifiednamespacespecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#usingDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_usingDeclaration(&mut self, ctx: &UsingDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#usingDirective}.
	 * @param ctx the parse tree
	 */
		fn visit_usingDirective(&mut self, ctx: &UsingDirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#asmDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_asmDefinition(&mut self, ctx: &AsmDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#linkageSpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_linkageSpecification(&mut self, ctx: &LinkageSpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeSpecifierSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeSpecifierSeq(&mut self, ctx: &AttributeSpecifierSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeSpecifier(&mut self, ctx: &AttributeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#alignmentspecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_alignmentspecifier(&mut self, ctx: &AlignmentspecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeList}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeList(&mut self, ctx: &AttributeListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attribute}.
	 * @param ctx the parse tree
	 */
		fn visit_attribute(&mut self, ctx: &AttributeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeNamespace}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeNamespace(&mut self, ctx: &AttributeNamespaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#attributeArgumentClause}.
	 * @param ctx the parse tree
	 */
		fn visit_attributeArgumentClause(&mut self, ctx: &AttributeArgumentClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#balancedTokenSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_balancedTokenSeq(&mut self, ctx: &BalancedTokenSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#balancedtoken}.
	 * @param ctx the parse tree
	 */
		fn visit_balancedtoken(&mut self, ctx: &BalancedtokenContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initDeclaratorList}.
	 * @param ctx the parse tree
	 */
		fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declarator}.
	 * @param ctx the parse tree
	 */
		fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_pointerDeclarator(&mut self, ctx: &PointerDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_noPointerDeclarator(&mut self, ctx: &NoPointerDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parametersAndQualifiers}.
	 * @param ctx the parse tree
	 */
		fn visit_parametersAndQualifiers(&mut self, ctx: &ParametersAndQualifiersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#trailingReturnType}.
	 * @param ctx the parse tree
	 */
		fn visit_trailingReturnType(&mut self, ctx: &TrailingReturnTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_pointerOperator(&mut self, ctx: &PointerOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#cvqualifierseq}.
	 * @param ctx the parse tree
	 */
		fn visit_cvqualifierseq(&mut self, ctx: &CvqualifierseqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#cvQualifier}.
	 * @param ctx the parse tree
	 */
		fn visit_cvQualifier(&mut self, ctx: &CvQualifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#refqualifier}.
	 * @param ctx the parse tree
	 */
		fn visit_refqualifier(&mut self, ctx: &RefqualifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#declaratorid}.
	 * @param ctx the parse tree
	 */
		fn visit_declaratorid(&mut self, ctx: &DeclaratoridContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#theTypeId}.
	 * @param ctx the parse tree
	 */
		fn visit_theTypeId(&mut self, ctx: &TheTypeIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#abstractDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_abstractDeclarator(&mut self, ctx: &AbstractDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pointerAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_pointerAbstractDeclarator(&mut self, ctx: &PointerAbstractDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_noPointerAbstractDeclarator(&mut self, ctx: &NoPointerAbstractDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#abstractPackDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_abstractPackDeclarator(&mut self, ctx: &AbstractPackDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noPointerAbstractPackDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_noPointerAbstractPackDeclarator(&mut self, ctx: &NoPointerAbstractPackDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parameterDeclarationClause}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterDeclarationClause(&mut self, ctx: &ParameterDeclarationClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parameterDeclarationList}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterDeclarationList(&mut self, ctx: &ParameterDeclarationListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#parameterDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionBody}.
	 * @param ctx the parse tree
	 */
		fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initializer}.
	 * @param ctx the parse tree
	 */
		fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#braceOrEqualInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_braceOrEqualInitializer(&mut self, ctx: &BraceOrEqualInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initializerClause}.
	 * @param ctx the parse tree
	 */
		fn visit_initializerClause(&mut self, ctx: &InitializerClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#initializerList}.
	 * @param ctx the parse tree
	 */
		fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#bracedInitList}.
	 * @param ctx the parse tree
	 */
		fn visit_bracedInitList(&mut self, ctx: &BracedInitListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#className}.
	 * @param ctx the parse tree
	 */
		fn visit_className(&mut self, ctx: &ClassNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_classSpecifier(&mut self, ctx: &ClassSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classHead}.
	 * @param ctx the parse tree
	 */
		fn visit_classHead(&mut self, ctx: &ClassHeadContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classHeadName}.
	 * @param ctx the parse tree
	 */
		fn visit_classHeadName(&mut self, ctx: &ClassHeadNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classVirtSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_classVirtSpecifier(&mut self, ctx: &ClassVirtSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classKey}.
	 * @param ctx the parse tree
	 */
		fn visit_classKey(&mut self, ctx: &ClassKeyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberSpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_memberSpecification(&mut self, ctx: &MemberSpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberdeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_memberdeclaration(&mut self, ctx: &MemberdeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberDeclaratorList}.
	 * @param ctx the parse tree
	 */
		fn visit_memberDeclaratorList(&mut self, ctx: &MemberDeclaratorListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memberDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_memberDeclarator(&mut self, ctx: &MemberDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#virtualSpecifierSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_virtualSpecifierSeq(&mut self, ctx: &VirtualSpecifierSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#virtualSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_virtualSpecifier(&mut self, ctx: &VirtualSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#pureSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_pureSpecifier(&mut self, ctx: &PureSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseClause}.
	 * @param ctx the parse tree
	 */
		fn visit_baseClause(&mut self, ctx: &BaseClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseSpecifierList}.
	 * @param ctx the parse tree
	 */
		fn visit_baseSpecifierList(&mut self, ctx: &BaseSpecifierListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_baseSpecifier(&mut self, ctx: &BaseSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#classOrDeclType}.
	 * @param ctx the parse tree
	 */
		fn visit_classOrDeclType(&mut self, ctx: &ClassOrDeclTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#baseTypeSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_baseTypeSpecifier(&mut self, ctx: &BaseTypeSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#accessSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_accessSpecifier(&mut self, ctx: &AccessSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conversionFunctionId}.
	 * @param ctx the parse tree
	 */
		fn visit_conversionFunctionId(&mut self, ctx: &ConversionFunctionIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conversionTypeId}.
	 * @param ctx the parse tree
	 */
		fn visit_conversionTypeId(&mut self, ctx: &ConversionTypeIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#conversionDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_conversionDeclarator(&mut self, ctx: &ConversionDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#constructorInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_constructorInitializer(&mut self, ctx: &ConstructorInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memInitializerList}.
	 * @param ctx the parse tree
	 */
		fn visit_memInitializerList(&mut self, ctx: &MemInitializerListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#memInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_memInitializer(&mut self, ctx: &MemInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#meminitializerid}.
	 * @param ctx the parse tree
	 */
		fn visit_meminitializerid(&mut self, ctx: &MeminitializeridContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#operatorFunctionId}.
	 * @param ctx the parse tree
	 */
		fn visit_operatorFunctionId(&mut self, ctx: &OperatorFunctionIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#literalOperatorId}.
	 * @param ctx the parse tree
	 */
		fn visit_literalOperatorId(&mut self, ctx: &LiteralOperatorIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_templateDeclaration(&mut self, ctx: &TemplateDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateparameterList}.
	 * @param ctx the parse tree
	 */
		fn visit_templateparameterList(&mut self, ctx: &TemplateparameterListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_templateParameter(&mut self, ctx: &TemplateParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#simpleTemplateId}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleTemplateId(&mut self, ctx: &SimpleTemplateIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateId}.
	 * @param ctx the parse tree
	 */
		fn visit_templateId(&mut self, ctx: &TemplateIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateName}.
	 * @param ctx the parse tree
	 */
		fn visit_templateName(&mut self, ctx: &TemplateNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateArgumentList}.
	 * @param ctx the parse tree
	 */
		fn visit_templateArgumentList(&mut self, ctx: &TemplateArgumentListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#templateArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeNameSpecifier}.
	 * @param ctx the parse tree
	 */
		fn visit_typeNameSpecifier(&mut self, ctx: &TypeNameSpecifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#explicitInstantiation}.
	 * @param ctx the parse tree
	 */
		fn visit_explicitInstantiation(&mut self, ctx: &ExplicitInstantiationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#explicitSpecialization}.
	 * @param ctx the parse tree
	 */
		fn visit_explicitSpecialization(&mut self, ctx: &ExplicitSpecializationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#tryBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_tryBlock(&mut self, ctx: &TryBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#functionTryBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_functionTryBlock(&mut self, ctx: &FunctionTryBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#handlerSeq}.
	 * @param ctx the parse tree
	 */
		fn visit_handlerSeq(&mut self, ctx: &HandlerSeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#handler}.
	 * @param ctx the parse tree
	 */
		fn visit_handler(&mut self, ctx: &HandlerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#exceptionDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_exceptionDeclaration(&mut self, ctx: &ExceptionDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#throwExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_throwExpression(&mut self, ctx: &ThrowExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#exceptionSpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_exceptionSpecification(&mut self, ctx: &ExceptionSpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#dynamicExceptionSpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_dynamicExceptionSpecification(&mut self, ctx: &DynamicExceptionSpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#typeIdList}.
	 * @param ctx the parse tree
	 */
		fn visit_typeIdList(&mut self, ctx: &TypeIdListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#noeExceptSpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_noeExceptSpecification(&mut self, ctx: &NoeExceptSpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#theOperator}.
	 * @param ctx the parse tree
	 */
		fn visit_theOperator(&mut self, ctx: &TheOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link CPP14Parser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> CPP14ParserVisitor<'input> for T
where
	T: CPP14ParserVisitorCompat<'input>
{
	fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_translationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_primaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_idExpression(&mut self, ctx: &IdExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_idExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unqualifiedId(&mut self, ctx: &UnqualifiedIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_unqualifiedId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedId(&mut self, ctx: &QualifiedIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_qualifiedId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nestedNameSpecifier(&mut self, ctx: &NestedNameSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_nestedNameSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_lambdaExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaIntroducer(&mut self, ctx: &LambdaIntroducerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_lambdaIntroducer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaCapture(&mut self, ctx: &LambdaCaptureContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_lambdaCapture(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_captureDefault(&mut self, ctx: &CaptureDefaultContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_captureDefault(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_captureList(&mut self, ctx: &CaptureListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_captureList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_capture(&mut self, ctx: &CaptureContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_capture(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleCapture(&mut self, ctx: &SimpleCaptureContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_simpleCapture(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initcapture(&mut self, ctx: &InitcaptureContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_initcapture(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaDeclarator(&mut self, ctx: &LambdaDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_lambdaDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_postfixExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeIdOfTheTypeId(&mut self, ctx: &TypeIdOfTheTypeIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typeIdOfTheTypeId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_expressionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pseudoDestructorName(&mut self, ctx: &PseudoDestructorNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_pseudoDestructorName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_unaryOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_newExpression_(&mut self, ctx: &NewExpression_Context<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_newExpression_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_newPlacement(&mut self, ctx: &NewPlacementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_newPlacement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_newTypeId(&mut self, ctx: &NewTypeIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_newTypeId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_newDeclarator_(&mut self, ctx: &NewDeclarator_Context<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_newDeclarator_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noPointerNewDeclarator(&mut self, ctx: &NoPointerNewDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_noPointerNewDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_newInitializer_(&mut self, ctx: &NewInitializer_Context<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_newInitializer_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deleteExpression(&mut self, ctx: &DeleteExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_deleteExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noExceptExpression(&mut self, ctx: &NoExceptExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_noExceptExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_castExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pointerMemberExpression(&mut self, ctx: &PointerMemberExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_pointerMemberExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_multiplicativeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_additiveExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_shiftExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_shiftOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_relationalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_equalityExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_andExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_exclusiveOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_inclusiveOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_logicalAndExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_logicalOrExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_conditionalExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_assignmentExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_assignmentOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_constantExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_labeledStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_expressionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_compoundStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statementSeq(&mut self, ctx: &StatementSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_statementSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_selectionStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_condition(&mut self, ctx: &ConditionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_condition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_iterationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forInitStatement(&mut self, ctx: &ForInitStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_forInitStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forRangeDeclaration(&mut self, ctx: &ForRangeDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_forRangeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forRangeInitializer(&mut self, ctx: &ForRangeInitializerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_forRangeInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_jumpStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarationStatement(&mut self, ctx: &DeclarationStatementContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declarationStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarationseq(&mut self, ctx: &DeclarationseqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declarationseq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockDeclaration(&mut self, ctx: &BlockDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_blockDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_aliasDeclaration(&mut self, ctx: &AliasDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_aliasDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleDeclaration(&mut self, ctx: &SimpleDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_simpleDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_staticAssertDeclaration(&mut self, ctx: &StaticAssertDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_staticAssertDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_emptyDeclaration_(&mut self, ctx: &EmptyDeclaration_Context<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_emptyDeclaration_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeDeclaration(&mut self, ctx: &AttributeDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attributeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declSpecifier(&mut self, ctx: &DeclSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declSpecifierSeq(&mut self, ctx: &DeclSpecifierSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declSpecifierSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_storageClassSpecifier(&mut self, ctx: &StorageClassSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_storageClassSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionSpecifier(&mut self, ctx: &FunctionSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_functionSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typedefName(&mut self, ctx: &TypedefNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typedefName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_trailingTypeSpecifier(&mut self, ctx: &TrailingTypeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_trailingTypeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeSpecifierSeq(&mut self, ctx: &TypeSpecifierSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typeSpecifierSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_trailingTypeSpecifierSeq(&mut self, ctx: &TrailingTypeSpecifierSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_trailingTypeSpecifierSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleTypeLengthModifier(&mut self, ctx: &SimpleTypeLengthModifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_simpleTypeLengthModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleTypeSignednessModifier(&mut self, ctx: &SimpleTypeSignednessModifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_simpleTypeSignednessModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleTypeSpecifier(&mut self, ctx: &SimpleTypeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_simpleTypeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theTypeName(&mut self, ctx: &TheTypeNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_theTypeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decltypeSpecifier(&mut self, ctx: &DecltypeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_decltypeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elaboratedTypeSpecifier(&mut self, ctx: &ElaboratedTypeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_elaboratedTypeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumName(&mut self, ctx: &EnumNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumHead(&mut self, ctx: &EnumHeadContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumHead(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_opaqueEnumDeclaration(&mut self, ctx: &OpaqueEnumDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_opaqueEnumDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumkey(&mut self, ctx: &EnumkeyContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumkey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumbase(&mut self, ctx: &EnumbaseContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumbase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumeratorList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumeratorDefinition(&mut self, ctx: &EnumeratorDefinitionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumeratorDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_enumerator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namespaceName(&mut self, ctx: &NamespaceNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_namespaceName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_originalNamespaceName(&mut self, ctx: &OriginalNamespaceNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_originalNamespaceName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namespaceDefinition(&mut self, ctx: &NamespaceDefinitionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_namespaceDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namespaceAlias(&mut self, ctx: &NamespaceAliasContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_namespaceAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namespaceAliasDefinition(&mut self, ctx: &NamespaceAliasDefinitionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_namespaceAliasDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiednamespacespecifier(&mut self, ctx: &QualifiednamespacespecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_qualifiednamespacespecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_usingDeclaration(&mut self, ctx: &UsingDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_usingDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_usingDirective(&mut self, ctx: &UsingDirectiveContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_usingDirective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asmDefinition(&mut self, ctx: &AsmDefinitionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_asmDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_linkageSpecification(&mut self, ctx: &LinkageSpecificationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_linkageSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeSpecifierSeq(&mut self, ctx: &AttributeSpecifierSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attributeSpecifierSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeSpecifier(&mut self, ctx: &AttributeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attributeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_alignmentspecifier(&mut self, ctx: &AlignmentspecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_alignmentspecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeList(&mut self, ctx: &AttributeListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attributeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attribute(&mut self, ctx: &AttributeContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attribute(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeNamespace(&mut self, ctx: &AttributeNamespaceContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attributeNamespace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_attributeArgumentClause(&mut self, ctx: &AttributeArgumentClauseContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_attributeArgumentClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_balancedTokenSeq(&mut self, ctx: &BalancedTokenSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_balancedTokenSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_balancedtoken(&mut self, ctx: &BalancedtokenContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_balancedtoken(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_initDeclaratorList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_initDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pointerDeclarator(&mut self, ctx: &PointerDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_pointerDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noPointerDeclarator(&mut self, ctx: &NoPointerDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_noPointerDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parametersAndQualifiers(&mut self, ctx: &ParametersAndQualifiersContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_parametersAndQualifiers(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_trailingReturnType(&mut self, ctx: &TrailingReturnTypeContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_trailingReturnType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pointerOperator(&mut self, ctx: &PointerOperatorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_pointerOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cvqualifierseq(&mut self, ctx: &CvqualifierseqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_cvqualifierseq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_cvQualifier(&mut self, ctx: &CvQualifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_cvQualifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_refqualifier(&mut self, ctx: &RefqualifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_refqualifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declaratorid(&mut self, ctx: &DeclaratoridContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_declaratorid(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theTypeId(&mut self, ctx: &TheTypeIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_theTypeId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_abstractDeclarator(&mut self, ctx: &AbstractDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_abstractDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pointerAbstractDeclarator(&mut self, ctx: &PointerAbstractDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_pointerAbstractDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noPointerAbstractDeclarator(&mut self, ctx: &NoPointerAbstractDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_noPointerAbstractDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_abstractPackDeclarator(&mut self, ctx: &AbstractPackDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_abstractPackDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noPointerAbstractPackDeclarator(&mut self, ctx: &NoPointerAbstractPackDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_noPointerAbstractPackDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterDeclarationClause(&mut self, ctx: &ParameterDeclarationClauseContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_parameterDeclarationClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterDeclarationList(&mut self, ctx: &ParameterDeclarationListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_parameterDeclarationList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_parameterDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_functionDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_functionBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializer(&mut self, ctx: &InitializerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_initializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_braceOrEqualInitializer(&mut self, ctx: &BraceOrEqualInitializerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_braceOrEqualInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializerClause(&mut self, ctx: &InitializerClauseContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_initializerClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_initializerList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bracedInitList(&mut self, ctx: &BracedInitListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_bracedInitList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_className(&mut self, ctx: &ClassNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_className(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classSpecifier(&mut self, ctx: &ClassSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_classSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classHead(&mut self, ctx: &ClassHeadContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_classHead(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classHeadName(&mut self, ctx: &ClassHeadNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_classHeadName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classVirtSpecifier(&mut self, ctx: &ClassVirtSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_classVirtSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classKey(&mut self, ctx: &ClassKeyContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_classKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memberSpecification(&mut self, ctx: &MemberSpecificationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_memberSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memberdeclaration(&mut self, ctx: &MemberdeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_memberdeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memberDeclaratorList(&mut self, ctx: &MemberDeclaratorListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_memberDeclaratorList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memberDeclarator(&mut self, ctx: &MemberDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_memberDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_virtualSpecifierSeq(&mut self, ctx: &VirtualSpecifierSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_virtualSpecifierSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_virtualSpecifier(&mut self, ctx: &VirtualSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_virtualSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pureSpecifier(&mut self, ctx: &PureSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_pureSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseClause(&mut self, ctx: &BaseClauseContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_baseClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseSpecifierList(&mut self, ctx: &BaseSpecifierListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_baseSpecifierList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseSpecifier(&mut self, ctx: &BaseSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_baseSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classOrDeclType(&mut self, ctx: &ClassOrDeclTypeContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_classOrDeclType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_baseTypeSpecifier(&mut self, ctx: &BaseTypeSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_baseTypeSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_accessSpecifier(&mut self, ctx: &AccessSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_accessSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conversionFunctionId(&mut self, ctx: &ConversionFunctionIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_conversionFunctionId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conversionTypeId(&mut self, ctx: &ConversionTypeIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_conversionTypeId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_conversionDeclarator(&mut self, ctx: &ConversionDeclaratorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_conversionDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constructorInitializer(&mut self, ctx: &ConstructorInitializerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_constructorInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memInitializerList(&mut self, ctx: &MemInitializerListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_memInitializerList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memInitializer(&mut self, ctx: &MemInitializerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_memInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_meminitializerid(&mut self, ctx: &MeminitializeridContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_meminitializerid(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_operatorFunctionId(&mut self, ctx: &OperatorFunctionIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_operatorFunctionId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literalOperatorId(&mut self, ctx: &LiteralOperatorIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_literalOperatorId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateDeclaration(&mut self, ctx: &TemplateDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateparameterList(&mut self, ctx: &TemplateparameterListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateparameterList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateParameter(&mut self, ctx: &TemplateParameterContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typeParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleTemplateId(&mut self, ctx: &SimpleTemplateIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_simpleTemplateId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateId(&mut self, ctx: &TemplateIdContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateName(&mut self, ctx: &TemplateNameContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateArgumentList(&mut self, ctx: &TemplateArgumentListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateArgumentList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_templateArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeNameSpecifier(&mut self, ctx: &TypeNameSpecifierContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typeNameSpecifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explicitInstantiation(&mut self, ctx: &ExplicitInstantiationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_explicitInstantiation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explicitSpecialization(&mut self, ctx: &ExplicitSpecializationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_explicitSpecialization(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_tryBlock(&mut self, ctx: &TryBlockContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_tryBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionTryBlock(&mut self, ctx: &FunctionTryBlockContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_functionTryBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_handlerSeq(&mut self, ctx: &HandlerSeqContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_handlerSeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_handler(&mut self, ctx: &HandlerContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_handler(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exceptionDeclaration(&mut self, ctx: &ExceptionDeclarationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_exceptionDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_throwExpression(&mut self, ctx: &ThrowExpressionContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_throwExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exceptionSpecification(&mut self, ctx: &ExceptionSpecificationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_exceptionSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_dynamicExceptionSpecification(&mut self, ctx: &DynamicExceptionSpecificationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_dynamicExceptionSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeIdList(&mut self, ctx: &TypeIdListContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_typeIdList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noeExceptSpecification(&mut self, ctx: &NoeExceptSpecificationContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_noeExceptSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_theOperator(&mut self, ctx: &TheOperatorContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_theOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as CPP14ParserVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}