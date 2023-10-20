#![allow(nonstandard_style)]
// Generated from JavaParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::javaparser::*;

pub trait JavaParserListener<'input> : ParseTreeListener<'input,JavaParserContextType>{
/**
 * Enter a parse tree produced by {@link JavaParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn enter_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#compilationUnit}.
 * @param ctx the parse tree
 */
fn exit_compilationUnit(&mut self, _ctx: &CompilationUnitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#packageDeclaration}.
 * @param ctx the parse tree
 */
fn enter_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#packageDeclaration}.
 * @param ctx the parse tree
 */
fn exit_packageDeclaration(&mut self, _ctx: &PackageDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#importDeclaration}.
 * @param ctx the parse tree
 */
fn enter_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#importDeclaration}.
 * @param ctx the parse tree
 */
fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_typeDeclaration(&mut self, _ctx: &TypeDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#modifier}.
 * @param ctx the parse tree
 */
fn enter_modifier(&mut self, _ctx: &ModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#modifier}.
 * @param ctx the parse tree
 */
fn exit_modifier(&mut self, _ctx: &ModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
 * @param ctx the parse tree
 */
fn enter_classOrInterfaceModifier(&mut self, _ctx: &ClassOrInterfaceModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classOrInterfaceModifier}.
 * @param ctx the parse tree
 */
fn exit_classOrInterfaceModifier(&mut self, _ctx: &ClassOrInterfaceModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#variableModifier}.
 * @param ctx the parse tree
 */
fn enter_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#variableModifier}.
 * @param ctx the parse tree
 */
fn exit_variableModifier(&mut self, _ctx: &VariableModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classDeclaration}.
 * @param ctx the parse tree
 */
fn enter_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classDeclaration}.
 * @param ctx the parse tree
 */
fn exit_classDeclaration(&mut self, _ctx: &ClassDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeParameters}.
 * @param ctx the parse tree
 */
fn enter_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeParameters}.
 * @param ctx the parse tree
 */
fn exit_typeParameters(&mut self, _ctx: &TypeParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeParameter}.
 * @param ctx the parse tree
 */
fn enter_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeParameter}.
 * @param ctx the parse tree
 */
fn exit_typeParameter(&mut self, _ctx: &TypeParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeBound}.
 * @param ctx the parse tree
 */
fn enter_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeBound}.
 * @param ctx the parse tree
 */
fn exit_typeBound(&mut self, _ctx: &TypeBoundContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#enumDeclaration}.
 * @param ctx the parse tree
 */
fn enter_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#enumDeclaration}.
 * @param ctx the parse tree
 */
fn exit_enumDeclaration(&mut self, _ctx: &EnumDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#enumConstants}.
 * @param ctx the parse tree
 */
fn enter_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#enumConstants}.
 * @param ctx the parse tree
 */
fn exit_enumConstants(&mut self, _ctx: &EnumConstantsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#enumConstant}.
 * @param ctx the parse tree
 */
fn enter_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#enumConstant}.
 * @param ctx the parse tree
 */
fn exit_enumConstant(&mut self, _ctx: &EnumConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
 * @param ctx the parse tree
 */
fn enter_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#enumBodyDeclarations}.
 * @param ctx the parse tree
 */
fn exit_enumBodyDeclarations(&mut self, _ctx: &EnumBodyDeclarationsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceDeclaration(&mut self, _ctx: &InterfaceDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classBody}.
 * @param ctx the parse tree
 */
fn enter_classBody(&mut self, _ctx: &ClassBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classBody}.
 * @param ctx the parse tree
 */
fn exit_classBody(&mut self, _ctx: &ClassBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceBody}.
 * @param ctx the parse tree
 */
fn enter_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceBody}.
 * @param ctx the parse tree
 */
fn exit_interfaceBody(&mut self, _ctx: &InterfaceBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classBodyDeclaration}.
 * @param ctx the parse tree
 */
fn enter_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classBodyDeclaration}.
 * @param ctx the parse tree
 */
fn exit_classBodyDeclaration(&mut self, _ctx: &ClassBodyDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#memberDeclaration}.
 * @param ctx the parse tree
 */
fn enter_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#memberDeclaration}.
 * @param ctx the parse tree
 */
fn exit_memberDeclaration(&mut self, _ctx: &MemberDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#methodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#methodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_methodDeclaration(&mut self, _ctx: &MethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#methodBody}.
 * @param ctx the parse tree
 */
fn enter_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#methodBody}.
 * @param ctx the parse tree
 */
fn exit_methodBody(&mut self, _ctx: &MethodBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
 * @param ctx the parse tree
 */
fn enter_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeTypeOrVoid}.
 * @param ctx the parse tree
 */
fn exit_typeTypeOrVoid(&mut self, _ctx: &TypeTypeOrVoidContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#genericMethodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_genericMethodDeclaration(&mut self, _ctx: &GenericMethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
 * @param ctx the parse tree
 */
fn enter_genericConstructorDeclaration(&mut self, _ctx: &GenericConstructorDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#genericConstructorDeclaration}.
 * @param ctx the parse tree
 */
fn exit_genericConstructorDeclaration(&mut self, _ctx: &GenericConstructorDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#constructorDeclaration}.
 * @param ctx the parse tree
 */
fn enter_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#constructorDeclaration}.
 * @param ctx the parse tree
 */
fn exit_constructorDeclaration(&mut self, _ctx: &ConstructorDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
 * @param ctx the parse tree
 */
fn enter_compactConstructorDeclaration(&mut self, _ctx: &CompactConstructorDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#compactConstructorDeclaration}.
 * @param ctx the parse tree
 */
fn exit_compactConstructorDeclaration(&mut self, _ctx: &CompactConstructorDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#fieldDeclaration}.
 * @param ctx the parse tree
 */
fn enter_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#fieldDeclaration}.
 * @param ctx the parse tree
 */
fn exit_fieldDeclaration(&mut self, _ctx: &FieldDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceBodyDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceBodyDeclaration(&mut self, _ctx: &InterfaceBodyDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceMemberDeclaration(&mut self, _ctx: &InterfaceMemberDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceMemberDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceMemberDeclaration(&mut self, _ctx: &InterfaceMemberDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#constDeclaration}.
 * @param ctx the parse tree
 */
fn enter_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#constDeclaration}.
 * @param ctx the parse tree
 */
fn exit_constDeclaration(&mut self, _ctx: &ConstDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#constantDeclarator}.
 * @param ctx the parse tree
 */
fn enter_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#constantDeclarator}.
 * @param ctx the parse tree
 */
fn exit_constantDeclarator(&mut self, _ctx: &ConstantDeclaratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceMethodDeclaration(&mut self, _ctx: &InterfaceMethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceMethodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceMethodDeclaration(&mut self, _ctx: &InterfaceMethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
 * @param ctx the parse tree
 */
fn enter_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceMethodModifier}.
 * @param ctx the parse tree
 */
fn exit_interfaceMethodModifier(&mut self, _ctx: &InterfaceMethodModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
 * @param ctx the parse tree
 */
fn enter_genericInterfaceMethodDeclaration(&mut self, _ctx: &GenericInterfaceMethodDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#genericInterfaceMethodDeclaration}.
 * @param ctx the parse tree
 */
fn exit_genericInterfaceMethodDeclaration(&mut self, _ctx: &GenericInterfaceMethodDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interfaceCommonBodyDeclaration(&mut self, _ctx: &InterfaceCommonBodyDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#interfaceCommonBodyDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interfaceCommonBodyDeclaration(&mut self, _ctx: &InterfaceCommonBodyDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#variableDeclarators}.
 * @param ctx the parse tree
 */
fn enter_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#variableDeclarators}.
 * @param ctx the parse tree
 */
fn exit_variableDeclarators(&mut self, _ctx: &VariableDeclaratorsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#variableDeclarator}.
 * @param ctx the parse tree
 */
fn enter_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#variableDeclarator}.
 * @param ctx the parse tree
 */
fn exit_variableDeclarator(&mut self, _ctx: &VariableDeclaratorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#variableDeclaratorId}.
 * @param ctx the parse tree
 */
fn enter_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#variableDeclaratorId}.
 * @param ctx the parse tree
 */
fn exit_variableDeclaratorId(&mut self, _ctx: &VariableDeclaratorIdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#variableInitializer}.
 * @param ctx the parse tree
 */
fn enter_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#variableInitializer}.
 * @param ctx the parse tree
 */
fn exit_variableInitializer(&mut self, _ctx: &VariableInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#arrayInitializer}.
 * @param ctx the parse tree
 */
fn enter_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#arrayInitializer}.
 * @param ctx the parse tree
 */
fn exit_arrayInitializer(&mut self, _ctx: &ArrayInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classOrInterfaceType}.
 * @param ctx the parse tree
 */
fn enter_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classOrInterfaceType}.
 * @param ctx the parse tree
 */
fn exit_classOrInterfaceType(&mut self, _ctx: &ClassOrInterfaceTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeArgument}.
 * @param ctx the parse tree
 */
fn enter_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeArgument}.
 * @param ctx the parse tree
 */
fn exit_typeArgument(&mut self, _ctx: &TypeArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#qualifiedNameList}.
 * @param ctx the parse tree
 */
fn enter_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#qualifiedNameList}.
 * @param ctx the parse tree
 */
fn exit_qualifiedNameList(&mut self, _ctx: &QualifiedNameListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#formalParameters}.
 * @param ctx the parse tree
 */
fn enter_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#formalParameters}.
 * @param ctx the parse tree
 */
fn exit_formalParameters(&mut self, _ctx: &FormalParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#receiverParameter}.
 * @param ctx the parse tree
 */
fn enter_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#receiverParameter}.
 * @param ctx the parse tree
 */
fn exit_receiverParameter(&mut self, _ctx: &ReceiverParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#formalParameterList}.
 * @param ctx the parse tree
 */
fn enter_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#formalParameterList}.
 * @param ctx the parse tree
 */
fn exit_formalParameterList(&mut self, _ctx: &FormalParameterListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#formalParameter}.
 * @param ctx the parse tree
 */
fn enter_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#formalParameter}.
 * @param ctx the parse tree
 */
fn exit_formalParameter(&mut self, _ctx: &FormalParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#lastFormalParameter}.
 * @param ctx the parse tree
 */
fn enter_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#lastFormalParameter}.
 * @param ctx the parse tree
 */
fn exit_lastFormalParameter(&mut self, _ctx: &LastFormalParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#lambdaLVTIList}.
 * @param ctx the parse tree
 */
fn enter_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#lambdaLVTIList}.
 * @param ctx the parse tree
 */
fn exit_lambdaLVTIList(&mut self, _ctx: &LambdaLVTIListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
 * @param ctx the parse tree
 */
fn enter_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#lambdaLVTIParameter}.
 * @param ctx the parse tree
 */
fn exit_lambdaLVTIParameter(&mut self, _ctx: &LambdaLVTIParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn enter_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#qualifiedName}.
 * @param ctx the parse tree
 */
fn exit_qualifiedName(&mut self, _ctx: &QualifiedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#integerLiteral}.
 * @param ctx the parse tree
 */
fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#integerLiteral}.
 * @param ctx the parse tree
 */
fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
 * @param ctx the parse tree
 */
fn enter_altAnnotationQualifiedName(&mut self, _ctx: &AltAnnotationQualifiedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#altAnnotationQualifiedName}.
 * @param ctx the parse tree
 */
fn exit_altAnnotationQualifiedName(&mut self, _ctx: &AltAnnotationQualifiedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotation}.
 * @param ctx the parse tree
 */
fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotation}.
 * @param ctx the parse tree
 */
fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#elementValuePairs}.
 * @param ctx the parse tree
 */
fn enter_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#elementValuePairs}.
 * @param ctx the parse tree
 */
fn exit_elementValuePairs(&mut self, _ctx: &ElementValuePairsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#elementValuePair}.
 * @param ctx the parse tree
 */
fn enter_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#elementValuePair}.
 * @param ctx the parse tree
 */
fn exit_elementValuePair(&mut self, _ctx: &ElementValuePairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#elementValue}.
 * @param ctx the parse tree
 */
fn enter_elementValue(&mut self, _ctx: &ElementValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#elementValue}.
 * @param ctx the parse tree
 */
fn exit_elementValue(&mut self, _ctx: &ElementValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
 * @param ctx the parse tree
 */
fn enter_elementValueArrayInitializer(&mut self, _ctx: &ElementValueArrayInitializerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#elementValueArrayInitializer}.
 * @param ctx the parse tree
 */
fn exit_elementValueArrayInitializer(&mut self, _ctx: &ElementValueArrayInitializerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationTypeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_annotationTypeDeclaration(&mut self, _ctx: &AnnotationTypeDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationTypeBody}.
 * @param ctx the parse tree
 */
fn enter_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationTypeBody}.
 * @param ctx the parse tree
 */
fn exit_annotationTypeBody(&mut self, _ctx: &AnnotationTypeBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
 * @param ctx the parse tree
 */
fn enter_annotationTypeElementDeclaration(&mut self, _ctx: &AnnotationTypeElementDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationTypeElementDeclaration}.
 * @param ctx the parse tree
 */
fn exit_annotationTypeElementDeclaration(&mut self, _ctx: &AnnotationTypeElementDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
 * @param ctx the parse tree
 */
fn enter_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationTypeElementRest}.
 * @param ctx the parse tree
 */
fn exit_annotationTypeElementRest(&mut self, _ctx: &AnnotationTypeElementRestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
 * @param ctx the parse tree
 */
fn enter_annotationMethodOrConstantRest(&mut self, _ctx: &AnnotationMethodOrConstantRestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationMethodOrConstantRest}.
 * @param ctx the parse tree
 */
fn exit_annotationMethodOrConstantRest(&mut self, _ctx: &AnnotationMethodOrConstantRestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationMethodRest}.
 * @param ctx the parse tree
 */
fn enter_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationMethodRest}.
 * @param ctx the parse tree
 */
fn exit_annotationMethodRest(&mut self, _ctx: &AnnotationMethodRestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#annotationConstantRest}.
 * @param ctx the parse tree
 */
fn enter_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#annotationConstantRest}.
 * @param ctx the parse tree
 */
fn exit_annotationConstantRest(&mut self, _ctx: &AnnotationConstantRestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#defaultValue}.
 * @param ctx the parse tree
 */
fn enter_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#defaultValue}.
 * @param ctx the parse tree
 */
fn exit_defaultValue(&mut self, _ctx: &DefaultValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#moduleDeclaration}.
 * @param ctx the parse tree
 */
fn enter_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#moduleDeclaration}.
 * @param ctx the parse tree
 */
fn exit_moduleDeclaration(&mut self, _ctx: &ModuleDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#moduleBody}.
 * @param ctx the parse tree
 */
fn enter_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#moduleBody}.
 * @param ctx the parse tree
 */
fn exit_moduleBody(&mut self, _ctx: &ModuleBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#moduleDirective}.
 * @param ctx the parse tree
 */
fn enter_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#moduleDirective}.
 * @param ctx the parse tree
 */
fn exit_moduleDirective(&mut self, _ctx: &ModuleDirectiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#requiresModifier}.
 * @param ctx the parse tree
 */
fn enter_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#requiresModifier}.
 * @param ctx the parse tree
 */
fn exit_requiresModifier(&mut self, _ctx: &RequiresModifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#recordDeclaration}.
 * @param ctx the parse tree
 */
fn enter_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#recordDeclaration}.
 * @param ctx the parse tree
 */
fn exit_recordDeclaration(&mut self, _ctx: &RecordDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#recordHeader}.
 * @param ctx the parse tree
 */
fn enter_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#recordHeader}.
 * @param ctx the parse tree
 */
fn exit_recordHeader(&mut self, _ctx: &RecordHeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#recordComponentList}.
 * @param ctx the parse tree
 */
fn enter_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#recordComponentList}.
 * @param ctx the parse tree
 */
fn exit_recordComponentList(&mut self, _ctx: &RecordComponentListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#recordComponent}.
 * @param ctx the parse tree
 */
fn enter_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#recordComponent}.
 * @param ctx the parse tree
 */
fn exit_recordComponent(&mut self, _ctx: &RecordComponentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#recordBody}.
 * @param ctx the parse tree
 */
fn enter_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#recordBody}.
 * @param ctx the parse tree
 */
fn exit_recordBody(&mut self, _ctx: &RecordBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#blockStatement}.
 * @param ctx the parse tree
 */
fn enter_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#blockStatement}.
 * @param ctx the parse tree
 */
fn exit_blockStatement(&mut self, _ctx: &BlockStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#localVariableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#localVariableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_localVariableDeclaration(&mut self, _ctx: &LocalVariableDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeIdentifier}.
 * @param ctx the parse tree
 */
fn enter_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeIdentifier}.
 * @param ctx the parse tree
 */
fn exit_typeIdentifier(&mut self, _ctx: &TypeIdentifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#localTypeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#localTypeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_localTypeDeclaration(&mut self, _ctx: &LocalTypeDeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#catchClause}.
 * @param ctx the parse tree
 */
fn enter_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#catchClause}.
 * @param ctx the parse tree
 */
fn exit_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#catchType}.
 * @param ctx the parse tree
 */
fn enter_catchType(&mut self, _ctx: &CatchTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#catchType}.
 * @param ctx the parse tree
 */
fn exit_catchType(&mut self, _ctx: &CatchTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#finallyBlock}.
 * @param ctx the parse tree
 */
fn enter_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#finallyBlock}.
 * @param ctx the parse tree
 */
fn exit_finallyBlock(&mut self, _ctx: &FinallyBlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#resourceSpecification}.
 * @param ctx the parse tree
 */
fn enter_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#resourceSpecification}.
 * @param ctx the parse tree
 */
fn exit_resourceSpecification(&mut self, _ctx: &ResourceSpecificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#resources}.
 * @param ctx the parse tree
 */
fn enter_resources(&mut self, _ctx: &ResourcesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#resources}.
 * @param ctx the parse tree
 */
fn exit_resources(&mut self, _ctx: &ResourcesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#resource}.
 * @param ctx the parse tree
 */
fn enter_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#resource}.
 * @param ctx the parse tree
 */
fn exit_resource(&mut self, _ctx: &ResourceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
 * @param ctx the parse tree
 */
fn enter_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#switchBlockStatementGroup}.
 * @param ctx the parse tree
 */
fn exit_switchBlockStatementGroup(&mut self, _ctx: &SwitchBlockStatementGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#switchLabel}.
 * @param ctx the parse tree
 */
fn enter_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#switchLabel}.
 * @param ctx the parse tree
 */
fn exit_switchLabel(&mut self, _ctx: &SwitchLabelContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#forControl}.
 * @param ctx the parse tree
 */
fn enter_forControl(&mut self, _ctx: &ForControlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#forControl}.
 * @param ctx the parse tree
 */
fn exit_forControl(&mut self, _ctx: &ForControlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#forInit}.
 * @param ctx the parse tree
 */
fn enter_forInit(&mut self, _ctx: &ForInitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#forInit}.
 * @param ctx the parse tree
 */
fn exit_forInit(&mut self, _ctx: &ForInitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#enhancedForControl}.
 * @param ctx the parse tree
 */
fn enter_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#enhancedForControl}.
 * @param ctx the parse tree
 */
fn exit_enhancedForControl(&mut self, _ctx: &EnhancedForControlContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#parExpression}.
 * @param ctx the parse tree
 */
fn enter_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#parExpression}.
 * @param ctx the parse tree
 */
fn exit_parExpression(&mut self, _ctx: &ParExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#expressionList}.
 * @param ctx the parse tree
 */
fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#expressionList}.
 * @param ctx the parse tree
 */
fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#methodCall}.
 * @param ctx the parse tree
 */
fn enter_methodCall(&mut self, _ctx: &MethodCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#methodCall}.
 * @param ctx the parse tree
 */
fn exit_methodCall(&mut self, _ctx: &MethodCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn enter_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#lambdaExpression}.
 * @param ctx the parse tree
 */
fn exit_lambdaExpression(&mut self, _ctx: &LambdaExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn enter_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn exit_lambdaParameters(&mut self, _ctx: &LambdaParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#lambdaBody}.
 * @param ctx the parse tree
 */
fn enter_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#lambdaBody}.
 * @param ctx the parse tree
 */
fn exit_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#switchExpression}.
 * @param ctx the parse tree
 */
fn enter_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#switchExpression}.
 * @param ctx the parse tree
 */
fn exit_switchExpression(&mut self, _ctx: &SwitchExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#switchLabeledRule}.
 * @param ctx the parse tree
 */
fn enter_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#switchLabeledRule}.
 * @param ctx the parse tree
 */
fn exit_switchLabeledRule(&mut self, _ctx: &SwitchLabeledRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#guardedPattern}.
 * @param ctx the parse tree
 */
fn enter_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#guardedPattern}.
 * @param ctx the parse tree
 */
fn exit_guardedPattern(&mut self, _ctx: &GuardedPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#switchRuleOutcome}.
 * @param ctx the parse tree
 */
fn enter_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#switchRuleOutcome}.
 * @param ctx the parse tree
 */
fn exit_switchRuleOutcome(&mut self, _ctx: &SwitchRuleOutcomeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classType}.
 * @param ctx the parse tree
 */
fn enter_classType(&mut self, _ctx: &ClassTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classType}.
 * @param ctx the parse tree
 */
fn exit_classType(&mut self, _ctx: &ClassTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#creator}.
 * @param ctx the parse tree
 */
fn enter_creator(&mut self, _ctx: &CreatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#creator}.
 * @param ctx the parse tree
 */
fn exit_creator(&mut self, _ctx: &CreatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#createdName}.
 * @param ctx the parse tree
 */
fn enter_createdName(&mut self, _ctx: &CreatedNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#createdName}.
 * @param ctx the parse tree
 */
fn exit_createdName(&mut self, _ctx: &CreatedNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#innerCreator}.
 * @param ctx the parse tree
 */
fn enter_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#innerCreator}.
 * @param ctx the parse tree
 */
fn exit_innerCreator(&mut self, _ctx: &InnerCreatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#arrayCreatorRest}.
 * @param ctx the parse tree
 */
fn enter_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#arrayCreatorRest}.
 * @param ctx the parse tree
 */
fn exit_arrayCreatorRest(&mut self, _ctx: &ArrayCreatorRestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#classCreatorRest}.
 * @param ctx the parse tree
 */
fn enter_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#classCreatorRest}.
 * @param ctx the parse tree
 */
fn exit_classCreatorRest(&mut self, _ctx: &ClassCreatorRestContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
 * @param ctx the parse tree
 */
fn enter_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocation}.
 * @param ctx the parse tree
 */
fn exit_explicitGenericInvocation(&mut self, _ctx: &ExplicitGenericInvocationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
 * @param ctx the parse tree
 */
fn enter_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeArgumentsOrDiamond}.
 * @param ctx the parse tree
 */
fn exit_typeArgumentsOrDiamond(&mut self, _ctx: &TypeArgumentsOrDiamondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
 * @param ctx the parse tree
 */
fn enter_nonWildcardTypeArgumentsOrDiamond(&mut self, _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArgumentsOrDiamond}.
 * @param ctx the parse tree
 */
fn exit_nonWildcardTypeArgumentsOrDiamond(&mut self, _ctx: &NonWildcardTypeArgumentsOrDiamondContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
 * @param ctx the parse tree
 */
fn enter_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#nonWildcardTypeArguments}.
 * @param ctx the parse tree
 */
fn exit_nonWildcardTypeArguments(&mut self, _ctx: &NonWildcardTypeArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeList}.
 * @param ctx the parse tree
 */
fn enter_typeList(&mut self, _ctx: &TypeListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeList}.
 * @param ctx the parse tree
 */
fn exit_typeList(&mut self, _ctx: &TypeListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeType}.
 * @param ctx the parse tree
 */
fn enter_typeType(&mut self, _ctx: &TypeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeType}.
 * @param ctx the parse tree
 */
fn exit_typeType(&mut self, _ctx: &TypeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#primitiveType}.
 * @param ctx the parse tree
 */
fn enter_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#primitiveType}.
 * @param ctx the parse tree
 */
fn exit_primitiveType(&mut self, _ctx: &PrimitiveTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#typeArguments}.
 * @param ctx the parse tree
 */
fn enter_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#typeArguments}.
 * @param ctx the parse tree
 */
fn exit_typeArguments(&mut self, _ctx: &TypeArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#superSuffix}.
 * @param ctx the parse tree
 */
fn enter_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#superSuffix}.
 * @param ctx the parse tree
 */
fn exit_superSuffix(&mut self, _ctx: &SuperSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
 * @param ctx the parse tree
 */
fn enter_explicitGenericInvocationSuffix(&mut self, _ctx: &ExplicitGenericInvocationSuffixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#explicitGenericInvocationSuffix}.
 * @param ctx the parse tree
 */
fn exit_explicitGenericInvocationSuffix(&mut self, _ctx: &ExplicitGenericInvocationSuffixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link JavaParser#arguments}.
 * @param ctx the parse tree
 */
fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link JavaParser#arguments}.
 * @param ctx the parse tree
 */
fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : JavaParserListener<'input> }


