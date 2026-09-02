use syn::parse_macro_input;
use quote::quote;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("Describe can only be derived for strutcs"),
    };

    let field_pieces = match fields {
        Fields::Named(named) => named.named.into_iter().map(
            |f| {
                let ident = f.ident.unwrap();
                quote! { format!("{}: {:?}", stringify!(#ident), self.#ident) }
            }
        ),
        _ => panic!("Describe requires named fields"),
    };
    let output = quote! {
        impl Describe for #struct_name {
            fn describe(&self) -> String {
                let fields = vec![#(#field_pieces), *];
                format!("{} {{ {} }}", stringify!(#struct_name), fields.join(", "))
            }
        }
    };
    output.into()
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}
