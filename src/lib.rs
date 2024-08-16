use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(FieldAccess)]
pub fn macro_derive_init(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => return quote! {}.into(),
        },
        _ => return quote! {}.into(),
    };

    let mut funcs = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let get = Ident::new(&format!("get_{}", field_name), field_name.span());
        let set = Ident::new(&format!("set_{}", field_name), field_name.span());
        let with = Ident::new(&format!("with_{}", field_name), field_name.span());

        funcs.push(quote! {
            pub fn #get(&self) -> #field_type {
                self.#field_name
            }

            pub fn #set(&mut self, set_value : #field_type) {
                self.#field_name = set_value;
            }

            pub fn #with(mut self, set_value : #field_type) -> Self {
                self.#field_name = set_value;
                self
            }
        });
    }

    quote! {
        impl #struct_name {
            #(#funcs)*
        }
    }.into()
}