use std::str::FromStr;
use syn::{ItemStruct,parse::Parse, parse_macro_input, Token, bracketed, parenthesized};
use proc_macro2::TokenStream;
use kaorust_derive::state;

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
           "name" => {
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
struct S1;

//#[state(name = S1, super_state = Top)]
//impl State<name> for UserStateMachine{
//}

fn main() {
    let attr = "name = S1, super_state_name = Top";
    let item = "impl State<name> for UserStateMachine{ }";
    
    let attr_tokens = TokenStream::from_str(attr).unwrap();
    let attr_ast : AttrStateDecl = syn::parse2(attr_tokens).unwrap(); 
    println!("{:?} {:?}", attr_ast.state_name, attr_ast.super_state_name);    
    //let item_tokens = TokenStream::from_str(item).unwrap();
    //let item_ast : syn::ItemStruct = syn::parse2(item_tokens).unwrap();

   //let attr_ast = parse_macro_input!(attr_tokens as AttributeArgs);
    //let at = attr_tokens as Att
    //println!("IS  {}", ident);
    //crate::state_impl(attr_tokens, item_tokens); 
}
