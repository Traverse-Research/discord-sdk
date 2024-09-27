use proc_macro::TokenStream;
use syn::parse::discouraged::Speculative;

#[derive(Debug)]
enum Parsed {
    ItemFn(syn::ItemFn),
    TraitFn(syn::TraitItemFn),
}

fn parse_type(input: syn::parse::ParseStream<'_>) -> Result<Parsed, syn::Error> {
    // Most cases where we need to separate between async and non-async are standard functions.
    // First check if we are trying to parse one of those.
    // We also need to do this first because normal functions can be incorrectly parsed as trait functions
    let fork = input.fork();
    if let Ok(item_fn) = fork.parse::<syn::ItemFn>() {
        input.advance_to(&fork);
        Ok(Parsed::ItemFn(item_fn))
    } else {
        // Alternatively, we might be working with a trait function. Attempt to parse as such.
        input.parse::<syn::TraitItemFn>().map(Parsed::TraitFn)
    }
}

#[proc_macro_attribute]
pub fn sdk_async(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use quote::ToTokens;
    let mut parsed = syn::parse_macro_input!(item with parse_type);

    let asyncness = match &mut parsed {
        Parsed::ItemFn(item_fn) => &mut item_fn.sig.asyncness,
        Parsed::TraitFn(trait_item_fn) => &mut trait_item_fn.sig.asyncness,
    };

    #[cfg(not(feature = "async"))]
    {
        *asyncness = None;
    }

    #[cfg(feature = "async")]
    {
        *asyncness = Some(syn::Token![async](Span::call_site()));
    }

    match parsed {
        Parsed::ItemFn(item_fn) => item_fn.to_token_stream().into(),
        Parsed::TraitFn(trait_item_fn) => trait_item_fn.to_token_stream().into(),
    }
}
