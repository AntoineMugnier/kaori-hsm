use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, visit_mut::VisitMut, Ident, ItemImpl, Token};

#[proc_macro_attribute]
pub fn state(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let output_token_stream = state_impl(
        proc_macro2::TokenStream::from(args),
        proc_macro2::TokenStream::from(item),
    );
    proc_macro::TokenStream::from(output_token_stream)
}

struct AttrStateDecl {
    super_state_tag: syn::Ident,
}

impl Parse for AttrStateDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let super_state_tag;

        let attr_name = input.parse::<syn::Ident>()?;

        match attr_name.to_string().as_str() {
            "super_state" => {
                input.parse::<Token![=]>()?;
                super_state_tag = input.parse::<syn::Ident>()?;
            }
            _ => {
                return Err(syn::Error::new(
                    attr_name.span(),
                    "expected field `super_state_name`",
                ))
            }
        }

        Ok(AttrStateDecl { super_state_tag })
    }
}

fn get_user_state_tag_from_item_impl_ast(item_impl_ast: &ItemImpl) -> Ident {
    let item_ast_trait = item_impl_ast.trait_.clone().unwrap().1;

    for segment in item_ast_trait.segments {
        if segment.ident.to_string() == "State" {
            if let syn::PathArguments::AngleBracketed(generic_arguments) = segment.arguments {
                let first_generic_argument = generic_arguments.args.first().unwrap();
                if let syn::GenericArgument::Type(first_generic_argument) = first_generic_argument {
                    if let syn::Type::Path(first_generic_argument) = first_generic_argument {
                        let first_generic_argument_ident =
                            first_generic_argument.path.segments[0].ident.clone();
                        return first_generic_argument_ident;
                    }
                }
            }
        }
    }
    panic!()
}

pub(crate) fn state_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    // Get the tag of the super state
    let attr_ast: AttrStateDecl = syn::parse2(args).unwrap();
    let super_state_tag_ident = attr_ast.super_state_tag;

    // Get the tag of the current state
    let mut item_ast: ItemImpl = syn::parse2(item).unwrap();
    let user_state_tag_ident = get_user_state_tag_from_item_impl_ast(&item_ast);

    // Create the function that will return the fn pointer to the super state
    let get_super_state_fn: syn::ImplItemFn;

    if super_state_tag_ident.to_string() == "Top" {
        get_super_state_fn = syn::parse2(quote!(
            fn get_parent_state() -> kaori_hsm::ParentState<Self> {
                kaori_hsm::ParentState::TopReached
            }
        ))
        .unwrap();
    } else {
        get_super_state_fn = syn::parse2(quote!(
            fn get_parent_state() -> kaori_hsm::ParentState<Self> {
                kaori_hsm::ParentState::Exists(kaori_hsm::State::<#super_state_tag_ident>::core_handle)
            }
        ))
        .unwrap();
    }

    let get_super_state_impl_item_fn = syn::ImplItem::Fn(get_super_state_fn);

    // Push the function into the impl item AST
    item_ast.items.push(get_super_state_impl_item_fn);

    // Generate code from the item impl AST
    quote! {struct #user_state_tag_ident{ } #item_ast}.into()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_state_impl() {
        let attr = "super_state = Top";
        let item = "impl kaori_hsm::State<StateName> for UserStateMachine{ }";

        let attr_tokens = TokenStream::from_str(attr).unwrap();
        let item_tokens = TokenStream::from_str(item).unwrap();
        let expected_str = "struct StateName { } impl kaori_hsm :: State < StateName > for UserStateMachine { fn get_parent_state () -> ParentState < Self > { ParentState :: TopReached } }";
        let res = crate::state_impl(attr_tokens, item_tokens);
        assert_eq!(expected_str, res.to_string());
    }
}
