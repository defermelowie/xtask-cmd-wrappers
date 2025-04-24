use darling::{ast::NestedMeta, FromMeta};
use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, ExprMethodCall, Field, FnArg, Ident, ImplItemFn, ItemImpl,
    ItemStruct,
};

/// Meta items for `#[cmd(...)]`
#[derive(Debug, FromMeta)]
struct CmdMeta {
    #[darling(default)]
    name: Option<String>,
}

/// The default prefix of a command argument.
///
/// i.e. the "--" in `ls --version`
fn default_prefix() -> String {
    "--".to_string()
}

/// Meta items for #[cmd::opt(...)]
#[derive(Debug, FromMeta)]
struct CmdOptMeta {
    #[darling(default)]
    no_val: bool,

    #[darling(default)]
    no_opt: bool,

    #[darling(default)]
    name: Option<String>,

    #[darling(default=default_prefix)]
    prefix: String,
}

/// Generate the constructor for a [std::process::Command] wrapper with a fixed `program` parameter.
fn constructor(ident: &Ident, program: String) -> ImplItemFn {
    parse_quote! {
        pub fn new() -> #ident {
            #ident(std::process::Command::new(#program))
        }
    }
}

/// Convert the field of a command struct to it's setterI
fn setter(field: &Field) -> ImplItemFn {
    // Destruct generic field info
    let ident = field.ident.as_ref().unwrap();
    let ty = &(field.ty);
    let doc_attr = field.attrs.iter().find(|a| a.path().is_ident("doc"));
    let arg_attr = field.attrs.iter().find(|a| a.path().is_ident("arg"));

    // Parse meta items of #[arg(...)]
    let cmd_opt;
    let mut errors = proc_macro2::TokenStream::new();
    if let Some(a) = arg_attr {
        cmd_opt = CmdOptMeta::from_meta(&a.meta)
            .map_err(|e| errors = e.write_errors())
            .ok();
    } else {
        cmd_opt = None;
        errors = quote! {compile_error!(concat!("Missing #[arg(...)] attribute on field: ", stringify!(#ident)))};
    }
    // FIXME: Check that no_val and no_opt aren't both set

    // Generate dummy function on error
    if !errors.is_empty() {
        let dummy_ident = format_ident!("{}_err", ident);
        return parse_quote! {
            pub fn #dummy_ident() {
                #errors
            }
        };
    }

    // No error: generate code from options
    let cmd_opt = cmd_opt.unwrap();
    let opt_prefix = cmd_opt.prefix;
    let opt_name: String = match cmd_opt.name {
        Some(s) => s,
        None => ident.to_string().to_lowercase(),
    };
    let opt = format!("{}{}", opt_prefix, opt_name);
    let param_val: Option<FnArg> = match cmd_opt.no_val {
        true => None,
        false => Some(parse_quote! {val: #ty}),
    };
    let arg_val: Option<ExprMethodCall> = match cmd_opt.no_val {
        true => None,
        false => Some(parse_quote! {self.0.arg(val)}),
    };
    let arg_opt: Option<ExprMethodCall> = match cmd_opt.no_opt {
        true => None,
        false => Some(parse_quote! {self.0.arg(#opt)}),
    };

    // Generate setter function
    parse_quote! {
        #doc_attr
        pub fn #ident(&mut self, #param_val) -> &mut Self {
            #arg_opt;
            #arg_val;
            self
        }
    }
}

/// Get string representation of command
fn string_repr() -> ImplItemFn {
    parse_quote! {
        pub fn string_repr(&self) -> String {
            let program = self.0.get_program().to_string_lossy();
            let args = self.0
                .get_args()
                .map(|a| a.to_string_lossy())
                .fold(String::new(), |s, a| s + " " + &a);
            format!("{}{}", program, args)
        }
    }
}

fn get_inner() -> ImplItemFn {
    parse_quote! {
        pub fn cmd(self) -> std::process::Command {
            self.0
        }
    }
}

/// Implementation of [Into<std::process::Command>]
fn into_std_command(ident: &Ident) -> ItemImpl {
    parse_quote! {
        impl Into<std::process::Command> for #ident {
            fn into(self) -> std::process::Command {
                self.cmd()
            }
        }
    }
}

#[proc_macro_attribute]
pub fn cmd(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse top level attributes
    let attr = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.into_compile_error()),
    };
    let attr = match CmdMeta::from_list(&attr) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    // Parse the input as Struct
    let item = parse_macro_input!(input as ItemStruct);

    // Extract useful info
    let ident = &(item.ident);
    let name = attr.name.unwrap_or(ident.to_string().to_lowercase());

    // Create constructor
    let constructor = constructor(ident, name);

    // Extract setters
    let setters = item.fields.iter().map(|f| setter(f)).collect_vec();

    // Create string representation function
    let string_repr = string_repr();

    // Create getter for inner command item
    let inner = get_inner();

    // Generate Into<std::process::Command> trait
    let into_std_command = into_std_command(ident);

    // Generate expanded code
    let expanded = quote! {
        struct #ident(std::process::Command);
        impl #ident {
            #constructor
            #inner
            #(#setters)*
            #string_repr
        }
        #into_std_command
    };

    TokenStream::from(expanded)
}
