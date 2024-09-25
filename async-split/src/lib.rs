use proc_macro::TokenStream;

#[derive(Debug)]
enum Parsed {
    ItemFn(syn::ItemFn),
    TraitFn(syn::TraitItemFn),
    // TODO: Add closure and maybe statement blocks
}

fn parse_type(input: syn::parse::ParseStream<'_>) -> Result<Parsed, syn::Error>{
    
//     // Most cases where we need to separate between async and non-async are standard functions.
//     // First check if we are trying to parse one of those.
//     // We also need to do this first because normal functions can be incorrectly parsed as trait functions
//     if let Ok(item_fn) = input.fork().parse::<syn::ItemFn>() {
//         println!("Just a function");
//         Ok(Parsed::ItemFn(item_fn))
//     } else {
//         println!("You should not be here");
//         // Alternatively, we might be working with a trait function. Attempt to parse as such.
//         // let a = input.parse::<syn::TraitItemFn>().map(Parsed::TraitFn);
//         // a
//         panic!("Nope");
//     }
    match dbg!(input.fork()).parse::<syn::ItemFn>() {
        Ok(item_fn) => Ok(Parsed::ItemFn(item_fn)),
        Err(_) => {
            input.parse::<syn::TraitItemFn>().map(Parsed::TraitFn)
        },
    }
}

// #[proc_macro_attribute]
// pub fn sdk_async(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     use quote::ToTokens;
//     let mut parsed = syn::parse_macro_input!(item with parse_type);

//     let signature = match &mut parsed {
//         Parsed::ItemFn(item_fn) => &mut item_fn.sig,
//         Parsed::TraitFn(trait_item_fn) => &mut trait_item_fn.sig,
//     };

//     #[cfg(not(feature = "async"))]
//     {
//         signature.asyncness = None;
//     }
    
//     #[cfg(feature = "async")]
//     {
//         signature.asyncness = Some(syn::Token![async](Span::call_site()));
//     }
    
//     dbg!(match parsed {
//         Parsed::ItemFn(item_fn) => item_fn.to_token_stream().into(),
//         Parsed::TraitFn(trait_item_fn) => trait_item_fn.to_token_stream().into(),
//     })
// }
#[proc_macro_attribute]
pub fn sdk_async(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use quote::ToTokens;
    let mut parsed = syn::parse_macro_input!(item with parse_type);

    let signature = match &mut parsed {
        Parsed::ItemFn(item_fn) => &mut item_fn.sig,
        Parsed::TraitFn(trait_item_fn) => &mut trait_item_fn.sig,
    };

    #[cfg(not(feature = "async"))]
    {
        signature.asyncness = None;
    }
    
    #[cfg(feature = "async")]
    {
        signature.asyncness = Some(syn::Token![async](Span::call_site()));
    }

    match parsed {
        Parsed::ItemFn(item_fn) => item_fn.to_token_stream().into(),
        Parsed::TraitFn(trait_item_fn) => trait_item_fn.to_token_stream().into(),
    }
}
