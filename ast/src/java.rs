use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ErrorNode, ParseTreeVisitorCompat, TerminalNode};
use antlr_rust::InputStream;
use syntree::Tree;

use crate::gen::javalexer::JavaLexer;
use crate::gen::javaparser::*;
use crate::gen::javaparservisitor::JavaParserVisitorCompat;
use crate::{visitor_result, SyntaxTree, TreeParseError, VisitorReturn};

use macros::auto_visitor;

#[derive(Debug)]
pub struct JavaTree {
    /// Contains all necessary indices to reconstruct the source code from the original with
    /// symbols. This tree also contains whitespace and variable names, so it may not work as
    /// well for comparisons.
    /// TODO: Figure if non-token structure tree is needed
    pub symbol_tree: syntree::Builder<JavaTreeItem, usize, usize>,
    /// Temporary variable for visitor
    tmp: VisitorReturn<()>,
}

impl Clone for JavaTree {
    fn clone(&self) -> Self {
        Self {
            symbol_tree: self.symbol_tree.clone(),
            tmp: Default::default(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for JavaTree {
    type Node = JavaParserContextType;
    type Return = VisitorReturn<()>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp
    }

    fn visit_terminal(&mut self, node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        // if node.symbol.start - *self.symbol_tree.cursor() as isize > 0 {
        //     visitor_result!(self.symbol_tree.token(JavaTreeItem::Whitespace, node.symbol.start as usize - self.symbol_tree.cursor()));
        // }

        visitor_result!(self.symbol_tree.token(JavaTreeItem::Terminal, node.symbol.text.len()));

        // visitor_result!(self
        //     .symbol_tree
        //     .token_empty(UniqueItem::new(JavaTreeItem::Terminal)));

        VisitorReturn(Ok(()))
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        VisitorReturn(Err(TreeParseError::InvalidNode))
    }
}

auto_visitor!(javaparser, JavaTree, JavaTreeItem);

impl SyntaxTree for JavaTree {
    type Item = JavaTreeItem;
    fn symbol_tree(
        self,
    ) -> anyhow::Result<Tree<Self::Item, usize, usize>, TreeParseError> {
        Ok(self.symbol_tree.build()?)
    }
}

impl TryFrom<String> for JavaTree {
    type Error = TreeParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lexer = JavaLexer::new(InputStream::new(value.as_str()));
        let mut parser = JavaParser::new(CommonTokenStream::new(lexer));

        let root = parser.compilationUnit()?;

        let mut tree = JavaTree {
            symbol_tree: Default::default(),
            tmp: Default::default(),
        };

        tree.visit(&*root).0?;

        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_parse;

    use super::JavaTree;

    test_parse!(
        comment_import_comment,
        JavaTree,
        r#"/*
TEST COMMENT
 */

import test.com;

// ANOTHER COMMENT

public class Test {
    public static void main(String[] args) {

    }
}

"#
    );

    // Test cases from https://infedu.vu.lt/journal/INFEDU/article/16/info (https://github.com/oscarkarnalim/sourcecodeplagiarismdataset)
    test_parse!(
        simple_print,
        JavaTree,
        r#"public class T1 {
    public static void main(String[] args) {
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
    }

}"#
    );

    test_parse!(
        simple_print_with_multiline,
        JavaTree,
        r#"public class T1 {
    /*
        This is a test of multi-line comments
     */
    public static void main(String[] args) {
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
        System.out.println("Welcome to Java");
    }

}"#
    );

    test_parse!(
        whitespace_beginning,
        JavaTree,
        r#"

import java.util.Scanner;

/**
 *
 * @author 020A6EC1A4D0C5BDB29FF826A462DA1C5D88CF08B60A4744AFFD95C61A0C3C7E
 */
public class Main {
    public static double hasilPertambahanDiagonal(double[][] m) {
        double sum = 0;

        for (int i = 0; i < m.length; i++)
            sum += m[i][i];

        return sum;
    }

    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Enter a 4 by 4 matrix row by row: ");
        double[][] n = new double[4][4];

        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 4; j++) {
                n[i][j] = input.nextDouble();
            }
        }

        System.out.print("Sum of the elements in the major diagonal is " + hasilPertambahanDiagonal(n));
    }
}"#
    );

    test_parse!(
        more_complex_with_multiline,
        JavaTree,
        r#"/*
* To change this license header, choose License Headers in Project Properties.
* To change this template file, choose Tools | Templates
* and open the template in the editor.
*/

import java.util.scanner;

/**
*
* @author B15130F5DDB6B5F1622EF91DAC4C1AAE
*/
public class Kasus5L6 {

    public static void main(String[] args) {
        //input data
        Scanner inp = new Scanner(System.in);
        System.out.print("Enter an integer: ");
        String angka = inp.next();
        for (int i = angka.length() - 1; i >= 0; i--) {
            System.out.print(angka.charAt(i));
        }
        System.out.println();
    }
}"#
    );

    test_parse!(
        volume,
        JavaTree,
        r#"import java.util.Scanner;

public class T2 {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        // Enter radius of the cylinder
        System.out.print("Enter the radius and length of a cylinder: ");
        double radius = input.nextDouble();
        double length = input.nextDouble();

        double area = radius * radius * 3.14159;
        double volume = area * length;

        System.out.println("The area is " + area);
        System.out.println("The volume of the cylinder is " + volume);
    }

}
"#
    );

    test_parse!(
        bmi,
        JavaTree,
        r#"import java.util.Scanner;

public class T3 {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        // Prompt the user to enter weight in pounds
        System.out.print("Enter weight in pounds: ");
        double weight = input.nextDouble();

        // Prompt the user to enter height
        System.out.print("Enter feet: ");
        double feet = input.nextDouble();
        System.out.print("Enter inches: ");
        double inches = input.nextDouble();

        double height = feet * 12 + inches;

        // Compute BMI
        double bmi = weight * 0.45359237 / ((height * 0.0254) * (height * 0.0254));

        // Display result
        System.out.println("BMI is " + bmi);
        if (bmi < 18.5)
            System.out.println("Underweight");
        else if (bmi < 25)
            System.out.println("Normal");
        else if (bmi < 30)
            System.out.println("Overweight");
        else
            System.out.println("Obese");
    }

}
"#
    );

    test_parse!(
        distance_conversion,
        JavaTree,
        r#"public class T4 {
    public static void main(String[] args) {
        System.out.println("Miles\t\tKilometers");
        System.out.println("-------------------------------");

        // Use while loop
        int miles = 1;
        while (miles <= 10) {
            System.out.println(miles + "\t\t" + miles * 1.609);
            miles++;
        }
    }

}
"#
    );

    test_parse!(
        int_reverse,
        JavaTree,
        r#"public class T5 {
    public static void main(String[] args) {
        System.out.print("Enter an integer: ");
        java.util.Scanner input = new java.util.Scanner(System.in);
        int number = input.nextInt();
        reverse(number);
    }

    public static void reverse(int number) {
        while (number != 0) {
            int remainder = number % 10;
            System.out.print(remainder);
            number = number / 10;
        }

        System.out.println();
    }

}"#
    );

    test_parse!(
        array_builder,
        JavaTree,
        r#"public class T6 {
    public static void main(String[] args) {
        java.util.Scanner input = new java.util.Scanner(System.in);
        int[] num = new int[10];

        for (int i = 0; i < 10; i++) {
            // Read a number
            System.out.print("Read a number: ");

            num[i] = input.nextInt();
        }

        // Display the array
        for (int i = 9; i >= 0; i--) {
            System.out.println(num[i]);
        }
    }

}"#
    );

    test_parse!(
        diagonal_sum,
        JavaTree,
        r#"import java.util.Scanner;

public class T7 {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);

        System.out.print("Enter a 4 by 4 matrix row by row: ");
        double[][] m = new double[4][4];

        for (int i = 0; i < 4; i++)
            for (int j = 0; j < 4; j++)
                m[i][j] = input.nextDouble();

        System.out.print("Sum of the elements in the major diagonal is " + sumMajorDiagonal(m));
    }

    public static double sumMajorDiagonal(double[][] m) {
        double sum = 0;

        for (int i = 0; i < m.length; i++)
            sum += m[i][i];

        return sum;
    }

}"#
    );
}
