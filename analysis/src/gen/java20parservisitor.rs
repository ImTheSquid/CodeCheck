#![allow(nonstandard_style)]
// Generated from Java20Parser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::java20parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link Java20Parser}.
 */
pub trait Java20ParserVisitor<'input>: ParseTreeVisitor<'input,Java20ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link Java20Parser#start}.
	 * @param ctx the parse tree
	 */
	fn visit_start(&mut self, ctx: &StartContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#literal}.
	 * @param ctx the parse tree
	 */
	fn visit_literal(&mut self, ctx: &LiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeIdentifier(&mut self, ctx: &TypeIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unqualifiedMethodIdentifier}.
	 * @param ctx the parse tree
	 */
	fn visit_unqualifiedMethodIdentifier(&mut self, ctx: &UnqualifiedMethodIdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#primitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#numericType}.
	 * @param ctx the parse tree
	 */
	fn visit_numericType(&mut self, ctx: &NumericTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#integralType}.
	 * @param ctx the parse tree
	 */
	fn visit_integralType(&mut self, ctx: &IntegralTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#floatingPointType}.
	 * @param ctx the parse tree
	 */
	fn visit_floatingPointType(&mut self, ctx: &FloatingPointTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#referenceType}.
	 * @param ctx the parse tree
	 */
	fn visit_referenceType(&mut self, ctx: &ReferenceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#coit}.
	 * @param ctx the parse tree
	 */
	fn visit_coit(&mut self, ctx: &CoitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classOrInterfaceType}.
	 * @param ctx the parse tree
	 */
	fn visit_classOrInterfaceType(&mut self, ctx: &ClassOrInterfaceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classType}.
	 * @param ctx the parse tree
	 */
	fn visit_classType(&mut self, ctx: &ClassTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceType}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceType(&mut self, ctx: &InterfaceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeVariable}.
	 * @param ctx the parse tree
	 */
	fn visit_typeVariable(&mut self, ctx: &TypeVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#arrayType}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#dims}.
	 * @param ctx the parse tree
	 */
	fn visit_dims(&mut self, ctx: &DimsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeParameterModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameterModifier(&mut self, ctx: &TypeParameterModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeBound}.
	 * @param ctx the parse tree
	 */
	fn visit_typeBound(&mut self, ctx: &TypeBoundContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#additionalBound}.
	 * @param ctx the parse tree
	 */
	fn visit_additionalBound(&mut self, ctx: &AdditionalBoundContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeArguments}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArguments(&mut self, ctx: &TypeArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeArgumentList}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArgumentList(&mut self, ctx: &TypeArgumentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeArgument}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArgument(&mut self, ctx: &TypeArgumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#wildcard}.
	 * @param ctx the parse tree
	 */
	fn visit_wildcard(&mut self, ctx: &WildcardContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#wildcardBounds}.
	 * @param ctx the parse tree
	 */
	fn visit_wildcardBounds(&mut self, ctx: &WildcardBoundsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#moduleName}.
	 * @param ctx the parse tree
	 */
	fn visit_moduleName(&mut self, ctx: &ModuleNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#packageName}.
	 * @param ctx the parse tree
	 */
	fn visit_packageName(&mut self, ctx: &PackageNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeName}.
	 * @param ctx the parse tree
	 */
	fn visit_typeName(&mut self, ctx: &TypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#packageOrTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_packageOrTypeName(&mut self, ctx: &PackageOrTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#expressionName}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionName(&mut self, ctx: &ExpressionNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodName}.
	 * @param ctx the parse tree
	 */
	fn visit_methodName(&mut self, ctx: &MethodNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#ambiguousName}.
	 * @param ctx the parse tree
	 */
	fn visit_ambiguousName(&mut self, ctx: &AmbiguousNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#compilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#ordinaryCompilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_ordinaryCompilationUnit(&mut self, ctx: &OrdinaryCompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#modularCompilationUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_modularCompilationUnit(&mut self, ctx: &ModularCompilationUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#packageDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_packageDeclaration(&mut self, ctx: &PackageDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#packageModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_packageModifier(&mut self, ctx: &PackageModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#importDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#singleTypeImportDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_singleTypeImportDeclaration(&mut self, ctx: &SingleTypeImportDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeImportOnDemandDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_typeImportOnDemandDeclaration(&mut self, ctx: &TypeImportOnDemandDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#singleStaticImportDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_singleStaticImportDeclaration(&mut self, ctx: &SingleStaticImportDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#staticImportOnDemandDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_staticImportOnDemandDeclaration(&mut self, ctx: &StaticImportOnDemandDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#topLevelClassOrInterfaceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_topLevelClassOrInterfaceDeclaration(&mut self, ctx: &TopLevelClassOrInterfaceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#moduleDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_moduleDeclaration(&mut self, ctx: &ModuleDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#moduleDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_moduleDirective(&mut self, ctx: &ModuleDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#requiresModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_requiresModifier(&mut self, ctx: &RequiresModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_classDeclaration(&mut self, ctx: &ClassDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#normalClassDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_normalClassDeclaration(&mut self, ctx: &NormalClassDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_classModifier(&mut self, ctx: &ClassModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameters(&mut self, ctx: &TypeParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeParameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_typeParameterList(&mut self, ctx: &TypeParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classExtends}.
	 * @param ctx the parse tree
	 */
	fn visit_classExtends(&mut self, ctx: &ClassExtendsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classImplements}.
	 * @param ctx the parse tree
	 */
	fn visit_classImplements(&mut self, ctx: &ClassImplementsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceTypeList}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceTypeList(&mut self, ctx: &InterfaceTypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classPermits}.
	 * @param ctx the parse tree
	 */
	fn visit_classPermits(&mut self, ctx: &ClassPermitsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classBody}.
	 * @param ctx the parse tree
	 */
	fn visit_classBody(&mut self, ctx: &ClassBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classBodyDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_classBodyDeclaration(&mut self, ctx: &ClassBodyDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classMemberDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_classMemberDeclaration(&mut self, ctx: &ClassMemberDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#fieldDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldDeclaration(&mut self, ctx: &FieldDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#fieldModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldModifier(&mut self, ctx: &FieldModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableDeclaratorList}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclaratorList(&mut self, ctx: &VariableDeclaratorListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclarator(&mut self, ctx: &VariableDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableDeclaratorId}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclaratorId(&mut self, ctx: &VariableDeclaratorIdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_variableInitializer(&mut self, ctx: &VariableInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannType(&mut self, ctx: &UnannTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannPrimitiveType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannPrimitiveType(&mut self, ctx: &UnannPrimitiveTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannReferenceType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannReferenceType(&mut self, ctx: &UnannReferenceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannClassOrInterfaceType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannClassOrInterfaceType(&mut self, ctx: &UnannClassOrInterfaceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#uCOIT}.
	 * @param ctx the parse tree
	 */
	fn visit_uCOIT(&mut self, ctx: &UCOITContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannClassType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannClassType(&mut self, ctx: &UnannClassTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannInterfaceType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannInterfaceType(&mut self, ctx: &UnannInterfaceTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannTypeVariable}.
	 * @param ctx the parse tree
	 */
	fn visit_unannTypeVariable(&mut self, ctx: &UnannTypeVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unannArrayType}.
	 * @param ctx the parse tree
	 */
	fn visit_unannArrayType(&mut self, ctx: &UnannArrayTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_methodDeclaration(&mut self, ctx: &MethodDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_methodModifier(&mut self, ctx: &MethodModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_methodHeader(&mut self, ctx: &MethodHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#result}.
	 * @param ctx the parse tree
	 */
	fn visit_result(&mut self, ctx: &ResultContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_methodDeclarator(&mut self, ctx: &MethodDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#receiverParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_receiverParameter(&mut self, ctx: &ReceiverParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#formalParameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_formalParameterList(&mut self, ctx: &FormalParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#formalParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_formalParameter(&mut self, ctx: &FormalParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableArityParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_variableArityParameter(&mut self, ctx: &VariableArityParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_variableModifier(&mut self, ctx: &VariableModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#throwsT}.
	 * @param ctx the parse tree
	 */
	fn visit_throwsT(&mut self, ctx: &ThrowsTContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#exceptionTypeList}.
	 * @param ctx the parse tree
	 */
	fn visit_exceptionTypeList(&mut self, ctx: &ExceptionTypeListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#exceptionType}.
	 * @param ctx the parse tree
	 */
	fn visit_exceptionType(&mut self, ctx: &ExceptionTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodBody}.
	 * @param ctx the parse tree
	 */
	fn visit_methodBody(&mut self, ctx: &MethodBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#instanceInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_instanceInitializer(&mut self, ctx: &InstanceInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#staticInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_staticInitializer(&mut self, ctx: &StaticInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constructorDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_constructorDeclaration(&mut self, ctx: &ConstructorDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constructorModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_constructorModifier(&mut self, ctx: &ConstructorModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constructorDeclarator}.
	 * @param ctx the parse tree
	 */
	fn visit_constructorDeclarator(&mut self, ctx: &ConstructorDeclaratorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#simpleTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleTypeName(&mut self, ctx: &SimpleTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constructorBody}.
	 * @param ctx the parse tree
	 */
	fn visit_constructorBody(&mut self, ctx: &ConstructorBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#explicitConstructorInvocation}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitConstructorInvocation(&mut self, ctx: &ExplicitConstructorInvocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enumDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDeclaration(&mut self, ctx: &EnumDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enumBody}.
	 * @param ctx the parse tree
	 */
	fn visit_enumBody(&mut self, ctx: &EnumBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enumConstantList}.
	 * @param ctx the parse tree
	 */
	fn visit_enumConstantList(&mut self, ctx: &EnumConstantListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enumConstant}.
	 * @param ctx the parse tree
	 */
	fn visit_enumConstant(&mut self, ctx: &EnumConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enumConstantModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_enumConstantModifier(&mut self, ctx: &EnumConstantModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enumBodyDeclarations}.
	 * @param ctx the parse tree
	 */
	fn visit_enumBodyDeclarations(&mut self, ctx: &EnumBodyDeclarationsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_recordDeclaration(&mut self, ctx: &RecordDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordHeader}.
	 * @param ctx the parse tree
	 */
	fn visit_recordHeader(&mut self, ctx: &RecordHeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordComponentList}.
	 * @param ctx the parse tree
	 */
	fn visit_recordComponentList(&mut self, ctx: &RecordComponentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordComponent}.
	 * @param ctx the parse tree
	 */
	fn visit_recordComponent(&mut self, ctx: &RecordComponentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableArityRecordComponent}.
	 * @param ctx the parse tree
	 */
	fn visit_variableArityRecordComponent(&mut self, ctx: &VariableArityRecordComponentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordComponentModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_recordComponentModifier(&mut self, ctx: &RecordComponentModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordBody}.
	 * @param ctx the parse tree
	 */
	fn visit_recordBody(&mut self, ctx: &RecordBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#recordBodyDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_recordBodyDeclaration(&mut self, ctx: &RecordBodyDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#compactConstructorDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_compactConstructorDeclaration(&mut self, ctx: &CompactConstructorDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceDeclaration(&mut self, ctx: &InterfaceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#normalInterfaceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_normalInterfaceDeclaration(&mut self, ctx: &NormalInterfaceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceModifier(&mut self, ctx: &InterfaceModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceExtends}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceExtends(&mut self, ctx: &InterfaceExtendsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfacePermits}.
	 * @param ctx the parse tree
	 */
	fn visit_interfacePermits(&mut self, ctx: &InterfacePermitsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceBody}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceBody(&mut self, ctx: &InterfaceBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceMemberDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceMemberDeclaration(&mut self, ctx: &InterfaceMemberDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constantDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_constantDeclaration(&mut self, ctx: &ConstantDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constantModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_constantModifier(&mut self, ctx: &ConstantModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceMethodDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceMethodDeclaration(&mut self, ctx: &InterfaceMethodDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#interfaceMethodModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_interfaceMethodModifier(&mut self, ctx: &InterfaceMethodModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#annotationInterfaceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationInterfaceDeclaration(&mut self, ctx: &AnnotationInterfaceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#annotationInterfaceBody}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationInterfaceBody(&mut self, ctx: &AnnotationInterfaceBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#annotationInterfaceMemberDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationInterfaceMemberDeclaration(&mut self, ctx: &AnnotationInterfaceMemberDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#annotationInterfaceElementDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationInterfaceElementDeclaration(&mut self, ctx: &AnnotationInterfaceElementDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#annotationInterfaceElementModifier}.
	 * @param ctx the parse tree
	 */
	fn visit_annotationInterfaceElementModifier(&mut self, ctx: &AnnotationInterfaceElementModifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#defaultValue}.
	 * @param ctx the parse tree
	 */
	fn visit_defaultValue(&mut self, ctx: &DefaultValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#annotation}.
	 * @param ctx the parse tree
	 */
	fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#normalAnnotation}.
	 * @param ctx the parse tree
	 */
	fn visit_normalAnnotation(&mut self, ctx: &NormalAnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#elementValuePairList}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValuePairList(&mut self, ctx: &ElementValuePairListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#elementValuePair}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValuePair(&mut self, ctx: &ElementValuePairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#elementValue}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValue(&mut self, ctx: &ElementValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#elementValueArrayInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValueArrayInitializer(&mut self, ctx: &ElementValueArrayInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#elementValueList}.
	 * @param ctx the parse tree
	 */
	fn visit_elementValueList(&mut self, ctx: &ElementValueListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#markerAnnotation}.
	 * @param ctx the parse tree
	 */
	fn visit_markerAnnotation(&mut self, ctx: &MarkerAnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#singleElementAnnotation}.
	 * @param ctx the parse tree
	 */
	fn visit_singleElementAnnotation(&mut self, ctx: &SingleElementAnnotationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#arrayInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayInitializer(&mut self, ctx: &ArrayInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableInitializerList}.
	 * @param ctx the parse tree
	 */
	fn visit_variableInitializerList(&mut self, ctx: &VariableInitializerListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#blockStatements}.
	 * @param ctx the parse tree
	 */
	fn visit_blockStatements(&mut self, ctx: &BlockStatementsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#blockStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_blockStatement(&mut self, ctx: &BlockStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#localClassOrInterfaceDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_localClassOrInterfaceDeclaration(&mut self, ctx: &LocalClassOrInterfaceDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#localVariableDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_localVariableDeclaration(&mut self, ctx: &LocalVariableDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#localVariableType}.
	 * @param ctx the parse tree
	 */
	fn visit_localVariableType(&mut self, ctx: &LocalVariableTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#localVariableDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_localVariableDeclarationStatement(&mut self, ctx: &LocalVariableDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#statementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_statementNoShortIf(&mut self, ctx: &StatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#statementWithoutTrailingSubstatement}.
	 * @param ctx the parse tree
	 */
	fn visit_statementWithoutTrailingSubstatement(&mut self, ctx: &StatementWithoutTrailingSubstatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#emptyStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_emptyStatement(&mut self, ctx: &EmptyStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#labeledStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#labeledStatementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_labeledStatementNoShortIf(&mut self, ctx: &LabeledStatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#expressionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#statementExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_statementExpression(&mut self, ctx: &StatementExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#ifThenStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifThenStatement(&mut self, ctx: &IfThenStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#ifThenElseStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifThenElseStatement(&mut self, ctx: &IfThenElseStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#ifThenElseStatementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_ifThenElseStatementNoShortIf(&mut self, ctx: &IfThenElseStatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#assertStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_assertStatement(&mut self, ctx: &AssertStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#switchStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#switchBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_switchBlock(&mut self, ctx: &SwitchBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#switchRule}.
	 * @param ctx the parse tree
	 */
	fn visit_switchRule(&mut self, ctx: &SwitchRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#switchBlockStatementGroup}.
	 * @param ctx the parse tree
	 */
	fn visit_switchBlockStatementGroup(&mut self, ctx: &SwitchBlockStatementGroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#switchLabel}.
	 * @param ctx the parse tree
	 */
	fn visit_switchLabel(&mut self, ctx: &SwitchLabelContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#caseConstant}.
	 * @param ctx the parse tree
	 */
	fn visit_caseConstant(&mut self, ctx: &CaseConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#whileStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#whileStatementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_whileStatementNoShortIf(&mut self, ctx: &WhileStatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#doStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_doStatement(&mut self, ctx: &DoStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#forStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#forStatementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_forStatementNoShortIf(&mut self, ctx: &ForStatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#basicForStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_basicForStatement(&mut self, ctx: &BasicForStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#basicForStatementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_basicForStatementNoShortIf(&mut self, ctx: &BasicForStatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#forInit}.
	 * @param ctx the parse tree
	 */
	fn visit_forInit(&mut self, ctx: &ForInitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#forUpdate}.
	 * @param ctx the parse tree
	 */
	fn visit_forUpdate(&mut self, ctx: &ForUpdateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#statementExpressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_statementExpressionList(&mut self, ctx: &StatementExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enhancedForStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_enhancedForStatement(&mut self, ctx: &EnhancedForStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#enhancedForStatementNoShortIf}.
	 * @param ctx the parse tree
	 */
	fn visit_enhancedForStatementNoShortIf(&mut self, ctx: &EnhancedForStatementNoShortIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#breakStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#continueStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#returnStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#throwStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_throwStatement(&mut self, ctx: &ThrowStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#synchronizedStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_synchronizedStatement(&mut self, ctx: &SynchronizedStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#tryStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_tryStatement(&mut self, ctx: &TryStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#catches}.
	 * @param ctx the parse tree
	 */
	fn visit_catches(&mut self, ctx: &CatchesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#catchClause}.
	 * @param ctx the parse tree
	 */
	fn visit_catchClause(&mut self, ctx: &CatchClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#catchFormalParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_catchFormalParameter(&mut self, ctx: &CatchFormalParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#catchType}.
	 * @param ctx the parse tree
	 */
	fn visit_catchType(&mut self, ctx: &CatchTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#finallyBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_finallyBlock(&mut self, ctx: &FinallyBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#tryWithResourcesStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_tryWithResourcesStatement(&mut self, ctx: &TryWithResourcesStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#resourceSpecification}.
	 * @param ctx the parse tree
	 */
	fn visit_resourceSpecification(&mut self, ctx: &ResourceSpecificationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#resourceList}.
	 * @param ctx the parse tree
	 */
	fn visit_resourceList(&mut self, ctx: &ResourceListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#resource}.
	 * @param ctx the parse tree
	 */
	fn visit_resource(&mut self, ctx: &ResourceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#variableAccess}.
	 * @param ctx the parse tree
	 */
	fn visit_variableAccess(&mut self, ctx: &VariableAccessContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#yieldStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_yieldStatement(&mut self, ctx: &YieldStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#pattern}.
	 * @param ctx the parse tree
	 */
	fn visit_pattern(&mut self, ctx: &PatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typePattern}.
	 * @param ctx the parse tree
	 */
	fn visit_typePattern(&mut self, ctx: &TypePatternContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#primary}.
	 * @param ctx the parse tree
	 */
	fn visit_primary(&mut self, ctx: &PrimaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#primaryNoNewArray}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryNoNewArray(&mut self, ctx: &PrimaryNoNewArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#pNNA}.
	 * @param ctx the parse tree
	 */
	fn visit_pNNA(&mut self, ctx: &PNNAContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_classLiteral(&mut self, ctx: &ClassLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classInstanceCreationExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_classInstanceCreationExpression(&mut self, ctx: &ClassInstanceCreationExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unqualifiedClassInstanceCreationExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unqualifiedClassInstanceCreationExpression(&mut self, ctx: &UnqualifiedClassInstanceCreationExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#classOrInterfaceTypeToInstantiate}.
	 * @param ctx the parse tree
	 */
	fn visit_classOrInterfaceTypeToInstantiate(&mut self, ctx: &ClassOrInterfaceTypeToInstantiateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#typeArgumentsOrDiamond}.
	 * @param ctx the parse tree
	 */
	fn visit_typeArgumentsOrDiamond(&mut self, ctx: &TypeArgumentsOrDiamondContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#arrayCreationExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayCreationExpression(&mut self, ctx: &ArrayCreationExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#arrayCreationExpressionWithoutInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayCreationExpressionWithoutInitializer(&mut self, ctx: &ArrayCreationExpressionWithoutInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#arrayCreationExpressionWithInitializer}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayCreationExpressionWithInitializer(&mut self, ctx: &ArrayCreationExpressionWithInitializerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#dimExprs}.
	 * @param ctx the parse tree
	 */
	fn visit_dimExprs(&mut self, ctx: &DimExprsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#dimExpr}.
	 * @param ctx the parse tree
	 */
	fn visit_dimExpr(&mut self, ctx: &DimExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#arrayAccess}.
	 * @param ctx the parse tree
	 */
	fn visit_arrayAccess(&mut self, ctx: &ArrayAccessContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#fieldAccess}.
	 * @param ctx the parse tree
	 */
	fn visit_fieldAccess(&mut self, ctx: &FieldAccessContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodInvocation}.
	 * @param ctx the parse tree
	 */
	fn visit_methodInvocation(&mut self, ctx: &MethodInvocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#argumentList}.
	 * @param ctx the parse tree
	 */
	fn visit_argumentList(&mut self, ctx: &ArgumentListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#methodReference}.
	 * @param ctx the parse tree
	 */
	fn visit_methodReference(&mut self, ctx: &MethodReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#postfixExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#pfE}.
	 * @param ctx the parse tree
	 */
	fn visit_pfE(&mut self, ctx: &PfEContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#postIncrementExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postIncrementExpression(&mut self, ctx: &PostIncrementExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#postDecrementExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_postDecrementExpression(&mut self, ctx: &PostDecrementExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#preIncrementExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_preIncrementExpression(&mut self, ctx: &PreIncrementExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#preDecrementExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_preDecrementExpression(&mut self, ctx: &PreDecrementExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#unaryExpressionNotPlusMinus}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpressionNotPlusMinus(&mut self, ctx: &UnaryExpressionNotPlusMinusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#castExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#multiplicativeExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#additiveExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#shiftExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#relationalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#equalityExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#andExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#exclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#inclusiveOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#conditionalAndExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalAndExpression(&mut self, ctx: &ConditionalAndExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#conditionalOrExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalOrExpression(&mut self, ctx: &ConditionalOrExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#conditionalExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#assignmentExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#assignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment(&mut self, ctx: &AssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#leftHandSide}.
	 * @param ctx the parse tree
	 */
	fn visit_leftHandSide(&mut self, ctx: &LeftHandSideContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#assignmentOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#lambdaExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#lambdaParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaParameters(&mut self, ctx: &LambdaParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#lambdaParameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaParameterList(&mut self, ctx: &LambdaParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#lambdaParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaParameter(&mut self, ctx: &LambdaParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#lambdaParameterType}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaParameterType(&mut self, ctx: &LambdaParameterTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#lambdaBody}.
	 * @param ctx the parse tree
	 */
	fn visit_lambdaBody(&mut self, ctx: &LambdaBodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#switchExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_switchExpression(&mut self, ctx: &SwitchExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link Java20Parser#constantExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'input>) { self.visit_children(ctx) }


}