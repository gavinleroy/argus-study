extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Mineral)]
pub fn mineral_derive(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let ast = syn::parse(input).unwrap();

  // Build the impl
  impl_mineral(&ast)
}

fn impl_mineral(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote::quote! {
      impl Mineral for #name {}
  };
  gen.into()
}
