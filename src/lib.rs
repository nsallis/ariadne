// In your `proc_macro_crate/src/lib.rs`
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct, Data, Fields};

#[proc_macro_attribute]
pub fn define_as_grid(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(item as DeriveInput);
    // let original_name = &input.ident;

    // Construct a new name for the derived struct
    // let derived_name = syn::Ident::new(&format!("{}Grid", original_name), original_name.span());

    // // Extract fields from the original struct (example for named fields)
    // let fields = if let Data::Struct(data_struct) = &input.data {
    //     if let Fields::Named(fields_named) = &data_struct.fields {
    //         &fields_named.named
    //     } else {
    //         panic!("Only named fields are supported for this example.");
    //     }
    // } else {
    //     panic!("Only structs are supported for this example.");
    // };

    // let field_names = fields.iter().map(|f| &f.ident);
    // let field_types = fields.iter().map(|f| &f.ty);
    let input = item.clone();
    let original = item.clone();
    let input_struct = parse_macro_input!(item as ItemStruct);
    let ast = parse_macro_input!(input as DeriveInput);
    let original_struct = parse_macro_input!(original as DeriveInput);
    let struct_data = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("MyMacro can only be applied to structs");
    };
        let fields = if let Fields::Named(fields_named) = struct_data.fields {
        fields_named.named
    } else {
        panic!("MyMacro only supports structs with named fields");
    };
    let required_field_found = fields.iter().any(|field| {
        field.ident.as_ref().map_or(false, |ident| ident == "id")
    });

    if !required_field_found {
        panic!("Struct must contain a field named 'id'");
    }


    let original_name = input_struct.ident;
    let derived_name = syn::Ident::new(&format!("{}Grid", original_name), original_name.span());
     let fields = &input_struct.fields;
     let foo = quote!(pub foobar);


    let exp = quote! {
        #original_struct

        pub struct #derived_name {
            pub list: Vec<#original_name>
        }

        impl #derived_name {
            // add etc.
        }
    };

    exp.into()
}
