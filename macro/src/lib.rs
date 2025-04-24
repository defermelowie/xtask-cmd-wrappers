use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Field, Ident, ImplItemFn, ItemStruct};
use darling::FromMeta;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: Parse the top level attribute

    // Parse the input as Struct
    let struct_ = parse_macro_input!(input as ItemStruct);
    let struct_name = struct_.ident;

    // Loop over struct fields
    let mut setters = Vec::new();
    for field in struct_.fields.iter() {
        // Get field's identifier (name)
        let field_ident = field
            .ident
            .as_ref()
            .expect("Only structs with named fields can represent commands");

        // Extract field's type
        let field_type = &(field.ty);

        // Extract field's docstring
        let field_docstring = field.attrs.iter().find(|a| a.path().is_ident("doc"));

        // Extract field's #[arg(...)] attribute
        let arg_chain = match field.attrs.iter().find(|a| a.path().is_ident("arg")) {
            Some(a) => match a.parse_args::<Ident>().unwrap().to_string().as_str() {
                "single" => quote! {.arg(concat!("-", stringify!(#field_ident))) },
                "double" => quote! {.arg(concat!("--", stringify!(#field_ident))) },
                _ => panic!("Only #[arg(single)] and #[arg(double)] are allowed"),
            },
            None => TokenStream::new().into(),
        };

        // DEBUG: try to check if bool
        let setter: ImplItemFn = if field_type == &(parse_quote! { bool }) {
            parse_quote! {
                    #field_docstring
                    pub fn #field_ident(&mut self) -> &mut Self {
                        self.0#arg_chain;
                        self
                    }
            }
        } else {
            parse_quote! {
                    #field_docstring
                    pub fn #field_ident(&mut self, val: #field_type) -> &mut Self {
                        self.0#arg_chain.arg(val);
                        self
                    }
            }
        };

        setters.push(setter);
    }

    // Create expanded code
    let expanded = quote! {
        struct #struct_name(std::process::Command);

        impl #struct_name {
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug, FromMeta)]
struct CmdOptMeta {
    flag: bool,

    #[darling(default)]
    name: Option<String>,

    prefix: String,
}

#[proc_macro_attribute]
pub fn command2(attr: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: Parse top level attributes
    let _attr = attr;

    // Parse the input as Struct
    let item = parse_macro_input!(input as ItemStruct);
    let name = &(item.ident);

    // Create constructor
    let program = name.to_string();
    let constructor: ImplItemFn = parse_quote! {
        pub fn new() -> Self {
            Self(std::process::Command::new(#program))
        }
    };

    // Extract setters
    let setters = item.fields.iter().map(|f| field_to_setter(f)).collect_vec();

    // Generate expanded code
    let expanded = quote! {
        struct #name(std::process::Command);
        impl #name {
            #constructor
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}

/// Convert the field of a command struct to it's setter
fn field_to_setter(field: &Field) -> ImplItemFn {
    // Destruct field info
    let ident = field.ident.as_ref().unwrap();
    let ty = &(field.ty);
    let doc_attr = field.attrs.iter().find(|a| a.path().is_ident("doc"));
    let arg_attr = field.attrs.iter().find(|a| a.path().is_ident("arg"));

    // Update options from #[arg(...)] attribute
    let arg_meta: ArgsAttrMeta = if let Some(attr) = arg_attr {
        match deluxe::parse(attr.to_token_stream().into()) {
            Ok(parsed) => parsed,
            Err(e) => panic!("Could not parse #[arg(...)]: {}", e),
        }
    } else {
        ArgsAttrMeta {
            flag: false,
            name: Some(ident.to_string().to_lowercase()),
            prefix: "--".to_string(),
        }
    };

    // Generate setter code
    if arg_meta.flag {
        parse_quote! {
            #doc_attr
            pub fn #ident(&mut self) -> &mut Self {
                todo!("Set flag options");
            }
        }
    } else {
        parse_quote! {
            #doc_attr
            pub fn #ident(&mut self, val: #ty) -> &mut Self {
                todo!("Set value options");
            }
        }
    }
}
