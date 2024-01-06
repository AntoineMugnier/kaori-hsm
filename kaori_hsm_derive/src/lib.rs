use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, Ident, ItemImpl, Token};
/// Macro to call before every implementation of the `State<>` trait.
/// Allow to decrease verbosity of the trait implementation.
/// This is what the macro does:
/// - Create an empty structure named after the tag sent as a generic parameter
/// in the `State<>` trait implementation
/// - Implement the `State::get_parent_state()` method using the state tag of the parent provided
/// as the value of `super_state`.
///
/// There are two use cases of the macro depending on the category of the parent state (see example
/// below).
/// The first case being when the state has the top state as a parent. In this case, use the `Top`
/// keyword to define the value of `super_state`.
/// The second case being when the parent state is another user-defined state. In this case
/// set your custom state as the name of `super_state`.
/// ```rust,ignore
///# enum BasicEvt{A};
///# struct BasicStateMachine{}
///# impl TopState for BasicStateMachine{
///#     type Evt = BasicEvt;
///#
///#     fn init(&mut self) -> InitResult<Self> {
///#         init_transition!(S1)
///#     }
///# }
///
/// #[state(super_state= Top)]
/// impl State<S1> for BasicStateMachine {
///     fn init(&mut self) -> InitResult<Self> {
///         init_transition!(S11)
///     }
///
///     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///        match evt{
///            BasicEvt::A => {
///                handled!()
///            }
///            _ => ignored!()
///        }
///    }    
/// }
/// #[state(super_state= S1)]
/// impl State<S11> for BasicStateMachine {
///     fn handle(&mut self, evt: & BasicEvt) -> HandleResult<Self> {
///        match evt{
///            BasicEvt::A => {
///                transition!(S1)
///            }
///            _ => ignored!()
///        }
///    }
/// }
/// ```

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
        let expected_str = "struct StateName { } impl kaori_hsm :: State < StateName > for UserStateMachine { fn get_parent_state () -> kaori_hsm :: ParentState < Self > { kaori_hsm :: ParentState :: TopReached } }";
        let res = crate::state_impl(attr_tokens, item_tokens);
        assert_eq!(expected_str, res.to_string());
    }
}
