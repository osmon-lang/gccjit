#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

fn derive_typeable(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let name = s.ast().ident.to_string();

    let body = s.each(|bi| {let fname = bi.ast().ident.clone().unwrap(); quote!{ctx.new_field(None,#bi::get_type(),&#fname),}});
    s.bound_impl(quote!(gccjit_rs::ty::Typeable),quote! {
        fn get_type<'a,'ctx>(ctx: &'a gccjit_rs::ctx::Context<'ctx>) -> gccjit_rs::ty::Type<'a> {
            let fields = vec![#body];
            ctx.new_struct_type(None,&#name,&fields).as_type()
        }
    })
}

decl_derive!([Typeable] => derive_typeable);