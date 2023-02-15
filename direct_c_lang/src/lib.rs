use ironcc::preprocess_and_compile;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn c_lang(tokens: TokenStream) -> TokenStream {
    c_lang_impl(tokens)
}

#[cfg(target_arch = "x86_64")]
fn c_lang_impl(tokens: TokenStream) -> TokenStream {
    let src = tokens.to_string();
    let asm = preprocess_and_compile(src).unwrap_or_else(|err| panic!("{:?}", err));
    quote!(
        global_asm!(#asm);
    )
    .into()
}
#[cfg(not(target_arch = "x86_64"))]
fn c_lang_impl(tokens: TokenStream) -> TokenStream {
    panic!("only support x86_64 arch");
}
