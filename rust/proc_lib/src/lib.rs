use proc_macro::TokenStream;

/// to be used on the `main` function like `#[advent_of_code::main("24/11")]`.
/// 
/// reads the input file for the given year and day into `const INPUT: &str` (in the `main` function).
/// it also sets up heap profiling with dhat-rs.
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the attribute as a string literal (assuming the input ID in "YY/DD" format)
    let input_id = syn::parse_macro_input!(attr as syn::LitStr);
    let input_file_path = syn::LitStr::new(&format!("../../../input/{}.txt", input_id.value()), input_id.span());

    // parse the input as a function (assuming the main function)
    let main_function_body = syn::parse_macro_input!(item as syn::ItemFn).block;

    // return the following code
    TokenStream::from(quote::quote! {
        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() {
            #[cfg(feature = "dhat-heap")]
            let _dhat_heap_profiler = dhat::Profiler::new_heap();

            // read the input file's content at compile time
            // and embed it into the binary
            const INPUT: &str = include_str!(#input_file_path);

            #main_function_body
        }
    })
}
