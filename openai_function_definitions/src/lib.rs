use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, token::Token};
use quote::quote;

#[proc_macro_attribute]
pub fn generate_function_schema(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // panic!("got here!");
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();

    // let mut properties = quote! {};
    // let mut required = quote! {};

    let mut properties_vec: Vec<TokenStream> = vec![];
    let mut required_vec: Vec<TokenStream> = Vec::new();
    // for input in &input.sig.inputs {
    //     if let syn::FnArg::Typed(arg) = input {
    //         let arg_name = &arg.pat;
    //         let arg_type = &arg.ty;

    //         // Determine the JSON type based on the Rust type
    //         let json_type = match *arg_type.to_owned() {
    //             syn::Type::Reference(ref_type) => {
    //                 if let syn::Type::Path(syn::TypePath { qself: None, path }) = &*ref_type.elem {
    //                     if path.is_ident("str") {
    //                         quote! { "string" }
    //                     } else {
    //                         quote! { "unknown" }
    //                     }
    //                 } else {
    //                     quote! { "unknown" }
    //                 }
    //             },
    //             _ => quote! { "unknown" }
    //         };

    //         // properties.extend(quote! {
    //         //     stringify!(#arg_name): {
    //         //         "type": #json_type,
    //         //     },
    //         // });

    //         // required.extend(quote! {
    //         //     stringify!(#arg_name),
    //         // });

    //         properties_vec.push(quote! {
    //             stringify!(#arg_name): {
    //                 "type": #json_type
    //             }
    //         });

    //         required_vec.push(quote! {
    //             stringify!(#arg_name)
    //         });
    //     }
    // }
    // panic!("got here!");


    // let properties = quote! { #(#properties_vec),* };
    // let required = quote! { #(#required_vec),* };

    let expanded = quote! {
        #input

        const SCHEMA: &str = #fn_name_str;// {
        //     concat!(
        //         "{",
        //             "\"name\": ", #fn_name_str.as_str(), ",",
        //             "\"description\": null,",
        //             "\"parameters\": {",
        //                 "\"type\": \"object\",",
        //                 "\"properties\": {",
        //                     // #properties,
        //                 "},",
        //                 "\"required\": [",
        //                     // #required,
        //                 "]",
        //             "}",
        //         "}"
        //     )
        // };
    };

    TokenStream::from(expanded)
}
