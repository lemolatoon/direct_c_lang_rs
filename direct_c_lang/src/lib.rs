use ironcc::{
    analyze::{BaseType, ConvFuncDef, Type},
    preprocess_and_compile,
};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn c_lang(tokens: TokenStream) -> TokenStream {
    c_lang_impl(tokens)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(target_arch = "x86_64")]
fn c_lang_impl(tokens: TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
    use proc_macro2::Span;
    use quote::format_ident;

    let src = tokens.to_string();
    let asm = preprocess_and_compile(src.clone()).unwrap_or_else(|err| panic!("{:?}", err));
    let ast = ironcc::converted_ast(src).unwrap_or_else(|err| {
        syn::Error::new(Span::call_site(), format!("{:?}", err)).into_compile_error();
        unreachable!()
    });
    let mut declarations = quote!();
    for program in ast {
        if let ironcc::analyze::ConvProgramKind::Func(func) = program {
            let ident = proc_macro2::Ident::new(&func.name, proc_macro2::Span::call_site());
            let ret_ty_tokens = if let Type::Func { ret_ty, .. } = func.ty.clone() {
                c_type_to_token_stream(*ret_ty)?
            } else {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "INTERNAL C COMPILER ERROR: ConvFuncDef's type MUST be Type::Func",
                ));
            };
            let mut args_token = quote!();
            for (idx, arg) in func
                .args
                .into_iter()
                .map(|lvar| lvar.ty)
                .map(c_type_to_token_stream)
                .enumerate()
            {
                let ident = format_ident!("arg{}", idx);
                let arg = arg?;
                if idx == 0 {
                    args_token = quote!(#ident : #arg);
                } else {
                    args_token = quote!(, #ident: #arg);
                }
            }
            declarations = quote!(
                #declarations
                fn #ident(#args_token) -> #ret_ty_tokens;
            );
        }
    }
    Ok(quote!(
        extern "C" {
            #declarations
        }
        global_asm!(#asm);
    ))
}

fn c_type_to_token_stream(ty: Type) -> Result<proc_macro2::TokenStream, syn::Error> {
    Ok(match ty {
        Type::Void => quote!(::core::ffi::c_void),
        Type::InComplete(_) => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "INTERNAL C COMPILER ERROR: C function's type MUST NOT be InComplete type",
        ))?,
        Type::Base(BaseType::Char) => quote!(::core::ffi:::c_char),
        Type::Base(BaseType::Int) => quote!(::core::ffi::c_int),
        Type::Ptr(ty) => {
            if let Type::Func {
                ret_ty,
                args,
                is_flexible,
            } = *ty
            {
                if is_flexible {
                    Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "C function with flexible args cannot be interpretered to Rust type",
                    ))?;
                }
                let args_tokens = args.into_iter().map(|ty| c_type_to_token_stream(ty)).fold(
                    Ok::<proc_macro2::TokenStream, syn::Error>(quote!()),
                    |acc, token| {
                        let acc = acc?;
                        let token = token?;
                        Ok(quote!(#acc , #token))
                    },
                )?;
                let ret_ty_token = c_type_to_token_stream(*ret_ty)?;
                quote!(fn(#args_tokens) -> #ret_ty_token)
            } else {
                let ptr_to_tokens = c_type_to_token_stream(*ty)?;
                quote!(*mut #ptr_to_tokens)
            }
        }
        Type::Func { .. } => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "INTERNAL C COMPILER ERROR: FUNCTION TYPE CANNOT BE INTERPRETERED FOR RUST TYPE",
        ))?,
        Type::Array(arr, size) => {
            let arr_base_ty = c_type_to_token_stream(*arr)?;
            let size = proc_macro2::Literal::usize_unsuffixed(size);
            quote!([#arr_base_ty; #size])
        }
        Type::Struct(_) => {
            unimplemented!("clang struct interpretation is not currently implemented")
        }
    })
}

#[cfg(not(target_arch = "x86_64"))]
fn c_lang_impl(tokens: TokenStream) -> TokenStream {
    compile_error!("only support x86_64 arch");
}
