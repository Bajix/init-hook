use darling::{ast::NestedMeta, error::Accumulator};
use quote::quote;
use syn::{Ident, ItemFn, parse_macro_input, spanned::Spanned};

/// Register function to be called during initialization
#[proc_macro_attribute]
pub fn call_on_init(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut errors = Accumulator::default();

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(err) => {
            return darling::Error::from(err).write_errors().into();
        }
    };

    for nested_meta in attr_args {
        errors.push(darling::Error::custom("args not supported").with_span(&nested_meta.span()));
    }

    let input = parse_macro_input!(input as ItemFn);

    if input.sig.asyncness.is_some() {
        errors.push(
            darling::Error::custom(
                "the `async` keywork cannot be used on the function declaration",
            )
            .with_span(&input.sig.fn_token.span()),
        );
    }

    if let Err(err) = errors.finish() {
        return err.write_errors().into();
    }

    let ident = input.sig.ident.clone();
    let static_ident = Ident::new(&ident.to_string().to_uppercase(), ident.span());

    let expanded = if input.sig.unsafety.is_some() {
        quote! {
            #[init_hook::linkme::distributed_slice(init_hook::__private::UNSAFE_INIT_FNS)]
            #[linkme(crate = init_hook::linkme)]
            static #static_ident: unsafe fn() = #ident;

            #input
        }
    } else {
        quote! {
            #[init_hook::linkme::distributed_slice(init_hook::__private::INIT_FNS)]
            #[linkme(crate = init_hook::linkme)]
            static #static_ident: fn() = #ident;

            #input
        }
    };

    expanded.into()
}
