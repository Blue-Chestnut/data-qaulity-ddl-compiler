use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ValidColumnRule)]
pub fn valid_column_rule_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_valid_column_macro(&ast)
}

fn impl_valid_column_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ValidColumnRule for #name {
            fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
                return Ok(String::from("valid"));
            }
        }
    };
    gen.into()
}