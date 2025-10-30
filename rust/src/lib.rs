use proc_macro::TokenStream;

/// supposed to be used on the `main` function to set up heap profiling with `dhat-rs`
#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    // parse the input as a function (assuming the main function)
    let main_function: syn::ItemFn = syn::parse_macro_input!(item);
    let main_function_body = &main_function.block;

    // return the following code
    TokenStream::from(quote::quote! {
        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() {
            #[cfg(feature = "dhat-heap")]
            let _dhat_heap_profiler = dhat::Profiler::new_heap();

            #main_function_body
        }
    })
}
