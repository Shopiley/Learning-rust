// comes with rust, allows us to read and manipulate rust code
extern crate proc_macro;

use proc_macro::TokenStream; // data type of the input and output of a proc_macro

//short for syntax, allows us to take a string of rust code and turn it into a 
// syntax tree data structure that we can operate on
use syn;

// does the opposite
use quote::quote;

#[proc_macro_derive(HelloMacro)]    //decorator to indicate that this is a custom-derived macro named HelloMacro + the trait to be implemented
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // this function constructs an abstract syntax tree from the rust code
    let ast = syn::parse(input).unwrap();   //unwrap - function will panic if parsing fails

    // this function does the actual work, it contains the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; //the name of the name of the struct or enum for which shape implementation is to be derived
    let gen = quote! {

        //generate our trait implementation for the #name above
        impl HelloMacro for #name {
            // custom implementation for associated trait function
            fn hello_macro() {
                println! (
                    "Hello, Macro! My name is {}!", stringify!(#name)
                );
            }
        }
    };

    // turn the output of the quote macro into a token stream 
    gen.into()
}