use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[proc_macro_derive(Shape)]
pub fn rect_shape_derive(input: TokenStream) -> TokenStream {
    //Use syn to parse the input
    let ast = syn::parse(input).unwrap();
    

    //build the trait implementation
    impl_rect_shape(&ast)
}

fn impl_rect_shape(ast: &DeriveInput) -> TokenStream {
    
    //get the name of the struct or enum for which shape implementation is to be derived
    let name = &ast.ident;

    //generate our shape implementation for the #name above
    let gen = quote! {

        impl Shape for #name {
            ///Associated function used to create a new shape
            /// Not a method because it is not taking in self
        
            fn new(length: f32, width: f32, name: &'static str) -> Self {
                // If the name of the element is the same as the name of the variable, you can just write it alone like that
                #name {
                    length,
                    width,
                    name,  
                }
            }
        
            ///Area method
            fn area(&self) -> f32 {
                self.length * self.width
            }
        
            fn get_length(&self) -> f32 {
                self.length
            }
        
            fn set_length(&mut self, length: f32) {
                self.length = length;
            }
        
            fn get_width(&self) -> f32 {
                self.width
            }
        
            fn set_width(&mut self, width: f32) {
                self.width = width;
            }
        
            fn get_name(&self) -> &'static str {
                self.name
            }
        
            fn set_name(&mut self, name: &'static str) {
                self.name = name;
            }
        }
        
    };

    gen.into();
print!("iodns");
}
