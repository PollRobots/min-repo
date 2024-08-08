use proc_macro::{TokenStream, TokenTree};
use quote::{quote, quote_spanned};

#[proc_macro]
pub fn fizzbuz(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut index = 0;
    let mut errors = Vec::new();
    loop {
        index += 1;

        match &iter.next() {
            Some(token_tree) => {
                let expected = match (index % 3, index % 5) {
                    (0, 0) => Some("fizz_buzz"),
                    (0, _) => Some("fizz"),
                    (_, 0) => Some("buzz"),
                    _ => None,
                };

                match expected {
                    Some(value) => {
                        if !matches!(token_tree, TokenTree::Ident(ident) if ident.to_string() == value)
                        {
                            let msg = format!("Expecting {value}");
                            errors.push(
                                quote_spanned! {token_tree.span().into() => compile_error!(#msg)},
                            )
                        }
                    }
                    None => {
                        if !matches!(token_tree, TokenTree::Literal(literal) if literal.to_string() == index.to_string())
                        {
                            let msg = format!("Expecting {index}");
                            errors.push(
                                quote_spanned! {token_tree.span().into() => compile_error!(#msg)},
                            )
                        }
                    }
                }
            }
            None => break,
        }

        // eat comma
        match &iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => {}
            Some(token_tree) => errors
                .push(quote_spanned! {token_tree.span().into() => compile_error!("Expecting ','")}),
            None => break,
        }
    }

    if errors.is_empty() {
        quote! { true }.into()
    } else {
        quote! { [ #(#errors),* ]}.into()
    }
}
