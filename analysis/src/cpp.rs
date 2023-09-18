use antlr_rust::tree::ParseTreeVisitorCompat;

use crate::gen::cpp14parservisitor::CPP14ParserVisitorCompat;
use crate::{SyntaxTree, VisitorReturn};
use crate::gen::cpp14parser::*;

#[derive(Debug, Clone)]
pub enum CppTree {

}

impl ParseTreeVisitorCompat<'_> for CppTree {
    type Node = CPP14ParserContextType;
    type Return = VisitorReturn<Self>;

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::new(Self::Return::default()))
    }
}

impl CPP14ParserVisitorCompat<'_> for CppTree {
    fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_idExpression(&mut self, ctx: &IdExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_unqualifiedId(&mut self, ctx: &UnqualifiedIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_qualifiedId(&mut self, ctx: &QualifiedIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_nestedNameSpecifier(&mut self, ctx: &NestedNameSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_lambdaExpression(&mut self, ctx: &LambdaExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_lambdaIntroducer(&mut self, ctx: &LambdaIntroducerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_lambdaCapture(&mut self, ctx: &LambdaCaptureContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_captureDefault(&mut self, ctx: &CaptureDefaultContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_captureList(&mut self, ctx: &CaptureListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_capture(&mut self, ctx: &CaptureContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_simpleCapture(&mut self, ctx: &SimpleCaptureContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_initcapture(&mut self, ctx: &InitcaptureContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_lambdaDeclarator(&mut self, ctx: &LambdaDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typeIdOfTheTypeId(&mut self, ctx: &TypeIdOfTheTypeIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_pseudoDestructorName(&mut self, ctx: &PseudoDestructorNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_newExpression_(&mut self, ctx: &NewExpression_Context<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_newPlacement(&mut self, ctx: &NewPlacementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_newTypeId(&mut self, ctx: &NewTypeIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_newDeclarator_(&mut self, ctx: &NewDeclarator_Context<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_noPointerNewDeclarator(&mut self, ctx: &NoPointerNewDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_newInitializer_(&mut self, ctx: &NewInitializer_Context<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_deleteExpression(&mut self, ctx: &DeleteExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_noExceptExpression(&mut self, ctx: &NoExceptExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_pointerMemberExpression(&mut self, ctx: &PointerMemberExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_multiplicativeExpression(&mut self, ctx: &MultiplicativeExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_shiftOperator(&mut self, ctx: &ShiftOperatorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_relationalExpression(&mut self, ctx: &RelationalExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_exclusiveOrExpression(&mut self, ctx: &ExclusiveOrExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_inclusiveOrExpression(&mut self, ctx: &InclusiveOrExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_logicalAndExpression(&mut self, ctx: &LogicalAndExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_conditionalExpression(&mut self, ctx: &ConditionalExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_assignmentExpression(&mut self, ctx: &AssignmentExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_statement(&mut self, ctx: &StatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_statementSeq(&mut self, ctx: &StatementSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_condition(&mut self, ctx: &ConditionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_forInitStatement(&mut self, ctx: &ForInitStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_forRangeDeclaration(&mut self, ctx: &ForRangeDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_forRangeInitializer(&mut self, ctx: &ForRangeInitializerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declarationStatement(&mut self, ctx: &DeclarationStatementContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declarationseq(&mut self, ctx: &DeclarationseqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declaration(&mut self, ctx: &DeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_blockDeclaration(&mut self, ctx: &BlockDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_aliasDeclaration(&mut self, ctx: &AliasDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_simpleDeclaration(&mut self, ctx: &SimpleDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_staticAssertDeclaration(&mut self, ctx: &StaticAssertDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_emptyDeclaration_(&mut self, ctx: &EmptyDeclaration_Context<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attributeDeclaration(&mut self, ctx: &AttributeDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declSpecifier(&mut self, ctx: &DeclSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declSpecifierSeq(&mut self, ctx: &DeclSpecifierSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_storageClassSpecifier(&mut self, ctx: &StorageClassSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_functionSpecifier(&mut self, ctx: &FunctionSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typedefName(&mut self, ctx: &TypedefNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_trailingTypeSpecifier(&mut self, ctx: &TrailingTypeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typeSpecifierSeq(&mut self, ctx: &TypeSpecifierSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_trailingTypeSpecifierSeq(&mut self, ctx: &TrailingTypeSpecifierSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_simpleTypeLengthModifier(&mut self, ctx: &SimpleTypeLengthModifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_simpleTypeSignednessModifier(&mut self, ctx: &SimpleTypeSignednessModifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_simpleTypeSpecifier(&mut self, ctx: &SimpleTypeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_theTypeName(&mut self, ctx: &TheTypeNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_decltypeSpecifier(&mut self, ctx: &DecltypeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_elaboratedTypeSpecifier(&mut self, ctx: &ElaboratedTypeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumName(&mut self, ctx: &EnumNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumHead(&mut self, ctx: &EnumHeadContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_opaqueEnumDeclaration(&mut self, ctx: &OpaqueEnumDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumkey(&mut self, ctx: &EnumkeyContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumbase(&mut self, ctx: &EnumbaseContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumeratorDefinition(&mut self, ctx: &EnumeratorDefinitionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_namespaceName(&mut self, ctx: &NamespaceNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_originalNamespaceName(&mut self, ctx: &OriginalNamespaceNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_namespaceDefinition(&mut self, ctx: &NamespaceDefinitionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_namespaceAlias(&mut self, ctx: &NamespaceAliasContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_namespaceAliasDefinition(&mut self, ctx: &NamespaceAliasDefinitionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_qualifiednamespacespecifier(&mut self, ctx: &QualifiednamespacespecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_usingDeclaration(&mut self, ctx: &UsingDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_usingDirective(&mut self, ctx: &UsingDirectiveContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_asmDefinition(&mut self, ctx: &AsmDefinitionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_linkageSpecification(&mut self, ctx: &LinkageSpecificationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attributeSpecifierSeq(&mut self, ctx: &AttributeSpecifierSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attributeSpecifier(&mut self, ctx: &AttributeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_alignmentspecifier(&mut self, ctx: &AlignmentspecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attributeList(&mut self, ctx: &AttributeListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attribute(&mut self, ctx: &AttributeContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attributeNamespace(&mut self, ctx: &AttributeNamespaceContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_attributeArgumentClause(&mut self, ctx: &AttributeArgumentClauseContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_balancedTokenSeq(&mut self, ctx: &BalancedTokenSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_balancedtoken(&mut self, ctx: &BalancedtokenContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declarator(&mut self, ctx: &DeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_pointerDeclarator(&mut self, ctx: &PointerDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_noPointerDeclarator(&mut self, ctx: &NoPointerDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_parametersAndQualifiers(&mut self, ctx: &ParametersAndQualifiersContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_trailingReturnType(&mut self, ctx: &TrailingReturnTypeContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_pointerOperator(&mut self, ctx: &PointerOperatorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_cvqualifierseq(&mut self, ctx: &CvqualifierseqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_cvQualifier(&mut self, ctx: &CvQualifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_refqualifier(&mut self, ctx: &RefqualifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_declaratorid(&mut self, ctx: &DeclaratoridContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_theTypeId(&mut self, ctx: &TheTypeIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_abstractDeclarator(&mut self, ctx: &AbstractDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_pointerAbstractDeclarator(&mut self, ctx: &PointerAbstractDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_noPointerAbstractDeclarator(&mut self, ctx: &NoPointerAbstractDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_abstractPackDeclarator(&mut self, ctx: &AbstractPackDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_noPointerAbstractPackDeclarator(&mut self, ctx: &NoPointerAbstractPackDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_parameterDeclarationClause(&mut self, ctx: &ParameterDeclarationClauseContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_parameterDeclarationList(&mut self, ctx: &ParameterDeclarationListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_parameterDeclaration(&mut self, ctx: &ParameterDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_functionBody(&mut self, ctx: &FunctionBodyContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_initializer(&mut self, ctx: &InitializerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_braceOrEqualInitializer(&mut self, ctx: &BraceOrEqualInitializerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_initializerClause(&mut self, ctx: &InitializerClauseContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_initializerList(&mut self, ctx: &InitializerListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_bracedInitList(&mut self, ctx: &BracedInitListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_className(&mut self, ctx: &ClassNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_classSpecifier(&mut self, ctx: &ClassSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_classHead(&mut self, ctx: &ClassHeadContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_classHeadName(&mut self, ctx: &ClassHeadNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_classVirtSpecifier(&mut self, ctx: &ClassVirtSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_classKey(&mut self, ctx: &ClassKeyContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_memberSpecification(&mut self, ctx: &MemberSpecificationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_memberdeclaration(&mut self, ctx: &MemberdeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_memberDeclaratorList(&mut self, ctx: &MemberDeclaratorListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_memberDeclarator(&mut self, ctx: &MemberDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_virtualSpecifierSeq(&mut self, ctx: &VirtualSpecifierSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_virtualSpecifier(&mut self, ctx: &VirtualSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_pureSpecifier(&mut self, ctx: &PureSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_baseClause(&mut self, ctx: &BaseClauseContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_baseSpecifierList(&mut self, ctx: &BaseSpecifierListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_baseSpecifier(&mut self, ctx: &BaseSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_classOrDeclType(&mut self, ctx: &ClassOrDeclTypeContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_baseTypeSpecifier(&mut self, ctx: &BaseTypeSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_accessSpecifier(&mut self, ctx: &AccessSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_conversionFunctionId(&mut self, ctx: &ConversionFunctionIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_conversionTypeId(&mut self, ctx: &ConversionTypeIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_conversionDeclarator(&mut self, ctx: &ConversionDeclaratorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_constructorInitializer(&mut self, ctx: &ConstructorInitializerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_memInitializerList(&mut self, ctx: &MemInitializerListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_memInitializer(&mut self, ctx: &MemInitializerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_meminitializerid(&mut self, ctx: &MeminitializeridContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_operatorFunctionId(&mut self, ctx: &OperatorFunctionIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_literalOperatorId(&mut self, ctx: &LiteralOperatorIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateDeclaration(&mut self, ctx: &TemplateDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateparameterList(&mut self, ctx: &TemplateparameterListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateParameter(&mut self, ctx: &TemplateParameterContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typeParameter(&mut self, ctx: &TypeParameterContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_simpleTemplateId(&mut self, ctx: &SimpleTemplateIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateId(&mut self, ctx: &TemplateIdContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateName(&mut self, ctx: &TemplateNameContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateArgumentList(&mut self, ctx: &TemplateArgumentListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_templateArgument(&mut self, ctx: &TemplateArgumentContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typeNameSpecifier(&mut self, ctx: &TypeNameSpecifierContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_explicitInstantiation(&mut self, ctx: &ExplicitInstantiationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_explicitSpecialization(&mut self, ctx: &ExplicitSpecializationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_tryBlock(&mut self, ctx: &TryBlockContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_functionTryBlock(&mut self, ctx: &FunctionTryBlockContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_handlerSeq(&mut self, ctx: &HandlerSeqContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_handler(&mut self, ctx: &HandlerContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_exceptionDeclaration(&mut self, ctx: &ExceptionDeclarationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_throwExpression(&mut self, ctx: &ThrowExpressionContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_exceptionSpecification(&mut self, ctx: &ExceptionSpecificationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_dynamicExceptionSpecification(&mut self, ctx: &DynamicExceptionSpecificationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_typeIdList(&mut self, ctx: &TypeIdListContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_noeExceptSpecification(&mut self, ctx: &NoeExceptSpecificationContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_theOperator(&mut self, ctx: &TheOperatorContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }

    fn visit_literal(&mut self, ctx: &LiteralContext<'_>) -> Self::Return {
			    self.visit_children(ctx)
		    }
}