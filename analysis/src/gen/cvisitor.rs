#![allow(nonstandard_style)]
// Generated from C.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::cparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CParser}.
 */
pub trait CVisitor<'input>: ParseTreeVisitor<'input,CParserContextType>{
	/**
	 * Visit a parse tree produced by {@link CParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#genericSelection}.
	 * @param ctx the parse tree
	 */
	fn visit_genericSelection(&mut self, ctx: &GenericSelectionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#genericAssocList}.
	 * @param ctx the parse tree
	 */
	fn visit_genericAssocList(&mut self, ctx: &GenericAssocListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#genericAssociation}.
	 * @param ctx the parse tree
	 */
	fn visit_genericAssociation(&mut self, ctx: &GenericAssociationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#argumentExpressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_argumentExpressionList(&mut self, ctx: &ArgumentExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#unaryOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#castExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#shiftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#relationalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#andExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#exclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#inclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#logicalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#logicalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#conditionalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#assignmentExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#assignmentOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#constantExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#declaration}.
	 * @param ctx the parse tree
	 */
	fn visit_declaration(&mut self, ctx: &DeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#declarationSpecifiers}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationSpecifiers(&mut self, ctx: &DeclarationSpecifiersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#declarationSpecifiers2}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationSpecifiers2(&mut self, ctx: &DeclarationSpecifiers2Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#declarationSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationSpecifier(&mut self, ctx: &DeclarationSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#initDeclaratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#initDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#storageClassSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_storageClassSpecifier(&mut self, ctx: &StorageClassSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#typeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#structOrUnionSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_structOrUnionSpecifier(&mut self, ctx: &StructOrUnionSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#structOrUnion}.
	 * @param ctx the parse tree
	 */
	fn visit_structOrUnion(&mut self, ctx: &StructOrUnionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#structDeclarationList}.
	 * @param ctx the parse tree
	 */
	fn visit_structDeclarationList(&mut self, ctx: &StructDeclarationListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#structDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_structDeclaration(&mut self, ctx: &StructDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#specifierQualifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_specifierQualifierList(&mut self, ctx: &SpecifierQualifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#structDeclaratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_structDeclaratorList(&mut self, ctx: &StructDeclaratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#structDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_structDeclarator(&mut self, ctx: &StructDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#enumSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#enumeratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#enumerator}.
	 * @param ctx the parse tree
	 */
	fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#enumerationConstant}.
	 * @param ctx the parse tree
	 */
	fn visit_enumerationConstant(&mut self, ctx: &EnumerationConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#atomicTypeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_atomicTypeSpecifier(&mut self, ctx: &AtomicTypeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#typeQualifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeQualifier(&mut self, ctx: &TypeQualifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#functionSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_functionSpecifier(&mut self, ctx: &FunctionSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#alignmentSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_alignmentSpecifier(&mut self, ctx: &AlignmentSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#declarator}.
	 * @param ctx the parse tree
	 */
	fn visit_declarator(&mut self, ctx: &DeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#directDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_directDeclarator(&mut self, ctx: &DirectDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#vcSpecificModifer}.
	 * @param ctx the parse tree
	 */
	fn visit_vcSpecificModifer(&mut self, ctx: &VcSpecificModiferContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#gccDeclaratorExtension}.
	 * @param ctx the parse tree
	 */
	fn visit_gccDeclaratorExtension(&mut self, ctx: &GccDeclaratorExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#gccAttributeSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_gccAttributeSpecifier(&mut self, ctx: &GccAttributeSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#gccAttributeList}.
	 * @param ctx the parse tree
	 */
	fn visit_gccAttributeList(&mut self, ctx: &GccAttributeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#gccAttribute}.
	 * @param ctx the parse tree
	 */
	fn visit_gccAttribute(&mut self, ctx: &GccAttributeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#nestedParenthesesBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_nestedParenthesesBlock(&mut self, ctx: &NestedParenthesesBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#pointer}.
	 * @param ctx the parse tree
	 */
	fn visit_pointer(&mut self, ctx: &PointerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#typeQualifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_typeQualifierList(&mut self, ctx: &TypeQualifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#parameterTypeList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterTypeList(&mut self, ctx: &ParameterTypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#parameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#parameterDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#typeName}.
	 * @param ctx the parse tree
	 */
	fn visit_typeName(&mut self, ctx: &TypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#abstractDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_abstractDeclarator(&mut self, ctx: &AbstractDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#directAbstractDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_directAbstractDeclarator(&mut self, ctx: &DirectAbstractDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#typedefName}.
	 * @param ctx the parse tree
	 */
	fn visit_typedefName(&mut self, ctx: &TypedefNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#initializer}.
	 * @param ctx the parse tree
	 */
	fn visit_initializer(&mut self, ctx: &InitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#initializerList}.
	 * @param ctx the parse tree
	 */
	fn visit_initializerList(&mut self, ctx: &InitializerListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#designation}.
	 * @param ctx the parse tree
	 */
	fn visit_designation(&mut self, ctx: &DesignationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#designatorList}.
	 * @param ctx the parse tree
	 */
	fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#designator}.
	 * @param ctx the parse tree
	 */
	fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#staticAssertDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_staticAssertDeclaration(&mut self, ctx: &StaticAssertDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#labeledStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#compoundStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#blockItemList}.
	 * @param ctx the parse tree
	 */
	fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#blockItem}.
	 * @param ctx the parse tree
	 */
	fn visit_blockItem(&mut self, ctx: &BlockItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#expressionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#selectionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#iterationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#forCondition}.
	 * @param ctx the parse tree
	 */
	fn visit_forCondition(&mut self, ctx: &ForConditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#forDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#forExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#jumpStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#translationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#externalDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link CParser#declarationList}.
	 * @param ctx the parse tree
	 */
	fn visit_declarationList(&mut self, ctx: &DeclarationListContext<'input>) { self.visit_children(ctx) }


}