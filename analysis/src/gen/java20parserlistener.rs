#![allow(nonstandard_style)]
// Generated from Java20Parser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::java20parser::*;

pub trait Java20ParserListener<'input> : ParseTreeListener<'input,Java20ParserContextType>{
/**
 * Enter a parse tree produced by {@link Java20Parser#start}.
 * @param ctx the parse tree
 */
fn enter_start(&mut self, _ctx: &StartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#start}.
 * @param ctx the parse tree
 */
fn exit_start(&mut self, _ctx: &StartContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeIdentifier}.
 * @param ctx the parse tree
 */
fn enter_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeIdentifier}.
 * @param ctx the parse tree
 */
fn exit_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unqualifiedMethodIdentifier}.
 * @param ctx the parse tree
 */
fn enter_unqualifiedMethodIdentifier(&mut self, _ctx: &UnqualifiedMethodIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unqualifiedMethodIdentifier}.
 * @param ctx the parse tree
 */
fn exit_unqualifiedMethodIdentifier(&mut self, _ctx: &UnqualifiedMethodIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#numericType}.
 * @param ctx the parse tree
 */
fn enter_numericType(&mut self, _ctx: &NumericTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#numericType}.
 * @param ctx the parse tree
 */
fn exit_numericType(&mut self, _ctx: &NumericTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#integralType}.
 * @param ctx the parse tree
 */
fn enter_integralType(&mut self, _ctx: &IntegralTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#integralType}.
 * @param ctx the parse tree
 */
fn exit_integralType(&mut self, _ctx: &IntegralTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#floatingPointType}.
 * @param ctx the parse tree
 */
fn enter_floatingPointType(&mut self, _ctx: &FloatingPointTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#floatingPointType}.
 * @param ctx the parse tree
 */
fn exit_floatingPointType(&mut self, _ctx: &FloatingPointTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#referenceType}.
 * @param ctx the parse tree
 */
fn enter_referenceType(&mut self, _ctx: &ReferenceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#referenceType}.
 * @param ctx the parse tree
 */
fn exit_referenceType(&mut self, _ctx: &ReferenceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#coit}.
 * @param ctx the parse tree
 */
fn enter_coit(&mut self, _ctx: &CoitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#coit}.
 * @param ctx the parse tree
 */
fn exit_coit(&mut self, _ctx: &CoitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classOrInterfaceType}.
 * @param ctx the parse tree
 */
fn enter_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classOrInterfaceType}.
 * @param ctx the parse tree
 */
fn exit_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classType}.
 * @param ctx the parse tree
 */
fn enter_classType(&mut self, _ctx: &ClassTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classType}.
 * @param ctx the parse tree
 */
fn exit_classType(&mut self, _ctx: &ClassTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceType}.
 * @param ctx the parse tree
 */
fn enter_interfaceType(&mut self, _ctx: &InterfaceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceType}.
 * @param ctx the parse tree
 */
fn exit_interfaceType(&mut self, _ctx: &InterfaceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeVariable}.
 * @param ctx the parse tree
 */
fn enter_typeVariable(&mut self, _ctx: &TypeVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeVariable}.
 * @param ctx the parse tree
 */
fn exit_typeVariable(&mut self, _ctx: &TypeVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#arrayType}.
 * @param ctx the parse tree
 */
fn enter_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#arrayType}.
 * @param ctx the parse tree
 */
fn exit_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#dims}.
 * @param ctx the parse tree
 */
fn enter_dims(&mut self, _ctx: &DimsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#dims}.
 * @param ctx the parse tree
 */
fn exit_dims(&mut self, _ctx: &DimsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeParameter}.
 * @param ctx the parse tree
 */
fn enter_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeParameter}.
 * @param ctx the parse tree
 */
fn exit_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeParameterModifier}.
 * @param ctx the parse tree
 */
fn enter_typeParameterModifier(&mut self, _ctx: &TypeParameterModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeParameterModifier}.
 * @param ctx the parse tree
 */
fn exit_typeParameterModifier(&mut self, _ctx: &TypeParameterModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeBound}.
 * @param ctx the parse tree
 */
fn enter_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeBound}.
 * @param ctx the parse tree
 */
fn exit_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#additionalBound}.
 * @param ctx the parse tree
 */
fn enter_additionalBound(&mut self, _ctx: &AdditionalBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#additionalBound}.
 * @param ctx the parse tree
 */
fn exit_additionalBound(&mut self, _ctx: &AdditionalBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeArguments}.
 * @param ctx the parse tree
 */
fn enter_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeArguments}.
 * @param ctx the parse tree
 */
fn exit_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeArgumentList}.
 * @param ctx the parse tree
 */
fn enter_typeArgumentList(&mut self, _ctx: &TypeArgumentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeArgumentList}.
 * @param ctx the parse tree
 */
fn exit_typeArgumentList(&mut self, _ctx: &TypeArgumentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeArgument}.
 * @param ctx the parse tree
 */
fn enter_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeArgument}.
 * @param ctx the parse tree
 */
fn exit_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#wildcard}.
 * @param ctx the parse tree
 */
fn enter_wildcard(&mut self, _ctx: &WildcardContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#wildcard}.
 * @param ctx the parse tree
 */
fn exit_wildcard(&mut self, _ctx: &WildcardContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#wildcardBounds}.
 * @param ctx the parse tree
 */
fn enter_wildcardBounds(&mut self, _ctx: &WildcardBoundsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#wildcardBounds}.
 * @param ctx the parse tree
 */
fn exit_wildcardBounds(&mut self, _ctx: &WildcardBoundsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#moduleName}.
 * @param ctx the parse tree
 */
fn enter_moduleName(&mut self, _ctx: &ModuleNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#moduleName}.
 * @param ctx the parse tree
 */
fn exit_moduleName(&mut self, _ctx: &ModuleNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#packageName}.
 * @param ctx the parse tree
 */
fn enter_packageName(&mut self, _ctx: &PackageNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#packageName}.
 * @param ctx the parse tree
 */
fn exit_packageName(&mut self, _ctx: &PackageNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeName}.
 * @param ctx the parse tree
 */
fn enter_typeName(&mut self, _ctx: &TypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeName}.
 * @param ctx the parse tree
 */
fn exit_typeName(&mut self, _ctx: &TypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#packageOrTypeName}.
 * @param ctx the parse tree
 */
fn enter_packageOrTypeName(&mut self, _ctx: &PackageOrTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#packageOrTypeName}.
 * @param ctx the parse tree
 */
fn exit_packageOrTypeName(&mut self, _ctx: &PackageOrTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#expressionName}.
 * @param ctx the parse tree
 */
fn enter_expressionName(&mut self, _ctx: &ExpressionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#expressionName}.
 * @param ctx the parse tree
 */
fn exit_expressionName(&mut self, _ctx: &ExpressionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodName}.
 * @param ctx the parse tree
 */
fn enter_methodName(&mut self, _ctx: &MethodNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodName}.
 * @param ctx the parse tree
 */
fn exit_methodName(&mut self, _ctx: &MethodNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#ambiguousName}.
 * @param ctx the parse tree
 */
fn enter_ambiguousName(&mut self, _ctx: &AmbiguousNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#ambiguousName}.
 * @param ctx the parse tree
 */
fn exit_ambiguousName(&mut self, _ctx: &AmbiguousNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#compilationUnit}.
 * @param ctx the parse tree
 */
fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#compilationUnit}.
 * @param ctx the parse tree
 */
fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#ordinaryCompilationUnit}.
 * @param ctx the parse tree
 */
fn enter_ordinaryCompilationUnit(&mut self, _ctx: &OrdinaryCompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#ordinaryCompilationUnit}.
 * @param ctx the parse tree
 */
fn exit_ordinaryCompilationUnit(&mut self, _ctx: &OrdinaryCompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#modularCompilationUnit}.
 * @param ctx the parse tree
 */
fn enter_modularCompilationUnit(&mut self, _ctx: &ModularCompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#modularCompilationUnit}.
 * @param ctx the parse tree
 */
fn exit_modularCompilationUnit(&mut self, _ctx: &ModularCompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#packageDeclaration}.
 * @param ctx the parse tree
 */
fn enter_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#packageDeclaration}.
 * @param ctx the parse tree
 */
fn exit_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#packageModifier}.
 * @param ctx the parse tree
 */
fn enter_packageModifier(&mut self, _ctx: &PackageModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#packageModifier}.
 * @param ctx the parse tree
 */
fn exit_packageModifier(&mut self, _ctx: &PackageModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#importDeclaration}.
 * @param ctx the parse tree
 */
fn enter_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#importDeclaration}.
 * @param ctx the parse tree
 */
fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#singleTypeImportDeclaration}.
 * @param ctx the parse tree
 */
fn enter_singleTypeImportDeclaration(&mut self, _ctx: &SingleTypeImportDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#singleTypeImportDeclaration}.
 * @param ctx the parse tree
 */
fn exit_singleTypeImportDeclaration(&mut self, _ctx: &SingleTypeImportDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeImportOnDemandDeclaration}.
 * @param ctx the parse tree
 */
fn enter_typeImportOnDemandDeclaration(&mut self, _ctx: &TypeImportOnDemandDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeImportOnDemandDeclaration}.
 * @param ctx the parse tree
 */
fn exit_typeImportOnDemandDeclaration(&mut self, _ctx: &TypeImportOnDemandDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#singleStaticImportDeclaration}.
 * @param ctx the parse tree
 */
fn enter_singleStaticImportDeclaration(&mut self, _ctx: &SingleStaticImportDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#singleStaticImportDeclaration}.
 * @param ctx the parse tree
 */
fn exit_singleStaticImportDeclaration(&mut self, _ctx: &SingleStaticImportDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#staticImportOnDemandDeclaration}.
 * @param ctx the parse tree
 */
fn enter_staticImportOnDemandDeclaration(&mut self, _ctx: &StaticImportOnDemandDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#staticImportOnDemandDeclaration}.
 * @param ctx the parse tree
 */
fn exit_staticImportOnDemandDeclaration(&mut self, _ctx: &StaticImportOnDemandDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#topLevelClassOrInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_topLevelClassOrInterfaceDeclaration(&mut self, _ctx: &TopLevelClassOrInterfaceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#topLevelClassOrInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_topLevelClassOrInterfaceDeclaration(&mut self, _ctx: &TopLevelClassOrInterfaceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#moduleDeclaration}.
 * @param ctx the parse tree
 */
fn enter_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#moduleDeclaration}.
 * @param ctx the parse tree
 */
fn exit_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#moduleDirective}.
 * @param ctx the parse tree
 */
fn enter_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#moduleDirective}.
 * @param ctx the parse tree
 */
fn exit_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#requiresModifier}.
 * @param ctx the parse tree
 */
fn enter_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#requiresModifier}.
 * @param ctx the parse tree
 */
fn exit_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classDeclaration}.
 * @param ctx the parse tree
 */
fn enter_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classDeclaration}.
 * @param ctx the parse tree
 */
fn exit_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#normalClassDeclaration}.
 * @param ctx the parse tree
 */
fn enter_normalClassDeclaration(&mut self, _ctx: &NormalClassDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#normalClassDeclaration}.
 * @param ctx the parse tree
 */
fn exit_normalClassDeclaration(&mut self, _ctx: &NormalClassDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classModifier}.
 * @param ctx the parse tree
 */
fn enter_classModifier(&mut self, _ctx: &ClassModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classModifier}.
 * @param ctx the parse tree
 */
fn exit_classModifier(&mut self, _ctx: &ClassModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeParameters}.
 * @param ctx the parse tree
 */
fn enter_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeParameters}.
 * @param ctx the parse tree
 */
fn exit_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeParameterList}.
 * @param ctx the parse tree
 */
fn enter_typeParameterList(&mut self, _ctx: &TypeParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeParameterList}.
 * @param ctx the parse tree
 */
fn exit_typeParameterList(&mut self, _ctx: &TypeParameterListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classExtends}.
 * @param ctx the parse tree
 */
fn enter_classExtends(&mut self, _ctx: &ClassExtendsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classExtends}.
 * @param ctx the parse tree
 */
fn exit_classExtends(&mut self, _ctx: &ClassExtendsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classImplements}.
 * @param ctx the parse tree
 */
fn enter_classImplements(&mut self, _ctx: &ClassImplementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classImplements}.
 * @param ctx the parse tree
 */
fn exit_classImplements(&mut self, _ctx: &ClassImplementsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceTypeList}.
 * @param ctx the parse tree
 */
fn enter_interfaceTypeList(&mut self, _ctx: &InterfaceTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceTypeList}.
 * @param ctx the parse tree
 */
fn exit_interfaceTypeList(&mut self, _ctx: &InterfaceTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classPermits}.
 * @param ctx the parse tree
 */
fn enter_classPermits(&mut self, _ctx: &ClassPermitsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classPermits}.
 * @param ctx the parse tree
 */
fn exit_classPermits(&mut self, _ctx: &ClassPermitsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classBody}.
 * @param ctx the parse tree
 */
fn enter_classBody(&mut self, _ctx: &ClassBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classBody}.
 * @param ctx the parse tree
 */
fn exit_classBody(&mut self, _ctx: &ClassBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classBodyDeclaration}.
 * @param ctx the parse tree
 */
fn enter_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classBodyDeclaration}.
 * @param ctx the parse tree
 */
fn exit_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classMemberDeclaration}.
 * @param ctx the parse tree
 */
fn enter_classMemberDeclaration(&mut self, _ctx: &ClassMemberDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classMemberDeclaration}.
 * @param ctx the parse tree
 */
fn exit_classMemberDeclaration(&mut self, _ctx: &ClassMemberDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#fieldDeclaration}.
 * @param ctx the parse tree
 */
fn enter_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#fieldDeclaration}.
 * @param ctx the parse tree
 */
fn exit_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#fieldModifier}.
 * @param ctx the parse tree
 */
fn enter_fieldModifier(&mut self, _ctx: &FieldModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#fieldModifier}.
 * @param ctx the parse tree
 */
fn exit_fieldModifier(&mut self, _ctx: &FieldModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableDeclaratorList}.
 * @param ctx the parse tree
 */
fn enter_variableDeclaratorList(&mut self, _ctx: &VariableDeclaratorListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableDeclaratorList}.
 * @param ctx the parse tree
 */
fn exit_variableDeclaratorList(&mut self, _ctx: &VariableDeclaratorListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableDeclarator}.
 * @param ctx the parse tree
 */
fn enter_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableDeclarator}.
 * @param ctx the parse tree
 */
fn exit_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableDeclaratorId}.
 * @param ctx the parse tree
 */
fn enter_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableDeclaratorId}.
 * @param ctx the parse tree
 */
fn exit_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableInitializer}.
 * @param ctx the parse tree
 */
fn enter_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableInitializer}.
 * @param ctx the parse tree
 */
fn exit_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannType}.
 * @param ctx the parse tree
 */
fn enter_unannType(&mut self, _ctx: &UnannTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannType}.
 * @param ctx the parse tree
 */
fn exit_unannType(&mut self, _ctx: &UnannTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannPrimitiveType}.
 * @param ctx the parse tree
 */
fn enter_unannPrimitiveType(&mut self, _ctx: &UnannPrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannPrimitiveType}.
 * @param ctx the parse tree
 */
fn exit_unannPrimitiveType(&mut self, _ctx: &UnannPrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannReferenceType}.
 * @param ctx the parse tree
 */
fn enter_unannReferenceType(&mut self, _ctx: &UnannReferenceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannReferenceType}.
 * @param ctx the parse tree
 */
fn exit_unannReferenceType(&mut self, _ctx: &UnannReferenceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannClassOrInterfaceType}.
 * @param ctx the parse tree
 */
fn enter_unannClassOrInterfaceType(&mut self, _ctx: &UnannClassOrInterfaceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannClassOrInterfaceType}.
 * @param ctx the parse tree
 */
fn exit_unannClassOrInterfaceType(&mut self, _ctx: &UnannClassOrInterfaceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#uCOIT}.
 * @param ctx the parse tree
 */
fn enter_uCOIT(&mut self, _ctx: &UCOITContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#uCOIT}.
 * @param ctx the parse tree
 */
fn exit_uCOIT(&mut self, _ctx: &UCOITContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannClassType}.
 * @param ctx the parse tree
 */
fn enter_unannClassType(&mut self, _ctx: &UnannClassTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannClassType}.
 * @param ctx the parse tree
 */
fn exit_unannClassType(&mut self, _ctx: &UnannClassTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannInterfaceType}.
 * @param ctx the parse tree
 */
fn enter_unannInterfaceType(&mut self, _ctx: &UnannInterfaceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannInterfaceType}.
 * @param ctx the parse tree
 */
fn exit_unannInterfaceType(&mut self, _ctx: &UnannInterfaceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannTypeVariable}.
 * @param ctx the parse tree
 */
fn enter_unannTypeVariable(&mut self, _ctx: &UnannTypeVariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannTypeVariable}.
 * @param ctx the parse tree
 */
fn exit_unannTypeVariable(&mut self, _ctx: &UnannTypeVariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unannArrayType}.
 * @param ctx the parse tree
 */
fn enter_unannArrayType(&mut self, _ctx: &UnannArrayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unannArrayType}.
 * @param ctx the parse tree
 */
fn exit_unannArrayType(&mut self, _ctx: &UnannArrayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodModifier}.
 * @param ctx the parse tree
 */
fn enter_methodModifier(&mut self, _ctx: &MethodModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodModifier}.
 * @param ctx the parse tree
 */
fn exit_methodModifier(&mut self, _ctx: &MethodModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodHeader}.
 * @param ctx the parse tree
 */
fn enter_methodHeader(&mut self, _ctx: &MethodHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodHeader}.
 * @param ctx the parse tree
 */
fn exit_methodHeader(&mut self, _ctx: &MethodHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#result}.
 * @param ctx the parse tree
 */
fn enter_result(&mut self, _ctx: &ResultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#result}.
 * @param ctx the parse tree
 */
fn exit_result(&mut self, _ctx: &ResultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodDeclarator}.
 * @param ctx the parse tree
 */
fn enter_methodDeclarator(&mut self, _ctx: &MethodDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodDeclarator}.
 * @param ctx the parse tree
 */
fn exit_methodDeclarator(&mut self, _ctx: &MethodDeclaratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#receiverParameter}.
 * @param ctx the parse tree
 */
fn enter_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#receiverParameter}.
 * @param ctx the parse tree
 */
fn exit_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#formalParameterList}.
 * @param ctx the parse tree
 */
fn enter_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#formalParameterList}.
 * @param ctx the parse tree
 */
fn exit_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#formalParameter}.
 * @param ctx the parse tree
 */
fn enter_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#formalParameter}.
 * @param ctx the parse tree
 */
fn exit_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableArityParameter}.
 * @param ctx the parse tree
 */
fn enter_variableArityParameter(&mut self, _ctx: &VariableArityParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableArityParameter}.
 * @param ctx the parse tree
 */
fn exit_variableArityParameter(&mut self, _ctx: &VariableArityParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableModifier}.
 * @param ctx the parse tree
 */
fn enter_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableModifier}.
 * @param ctx the parse tree
 */
fn exit_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#throwsT}.
 * @param ctx the parse tree
 */
fn enter_throwsT(&mut self, _ctx: &ThrowsTContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#throwsT}.
 * @param ctx the parse tree
 */
fn exit_throwsT(&mut self, _ctx: &ThrowsTContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#exceptionTypeList}.
 * @param ctx the parse tree
 */
fn enter_exceptionTypeList(&mut self, _ctx: &ExceptionTypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#exceptionTypeList}.
 * @param ctx the parse tree
 */
fn exit_exceptionTypeList(&mut self, _ctx: &ExceptionTypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#exceptionType}.
 * @param ctx the parse tree
 */
fn enter_exceptionType(&mut self, _ctx: &ExceptionTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#exceptionType}.
 * @param ctx the parse tree
 */
fn exit_exceptionType(&mut self, _ctx: &ExceptionTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodBody}.
 * @param ctx the parse tree
 */
fn enter_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodBody}.
 * @param ctx the parse tree
 */
fn exit_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#instanceInitializer}.
 * @param ctx the parse tree
 */
fn enter_instanceInitializer(&mut self, _ctx: &InstanceInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#instanceInitializer}.
 * @param ctx the parse tree
 */
fn exit_instanceInitializer(&mut self, _ctx: &InstanceInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#staticInitializer}.
 * @param ctx the parse tree
 */
fn enter_staticInitializer(&mut self, _ctx: &StaticInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#staticInitializer}.
 * @param ctx the parse tree
 */
fn exit_staticInitializer(&mut self, _ctx: &StaticInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constructorDeclaration}.
 * @param ctx the parse tree
 */
fn enter_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constructorDeclaration}.
 * @param ctx the parse tree
 */
fn exit_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constructorModifier}.
 * @param ctx the parse tree
 */
fn enter_constructorModifier(&mut self, _ctx: &ConstructorModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constructorModifier}.
 * @param ctx the parse tree
 */
fn exit_constructorModifier(&mut self, _ctx: &ConstructorModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constructorDeclarator}.
 * @param ctx the parse tree
 */
fn enter_constructorDeclarator(&mut self, _ctx: &ConstructorDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constructorDeclarator}.
 * @param ctx the parse tree
 */
fn exit_constructorDeclarator(&mut self, _ctx: &ConstructorDeclaratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#simpleTypeName}.
 * @param ctx the parse tree
 */
fn enter_simpleTypeName(&mut self, _ctx: &SimpleTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#simpleTypeName}.
 * @param ctx the parse tree
 */
fn exit_simpleTypeName(&mut self, _ctx: &SimpleTypeNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constructorBody}.
 * @param ctx the parse tree
 */
fn enter_constructorBody(&mut self, _ctx: &ConstructorBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constructorBody}.
 * @param ctx the parse tree
 */
fn exit_constructorBody(&mut self, _ctx: &ConstructorBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#explicitConstructorInvocation}.
 * @param ctx the parse tree
 */
fn enter_explicitConstructorInvocation(&mut self, _ctx: &ExplicitConstructorInvocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#explicitConstructorInvocation}.
 * @param ctx the parse tree
 */
fn exit_explicitConstructorInvocation(&mut self, _ctx: &ExplicitConstructorInvocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enumDeclaration}.
 * @param ctx the parse tree
 */
fn enter_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enumDeclaration}.
 * @param ctx the parse tree
 */
fn exit_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enumBody}.
 * @param ctx the parse tree
 */
fn enter_enumBody(&mut self, _ctx: &EnumBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enumBody}.
 * @param ctx the parse tree
 */
fn exit_enumBody(&mut self, _ctx: &EnumBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enumConstantList}.
 * @param ctx the parse tree
 */
fn enter_enumConstantList(&mut self, _ctx: &EnumConstantListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enumConstantList}.
 * @param ctx the parse tree
 */
fn exit_enumConstantList(&mut self, _ctx: &EnumConstantListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enumConstant}.
 * @param ctx the parse tree
 */
fn enter_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enumConstant}.
 * @param ctx the parse tree
 */
fn exit_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enumConstantModifier}.
 * @param ctx the parse tree
 */
fn enter_enumConstantModifier(&mut self, _ctx: &EnumConstantModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enumConstantModifier}.
 * @param ctx the parse tree
 */
fn exit_enumConstantModifier(&mut self, _ctx: &EnumConstantModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enumBodyDeclarations}.
 * @param ctx the parse tree
 */
fn enter_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enumBodyDeclarations}.
 * @param ctx the parse tree
 */
fn exit_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordDeclaration}.
 * @param ctx the parse tree
 */
fn enter_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordDeclaration}.
 * @param ctx the parse tree
 */
fn exit_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordHeader}.
 * @param ctx the parse tree
 */
fn enter_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordHeader}.
 * @param ctx the parse tree
 */
fn exit_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordComponentList}.
 * @param ctx the parse tree
 */
fn enter_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordComponentList}.
 * @param ctx the parse tree
 */
fn exit_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordComponent}.
 * @param ctx the parse tree
 */
fn enter_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordComponent}.
 * @param ctx the parse tree
 */
fn exit_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableArityRecordComponent}.
 * @param ctx the parse tree
 */
fn enter_variableArityRecordComponent(&mut self, _ctx: &VariableArityRecordComponentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableArityRecordComponent}.
 * @param ctx the parse tree
 */
fn exit_variableArityRecordComponent(&mut self, _ctx: &VariableArityRecordComponentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordComponentModifier}.
 * @param ctx the parse tree
 */
fn enter_recordComponentModifier(&mut self, _ctx: &RecordComponentModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordComponentModifier}.
 * @param ctx the parse tree
 */
fn exit_recordComponentModifier(&mut self, _ctx: &RecordComponentModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordBody}.
 * @param ctx the parse tree
 */
fn enter_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordBody}.
 * @param ctx the parse tree
 */
fn exit_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#recordBodyDeclaration}.
 * @param ctx the parse tree
 */
fn enter_recordBodyDeclaration(&mut self, _ctx: &RecordBodyDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#recordBodyDeclaration}.
 * @param ctx the parse tree
 */
fn exit_recordBodyDeclaration(&mut self, _ctx: &RecordBodyDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#compactConstructorDeclaration}.
 * @param ctx the parse tree
 */
fn enter_compactConstructorDeclaration(&mut self, _ctx: &CompactConstructorDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#compactConstructorDeclaration}.
 * @param ctx the parse tree
 */
fn exit_compactConstructorDeclaration(&mut self, _ctx: &CompactConstructorDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#normalInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_normalInterfaceDeclaration(&mut self, _ctx: &NormalInterfaceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#normalInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_normalInterfaceDeclaration(&mut self, _ctx: &NormalInterfaceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceModifier}.
 * @param ctx the parse tree
 */
fn enter_interfaceModifier(&mut self, _ctx: &InterfaceModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceModifier}.
 * @param ctx the parse tree
 */
fn exit_interfaceModifier(&mut self, _ctx: &InterfaceModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceExtends}.
 * @param ctx the parse tree
 */
fn enter_interfaceExtends(&mut self, _ctx: &InterfaceExtendsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceExtends}.
 * @param ctx the parse tree
 */
fn exit_interfaceExtends(&mut self, _ctx: &InterfaceExtendsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfacePermits}.
 * @param ctx the parse tree
 */
fn enter_interfacePermits(&mut self, _ctx: &InterfacePermitsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfacePermits}.
 * @param ctx the parse tree
 */
fn exit_interfacePermits(&mut self, _ctx: &InterfacePermitsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceBody}.
 * @param ctx the parse tree
 */
fn enter_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceBody}.
 * @param ctx the parse tree
 */
fn exit_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceMemberDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceMemberDeclaration(&mut self, _ctx: &InterfaceMemberDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceMemberDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceMemberDeclaration(&mut self, _ctx: &InterfaceMemberDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constantDeclaration}.
 * @param ctx the parse tree
 */
fn enter_constantDeclaration(&mut self, _ctx: &ConstantDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constantDeclaration}.
 * @param ctx the parse tree
 */
fn exit_constantDeclaration(&mut self, _ctx: &ConstantDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constantModifier}.
 * @param ctx the parse tree
 */
fn enter_constantModifier(&mut self, _ctx: &ConstantModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constantModifier}.
 * @param ctx the parse tree
 */
fn exit_constantModifier(&mut self, _ctx: &ConstantModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceMethodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceMethodDeclaration(&mut self, _ctx: &InterfaceMethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceMethodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceMethodDeclaration(&mut self, _ctx: &InterfaceMethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#interfaceMethodModifier}.
 * @param ctx the parse tree
 */
fn enter_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#interfaceMethodModifier}.
 * @param ctx the parse tree
 */
fn exit_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#annotationInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_annotationInterfaceDeclaration(&mut self, _ctx: &AnnotationInterfaceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#annotationInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_annotationInterfaceDeclaration(&mut self, _ctx: &AnnotationInterfaceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#annotationInterfaceBody}.
 * @param ctx the parse tree
 */
fn enter_annotationInterfaceBody(&mut self, _ctx: &AnnotationInterfaceBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#annotationInterfaceBody}.
 * @param ctx the parse tree
 */
fn exit_annotationInterfaceBody(&mut self, _ctx: &AnnotationInterfaceBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#annotationInterfaceMemberDeclaration}.
 * @param ctx the parse tree
 */
fn enter_annotationInterfaceMemberDeclaration(&mut self, _ctx: &AnnotationInterfaceMemberDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#annotationInterfaceMemberDeclaration}.
 * @param ctx the parse tree
 */
fn exit_annotationInterfaceMemberDeclaration(&mut self, _ctx: &AnnotationInterfaceMemberDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#annotationInterfaceElementDeclaration}.
 * @param ctx the parse tree
 */
fn enter_annotationInterfaceElementDeclaration(&mut self, _ctx: &AnnotationInterfaceElementDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#annotationInterfaceElementDeclaration}.
 * @param ctx the parse tree
 */
fn exit_annotationInterfaceElementDeclaration(&mut self, _ctx: &AnnotationInterfaceElementDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#annotationInterfaceElementModifier}.
 * @param ctx the parse tree
 */
fn enter_annotationInterfaceElementModifier(&mut self, _ctx: &AnnotationInterfaceElementModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#annotationInterfaceElementModifier}.
 * @param ctx the parse tree
 */
fn exit_annotationInterfaceElementModifier(&mut self, _ctx: &AnnotationInterfaceElementModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#defaultValue}.
 * @param ctx the parse tree
 */
fn enter_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#defaultValue}.
 * @param ctx the parse tree
 */
fn exit_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#annotation}.
 * @param ctx the parse tree
 */
fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#annotation}.
 * @param ctx the parse tree
 */
fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#normalAnnotation}.
 * @param ctx the parse tree
 */
fn enter_normalAnnotation(&mut self, _ctx: &NormalAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#normalAnnotation}.
 * @param ctx the parse tree
 */
fn exit_normalAnnotation(&mut self, _ctx: &NormalAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#elementValuePairList}.
 * @param ctx the parse tree
 */
fn enter_elementValuePairList(&mut self, _ctx: &ElementValuePairListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#elementValuePairList}.
 * @param ctx the parse tree
 */
fn exit_elementValuePairList(&mut self, _ctx: &ElementValuePairListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#elementValuePair}.
 * @param ctx the parse tree
 */
fn enter_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#elementValuePair}.
 * @param ctx the parse tree
 */
fn exit_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#elementValue}.
 * @param ctx the parse tree
 */
fn enter_elementValue(&mut self, _ctx: &ElementValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#elementValue}.
 * @param ctx the parse tree
 */
fn exit_elementValue(&mut self, _ctx: &ElementValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#elementValueArrayInitializer}.
 * @param ctx the parse tree
 */
fn enter_elementValueArrayInitializer(&mut self, _ctx: &ElementValueArrayInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#elementValueArrayInitializer}.
 * @param ctx the parse tree
 */
fn exit_elementValueArrayInitializer(&mut self, _ctx: &ElementValueArrayInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#elementValueList}.
 * @param ctx the parse tree
 */
fn enter_elementValueList(&mut self, _ctx: &ElementValueListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#elementValueList}.
 * @param ctx the parse tree
 */
fn exit_elementValueList(&mut self, _ctx: &ElementValueListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#markerAnnotation}.
 * @param ctx the parse tree
 */
fn enter_markerAnnotation(&mut self, _ctx: &MarkerAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#markerAnnotation}.
 * @param ctx the parse tree
 */
fn exit_markerAnnotation(&mut self, _ctx: &MarkerAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#singleElementAnnotation}.
 * @param ctx the parse tree
 */
fn enter_singleElementAnnotation(&mut self, _ctx: &SingleElementAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#singleElementAnnotation}.
 * @param ctx the parse tree
 */
fn exit_singleElementAnnotation(&mut self, _ctx: &SingleElementAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#arrayInitializer}.
 * @param ctx the parse tree
 */
fn enter_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#arrayInitializer}.
 * @param ctx the parse tree
 */
fn exit_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableInitializerList}.
 * @param ctx the parse tree
 */
fn enter_variableInitializerList(&mut self, _ctx: &VariableInitializerListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableInitializerList}.
 * @param ctx the parse tree
 */
fn exit_variableInitializerList(&mut self, _ctx: &VariableInitializerListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#blockStatements}.
 * @param ctx the parse tree
 */
fn enter_blockStatements(&mut self, _ctx: &BlockStatementsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#blockStatements}.
 * @param ctx the parse tree
 */
fn exit_blockStatements(&mut self, _ctx: &BlockStatementsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#blockStatement}.
 * @param ctx the parse tree
 */
fn enter_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#blockStatement}.
 * @param ctx the parse tree
 */
fn exit_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#localClassOrInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_localClassOrInterfaceDeclaration(&mut self, _ctx: &LocalClassOrInterfaceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#localClassOrInterfaceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_localClassOrInterfaceDeclaration(&mut self, _ctx: &LocalClassOrInterfaceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#localVariableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#localVariableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#localVariableType}.
 * @param ctx the parse tree
 */
fn enter_localVariableType(&mut self, _ctx: &LocalVariableTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#localVariableType}.
 * @param ctx the parse tree
 */
fn exit_localVariableType(&mut self, _ctx: &LocalVariableTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#localVariableDeclarationStatement}.
 * @param ctx the parse tree
 */
fn enter_localVariableDeclarationStatement(&mut self, _ctx: &LocalVariableDeclarationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#localVariableDeclarationStatement}.
 * @param ctx the parse tree
 */
fn exit_localVariableDeclarationStatement(&mut self, _ctx: &LocalVariableDeclarationStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#statementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_statementNoShortIf(&mut self, _ctx: &StatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#statementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_statementNoShortIf(&mut self, _ctx: &StatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#statementWithoutTrailingSubstatement}.
 * @param ctx the parse tree
 */
fn enter_statementWithoutTrailingSubstatement(&mut self, _ctx: &StatementWithoutTrailingSubstatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#statementWithoutTrailingSubstatement}.
 * @param ctx the parse tree
 */
fn exit_statementWithoutTrailingSubstatement(&mut self, _ctx: &StatementWithoutTrailingSubstatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#emptyStatement}.
 * @param ctx the parse tree
 */
fn enter_emptyStatement(&mut self, _ctx: &EmptyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#emptyStatement}.
 * @param ctx the parse tree
 */
fn exit_emptyStatement(&mut self, _ctx: &EmptyStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#labeledStatement}.
 * @param ctx the parse tree
 */
fn enter_labeledStatement(&mut self, _ctx: &LabeledStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#labeledStatement}.
 * @param ctx the parse tree
 */
fn exit_labeledStatement(&mut self, _ctx: &LabeledStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#labeledStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_labeledStatementNoShortIf(&mut self, _ctx: &LabeledStatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#labeledStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_labeledStatementNoShortIf(&mut self, _ctx: &LabeledStatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#expressionStatement}.
 * @param ctx the parse tree
 */
fn enter_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#expressionStatement}.
 * @param ctx the parse tree
 */
fn exit_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#statementExpression}.
 * @param ctx the parse tree
 */
fn enter_statementExpression(&mut self, _ctx: &StatementExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#statementExpression}.
 * @param ctx the parse tree
 */
fn exit_statementExpression(&mut self, _ctx: &StatementExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#ifThenStatement}.
 * @param ctx the parse tree
 */
fn enter_ifThenStatement(&mut self, _ctx: &IfThenStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#ifThenStatement}.
 * @param ctx the parse tree
 */
fn exit_ifThenStatement(&mut self, _ctx: &IfThenStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#ifThenElseStatement}.
 * @param ctx the parse tree
 */
fn enter_ifThenElseStatement(&mut self, _ctx: &IfThenElseStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#ifThenElseStatement}.
 * @param ctx the parse tree
 */
fn exit_ifThenElseStatement(&mut self, _ctx: &IfThenElseStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#ifThenElseStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_ifThenElseStatementNoShortIf(&mut self, _ctx: &IfThenElseStatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#ifThenElseStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_ifThenElseStatementNoShortIf(&mut self, _ctx: &IfThenElseStatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#assertStatement}.
 * @param ctx the parse tree
 */
fn enter_assertStatement(&mut self, _ctx: &AssertStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#assertStatement}.
 * @param ctx the parse tree
 */
fn exit_assertStatement(&mut self, _ctx: &AssertStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#switchStatement}.
 * @param ctx the parse tree
 */
fn enter_switchStatement(&mut self, _ctx: &SwitchStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#switchStatement}.
 * @param ctx the parse tree
 */
fn exit_switchStatement(&mut self, _ctx: &SwitchStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#switchBlock}.
 * @param ctx the parse tree
 */
fn enter_switchBlock(&mut self, _ctx: &SwitchBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#switchBlock}.
 * @param ctx the parse tree
 */
fn exit_switchBlock(&mut self, _ctx: &SwitchBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#switchRule}.
 * @param ctx the parse tree
 */
fn enter_switchRule(&mut self, _ctx: &SwitchRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#switchRule}.
 * @param ctx the parse tree
 */
fn exit_switchRule(&mut self, _ctx: &SwitchRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#switchBlockStatementGroup}.
 * @param ctx the parse tree
 */
fn enter_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#switchBlockStatementGroup}.
 * @param ctx the parse tree
 */
fn exit_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#switchLabel}.
 * @param ctx the parse tree
 */
fn enter_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#switchLabel}.
 * @param ctx the parse tree
 */
fn exit_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#caseConstant}.
 * @param ctx the parse tree
 */
fn enter_caseConstant(&mut self, _ctx: &CaseConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#caseConstant}.
 * @param ctx the parse tree
 */
fn exit_caseConstant(&mut self, _ctx: &CaseConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#whileStatement}.
 * @param ctx the parse tree
 */
fn enter_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#whileStatement}.
 * @param ctx the parse tree
 */
fn exit_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#whileStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_whileStatementNoShortIf(&mut self, _ctx: &WhileStatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#whileStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_whileStatementNoShortIf(&mut self, _ctx: &WhileStatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#doStatement}.
 * @param ctx the parse tree
 */
fn enter_doStatement(&mut self, _ctx: &DoStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#doStatement}.
 * @param ctx the parse tree
 */
fn exit_doStatement(&mut self, _ctx: &DoStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#forStatement}.
 * @param ctx the parse tree
 */
fn enter_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#forStatement}.
 * @param ctx the parse tree
 */
fn exit_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#forStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_forStatementNoShortIf(&mut self, _ctx: &ForStatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#forStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_forStatementNoShortIf(&mut self, _ctx: &ForStatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#basicForStatement}.
 * @param ctx the parse tree
 */
fn enter_basicForStatement(&mut self, _ctx: &BasicForStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#basicForStatement}.
 * @param ctx the parse tree
 */
fn exit_basicForStatement(&mut self, _ctx: &BasicForStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#basicForStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_basicForStatementNoShortIf(&mut self, _ctx: &BasicForStatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#basicForStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_basicForStatementNoShortIf(&mut self, _ctx: &BasicForStatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#forInit}.
 * @param ctx the parse tree
 */
fn enter_forInit(&mut self, _ctx: &ForInitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#forInit}.
 * @param ctx the parse tree
 */
fn exit_forInit(&mut self, _ctx: &ForInitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#forUpdate}.
 * @param ctx the parse tree
 */
fn enter_forUpdate(&mut self, _ctx: &ForUpdateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#forUpdate}.
 * @param ctx the parse tree
 */
fn exit_forUpdate(&mut self, _ctx: &ForUpdateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#statementExpressionList}.
 * @param ctx the parse tree
 */
fn enter_statementExpressionList(&mut self, _ctx: &StatementExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#statementExpressionList}.
 * @param ctx the parse tree
 */
fn exit_statementExpressionList(&mut self, _ctx: &StatementExpressionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enhancedForStatement}.
 * @param ctx the parse tree
 */
fn enter_enhancedForStatement(&mut self, _ctx: &EnhancedForStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enhancedForStatement}.
 * @param ctx the parse tree
 */
fn exit_enhancedForStatement(&mut self, _ctx: &EnhancedForStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#enhancedForStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn enter_enhancedForStatementNoShortIf(&mut self, _ctx: &EnhancedForStatementNoShortIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#enhancedForStatementNoShortIf}.
 * @param ctx the parse tree
 */
fn exit_enhancedForStatementNoShortIf(&mut self, _ctx: &EnhancedForStatementNoShortIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#breakStatement}.
 * @param ctx the parse tree
 */
fn enter_breakStatement(&mut self, _ctx: &BreakStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#breakStatement}.
 * @param ctx the parse tree
 */
fn exit_breakStatement(&mut self, _ctx: &BreakStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#continueStatement}.
 * @param ctx the parse tree
 */
fn enter_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#continueStatement}.
 * @param ctx the parse tree
 */
fn exit_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#returnStatement}.
 * @param ctx the parse tree
 */
fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#returnStatement}.
 * @param ctx the parse tree
 */
fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#throwStatement}.
 * @param ctx the parse tree
 */
fn enter_throwStatement(&mut self, _ctx: &ThrowStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#throwStatement}.
 * @param ctx the parse tree
 */
fn exit_throwStatement(&mut self, _ctx: &ThrowStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#synchronizedStatement}.
 * @param ctx the parse tree
 */
fn enter_synchronizedStatement(&mut self, _ctx: &SynchronizedStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#synchronizedStatement}.
 * @param ctx the parse tree
 */
fn exit_synchronizedStatement(&mut self, _ctx: &SynchronizedStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#tryStatement}.
 * @param ctx the parse tree
 */
fn enter_tryStatement(&mut self, _ctx: &TryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#tryStatement}.
 * @param ctx the parse tree
 */
fn exit_tryStatement(&mut self, _ctx: &TryStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#catches}.
 * @param ctx the parse tree
 */
fn enter_catches(&mut self, _ctx: &CatchesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#catches}.
 * @param ctx the parse tree
 */
fn exit_catches(&mut self, _ctx: &CatchesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#catchClause}.
 * @param ctx the parse tree
 */
fn enter_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#catchClause}.
 * @param ctx the parse tree
 */
fn exit_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#catchFormalParameter}.
 * @param ctx the parse tree
 */
fn enter_catchFormalParameter(&mut self, _ctx: &CatchFormalParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#catchFormalParameter}.
 * @param ctx the parse tree
 */
fn exit_catchFormalParameter(&mut self, _ctx: &CatchFormalParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#catchType}.
 * @param ctx the parse tree
 */
fn enter_catchType(&mut self, _ctx: &CatchTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#catchType}.
 * @param ctx the parse tree
 */
fn exit_catchType(&mut self, _ctx: &CatchTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#finallyBlock}.
 * @param ctx the parse tree
 */
fn enter_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#finallyBlock}.
 * @param ctx the parse tree
 */
fn exit_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#tryWithResourcesStatement}.
 * @param ctx the parse tree
 */
fn enter_tryWithResourcesStatement(&mut self, _ctx: &TryWithResourcesStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#tryWithResourcesStatement}.
 * @param ctx the parse tree
 */
fn exit_tryWithResourcesStatement(&mut self, _ctx: &TryWithResourcesStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#resourceSpecification}.
 * @param ctx the parse tree
 */
fn enter_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#resourceSpecification}.
 * @param ctx the parse tree
 */
fn exit_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#resourceList}.
 * @param ctx the parse tree
 */
fn enter_resourceList(&mut self, _ctx: &ResourceListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#resourceList}.
 * @param ctx the parse tree
 */
fn exit_resourceList(&mut self, _ctx: &ResourceListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#resource}.
 * @param ctx the parse tree
 */
fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#resource}.
 * @param ctx the parse tree
 */
fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#variableAccess}.
 * @param ctx the parse tree
 */
fn enter_variableAccess(&mut self, _ctx: &VariableAccessContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#variableAccess}.
 * @param ctx the parse tree
 */
fn exit_variableAccess(&mut self, _ctx: &VariableAccessContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#yieldStatement}.
 * @param ctx the parse tree
 */
fn enter_yieldStatement(&mut self, _ctx: &YieldStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#yieldStatement}.
 * @param ctx the parse tree
 */
fn exit_yieldStatement(&mut self, _ctx: &YieldStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typePattern}.
 * @param ctx the parse tree
 */
fn enter_typePattern(&mut self, _ctx: &TypePatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typePattern}.
 * @param ctx the parse tree
 */
fn exit_typePattern(&mut self, _ctx: &TypePatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#primaryNoNewArray}.
 * @param ctx the parse tree
 */
fn enter_primaryNoNewArray(&mut self, _ctx: &PrimaryNoNewArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#primaryNoNewArray}.
 * @param ctx the parse tree
 */
fn exit_primaryNoNewArray(&mut self, _ctx: &PrimaryNoNewArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#pNNA}.
 * @param ctx the parse tree
 */
fn enter_pNNA(&mut self, _ctx: &PNNAContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#pNNA}.
 * @param ctx the parse tree
 */
fn exit_pNNA(&mut self, _ctx: &PNNAContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classLiteral}.
 * @param ctx the parse tree
 */
fn enter_classLiteral(&mut self, _ctx: &ClassLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classLiteral}.
 * @param ctx the parse tree
 */
fn exit_classLiteral(&mut self, _ctx: &ClassLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classInstanceCreationExpression}.
 * @param ctx the parse tree
 */
fn enter_classInstanceCreationExpression(&mut self, _ctx: &ClassInstanceCreationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classInstanceCreationExpression}.
 * @param ctx the parse tree
 */
fn exit_classInstanceCreationExpression(&mut self, _ctx: &ClassInstanceCreationExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unqualifiedClassInstanceCreationExpression}.
 * @param ctx the parse tree
 */
fn enter_unqualifiedClassInstanceCreationExpression(&mut self, _ctx: &UnqualifiedClassInstanceCreationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unqualifiedClassInstanceCreationExpression}.
 * @param ctx the parse tree
 */
fn exit_unqualifiedClassInstanceCreationExpression(&mut self, _ctx: &UnqualifiedClassInstanceCreationExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#classOrInterfaceTypeToInstantiate}.
 * @param ctx the parse tree
 */
fn enter_classOrInterfaceTypeToInstantiate(&mut self, _ctx: &ClassOrInterfaceTypeToInstantiateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#classOrInterfaceTypeToInstantiate}.
 * @param ctx the parse tree
 */
fn exit_classOrInterfaceTypeToInstantiate(&mut self, _ctx: &ClassOrInterfaceTypeToInstantiateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#typeArgumentsOrDiamond}.
 * @param ctx the parse tree
 */
fn enter_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#typeArgumentsOrDiamond}.
 * @param ctx the parse tree
 */
fn exit_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#arrayCreationExpression}.
 * @param ctx the parse tree
 */
fn enter_arrayCreationExpression(&mut self, _ctx: &ArrayCreationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#arrayCreationExpression}.
 * @param ctx the parse tree
 */
fn exit_arrayCreationExpression(&mut self, _ctx: &ArrayCreationExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#arrayCreationExpressionWithoutInitializer}.
 * @param ctx the parse tree
 */
fn enter_arrayCreationExpressionWithoutInitializer(&mut self, _ctx: &ArrayCreationExpressionWithoutInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#arrayCreationExpressionWithoutInitializer}.
 * @param ctx the parse tree
 */
fn exit_arrayCreationExpressionWithoutInitializer(&mut self, _ctx: &ArrayCreationExpressionWithoutInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#arrayCreationExpressionWithInitializer}.
 * @param ctx the parse tree
 */
fn enter_arrayCreationExpressionWithInitializer(&mut self, _ctx: &ArrayCreationExpressionWithInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#arrayCreationExpressionWithInitializer}.
 * @param ctx the parse tree
 */
fn exit_arrayCreationExpressionWithInitializer(&mut self, _ctx: &ArrayCreationExpressionWithInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#dimExprs}.
 * @param ctx the parse tree
 */
fn enter_dimExprs(&mut self, _ctx: &DimExprsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#dimExprs}.
 * @param ctx the parse tree
 */
fn exit_dimExprs(&mut self, _ctx: &DimExprsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#dimExpr}.
 * @param ctx the parse tree
 */
fn enter_dimExpr(&mut self, _ctx: &DimExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#dimExpr}.
 * @param ctx the parse tree
 */
fn exit_dimExpr(&mut self, _ctx: &DimExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#arrayAccess}.
 * @param ctx the parse tree
 */
fn enter_arrayAccess(&mut self, _ctx: &ArrayAccessContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#arrayAccess}.
 * @param ctx the parse tree
 */
fn exit_arrayAccess(&mut self, _ctx: &ArrayAccessContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#fieldAccess}.
 * @param ctx the parse tree
 */
fn enter_fieldAccess(&mut self, _ctx: &FieldAccessContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#fieldAccess}.
 * @param ctx the parse tree
 */
fn exit_fieldAccess(&mut self, _ctx: &FieldAccessContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodInvocation}.
 * @param ctx the parse tree
 */
fn enter_methodInvocation(&mut self, _ctx: &MethodInvocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodInvocation}.
 * @param ctx the parse tree
 */
fn exit_methodInvocation(&mut self, _ctx: &MethodInvocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#argumentList}.
 * @param ctx the parse tree
 */
fn enter_argumentList(&mut self, _ctx: &ArgumentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#argumentList}.
 * @param ctx the parse tree
 */
fn exit_argumentList(&mut self, _ctx: &ArgumentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#methodReference}.
 * @param ctx the parse tree
 */
fn enter_methodReference(&mut self, _ctx: &MethodReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#methodReference}.
 * @param ctx the parse tree
 */
fn exit_methodReference(&mut self, _ctx: &MethodReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn enter_postfixExpression(&mut self, _ctx: &PostfixExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#postfixExpression}.
 * @param ctx the parse tree
 */
fn exit_postfixExpression(&mut self, _ctx: &PostfixExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#pfE}.
 * @param ctx the parse tree
 */
fn enter_pfE(&mut self, _ctx: &PfEContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#pfE}.
 * @param ctx the parse tree
 */
fn exit_pfE(&mut self, _ctx: &PfEContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#postIncrementExpression}.
 * @param ctx the parse tree
 */
fn enter_postIncrementExpression(&mut self, _ctx: &PostIncrementExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#postIncrementExpression}.
 * @param ctx the parse tree
 */
fn exit_postIncrementExpression(&mut self, _ctx: &PostIncrementExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#postDecrementExpression}.
 * @param ctx the parse tree
 */
fn enter_postDecrementExpression(&mut self, _ctx: &PostDecrementExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#postDecrementExpression}.
 * @param ctx the parse tree
 */
fn exit_postDecrementExpression(&mut self, _ctx: &PostDecrementExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#preIncrementExpression}.
 * @param ctx the parse tree
 */
fn enter_preIncrementExpression(&mut self, _ctx: &PreIncrementExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#preIncrementExpression}.
 * @param ctx the parse tree
 */
fn exit_preIncrementExpression(&mut self, _ctx: &PreIncrementExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#preDecrementExpression}.
 * @param ctx the parse tree
 */
fn enter_preDecrementExpression(&mut self, _ctx: &PreDecrementExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#preDecrementExpression}.
 * @param ctx the parse tree
 */
fn exit_preDecrementExpression(&mut self, _ctx: &PreDecrementExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#unaryExpressionNotPlusMinus}.
 * @param ctx the parse tree
 */
fn enter_unaryExpressionNotPlusMinus(&mut self, _ctx: &UnaryExpressionNotPlusMinusContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#unaryExpressionNotPlusMinus}.
 * @param ctx the parse tree
 */
fn exit_unaryExpressionNotPlusMinus(&mut self, _ctx: &UnaryExpressionNotPlusMinusContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#castExpression}.
 * @param ctx the parse tree
 */
fn enter_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#castExpression}.
 * @param ctx the parse tree
 */
fn exit_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#multiplicativeExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#additiveExpression}.
 * @param ctx the parse tree
 */
fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#shiftExpression}.
 * @param ctx the parse tree
 */
fn enter_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#shiftExpression}.
 * @param ctx the parse tree
 */
fn exit_shiftExpression(&mut self, _ctx: &ShiftExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#relationalExpression}.
 * @param ctx the parse tree
 */
fn enter_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#relationalExpression}.
 * @param ctx the parse tree
 */
fn exit_relationalExpression(&mut self, _ctx: &RelationalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn enter_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#equalityExpression}.
 * @param ctx the parse tree
 */
fn exit_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#andExpression}.
 * @param ctx the parse tree
 */
fn enter_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#andExpression}.
 * @param ctx the parse tree
 */
fn exit_andExpression(&mut self, _ctx: &AndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#exclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn enter_exclusiveOrExpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#exclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn exit_exclusiveOrExpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#inclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn enter_inclusiveOrExpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#inclusiveOrExpression}.
 * @param ctx the parse tree
 */
fn exit_inclusiveOrExpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#conditionalAndExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalAndExpression(&mut self, _ctx: &ConditionalAndExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#conditionalAndExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalAndExpression(&mut self, _ctx: &ConditionalAndExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#conditionalOrExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalOrExpression(&mut self, _ctx: &ConditionalOrExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#conditionalOrExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalOrExpression(&mut self, _ctx: &ConditionalOrExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#conditionalExpression}.
 * @param ctx the parse tree
 */
fn enter_conditionalExpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#conditionalExpression}.
 * @param ctx the parse tree
 */
fn exit_conditionalExpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#assignmentExpression}.
 * @param ctx the parse tree
 */
fn enter_assignmentExpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#assignmentExpression}.
 * @param ctx the parse tree
 */
fn exit_assignmentExpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#assignment}.
 * @param ctx the parse tree
 */
fn enter_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#assignment}.
 * @param ctx the parse tree
 */
fn exit_assignment(&mut self, _ctx: &AssignmentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#leftHandSide}.
 * @param ctx the parse tree
 */
fn enter_leftHandSide(&mut self, _ctx: &LeftHandSideContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#leftHandSide}.
 * @param ctx the parse tree
 */
fn exit_leftHandSide(&mut self, _ctx: &LeftHandSideContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#assignmentOperator}.
 * @param ctx the parse tree
 */
fn enter_assignmentOperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#assignmentOperator}.
 * @param ctx the parse tree
 */
fn exit_assignmentOperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn enter_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn exit_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#lambdaParameterList}.
 * @param ctx the parse tree
 */
fn enter_lambdaParameterList(&mut self, _ctx: &LambdaParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#lambdaParameterList}.
 * @param ctx the parse tree
 */
fn exit_lambdaParameterList(&mut self, _ctx: &LambdaParameterListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#lambdaParameter}.
 * @param ctx the parse tree
 */
fn enter_lambdaParameter(&mut self, _ctx: &LambdaParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#lambdaParameter}.
 * @param ctx the parse tree
 */
fn exit_lambdaParameter(&mut self, _ctx: &LambdaParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#lambdaParameterType}.
 * @param ctx the parse tree
 */
fn enter_lambdaParameterType(&mut self, _ctx: &LambdaParameterTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#lambdaParameterType}.
 * @param ctx the parse tree
 */
fn exit_lambdaParameterType(&mut self, _ctx: &LambdaParameterTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#lambdaBody}.
 * @param ctx the parse tree
 */
fn enter_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#lambdaBody}.
 * @param ctx the parse tree
 */
fn exit_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#switchExpression}.
 * @param ctx the parse tree
 */
fn enter_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#switchExpression}.
 * @param ctx the parse tree
 */
fn exit_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link Java20Parser#constantExpression}.
 * @param ctx the parse tree
 */
fn enter_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link Java20Parser#constantExpression}.
 * @param ctx the parse tree
 */
fn exit_constantExpression(&mut self, _ctx: &ConstantExpressionContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : Java20ParserListener<'input> }


