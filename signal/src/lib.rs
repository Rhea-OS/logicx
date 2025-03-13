use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Block, Expr, Token};
use syn::parse::{Parse, ParseStream};

// Full credit to ChatGPT for this one

struct SimpleClosure {
    is_move: bool,
    params: Vec<(bool, Ident)>,
    body: Expr,
}

impl Parse for SimpleClosure {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let is_move = input.peek(Token![move]);
        if is_move {
            input.parse::<Token![move]>()?;
        }

        let mut params = Vec::new();
        input.parse::<Token![|]>()?; // Parse the opening pipe

        while !input.peek(Token![|]) {
            let mut mutable = false;

            if input.peek(Token![mut]) {
                input.parse::<Token![mut]>()?;
                mutable = true;
            }

            let ident: Ident = input.parse()?;
            params.push((mutable, ident));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        input.parse::<Token![|]>()?; // Parse the closing pipe
        let body: Expr = input.parse()?; // Parse any expression, block or otherwise

        Ok(SimpleClosure {
            is_move,
            params,
            body,
        })
    }
}

#[proc_macro]
pub fn signal(input: TokenStream) -> TokenStream {
    let closure = syn::parse_macro_input!(input as SimpleClosure);

    let transformed_stmts = closure.params.iter().map(|(is_mut, ident)| {
        if *is_mut {
            quote! {
                let mut #ident = #ident.write();
                // let mut #ident = std::ops::DerefMut::deref_mut(&#ident);
            }
        } else {
            quote! {
                let #ident = #ident.read();
                // let #ident = std::ops::Deref::deref(&#ident);
            }
        }
    });

    let body = &closure.body;
    let move_token = if closure.is_move { quote! { move } } else { quote! {} };

    let expanded = quote! {
        {
            #(#transformed_stmts)*
            #move_token { #body }
        }
    };

    expanded.into()
}