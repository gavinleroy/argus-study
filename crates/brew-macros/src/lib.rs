extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(HumanFood)]
pub fn food_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let ast = syn::parse(input).unwrap();

  // Build the impl
  impl_food(&ast)
}

fn impl_food(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote::quote! {
      impl HumanFood for #name {}
  };
  gen.into()
}
