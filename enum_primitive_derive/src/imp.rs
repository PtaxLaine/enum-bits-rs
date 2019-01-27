use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Type};

pub fn impl_enum_primitive_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let suffix = find_repr_ident(&ast).unwrap_or_else(|| Ident::new("i32", Span::call_site()));
    let variants = find_variants(&ast);

    let r_map = gen_map_reader(&name, &variants, &suffix);

    let r_ident = Ident::new(&format!("read_{}", &suffix), Span::call_site());
    let w_ident = Ident::new(&format!("write_{}", &suffix), Span::call_site());

    let gen = quote! {
        impl enum_primitive::EnumPrimitive for #name where #name: Clone {
            fn #r_ident(i: #suffix) -> Option<#name>{
                #r_map
                MAP.get(&i).cloned()
            }

            fn #w_ident(&self) -> Option<#suffix>{
                Some(self.clone() as #suffix)
            }
        }
    };

    gen
}

fn gen_map_reader(name: &Ident, variants: &[(&Ident, Expr)], type_ident: &Ident) -> TokenStream {
    let premap: Vec<TokenStream> = variants
        .iter()
        .map(|(ident, _)| quote!{#ident as #type_ident, #ident})
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

fn find_variants<'a>(ast: &'a DeriveInput) -> Vec<(&'a Ident, Expr)> {
    if let Data::Enum(ref data) = ast.data {
        let mut variants = Vec::new();
        for x in data.variants.iter() {
            let ident = &x.ident;
            let discriminant = &x
                .discriminant
                .as_ref()
                .expect("EnumPrimitive need explicit enum value");
            variants.push((ident, discriminant.1.clone()));
        }
        variants
    } else {
        panic!("EnumPrimitive applicable to enum only")
    }
}
