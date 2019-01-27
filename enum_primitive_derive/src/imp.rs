use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Type};

pub fn impl_enum_primitive_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let suffix = find_repr_ident(&ast).expect("please use #[repr(i32)]");
    let variants = find_variants(&ast);

    let r_map = gen_map_reader(&name, &variants, &suffix);

    let r_ident = Ident::new(&format!("read_{}", &suffix), Span::call_site());
    let w_ident = Ident::new(&format!("write_{}", &suffix), Span::call_site());

    let gen = quote! {
        impl enum_primitive::EnumPrimitive for #name {
            fn #r_ident(i: #suffix) -> Option<#name>{
                #r_map
                MAP.get(&i).map(|x|unsafe{std::mem::transmute_copy(x)})
            }

            fn #w_ident(&self) -> Option<#suffix>{
                Some(unsafe{std::mem::transmute_copy(self)})
            }
        }
    };

    gen
}

fn gen_map_reader(name: &Ident, variants: &[&Ident], type_ident: &Ident) -> TokenStream {
    let premap: Vec<TokenStream> = variants
        .iter()
        .map(|ident| quote! {#ident as #type_ident, #ident})
        .collect();

    quote! {
        use enum_primitive::lazy_static;
        use std::collections::HashMap;
        lazy_static! {
            static ref MAP: HashMap<#type_ident, #name> = {
                let mut map = HashMap::<#type_ident, #name>::new();
                {
                    use #name::*;
                    #( map.insert(#premap); )*
                }
                map
            };
        }
    }
}

fn find_repr_ident(ast: &DeriveInput) -> Option<Ident> {
    for x in ast.attrs.iter() {
        let ident = &x.path.segments.first().unwrap().value().ident;
        if ident == "repr" {
            let ast: Type = syn::parse2(x.tts.clone()).unwrap();
            if let Type::Paren(paren) = ast {
                if let Type::Path(path) = (*paren.elem).clone() {
                    if let Some(ident) = path
                        .path
                        .segments
                        .first()
                        .map(|x| x.into_value().ident.clone())
                    {
                        return Some(ident);
                    }
                }
            }
        }
    }
    None
}

fn find_variants<'a>(ast: &'a DeriveInput) -> Vec<&'a Ident> {
    if let Data::Enum(ref data) = ast.data {
        let mut variants = Vec::new();
        for x in data.variants.iter() {
            variants.push(&x.ident);
        }
        variants
    } else {
        panic!("EnumPrimitive applicable to enum only")
    }
}
