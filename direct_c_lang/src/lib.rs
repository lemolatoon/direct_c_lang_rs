use proc_macro::TokenStream;

#[proc_macro]
pub fn c_lang(tokens: TokenStream) -> TokenStream {
    c_lang_impl(tokens)
}

fn c_lang_impl(tokens: TokenStream) -> TokenStream {
    todo!()
}