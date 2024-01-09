use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;
use std::{env, fs};
use syn::__private::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{BareFnArg, Ident, Item, TraitItem};

/// Automatically implements a visitor for a given language
///
/// Arg 1: Name of the language to be used from ANTLR. If the language is split into a parser and
/// lexer definition, append "parser". Ex: `c`, `cpp14parser`, `java20parser`, `python3parser`
///
/// Arg 2: Holding struct identifier. Ex: `CTree`
///
/// Arg 3: Tree item identifier. Ex: `CTreeItem`
#[proc_macro]
pub fn auto_visitor(args: TokenStream) -> TokenStream {
    // Parse arguments
    let parser = Punctuated::<BareFnArg, Comma>::parse_separated_nonempty;
    let args = parser.parse(args).unwrap();

    if args.len() != 3 {
        panic!("Invalid arguments!");
    }

    let mut args = args.iter();
    let name = args.next().unwrap();
    let holding_struct = args.next().unwrap().to_token_stream();
    let tree_enum = args.next().unwrap().to_token_stream();

    // Find the visitor file with the necessary trait
    // This expects to be in the root of the Cargo workspace when executed
    let path = env::current_dir().unwrap().join(PathBuf::from(format!(
        "analysis/src/gen/{}visitor.rs",
        name.to_token_stream().to_string().to_lowercase()
    )));

    // Read the file and extract all of the function names that need to be implemented
    let file = fs::read_to_string(path).unwrap();
    let visitor = syn::parse_file(&file).unwrap();
    let visitor_trait = visitor
        .items
        .iter()
        .find_map(|t| match t {
            Item::Trait(t) => Some(t),
            _ => None,
        })
        .unwrap();

    let functions = visitor_trait
        .items
        .iter()
        .filter_map(|ti| match ti {
            TraitItem::Fn(f) => Some(f),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Enum cases that are always needed but are not listed in the trait
    let always_enum_cases = ["Terminal", "Whitespace"]
        .iter()
        .map(|c| syn::parse_str::<Ident>(c).unwrap());

    // Generate function and enumeration definitions
    let generated_enum_cases = functions
        .iter()
        .map(|f| {
            let name = &f.sig.ident;
            let name_str = name.to_string();

            let parser_rule = name_str.split('_').nth(1).unwrap().to_string();
            let mut parser_rule_chars = parser_rule.chars().collect::<Vec<_>>();
            parser_rule_chars[0] = parser_rule_chars[0].to_ascii_uppercase();
            let pascal_case_parser_rule: String = parser_rule_chars.into_iter().collect();
            let pascal_case_parser_rule: Ident = syn::parse_str(&pascal_case_parser_rule).unwrap();

            pascal_case_parser_rule
        })
        .chain(always_enum_cases)
        .collect::<Vec<_>>();

    let generated_functions = functions
        .iter()
        .zip(generated_enum_cases.iter())
        .map(|(f, e)| {
            let name = &f.sig.ident;

            let args = &f.sig.inputs;

            quote! {
                fn #name(#args) -> Self::Return {
                    // Open a tree node and make sure it was successful
                    visitor_result!(self.symbol_tree.open(#tree_enum::#e));

                    // Visit children nodes
                    visitor_result!(self.visit_children(ctx).0);

                    // Close the tree node and make sure it was successful
                    visitor_result!(self.symbol_tree.close());

                    // Nothing wrong, return `Ok(())`
                    VisitorReturn(Ok(()))
                }
            }
        })
        .collect::<Vec<_>>();

    let trait_name = &visitor_trait.ident;
    let trait_name: Ident = syn::parse_str(&format!("{}Compat", trait_name)).unwrap();
    let res = quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum #tree_enum {
            #(#generated_enum_cases),*
        }

        #[allow(non_snake_case)]
        impl<'input> #trait_name<'input> for #holding_struct {
            #(#generated_functions)*
        }
    };

    res.into()
}
