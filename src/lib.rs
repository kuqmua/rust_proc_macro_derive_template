#[proc_macro_derive()]
pub fn (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //todo - use panic_location function from https://github.com/kuqmua/proc_macro_helpers instead of this std::panic::set_hook (reusage)
    std::panic::set_hook(Box::new(move |panic_info| {
        if let Some(location) = panic_info.location() {
            eprintln!("{proc_macro_name} panic occurred in {}:{}:{}", location.file(), location.line(), location.column());
        } else {
            eprintln!("{proc_macro_name} panic occurred but can't get location information...");
        }
    }));
    let proc_macro_name = "";
    let ast: syn::DeriveInput =
        syn::parse(input).unwrap_or_else(|_| panic!("{proc_macro_name} let ast: syn::DeriveInput = syn::parse(input) failed"));
    let ident = &ast.ident;
    match ast.data {
        syn::Data::Union(_) => panic!("{proc_macro_name} does not work on union!"),
        syn::Data::Struct(_) => panic!("{proc_macro_name} does not work on structs!"),
        syn::Data::Enum(_) => panic!("{proc_macro_name} does not work on enums!"),
    }
    let gen = quote::quote! {};
    //println!("{gen}");
    gen.into()
}
