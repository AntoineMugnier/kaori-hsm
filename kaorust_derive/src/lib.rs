use syn::{ItemImpl, parse::Parse, visit_mut::VisitMut, Token, Ident};
use quote::quote;
use proc_macro2::{TokenStream, Span};

#[proc_macro_attribute]
pub fn state(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output_token_stream = state_impl(proc_macro2::TokenStream::from(args), proc_macro2::TokenStream::from(item));
    proc_macro::TokenStream::from(output_token_stream)
}

struct AttrStateDecl{
    state_name : syn::Ident,
    super_state_name : syn::Ident
}

impl Parse for AttrStateDecl{

    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let state_name;
    let super_state_name;

        let attr_name = input.parse::<syn::Ident>()?;
        
    match attr_name.to_string().as_str(){
           "state_name" => {
                input.parse::<Token![=]>()?;
                state_name = input.parse::<syn::Ident>()?;
            }
             _ => { return Err(syn::Error::new(attr_name.span(), "expected field `name`"))}
        }

    input.parse::<Token![,]>()?;
    
        let attr_name = input.parse::<syn::Ident>()?;

        match attr_name.to_string().as_str(){
           "super_state_name" => {
                input.parse::<Token![=]>()?;
                super_state_name = input.parse::<syn::Ident>()?;
            }
             _ => { return Err(syn::Error::new(attr_name.span(), "expected field `super_state_name`"))}
        }

        Ok(AttrStateDecl{
            state_name,
            super_state_name
        })
    }
}
struct Visitor{
    ident_to_replace : Ident,
    new_ident : Ident
}
impl VisitMut for Visitor{
    fn visit_ident_mut(&mut self, node: &mut Ident){
        if node.to_string() == self.ident_to_replace.to_string(){
            *node = self.new_ident.clone();
        } 
    }
}

pub(crate) fn state_impl(args: TokenStream, item: TokenStream)-> TokenStream{
     
    let attr_ast : AttrStateDecl = syn::parse2(args).unwrap();
    let mut item_ast : ItemImpl = syn::parse2(item).unwrap();
    
    let user_state_ident = attr_ast.state_name;
    
    let super_user_state_ident = attr_ast.super_state_name;
    
    let ident_to_replace = Ident::new("state_name",Span::call_site());
    
    let mut visitor = Visitor{ident_to_replace, new_ident: user_state_ident.clone()};
    visitor.visit_item_impl_mut(&mut item_ast);
    
    let get_super_state_fn: syn::ImplItemFn;
    
    if super_user_state_ident.to_string() == "Top"{
    
        get_super_state_fn = syn::parse2( 
        quote!(
            fn get_parent_state() -> ParentState<Self> {
                ParentState::TopReached
            }
        )
    ).unwrap();
    }

    else{
         get_super_state_fn  = syn::parse2( 
            quote!(
                fn get_parent_state() -> ParentState<Self> {
                    ParentState::Exists(State::<#super_user_state_ident>::core_handle)        
                }
            )
        ).unwrap();
    }

   let get_super_state_impl_item_fn = syn::ImplItem::Fn(get_super_state_fn); 
    item_ast.items.push(get_super_state_impl_item_fn);
    
    //panic!("{:?}", item_ast.items[0]);
    
    quote! {struct #user_state_ident{ } #item_ast}.into()

}

#[cfg(test)]
mod test{
use std::str::FromStr;
use super::*;

#[test]
fn test_just_for_fn() {
    let attr = "state_name = S1, super_state_name = Top";
    let item = "impl State<state_name> for UserStateMachine{ }";
    
    let attr_tokens = TokenStream::from_str(attr).unwrap();
    let item_tokens = TokenStream::from_str(item).unwrap();

    let res = crate::state_impl(attr_tokens, item_tokens);
    panic!("{}", res.to_string());
   }
}
