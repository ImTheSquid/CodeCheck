use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;

use crate::gen::clexer::CLexer;
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
    StorageClassSpecifier,
    TypeSpecifierContext,
    StructOrUnionSpecifier,
    StructOrUnion,
    StructDeclarationList,
    StructDeclaration,
    SpecifierQualifierList,
    StructDeclaratorList,
    StructDeclarator,
    EnumSpecifier,
    EnumeratorList,
    Enumerator,
    EnumerationConstant,
    AtomicTypeSpecifier,
    TypeQualifier,
    InitializerList,
    Designation,
    DesignatorList,
    Designator,
    StaticAssertDeclaration,
    Statement,
    LabeledStatement,
    CompoundStatement,
    BlockItemList,
    BlockItem,
    ExpressionStatement,
    SelectionStatement,
    IterationStatement,
    ForCondition,
    ForDeclaration,
    ForExpression,
    JumpStatement,
    CompilationUnit,
    TranslationUnit,
    ExternalDeclaration,
    FunctionDefinition,
    DeclarationList,
    GenericSelection,
    GenericAssocList,
    GenericAssociation,
    PostfixExpression,
    ArgumentExpressionList,
    UnaryExpression,
    UnaryOperator,
    CastExpression,
    DigitSequence,
    MultiplicativeExpression,
    AdditiveExpression,
    ShiftExpression,
    RelationalExpression,
    EqualityExpression,
    AndExpression,
    ExclusiveOrExpression,
    InclusiveOrExpression,
    LogicalAndExpression,
    LogicalOrExpression,
    ConditionalExpression,
    AssignmentOperator,
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

        // Visit children nodes
        visitor_result!(self.visit_children(ctx).0);

        // Check the lexer rules and see if there are any that match
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier, Constant, StringLiteral_all);

        // Close the `PrimaryExpression` tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return `Ok(())`
        VisitorReturn(Ok(()))
    }

    fn visit_genericSelection(&mut self, ctx: &GenericSelectionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::GenericSelection));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_genericAssocList(&mut self, ctx: &GenericAssocListContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::GenericAssocList));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_genericAssociation(&mut self, ctx: &GenericAssociationContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::GenericAssociation));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_postfixExpression(&mut self, ctx: &PostfixExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::PostfixExpression));

        visitor_result!(self.visit_children(ctx).0);

        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier_all);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_argumentExpressionList(
        &mut self,
        ctx: &ArgumentExpressionListContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::ArgumentExpressionList));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::UnaryExpression));

        visitor_result!(self.visit_children(ctx).0);

        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_unaryOperator(&mut self, ctx: &UnaryOperatorContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::UnaryOperator));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::CastExpression));

        visitor_result!(self.visit_children(ctx).0);

        try_lexer_rules!(ctx, self.tree, CTreeItem, DigitSequence);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_multiplicativeExpression(
        &mut self,
        ctx: &MultiplicativeExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::MultiplicativeExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_additiveExpression(&mut self, ctx: &AdditiveExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::AdditiveExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_shiftExpression(&mut self, ctx: &ShiftExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::ShiftExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_relationalExpression(
        &mut self,
        ctx: &RelationalExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::RelationalExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_equalityExpression(&mut self, ctx: &EqualityExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::EqualityExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_andExpression(&mut self, ctx: &AndExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::AndExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_exclusiveOrExpression(
        &mut self,
        ctx: &ExclusiveOrExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::ExclusiveOrExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_inclusiveOrExpression(
        &mut self,
        ctx: &InclusiveOrExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::InclusiveOrExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_logicalAndExpression(
        &mut self,
        ctx: &LogicalAndExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::LogicalAndExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_logicalOrExpression(&mut self, ctx: &LogicalOrExpressionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::LogicalOrExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_conditionalExpression(
        &mut self,
        ctx: &ConditionalExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::ConditionalExpression));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_assignmentExpression(
        &mut self,
        ctx: &AssignmentExpressionContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::UnaryExpression));

        visitor_result!(self.visit_children(ctx).0);

        try_lexer_rules!(ctx, self.tree, CTreeItem, DigitSequence);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
    }

    fn visit_assignmentOperator(&mut self, ctx: &AssignmentOperatorContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::AssignmentOperator));

        visitor_result!(self.visit_children(ctx).0);

        visitor_result!(self.tree.close());

        VisitorReturn(Ok(()))
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
        visitor_result!(self.tree.open(CTreeItem::StorageClassSpecifier));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_typeSpecifier(&mut self, ctx: &TypeSpecifierContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::TypeSpecifierContext));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_structOrUnionSpecifier(
        &mut self,
        ctx: &StructOrUnionSpecifierContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::StructOrUnion));
        visitor_result!(self.visit_children(ctx).0);
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_structOrUnion(&mut self, ctx: &StructOrUnionContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::StructOrUnion));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_structDeclarationList(
        &mut self,
        ctx: &StructDeclarationListContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::StructDeclarationList));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_structDeclaration(&mut self, ctx: &StructDeclarationContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::StructDeclaration));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_specifierQualifierList(
        &mut self,
        ctx: &SpecifierQualifierListContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::SpecifierQualifierList));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_structDeclaratorList(
        &mut self,
        ctx: &StructDeclaratorListContext<'_>,
    ) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::StructDeclaratorList));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_structDeclarator(&mut self, ctx: &StructDeclaratorContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::StructDeclarator));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_enumSpecifier(&mut self, ctx: &EnumSpecifierContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::EnumSpecifier));
        visitor_result!(self.visit_children(ctx).0);
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_enumeratorList(&mut self, ctx: &EnumeratorListContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::EnumeratorList));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_enumerator(&mut self, ctx: &EnumeratorContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::Enumerator));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_enumerationConstant(&mut self, ctx: &EnumerationConstantContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::EnumerationConstant));
        visitor_result!(self.visit_children(ctx).0);
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_atomicTypeSpecifier(&mut self, ctx: &AtomicTypeSpecifierContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::AtomicTypeSpecifier));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))
    }

    fn visit_typeQualifier(&mut self, ctx: &TypeQualifierContext<'_>) -> Self::Return {
        visitor_result!(self.tree.open(CTreeItem::TypeQualifier));
        visitor_result!(self.visit_children(ctx).0);
        visitor_result!(self.tree.close());
        VisitorReturn(Ok(()))

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
        // Open a tree node of type 'InitializerList' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::InitializerList));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "InitializerList" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_designation(&mut self, ctx: &DesignationContext<'_>) -> Self::Return {
        // Open a tree node of type 'Designation' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::Designation));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "Designation" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_designatorList(&mut self, ctx: &DesignatorListContext<'_>) -> Self::Return {
        // Open a tree node of type 'DesignatorList' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::DesignatorList));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "DesignatorList" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_designator(&mut self, ctx: &DesignatorContext<'_>) -> Self::Return {
        // Open a tree node of type 'Designator' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::Designator));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Check lexer rules and see if there are any that match
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);
 
        // Close the "Designator" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_staticAssertDeclaration(
        &mut self,
        ctx: &StaticAssertDeclarationContext<'_>,
    ) -> Self::Return {
        // Open a tree node of type 'StaticAssertDeclaration' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::StaticAssertDeclaration));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "StaticAssertDeclaration" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'Statement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::Statement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "Statement" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_labeledStatement(&mut self, ctx: &LabeledStatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'LabeledStatement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::LabeledStatement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Check lexer rules and see if there are any that match
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);
 
        // Close the "LabeledStatement" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_compoundStatement(&mut self, ctx: &CompoundStatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'CompoundStatement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::CompoundStatement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "CompoundStatement" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_blockItemList(&mut self, ctx: &BlockItemListContext<'_>) -> Self::Return {
        // Open a tree node of type 'BlockItemList' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::BlockItemList));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "BlockItemList" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_blockItem(&mut self, ctx: &BlockItemContext<'_>) -> Self::Return {
        // Open a tree node of type 'BlockItem' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::BlockItem));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "BlockItem" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'ExpressionStatement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::ExpressionStatement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "ExpressionStatement" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_selectionStatement(&mut self, ctx: &SelectionStatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'SelectionStatement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::SelectionStatement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "SelectionStatement" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_iterationStatement(&mut self, ctx: &IterationStatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'IterationStatement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::IterationStatement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "IterationStatement" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_forCondition(&mut self, ctx: &ForConditionContext<'_>) -> Self::Return {
        // Open a tree node of type 'ForCondition' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::ForCondition));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "ForCondition" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_forDeclaration(&mut self, ctx: &ForDeclarationContext<'_>) -> Self::Return {
        // Open a tree node of type 'ForDeclaration' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::ForDeclaration));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
 
        // Close the "ForDeclaration" tree node and make sure it was successful
        visitor_result!(self.tree.close());
 
        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_forExpression(&mut self, ctx: &ForExpressionContext<'_>) -> Self::Return {
        // Open a tree node of type 'ForExpression' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::ForExpression));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Close the "ForExpression" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_jumpStatement(&mut self, ctx: &JumpStatementContext<'_>) -> Self::Return {
        // Open a tree node of type 'JumpStatement' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::JumpStatement));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Check lexer rules and see if there are any that match
        try_lexer_rules!(ctx, self.tree, CTreeItem, Identifier);

        // Close the "JumpStatement" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_compilationUnit(&mut self, ctx: &CompilationUnitContext<'_>) -> Self::Return {
        // Open a tree node of type 'CompilationUnit' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::CompilationUnit));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Close the "CompilationUnit" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_translationUnit(&mut self, ctx: &TranslationUnitContext<'_>) -> Self::Return {
        // Open a tree node of type 'TranslationUnit' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::TranslationUnit));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Close the "TranslationUnit" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_externalDeclaration(&mut self, ctx: &ExternalDeclarationContext<'_>) -> Self::Return {
        // Open a tree node of type 'ExternalDeclaration' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::ExternalDeclaration));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Close the "ExternalDeclaration" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))
    }

    fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'_>) -> Self::Return {
        // Open a tree node of type 'FunctionDefinition' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::FunctionDefinition));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);

        // Close the "DeclarationList" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))

    }

    fn visit_declarationList(&mut self, ctx: &DeclarationListContext<'_>) -> Self::Return {
        // Open a tree node of type 'DeclarationList' and made sure it was successful
        visitor_result!(self.tree.open(CTreeItem::DeclarationList));

        // Visit Children Nodes
        visitor_result!(self.visit_children(ctx).0);
        
        // Close the "Declaration List" tree node and make sure it was successful
        visitor_result!(self.tree.close());

        // Nothing wrong, return 'Ok(())'
        VisitorReturn(Ok(()))

    }
}

impl SyntaxTree<'_> for CTree {
    fn compare(&self, other: &Self) -> f64 {
        todo!()
    }

    fn worst_runtime_complexity(&self) -> crate::RuntimeComplexity {
        todo!()
    }

    fn runtime_complexity_of_fn<S: AsRef<str>>(&self, name: S) -> Option<crate::RuntimeComplexity> {
        todo!()
    }
}

impl TryFrom<&str> for CTree {
    type Error = TreeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lexer = CLexer::new(InputStream::new(value));
        let mut parser = CParser::new(CommonTokenStream::new(lexer));

        let root = parser.declarationList()?;

        let mut tree = CTree {
            tree: Default::default()
        };

        tree.visit(&*root).0?;

        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use super::CTree;

    #[test]
    fn c_parse() {
        let raw = 
r#"#include <stdio.h>

int
main(int argc, char **argv) {
    char *test = "world!";
    int myval = 5;
    printf("Hello, %s (%d)", test, myval);
}
"#;

        CTree::try_from(raw).unwrap();
    }

    #[test]
    fn c_parse_included() {
        let raw =
r#"# 1 "test.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 418 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "test.c" 2
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 1 3 4
# 64 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 1 3 4
# 68 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h" 1 3 4
# 678 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_symbol_aliasing.h" 1 3 4
# 679 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h" 2 3 4
# 744 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_posix_availability.h" 1 3 4
# 745 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/cdefs.h" 2 3 4
# 69 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/Availability.h" 1 3 4
# 167 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/Availability.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/AvailabilityVersions.h" 1 3 4
# 168 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/Availability.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/AvailabilityInternal.h" 1 3 4
# 153 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/AvailabilityInternal.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/AvailabilityInternalLegacy.h" 1 3 4
# 154 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/AvailabilityInternal.h" 2 3 4
# 169 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/Availability.h" 2 3 4
# 70 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4

# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types.h" 1 3 4
# 27 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h" 1 3 4
# 33 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/_types.h" 1 3 4
# 34 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/_types.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h" 1 3 4
# 15 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h" 3 4
typedef signed char __int8_t;



typedef unsigned char __uint8_t;
typedef short __int16_t;
typedef unsigned short __uint16_t;
typedef int __int32_t;
typedef unsigned int __uint32_t;
typedef long long __int64_t;
typedef unsigned long long __uint64_t;

typedef long __darwin_intptr_t;
typedef unsigned int __darwin_natural_t;
# 48 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/_types.h" 3 4
typedef int __darwin_ct_rune_t;





typedef union {
 char __mbstate8[128];
 long long _mbstateL;
} __mbstate_t;

typedef __mbstate_t __darwin_mbstate_t;


typedef long int __darwin_ptrdiff_t;







typedef long unsigned int __darwin_size_t;





typedef __builtin_va_list __darwin_va_list;





typedef int __darwin_wchar_t;




typedef __darwin_wchar_t __darwin_rune_t;


typedef int __darwin_wint_t;




typedef unsigned long __darwin_clock_t;
typedef __uint32_t __darwin_socklen_t;
typedef long __darwin_ssize_t;
typedef long __darwin_time_t;
# 35 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/_types.h" 2 3 4
# 34 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h" 2 3 4
# 55 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h" 3 4
typedef __int64_t __darwin_blkcnt_t;
typedef __int32_t __darwin_blksize_t;
typedef __int32_t __darwin_dev_t;
typedef unsigned int __darwin_fsblkcnt_t;
typedef unsigned int __darwin_fsfilcnt_t;
typedef __uint32_t __darwin_gid_t;
typedef __uint32_t __darwin_id_t;
typedef __uint64_t __darwin_ino64_t;

typedef __darwin_ino64_t __darwin_ino_t;



typedef __darwin_natural_t __darwin_mach_port_name_t;
typedef __darwin_mach_port_name_t __darwin_mach_port_t;
typedef __uint16_t __darwin_mode_t;
typedef __int64_t __darwin_off_t;
typedef __int32_t __darwin_pid_t;
typedef __uint32_t __darwin_sigset_t;
typedef __int32_t __darwin_suseconds_t;
typedef __uint32_t __darwin_uid_t;
typedef __uint32_t __darwin_useconds_t;
typedef unsigned char __darwin_uuid_t[16];
typedef char __darwin_uuid_string_t[37];

# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h" 1 3 4
# 57 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_pthread/_pthread_types.h" 3 4
struct __darwin_pthread_handler_rec {
 void (*__routine)(void *);
 void *__arg;
 struct __darwin_pthread_handler_rec *__next;
};

struct _opaque_pthread_attr_t {
 long __sig;
 char __opaque[56];
};

struct _opaque_pthread_cond_t {
 long __sig;
 char __opaque[40];
};

struct _opaque_pthread_condattr_t {
 long __sig;
 char __opaque[8];
};

struct _opaque_pthread_mutex_t {
 long __sig;
 char __opaque[56];
};

struct _opaque_pthread_mutexattr_t {
 long __sig;
 char __opaque[8];
};

struct _opaque_pthread_once_t {
 long __sig;
 char __opaque[8];
};

struct _opaque_pthread_rwlock_t {
 long __sig;
 char __opaque[192];
};

struct _opaque_pthread_rwlockattr_t {
 long __sig;
 char __opaque[16];
};

struct _opaque_pthread_t {
 long __sig;
 struct __darwin_pthread_handler_rec *__cleanup_stack;
 char __opaque[8176];
};

typedef struct _opaque_pthread_attr_t __darwin_pthread_attr_t;
typedef struct _opaque_pthread_cond_t __darwin_pthread_cond_t;
typedef struct _opaque_pthread_condattr_t __darwin_pthread_condattr_t;
typedef unsigned long __darwin_pthread_key_t;
typedef struct _opaque_pthread_mutex_t __darwin_pthread_mutex_t;
typedef struct _opaque_pthread_mutexattr_t __darwin_pthread_mutexattr_t;
typedef struct _opaque_pthread_once_t __darwin_pthread_once_t;
typedef struct _opaque_pthread_rwlock_t __darwin_pthread_rwlock_t;
typedef struct _opaque_pthread_rwlockattr_t __darwin_pthread_rwlockattr_t;
typedef struct _opaque_pthread_t *__darwin_pthread_t;
# 81 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types.h" 2 3 4
# 28 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types.h" 2 3 4
# 40 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_types.h" 3 4
typedef int __darwin_nl_item;
typedef int __darwin_wctrans_t;

typedef __uint32_t __darwin_wctype_t;
# 72 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4



# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/types.h" 1 3 4
# 37 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/types.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 1 3 4
# 55 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int8_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int8_t.h" 3 4
typedef signed char int8_t;
# 56 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int16_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int16_t.h" 3 4
typedef short int16_t;
# 57 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int32_t.h" 3 4
typedef int int32_t;
# 58 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int64_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_int64_t.h" 3 4
typedef long long int64_t;
# 59 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4

# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int8_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int8_t.h" 3 4
typedef unsigned char u_int8_t;
# 61 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int16_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int16_t.h" 3 4
typedef unsigned short u_int16_t;
# 62 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int32_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int32_t.h" 3 4
typedef unsigned int u_int32_t;
# 63 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int64_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_u_int64_t.h" 3 4
typedef unsigned long long u_int64_t;
# 64 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4


typedef int64_t register_t;




# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h" 1 3 4
# 30 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/types.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_intptr_t.h" 2 3 4

typedef __darwin_intptr_t intptr_t;
# 72 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h" 1 3 4
# 34 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_uintptr_t.h" 3 4
typedef unsigned long uintptr_t;
# 73 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 2 3 4




typedef u_int64_t user_addr_t;
typedef u_int64_t user_size_t;
typedef int64_t user_ssize_t;
typedef int64_t user_long_t;
typedef u_int64_t user_ulong_t;
typedef int64_t user_time_t;
typedef int64_t user_off_t;
# 104 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/arm/types.h" 3 4
typedef u_int64_t syscall_arg_t;
# 38 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/machine/types.h" 2 3 4
# 32 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_va_list.h" 2 3 4
typedef __darwin_va_list va_list;
# 76 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h" 3 4
typedef __darwin_size_t size_t;
# 77 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_null.h" 1 3 4
# 78 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4

# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stdio.h" 1 3 4
# 47 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/stdio.h" 3 4
int renameat(int, const char *, int, const char *) __attribute__((availability(macosx,introduced=10.10)));



int renamex_np(const char *, const char *, unsigned int) __attribute__((availability(macosx,introduced=10.12))) __attribute__((availability(ios,introduced=10.0))) __attribute__((availability(tvos,introduced=10.0))) __attribute__((availability(watchos,introduced=3.0)));
int renameatx_np(int, const char *, int, const char *, unsigned int) __attribute__((availability(macosx,introduced=10.12))) __attribute__((availability(ios,introduced=10.0))) __attribute__((availability(tvos,introduced=10.0))) __attribute__((availability(watchos,introduced=3.0)));
# 80 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 2 3 4

typedef __darwin_off_t fpos_t;
# 92 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 3 4
struct __sbuf {
 unsigned char *_base;
 int _size;
};


struct __sFILEX;
# 126 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_stdio.h" 3 4
typedef struct __sFILE {
 unsigned char *_p;
 int _r;
 int _w;
 short _flags;
 short _file;
 struct __sbuf _bf;
 int _lbfsize;


 void *_cookie;
 int (* _Nullable _close)(void *);
 int (* _Nullable _read) (void *, char *, int);
 fpos_t (* _Nullable _seek) (void *, fpos_t, int);
 int (* _Nullable _write)(void *, const char *, int);


 struct __sbuf _ub;
 struct __sFILEX *_extra;
 int _ur;


 unsigned char _ubuf[3];
 unsigned char _nbuf[1];


 struct __sbuf _lb;


 int _blksize;
 fpos_t _offset;
} FILE;
# 65 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 2 3 4

# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_seek_set.h" 1 3 4
# 67 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 2 3 4


extern FILE *__stdinp;
extern FILE *__stdoutp;
extern FILE *__stderrp;
# 134 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
void clearerr(FILE *);
int fclose(FILE *);
int feof(FILE *);
int ferror(FILE *);
int fflush(FILE *);
int fgetc(FILE *);
int fgetpos(FILE * restrict, fpos_t *);
char *fgets(char * restrict, int, FILE *);



FILE *fopen(const char * restrict __filename, const char * restrict __mode) __asm("_" "fopen" );

int fprintf(FILE * restrict, const char * restrict, ...) __attribute__((__format__ (__printf__, 2, 3)));
int fputc(int, FILE *);
int fputs(const char * restrict, FILE * restrict) __asm("_" "fputs" );
size_t fread(void * restrict __ptr, size_t __size, size_t __nitems, FILE * restrict __stream);
FILE *freopen(const char * restrict, const char * restrict,
                 FILE * restrict) __asm("_" "freopen" );
int fscanf(FILE * restrict, const char * restrict, ...) __attribute__((__format__ (__scanf__, 2, 3)));
int fseek(FILE *, long, int);
int fsetpos(FILE *, const fpos_t *);
long ftell(FILE *);
size_t fwrite(const void * restrict __ptr, size_t __size, size_t __nitems, FILE * restrict __stream) __asm("_" "fwrite" );
int getc(FILE *);
int getchar(void);


__attribute__((__deprecated__("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of gets(3), it is highly recommended that you use fgets(3) instead.")))

char *gets(char *);

void perror(const char *) __attribute__((__cold__));
int printf(const char * restrict, ...) __attribute__((__format__ (__printf__, 1, 2)));
int putc(int, FILE *);
int putchar(int);
int puts(const char *);
int remove(const char *);
int rename (const char *__old, const char *__new);
void rewind(FILE *);
int scanf(const char * restrict, ...) __attribute__((__format__ (__scanf__, 1, 2)));
void setbuf(FILE * restrict, char * restrict);
int setvbuf(FILE * restrict, char * restrict, int, size_t);

__attribute__((__availability__(swift, unavailable, message="Use snprintf instead.")))

__attribute__((__deprecated__("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use snprintf(3) instead.")))

int sprintf(char * restrict, const char * restrict, ...) __attribute__((__format__ (__printf__, 2, 3)));

int sscanf(const char * restrict, const char * restrict, ...) __attribute__((__format__ (__scanf__, 2, 3)));
FILE *tmpfile(void);

__attribute__((__availability__(swift, unavailable, message="Use mkstemp(3) instead.")))

__attribute__((__deprecated__("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of tmpnam(3), it is highly recommended that you use mkstemp(3) instead.")))

char *tmpnam(char *);

int ungetc(int, FILE *);
int vfprintf(FILE * restrict, const char * restrict, va_list) __attribute__((__format__ (__printf__, 2, 0)));
int vprintf(const char * restrict, va_list) __attribute__((__format__ (__printf__, 1, 0)));

__attribute__((__availability__(swift, unavailable, message="Use vsnprintf instead.")))

__attribute__((__deprecated__("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of sprintf(3), it is highly recommended that you use vsnprintf(3) instead.")))

int vsprintf(char * restrict, const char * restrict, va_list) __attribute__((__format__ (__printf__, 2, 0)));
# 213 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctermid.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/_ctermid.h" 3 4
char *ctermid(char *);
# 214 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 2 3 4






FILE *fdopen(int, const char *) __asm("_" "fdopen" );

int fileno(FILE *);
# 233 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
int pclose(FILE *) __attribute__((__availability__(swift, unavailable, message="Use posix_spawn APIs or NSTask instead. (On iOS, process spawning is unavailable.)")));



FILE *popen(const char *, const char *) __asm("_" "popen" ) __attribute__((__availability__(swift, unavailable, message="Use posix_spawn APIs or NSTask instead. (On iOS, process spawning is unavailable.)")));
# 252 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
int __srget(FILE *);
int __svfscanf(FILE *, const char *, va_list) __attribute__((__format__ (__scanf__, 2, 0)));
int __swbuf(int, FILE *);
# 263 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
inline __attribute__ ((__always_inline__)) int __sputc(int _c, FILE *_p) {
 if (--_p->_w >= 0 || (_p->_w >= _p->_lbfsize && (char)_c != '\n'))
  return (*_p->_p++ = _c);
 else
  return (__swbuf(_c, _p));
}
# 289 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
void flockfile(FILE *);
int ftrylockfile(FILE *);
void funlockfile(FILE *);
int getc_unlocked(FILE *);
int getchar_unlocked(void);
int putc_unlocked(int, FILE *);
int putchar_unlocked(int);



int getw(FILE *);
int putw(int, FILE *);


__attribute__((__availability__(swift, unavailable, message="Use mkstemp(3) instead.")))

__attribute__((__deprecated__("This function is provided for compatibility reasons only.  Due to security concerns inherent in the design of tempnam(3), it is highly recommended that you use mkstemp(3) instead.")))

char *tempnam(const char *__dir, const char *__prefix) __asm("_" "tempnam" );
# 327 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_off_t.h" 3 4
typedef __darwin_off_t off_t;
# 328 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 2 3 4


int fseeko(FILE * __stream, off_t __offset, int __whence);
off_t ftello(FILE * __stream);





int snprintf(char * restrict __str, size_t __size, const char * restrict __format, ...) __attribute__((__format__ (__printf__, 3, 4)));
int vfscanf(FILE * restrict __stream, const char * restrict __format, va_list) __attribute__((__format__ (__scanf__, 2, 0)));
int vscanf(const char * restrict __format, va_list) __attribute__((__format__ (__scanf__, 1, 0)));
int vsnprintf(char * restrict __str, size_t __size, const char * restrict __format, va_list) __attribute__((__format__ (__printf__, 3, 0)));
int vsscanf(const char * restrict __str, const char * restrict __format, va_list) __attribute__((__format__ (__scanf__, 2, 0)));
# 352 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/sys/_types/_ssize_t.h" 3 4
typedef __darwin_ssize_t ssize_t;
# 353 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 2 3 4


int dprintf(int, const char * restrict, ...) __attribute__((__format__ (__printf__, 2, 3))) __attribute__((availability(macosx,introduced=10.7)));
int vdprintf(int, const char * restrict, va_list) __attribute__((__format__ (__printf__, 2, 0))) __attribute__((availability(macosx,introduced=10.7)));
ssize_t getdelim(char ** restrict __linep, size_t * restrict __linecapp, int __delimiter, FILE * restrict __stream) __attribute__((availability(macosx,introduced=10.7)));
ssize_t getline(char ** restrict __linep, size_t * restrict __linecapp, FILE * restrict __stream) __attribute__((availability(macosx,introduced=10.7)));
FILE *fmemopen(void * restrict __buf, size_t __size, const char * restrict __mode) __attribute__((availability(macos,introduced=10.13))) __attribute__((availability(ios,introduced=11.0))) __attribute__((availability(tvos,introduced=11.0))) __attribute__((availability(watchos,introduced=4.0)));
FILE *open_memstream(char **__bufp, size_t *__sizep) __attribute__((availability(macos,introduced=10.13))) __attribute__((availability(ios,introduced=11.0))) __attribute__((availability(tvos,introduced=11.0))) __attribute__((availability(watchos,introduced=4.0)));
# 370 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
extern const int sys_nerr;
extern const char *const sys_errlist[];

int asprintf(char ** restrict, const char * restrict, ...) __attribute__((__format__ (__printf__, 2, 3)));
char *ctermid_r(char *);
char *fgetln(FILE *, size_t *);
const char *fmtcheck(const char *, const char *) __attribute__((format_arg(2)));
int fpurge(FILE *);
void setbuffer(FILE *, char *, int);
int setlinebuf(FILE *);
int vasprintf(char ** restrict, const char * restrict, va_list) __attribute__((__format__ (__printf__, 2, 0)));





FILE *funopen(const void *,
                 int (* _Nullable)(void *, char *, int),
                 int (* _Nullable)(void *, const char *, int),
                 fpos_t (* _Nullable)(void *, fpos_t, int),
                 int (* _Nullable)(void *));
# 409 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/secure/_stdio.h" 1 3 4
# 31 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/secure/_stdio.h" 3 4
# 1 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/secure/_common.h" 1 3 4
# 32 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/secure/_stdio.h" 2 3 4
# 42 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/secure/_stdio.h" 3 4
extern int __sprintf_chk (char * restrict, int, size_t,
     const char * restrict, ...);
# 52 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/secure/_stdio.h" 3 4
extern int __snprintf_chk (char * restrict, size_t, int, size_t,
      const char * restrict, ...);







extern int __vsprintf_chk (char * restrict, int, size_t,
      const char * restrict, va_list);







extern int __vsnprintf_chk (char * restrict, size_t, int, size_t,
       const char * restrict, va_list);
# 410 "/Applications/Xcode-13.4.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/stdio.h" 2 3 4
# 2 "test.c" 2

int
main(int argc, char **argv) {
    char *test = "world!";
    int myval = 5;
    printf("Hello, %s (%d)", test, myval);
}
"#;

        CTree::try_from(raw).unwrap();
    }
}
