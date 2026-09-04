use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

pub(super) fn expand(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let function_name = function.sig.ident.clone();
    let section_name = LitStr::new(
        &format!(".riscv_tests.{function_name}"),
        function_name.span(),
    );
    quote! {
        #function
        const _: () = {
            #[used]
            #[unsafe(link_section = #section_name)]

            static TEST_CASE: crate::testing::registry::Test =
                crate::testing::registry::Test::new(
                    concat!(
                        module_path!(),
                        "::",
                        stringify!(#function_name)
                    ),
                    #function_name,
                );
        };
    }
    .into()
}
