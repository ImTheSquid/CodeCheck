#![allow(nonstandard_style)]
// Generated from JavaParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::javaparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link JavaParser}.
 */
pub trait JavaParserVisitor<'input>: ParseTreeVisitor<'input,JavaParserContextType>{
	/**
	 * Visit a parse tree produced by {@link JavaParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#packageDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#importDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_typeDeclaration(&mut self, ctx: &TypeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#modifier}.
	 * @param ctx the parse tree
	 */
	fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_classOrInterfaceModifier(&mut self, ctx: &ClassOrInterfaceModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_variableModifier(&mut self, ctx: &VariableModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_classDeclaration(&mut self, ctx: &ClassDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeBound}.
	 * @param ctx the parse tree
	 */
	fn visit_typeBound(&mut self, ctx: &TypeBoundContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumConstants}.
	 * @param ctx the parse tree
	 */
	fn visit_enumConstants(&mut self, ctx: &EnumConstantsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumConstant}.
	 * @param ctx the parse tree
	 */
	fn visit_enumConstant(&mut self, ctx: &EnumConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
	 * @param ctx the parse tree
	 */
	fn visit_enumBodyDeclarations(&mut self, ctx: &EnumBodyDeclarationsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceDeclaration(&mut self, ctx: &InterfaceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classBody}.
	 * @param ctx the parse tree
	 */
	fn visit_classBody(&mut self, ctx: &ClassBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceBody}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceBody(&mut self, ctx: &InterfaceBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classBodyDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_classBodyDeclaration(&mut self, ctx: &ClassBodyDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#memberDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_memberDeclaration(&mut self, ctx: &MemberDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#methodDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_methodDeclaration(&mut self, ctx: &MethodDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#methodBody}.
	 * @param ctx the parse tree
	 */
	fn visit_methodBody(&mut self, ctx: &MethodBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
	 * @param ctx the parse tree
	 */
	fn visit_typeTypeOrVoid(&mut self, ctx: &TypeTypeOrVoidContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_genericMethodDeclaration(&mut self, ctx: &GenericMethodDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_genericConstructorDeclaration(&mut self, ctx: &GenericConstructorDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#constructorDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_constructorDeclaration(&mut self, ctx: &ConstructorDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_compactConstructorDeclaration(&mut self, ctx: &CompactConstructorDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#fieldDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldDeclaration(&mut self, ctx: &FieldDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceBodyDeclaration(&mut self, ctx: &InterfaceBodyDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceMemberDeclaration(&mut self, ctx: &InterfaceMemberDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#constDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_constDeclaration(&mut self, ctx: &ConstDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#constantDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_constantDeclarator(&mut self, ctx: &ConstantDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceMethodDeclaration(&mut self, ctx: &InterfaceMethodDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceMethodModifier(&mut self, ctx: &InterfaceMethodModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_genericInterfaceMethodDeclaration(&mut self, ctx: &GenericInterfaceMethodDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceCommonBodyDeclaration(&mut self, ctx: &InterfaceCommonBodyDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableDeclarators}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclarators(&mut self, ctx: &VariableDeclaratorsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableDeclaratorId}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclaratorId(&mut self, ctx: &VariableDeclaratorIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#arrayInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayInitializer(&mut self, ctx: &ArrayInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classOrInterfaceType}.
	 * @param ctx the parse tree
	 */
	fn visit_classOrInterfaceType(&mut self, ctx: &ClassOrInterfaceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#qualifiedNameList}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedNameList(&mut self, ctx: &QualifiedNameListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#formalParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_formalParameters(&mut self, ctx: &FormalParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#receiverParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_receiverParameter(&mut self, ctx: &ReceiverParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#formalParameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_formalParameterList(&mut self, ctx: &FormalParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#formalParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_formalParameter(&mut self, ctx: &FormalParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#lastFormalParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_lastFormalParameter(&mut self, ctx: &LastFormalParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaLVTIList}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaLVTIList(&mut self, ctx: &LambdaLVTIListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaLVTIParameter(&mut self, ctx: &LambdaLVTIParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#qualifiedName}.
	 * @param ctx the parse tree
	 */
	fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#integerLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#floatLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
	 * @param ctx the parse tree
	 */
	fn visit_altAnnotationQualifiedName(&mut self, ctx: &AltAnnotationQualifiedNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValuePairs}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValuePairs(&mut self, ctx: &ElementValuePairsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValuePair}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValuePair(&mut self, ctx: &ElementValuePairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValue}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValue(&mut self, ctx: &ElementValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValueArrayInitializer(&mut self, ctx: &ElementValueArrayInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationTypeDeclaration(&mut self, ctx: &AnnotationTypeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeBody}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationTypeBody(&mut self, ctx: &AnnotationTypeBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationTypeElementDeclaration(&mut self, ctx: &AnnotationTypeElementDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationTypeElementRest(&mut self, ctx: &AnnotationTypeElementRestContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationMethodOrConstantRest(&mut self, ctx: &AnnotationMethodOrConstantRestContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationMethodRest}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationMethodRest(&mut self, ctx: &AnnotationMethodRestContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationConstantRest}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationConstantRest(&mut self, ctx: &AnnotationConstantRestContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#defaultValue}.
	 * @param ctx the parse tree
	 */
	fn visit_defaultValue(&mut self, ctx: &DefaultValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#moduleDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_moduleDeclaration(&mut self, ctx: &ModuleDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#moduleBody}.
	 * @param ctx the parse tree
	 */
	fn visit_moduleBody(&mut self, ctx: &ModuleBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#moduleDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_moduleDirective(&mut self, ctx: &ModuleDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#requiresModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_requiresModifier(&mut self, ctx: &RequiresModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_recordDeclaration(&mut self, ctx: &RecordDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_recordHeader(&mut self, ctx: &RecordHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordComponentList}.
	 * @param ctx the parse tree
	 */
	fn visit_recordComponentList(&mut self, ctx: &RecordComponentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordComponent}.
	 * @param ctx the parse tree
	 */
	fn visit_recordComponent(&mut self, ctx: &RecordComponentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordBody}.
	 * @param ctx the parse tree
	 */
	fn visit_recordBody(&mut self, ctx: &RecordBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#blockStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_blockStatement(&mut self, ctx: &BlockStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#localVariableDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_localVariableDeclaration(&mut self, ctx: &LocalVariableDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeIdentifier(&mut self, ctx: &TypeIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#localTypeDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_localTypeDeclaration(&mut self, ctx: &LocalTypeDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#catchClause}.
	 * @param ctx the parse tree
	 */
	fn visit_catchClause(&mut self, ctx: &CatchClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#catchType}.
	 * @param ctx the parse tree
	 */
	fn visit_catchType(&mut self, ctx: &CatchTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#finallyBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_finallyBlock(&mut self, ctx: &FinallyBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#resourceSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_resourceSpecification(&mut self, ctx: &ResourceSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#resources}.
	 * @param ctx the parse tree
	 */
	fn visit_resources(&mut self, ctx: &ResourcesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#resource}.
	 * @param ctx the parse tree
	 */
	fn visit_resource(&mut self, ctx: &ResourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#forControl}.
	 * @param ctx the parse tree
	 */
	fn visit_forControl(&mut self, ctx: &ForControlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#forInit}.
	 * @param ctx the parse tree
	 */
	fn visit_forInit(&mut self, ctx: &ForInitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#enhancedForControl}.
	 * @param ctx the parse tree
	 */
	fn visit_enhancedForControl(&mut self, ctx: &EnhancedForControlContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#parExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_parExpression(&mut self, ctx: &ParExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#expressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#methodCall}.
	 * @param ctx the parse tree
	 */
	fn visit_methodCall(&mut self, ctx: &MethodCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern(&mut self, ctx: &PatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaParameters(&mut self, ctx: &LambdaParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaBody}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaBody(&mut self, ctx: &LambdaBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#primary}.
	 * @param ctx the parse tree
	 */
	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_switchExpression(&mut self, ctx: &SwitchExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchLabeledRule}.
	 * @param ctx the parse tree
	 */
	fn visit_switchLabeledRule(&mut self, ctx: &SwitchLabeledRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#guardedPattern}.
	 * @param ctx the parse tree
	 */
	fn visit_guardedPattern(&mut self, ctx: &GuardedPatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchRuleOutcome}.
	 * @param ctx the parse tree
	 */
	fn visit_switchRuleOutcome(&mut self, ctx: &SwitchRuleOutcomeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classType}.
	 * @param ctx the parse tree
	 */
	fn visit_classType(&mut self, ctx: &ClassTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#creator}.
	 * @param ctx the parse tree
	 */
	fn visit_creator(&mut self, ctx: &CreatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#createdName}.
	 * @param ctx the parse tree
	 */
	fn visit_createdName(&mut self, ctx: &CreatedNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#innerCreator}.
	 * @param ctx the parse tree
	 */
	fn visit_innerCreator(&mut self, ctx: &InnerCreatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#arrayCreatorRest}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayCreatorRest(&mut self, ctx: &ArrayCreatorRestContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#classCreatorRest}.
	 * @param ctx the parse tree
	 */
	fn visit_classCreatorRest(&mut self, ctx: &ClassCreatorRestContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitGenericInvocation(&mut self, ctx: &ExplicitGenericInvocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArgumentsOrDiamond(&mut self, ctx: &TypeArgumentsOrDiamondContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
	 * @param ctx the parse tree
	 */
	fn visit_nonWildcardTypeArgumentsOrDiamond(&mut self, ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
	 * @param ctx the parse tree
	 */
	fn visit_nonWildcardTypeArguments(&mut self, ctx: &NonWildcardTypeArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeList}.
	 * @param ctx the parse tree
	 */
	fn visit_typeList(&mut self, ctx: &TypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeType}.
	 * @param ctx the parse tree
	 */
	fn visit_typeType(&mut self, ctx: &TypeTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#primitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeArguments}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#superSuffix}.
	 * @param ctx the parse tree
	 */
	fn visit_superSuffix(&mut self, ctx: &SuperSuffixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitGenericInvocationSuffix(&mut self, ctx: &ExplicitGenericInvocationSuffixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link JavaParser#arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) { self.visit_children(ctx) }

}

pub trait JavaParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= JavaParserContextType>{
	/**
	 * Visit a parse tree produced by {@link JavaParser#compilationUnit}.
	 * @param ctx the parse tree
	 */
		fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#packageDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#importDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_typeDeclaration(&mut self, ctx: &TypeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#modifier}.
	 * @param ctx the parse tree
	 */
		fn visit_modifier(&mut self, ctx: &ModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_classOrInterfaceModifier(&mut self, ctx: &ClassOrInterfaceModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_variableModifier(&mut self, ctx: &VariableModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_classDeclaration(&mut self, ctx: &ClassDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeParameters}.
	 * @param ctx the parse tree
	 */
		fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeBound}.
	 * @param ctx the parse tree
	 */
		fn visit_typeBound(&mut self, ctx: &TypeBoundContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumConstants}.
	 * @param ctx the parse tree
	 */
		fn visit_enumConstants(&mut self, ctx: &EnumConstantsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumConstant}.
	 * @param ctx the parse tree
	 */
		fn visit_enumConstant(&mut self, ctx: &EnumConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
	 * @param ctx the parse tree
	 */
		fn visit_enumBodyDeclarations(&mut self, ctx: &EnumBodyDeclarationsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceDeclaration(&mut self, ctx: &InterfaceDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classBody}.
	 * @param ctx the parse tree
	 */
		fn visit_classBody(&mut self, ctx: &ClassBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceBody}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceBody(&mut self, ctx: &InterfaceBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classBodyDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_classBodyDeclaration(&mut self, ctx: &ClassBodyDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#memberDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_memberDeclaration(&mut self, ctx: &MemberDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#methodDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_methodDeclaration(&mut self, ctx: &MethodDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#methodBody}.
	 * @param ctx the parse tree
	 */
		fn visit_methodBody(&mut self, ctx: &MethodBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
	 * @param ctx the parse tree
	 */
		fn visit_typeTypeOrVoid(&mut self, ctx: &TypeTypeOrVoidContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_genericMethodDeclaration(&mut self, ctx: &GenericMethodDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_genericConstructorDeclaration(&mut self, ctx: &GenericConstructorDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#constructorDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_constructorDeclaration(&mut self, ctx: &ConstructorDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_compactConstructorDeclaration(&mut self, ctx: &CompactConstructorDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#fieldDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_fieldDeclaration(&mut self, ctx: &FieldDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceBodyDeclaration(&mut self, ctx: &InterfaceBodyDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceMemberDeclaration(&mut self, ctx: &InterfaceMemberDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#constDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_constDeclaration(&mut self, ctx: &ConstDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#constantDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_constantDeclarator(&mut self, ctx: &ConstantDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceMethodDeclaration(&mut self, ctx: &InterfaceMethodDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceMethodModifier(&mut self, ctx: &InterfaceMethodModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_genericInterfaceMethodDeclaration(&mut self, ctx: &GenericInterfaceMethodDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_interfaceCommonBodyDeclaration(&mut self, ctx: &InterfaceCommonBodyDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableDeclarators}.
	 * @param ctx the parse tree
	 */
		fn visit_variableDeclarators(&mut self, ctx: &VariableDeclaratorsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableDeclarator}.
	 * @param ctx the parse tree
	 */
		fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableDeclaratorId}.
	 * @param ctx the parse tree
	 */
		fn visit_variableDeclaratorId(&mut self, ctx: &VariableDeclaratorIdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#variableInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#arrayInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayInitializer(&mut self, ctx: &ArrayInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classOrInterfaceType}.
	 * @param ctx the parse tree
	 */
		fn visit_classOrInterfaceType(&mut self, ctx: &ClassOrInterfaceTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeArgument}.
	 * @param ctx the parse tree
	 */
		fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#qualifiedNameList}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedNameList(&mut self, ctx: &QualifiedNameListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#formalParameters}.
	 * @param ctx the parse tree
	 */
		fn visit_formalParameters(&mut self, ctx: &FormalParametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#receiverParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_receiverParameter(&mut self, ctx: &ReceiverParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#formalParameterList}.
	 * @param ctx the parse tree
	 */
		fn visit_formalParameterList(&mut self, ctx: &FormalParameterListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#formalParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_formalParameter(&mut self, ctx: &FormalParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#lastFormalParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_lastFormalParameter(&mut self, ctx: &LastFormalParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaLVTIList}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaLVTIList(&mut self, ctx: &LambdaLVTIListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaLVTIParameter(&mut self, ctx: &LambdaLVTIParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#qualifiedName}.
	 * @param ctx the parse tree
	 */
		fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#literal}.
	 * @param ctx the parse tree
	 */
		fn visit_literal(&mut self, ctx: &LiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#integerLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#floatLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
	 * @param ctx the parse tree
	 */
		fn visit_altAnnotationQualifiedName(&mut self, ctx: &AltAnnotationQualifiedNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotation}.
	 * @param ctx the parse tree
	 */
		fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValuePairs}.
	 * @param ctx the parse tree
	 */
		fn visit_elementValuePairs(&mut self, ctx: &ElementValuePairsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValuePair}.
	 * @param ctx the parse tree
	 */
		fn visit_elementValuePair(&mut self, ctx: &ElementValuePairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValue}.
	 * @param ctx the parse tree
	 */
		fn visit_elementValue(&mut self, ctx: &ElementValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
	 * @param ctx the parse tree
	 */
		fn visit_elementValueArrayInitializer(&mut self, ctx: &ElementValueArrayInitializerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationTypeDeclaration(&mut self, ctx: &AnnotationTypeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeBody}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationTypeBody(&mut self, ctx: &AnnotationTypeBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationTypeElementDeclaration(&mut self, ctx: &AnnotationTypeElementDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationTypeElementRest(&mut self, ctx: &AnnotationTypeElementRestContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationMethodOrConstantRest(&mut self, ctx: &AnnotationMethodOrConstantRestContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationMethodRest}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationMethodRest(&mut self, ctx: &AnnotationMethodRestContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#annotationConstantRest}.
	 * @param ctx the parse tree
	 */
		fn visit_annotationConstantRest(&mut self, ctx: &AnnotationConstantRestContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#defaultValue}.
	 * @param ctx the parse tree
	 */
		fn visit_defaultValue(&mut self, ctx: &DefaultValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#moduleDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_moduleDeclaration(&mut self, ctx: &ModuleDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#moduleBody}.
	 * @param ctx the parse tree
	 */
		fn visit_moduleBody(&mut self, ctx: &ModuleBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#moduleDirective}.
	 * @param ctx the parse tree
	 */
		fn visit_moduleDirective(&mut self, ctx: &ModuleDirectiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#requiresModifier}.
	 * @param ctx the parse tree
	 */
		fn visit_requiresModifier(&mut self, ctx: &RequiresModifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_recordDeclaration(&mut self, ctx: &RecordDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordHeader}.
	 * @param ctx the parse tree
	 */
		fn visit_recordHeader(&mut self, ctx: &RecordHeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordComponentList}.
	 * @param ctx the parse tree
	 */
		fn visit_recordComponentList(&mut self, ctx: &RecordComponentListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordComponent}.
	 * @param ctx the parse tree
	 */
		fn visit_recordComponent(&mut self, ctx: &RecordComponentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#recordBody}.
	 * @param ctx the parse tree
	 */
		fn visit_recordBody(&mut self, ctx: &RecordBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#blockStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_blockStatement(&mut self, ctx: &BlockStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#localVariableDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_localVariableDeclaration(&mut self, ctx: &LocalVariableDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#identifier}.
	 * @param ctx the parse tree
	 */
		fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeIdentifier}.
	 * @param ctx the parse tree
	 */
		fn visit_typeIdentifier(&mut self, ctx: &TypeIdentifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#localTypeDeclaration}.
	 * @param ctx the parse tree
	 */
		fn visit_localTypeDeclaration(&mut self, ctx: &LocalTypeDeclarationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#catchClause}.
	 * @param ctx the parse tree
	 */
		fn visit_catchClause(&mut self, ctx: &CatchClauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#catchType}.
	 * @param ctx the parse tree
	 */
		fn visit_catchType(&mut self, ctx: &CatchTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#finallyBlock}.
	 * @param ctx the parse tree
	 */
		fn visit_finallyBlock(&mut self, ctx: &FinallyBlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#resourceSpecification}.
	 * @param ctx the parse tree
	 */
		fn visit_resourceSpecification(&mut self, ctx: &ResourceSpecificationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#resources}.
	 * @param ctx the parse tree
	 */
		fn visit_resources(&mut self, ctx: &ResourcesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#resource}.
	 * @param ctx the parse tree
	 */
		fn visit_resource(&mut self, ctx: &ResourceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
	 * @param ctx the parse tree
	 */
		fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchLabel}.
	 * @param ctx the parse tree
	 */
		fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#forControl}.
	 * @param ctx the parse tree
	 */
		fn visit_forControl(&mut self, ctx: &ForControlContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#forInit}.
	 * @param ctx the parse tree
	 */
		fn visit_forInit(&mut self, ctx: &ForInitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#enhancedForControl}.
	 * @param ctx the parse tree
	 */
		fn visit_enhancedForControl(&mut self, ctx: &EnhancedForControlContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#parExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_parExpression(&mut self, ctx: &ParExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#expressionList}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#methodCall}.
	 * @param ctx the parse tree
	 */
		fn visit_methodCall(&mut self, ctx: &MethodCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#pattern}.
	 * @param ctx the parse tree
	 */
		fn visit_pattern(&mut self, ctx: &PatternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaParameters}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaParameters(&mut self, ctx: &LambdaParametersContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#lambdaBody}.
	 * @param ctx the parse tree
	 */
		fn visit_lambdaBody(&mut self, ctx: &LambdaBodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#primary}.
	 * @param ctx the parse tree
	 */
		fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_switchExpression(&mut self, ctx: &SwitchExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchLabeledRule}.
	 * @param ctx the parse tree
	 */
		fn visit_switchLabeledRule(&mut self, ctx: &SwitchLabeledRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#guardedPattern}.
	 * @param ctx the parse tree
	 */
		fn visit_guardedPattern(&mut self, ctx: &GuardedPatternContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#switchRuleOutcome}.
	 * @param ctx the parse tree
	 */
		fn visit_switchRuleOutcome(&mut self, ctx: &SwitchRuleOutcomeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classType}.
	 * @param ctx the parse tree
	 */
		fn visit_classType(&mut self, ctx: &ClassTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#creator}.
	 * @param ctx the parse tree
	 */
		fn visit_creator(&mut self, ctx: &CreatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#createdName}.
	 * @param ctx the parse tree
	 */
		fn visit_createdName(&mut self, ctx: &CreatedNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#innerCreator}.
	 * @param ctx the parse tree
	 */
		fn visit_innerCreator(&mut self, ctx: &InnerCreatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#arrayCreatorRest}.
	 * @param ctx the parse tree
	 */
		fn visit_arrayCreatorRest(&mut self, ctx: &ArrayCreatorRestContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#classCreatorRest}.
	 * @param ctx the parse tree
	 */
		fn visit_classCreatorRest(&mut self, ctx: &ClassCreatorRestContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
	 * @param ctx the parse tree
	 */
		fn visit_explicitGenericInvocation(&mut self, ctx: &ExplicitGenericInvocationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
	 * @param ctx the parse tree
	 */
		fn visit_typeArgumentsOrDiamond(&mut self, ctx: &TypeArgumentsOrDiamondContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
	 * @param ctx the parse tree
	 */
		fn visit_nonWildcardTypeArgumentsOrDiamond(&mut self, ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
	 * @param ctx the parse tree
	 */
		fn visit_nonWildcardTypeArguments(&mut self, ctx: &NonWildcardTypeArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeList}.
	 * @param ctx the parse tree
	 */
		fn visit_typeList(&mut self, ctx: &TypeListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeType}.
	 * @param ctx the parse tree
	 */
		fn visit_typeType(&mut self, ctx: &TypeTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#primitiveType}.
	 * @param ctx the parse tree
	 */
		fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#typeArguments}.
	 * @param ctx the parse tree
	 */
		fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#superSuffix}.
	 * @param ctx the parse tree
	 */
		fn visit_superSuffix(&mut self, ctx: &SuperSuffixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
	 * @param ctx the parse tree
	 */
		fn visit_explicitGenericInvocationSuffix(&mut self, ctx: &ExplicitGenericInvocationSuffixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link JavaParser#arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> JavaParserVisitor<'input> for T
where
	T: JavaParserVisitorCompat<'input>
{
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_compilationUnit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_packageDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_importDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeDeclaration(&mut self, ctx: &TypeDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_modifier(&mut self, ctx: &ModifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_modifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classOrInterfaceModifier(&mut self, ctx: &ClassOrInterfaceModifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classOrInterfaceModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableModifier(&mut self, ctx: &VariableModifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_variableModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classDeclaration(&mut self, ctx: &ClassDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeBound(&mut self, ctx: &TypeBoundContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeBound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_enumDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumConstants(&mut self, ctx: &EnumConstantsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_enumConstants(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumConstant(&mut self, ctx: &EnumConstantContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_enumConstant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enumBodyDeclarations(&mut self, ctx: &EnumBodyDeclarationsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_enumBodyDeclarations(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceDeclaration(&mut self, ctx: &InterfaceDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classBody(&mut self, ctx: &ClassBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceBody(&mut self, ctx: &InterfaceBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classBodyDeclaration(&mut self, ctx: &ClassBodyDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classBodyDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_memberDeclaration(&mut self, ctx: &MemberDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_memberDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_methodDeclaration(&mut self, ctx: &MethodDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_methodDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_methodBody(&mut self, ctx: &MethodBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_methodBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeTypeOrVoid(&mut self, ctx: &TypeTypeOrVoidContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeTypeOrVoid(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_genericMethodDeclaration(&mut self, ctx: &GenericMethodDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_genericMethodDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_genericConstructorDeclaration(&mut self, ctx: &GenericConstructorDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_genericConstructorDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constructorDeclaration(&mut self, ctx: &ConstructorDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_constructorDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_compactConstructorDeclaration(&mut self, ctx: &CompactConstructorDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_compactConstructorDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fieldDeclaration(&mut self, ctx: &FieldDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_fieldDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceBodyDeclaration(&mut self, ctx: &InterfaceBodyDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceBodyDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceMemberDeclaration(&mut self, ctx: &InterfaceMemberDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceMemberDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constDeclaration(&mut self, ctx: &ConstDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_constDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constantDeclarator(&mut self, ctx: &ConstantDeclaratorContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_constantDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceMethodDeclaration(&mut self, ctx: &InterfaceMethodDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceMethodDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceMethodModifier(&mut self, ctx: &InterfaceMethodModifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceMethodModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_genericInterfaceMethodDeclaration(&mut self, ctx: &GenericInterfaceMethodDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_genericInterfaceMethodDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_interfaceCommonBodyDeclaration(&mut self, ctx: &InterfaceCommonBodyDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_interfaceCommonBodyDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableDeclarators(&mut self, ctx: &VariableDeclaratorsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_variableDeclarators(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_variableDeclarator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableDeclaratorId(&mut self, ctx: &VariableDeclaratorIdContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_variableDeclaratorId(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_variableInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayInitializer(&mut self, ctx: &ArrayInitializerContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_arrayInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classOrInterfaceType(&mut self, ctx: &ClassOrInterfaceTypeContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classOrInterfaceType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedNameList(&mut self, ctx: &QualifiedNameListContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_qualifiedNameList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formalParameters(&mut self, ctx: &FormalParametersContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_formalParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_receiverParameter(&mut self, ctx: &ReceiverParameterContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_receiverParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formalParameterList(&mut self, ctx: &FormalParameterListContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_formalParameterList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_formalParameter(&mut self, ctx: &FormalParameterContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_formalParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lastFormalParameter(&mut self, ctx: &LastFormalParameterContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_lastFormalParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaLVTIList(&mut self, ctx: &LambdaLVTIListContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_lambdaLVTIList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaLVTIParameter(&mut self, ctx: &LambdaLVTIParameterContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_lambdaLVTIParameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_qualifiedName(&mut self, ctx: &QualifiedNameContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_qualifiedName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal(&mut self, ctx: &LiteralContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integerLiteral(&mut self, ctx: &IntegerLiteralContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_integerLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_floatLiteral(&mut self, ctx: &FloatLiteralContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_floatLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_altAnnotationQualifiedName(&mut self, ctx: &AltAnnotationQualifiedNameContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_altAnnotationQualifiedName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elementValuePairs(&mut self, ctx: &ElementValuePairsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_elementValuePairs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elementValuePair(&mut self, ctx: &ElementValuePairContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_elementValuePair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elementValue(&mut self, ctx: &ElementValueContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_elementValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elementValueArrayInitializer(&mut self, ctx: &ElementValueArrayInitializerContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_elementValueArrayInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationTypeDeclaration(&mut self, ctx: &AnnotationTypeDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationTypeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationTypeBody(&mut self, ctx: &AnnotationTypeBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationTypeBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationTypeElementDeclaration(&mut self, ctx: &AnnotationTypeElementDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationTypeElementDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationTypeElementRest(&mut self, ctx: &AnnotationTypeElementRestContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationTypeElementRest(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationMethodOrConstantRest(&mut self, ctx: &AnnotationMethodOrConstantRestContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationMethodOrConstantRest(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationMethodRest(&mut self, ctx: &AnnotationMethodRestContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationMethodRest(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_annotationConstantRest(&mut self, ctx: &AnnotationConstantRestContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_annotationConstantRest(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_defaultValue(&mut self, ctx: &DefaultValueContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_defaultValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moduleDeclaration(&mut self, ctx: &ModuleDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_moduleDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moduleBody(&mut self, ctx: &ModuleBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_moduleBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_moduleDirective(&mut self, ctx: &ModuleDirectiveContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_moduleDirective(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_requiresModifier(&mut self, ctx: &RequiresModifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_requiresModifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordDeclaration(&mut self, ctx: &RecordDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_recordDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordHeader(&mut self, ctx: &RecordHeaderContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_recordHeader(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordComponentList(&mut self, ctx: &RecordComponentListContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_recordComponentList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordComponent(&mut self, ctx: &RecordComponentContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_recordComponent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_recordBody(&mut self, ctx: &RecordBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_recordBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_blockStatement(&mut self, ctx: &BlockStatementContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_blockStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localVariableDeclaration(&mut self, ctx: &LocalVariableDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_localVariableDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeIdentifier(&mut self, ctx: &TypeIdentifierContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localTypeDeclaration(&mut self, ctx: &LocalTypeDeclarationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_localTypeDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_catchClause(&mut self, ctx: &CatchClauseContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_catchClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_catchType(&mut self, ctx: &CatchTypeContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_catchType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_finallyBlock(&mut self, ctx: &FinallyBlockContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_finallyBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resourceSpecification(&mut self, ctx: &ResourceSpecificationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_resourceSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resources(&mut self, ctx: &ResourcesContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_resources(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_resource(&mut self, ctx: &ResourceContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_resource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_switchBlockStatementGroup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_switchLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forControl(&mut self, ctx: &ForControlContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_forControl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_forInit(&mut self, ctx: &ForInitContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_forInit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_enhancedForControl(&mut self, ctx: &EnhancedForControlContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_enhancedForControl(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parExpression(&mut self, ctx: &ParExpressionContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_parExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_expressionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_methodCall(&mut self, ctx: &MethodCallContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_methodCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pattern(&mut self, ctx: &PatternContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_pattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_lambdaExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaParameters(&mut self, ctx: &LambdaParametersContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_lambdaParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambdaBody(&mut self, ctx: &LambdaBodyContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_lambdaBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_primary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchExpression(&mut self, ctx: &SwitchExpressionContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_switchExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchLabeledRule(&mut self, ctx: &SwitchLabeledRuleContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_switchLabeledRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_guardedPattern(&mut self, ctx: &GuardedPatternContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_guardedPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_switchRuleOutcome(&mut self, ctx: &SwitchRuleOutcomeContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_switchRuleOutcome(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classType(&mut self, ctx: &ClassTypeContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_creator(&mut self, ctx: &CreatorContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_creator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_createdName(&mut self, ctx: &CreatedNameContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_createdName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_innerCreator(&mut self, ctx: &InnerCreatorContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_innerCreator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arrayCreatorRest(&mut self, ctx: &ArrayCreatorRestContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_arrayCreatorRest(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_classCreatorRest(&mut self, ctx: &ClassCreatorRestContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_classCreatorRest(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explicitGenericInvocation(&mut self, ctx: &ExplicitGenericInvocationContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_explicitGenericInvocation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeArgumentsOrDiamond(&mut self, ctx: &TypeArgumentsOrDiamondContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeArgumentsOrDiamond(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonWildcardTypeArgumentsOrDiamond(&mut self, ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_nonWildcardTypeArgumentsOrDiamond(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nonWildcardTypeArguments(&mut self, ctx: &NonWildcardTypeArgumentsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_nonWildcardTypeArguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeList(&mut self, ctx: &TypeListContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeType(&mut self, ctx: &TypeTypeContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_primitiveType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_typeArguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_superSuffix(&mut self, ctx: &SuperSuffixContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_superSuffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explicitGenericInvocationSuffix(&mut self, ctx: &ExplicitGenericInvocationSuffixContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_explicitGenericInvocationSuffix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>){
		let result = <Self as JavaParserVisitorCompat>::visit_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}