use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;

use crate::gen::cparser::*;
use crate::gen::cvisitor::CVisitorCompat;
use crate::{SyntaxTree, VisitorReturn, TreeParseError, visitor_result, try_lexer_rules};

#[derive(Debug, Clone, Copy)]
pub enum CTreeItem {
    PrimaryExpression,
    Identifier,
    Constant,
    StringLiteral,
    Expression,
    ConstantExpression,
    Declaration,
    DeclarationSpecifiers,
    DeclarationSpecifiers2,
    DeclarationSpecifier,
    InitDeclaratorListContext,
    InitDeclarator,
}

#[derive(Debug, Clone)]
pub struct CTree {
    tree: syntree::Builder<CTreeItem, usize, usize>,
}

impl ParseTreeVisitorCompat<'_> for CTree {
    type Node = CParserContextType;
    type Return = VisitorReturn<()>;

    fn temp_result(&mut self) -> &mut Self::Return {
        Box::leak(Box::default())
    }
}

#[allow(non_snake_case)]
impl CVisitorCompat<'_> for CTree {
    fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'_>) -> Self::Return {
        // Open a tree node of type `PrimaryExpression` and make sure it was successful
        visitor_result!(self.tree.open(CTreeItem::PrimaryExpression));

        // Check the lexer rules and see if there are any that match. If so, use them as a leaf node and return
        if try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier, Constant, StringLiteral_all).is_some() {
            // Close the current tree node
            visitor_result!(self.tree.close());
            return VisitorReturn(Ok(()));
        }
        
        // Visit children nodes
        visitor_result!(self.visit_children(ctx).0);

        // Close the `PrimaryExpression` tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return `Ok(())`
        VisitorReturn(Ok(()))
    }

    fn visit_genericSelection(&mut self, ctx: &GenericSelectionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_genericAssocList(&mut self, ctx: &GenericAssocListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_genericAssociation(&mut self, ctx: &GenericAssociationContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_argumentExpressionList(
        &mut self,
        ctx: &ArgumentExpressionListContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_multiplicativeExpression(
        &mut self,
        ctx: &MultiplicativeExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relationalExpression(
        &mut self,
        ctx: &RelationalExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_exclusiveOrExpression(
        &mut self,
        ctx: &ExclusiveOrExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_inclusiveOrExpression(
        &mut self,
        ctx: &InclusiveOrExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_logicalAndExpression(
        &mut self,
        ctx: &LogicalAndExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_conditionalExpression(
        &mut self,
        ctx: &ConditionalExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_assignmentExpression(
        &mut self,
        ctx: &AssignmentExpressionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::Expression));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_constantExpression(&mut self, ctx: &ConstantExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::ConstantExpression));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_declaration(&mut self, ctx: &DeclarationContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::Declaration));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_declarationSpecifiers(
        &mut self,
        ctx: &DeclarationSpecifiersContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::DeclarationSpecifiers));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_declarationSpecifiers2(
        &mut self,
        ctx: &DeclarationSpecifiers2Context<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::DeclarationSpecifiers2));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_declarationSpecifier(
        &mut self,
        ctx: &DeclarationSpecifierContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::DeclarationSpecifier));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_initDeclaratorList(&mut self, ctx: &InitDeclaratorListContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::InitDeclaratorListContext));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_initDeclarator(&mut self, ctx: &InitDeclaratorContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::InitDeclarator));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_storageClassSpecifier(
        &mut self,
        ctx: &StorageClassSpecifierContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_structOrUnionSpecifier(
        &mut self,
        ctx: &StructOrUnionSpecifierContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_structOrUnion(&mut self, ctx: &StructOrUnionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_structDeclarationList(
        &mut self,
        ctx: &StructDeclarationListContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_structDeclaration(&mut self, ctx: &StructDeclarationContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_specifierQualifierList(
        &mut self,
        ctx: &SpecifierQualifierListContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_structDeclaratorList(
        &mut self,
        ctx: &StructDeclaratorListContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_structDeclarator(&mut self, ctx: &StructDeclaratorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_enumerationConstant(&mut self, ctx: &EnumerationConstantContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_atomicTypeSpecifier(&mut self, ctx: &AtomicTypeSpecifierContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_typeQualifier(&mut self, ctx: &TypeQualifierContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_functionSpecifier(&mut self, ctx: &FunctionSpecifierContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_alignmentSpecifier(&mut self, ctx: &AlignmentSpecifierContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_declarator(&mut self, ctx: &DeclaratorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_directDeclarator(&mut self, ctx: &DirectDeclaratorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_vcSpecificModifer(&mut self, ctx: &VcSpecificModiferContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_gccDeclaratorExtension(
        &mut self,
        ctx: &GccDeclaratorExtensionContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_gccAttributeSpecifier(
        &mut self,
        ctx: &GccAttributeSpecifierContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_gccAttributeList(&mut self, ctx: &GccAttributeListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_gccAttribute(&mut self, ctx: &GccAttributeContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_nestedParenthesesBlock(
        &mut self,
        ctx: &NestedParenthesesBlockContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_pointer(&mut self, ctx: &PointerContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_typeQualifierList(&mut self, ctx: &TypeQualifierListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_parameterTypeList(&mut self, ctx: &ParameterTypeListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_parameterList(&mut self, ctx: &ParameterListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_parameterDeclaration(
        &mut self,
        ctx: &ParameterDeclarationContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_typeName(&mut self, ctx: &TypeNameContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_abstractDeclarator(&mut self, ctx: &AbstractDeclaratorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_directAbstractDeclarator(
        &mut self,
        ctx: &DirectAbstractDeclaratorContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_typedefName(&mut self, ctx: &TypedefNameContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_initializer(&mut self, ctx: &InitializerContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_initializerList(&mut self, ctx: &InitializerListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_designation(&mut self, ctx: &DesignationContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_designator(&mut self, ctx: &DesignatorContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_staticAssertDeclaration(
        &mut self,
        ctx: &StaticAssertDeclarationContext<'_>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_blockItem(&mut self, ctx: &BlockItemContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_forCondition(&mut self, ctx: &ForConditionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_declarationList(&mut self, ctx: &DeclarationListContext<'_>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl SyntaxTree for CTree {
    fn compare(&self, _other: &Self) -> f64 {
        todo!()
    }

    fn worst_runtime_complexity(&self) -> crate::RuntimeComplexity {
        todo!()
    }

    fn runtime_complexity_of_fn<S: AsRef<str>>(&self, _name: S) -> Option<crate::RuntimeComplexity> {
        todo!()
    }
}
