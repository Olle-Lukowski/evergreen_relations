use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    DeriveInput,
};

#[proc_macro_derive(Relation, attributes(relation))]
pub fn derive_relation(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ty = &input.ident;

    let Some(relation) = input
        .attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("relation"))
    else {
        return syn::Error::new_spanned(ty, "expected `relation` attribute")
            .to_compile_error()
            .into();
    };

    let RelationAttributes { source, target } = match relation.parse_args::<RelationAttributes>() {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {
        #[automatically_derived]
        impl ::evergreen_relations::relation::Relation for #ty {
            type Source = #source;
            type Target = #target;
        }
    }
    .into()
}

struct RelationAttributes {
    source: syn::Type,
    target: syn::Type,
}

impl Parse for RelationAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let mut source = None;
        let mut target = None;

        let fields: Punctuated<TypeField, syn::Token![,]> =
            input.parse_terminated(Parse::parse, syn::Token![,])?;

        for field in fields {
            match field.name.to_string().as_str() {
                "source" => source = Some(field.ty),
                "target" => target = Some(field.ty),
                _ => return Err(syn::Error::new_spanned(field.name, "unknown attribute")),
            }
        }

        Ok(Self {
            source: source.ok_or_else(|| syn::Error::new(span, "missing `source` attribute"))?,
            target: target.ok_or_else(|| syn::Error::new(span, "missing `target` attribute"))?,
        })
    }
}

#[proc_macro_derive(Relatable, attributes(relatable))]
pub fn derive_relatable(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ty = &input.ident;

    let Some(relatable) = input
        .attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("relatable"))
    else {
        return syn::Error::new_spanned(ty, "expected `relatable` attribute")
            .to_compile_error()
            .into();
    };

    let RelatableAttributes {
        relation,
        opposite,
        symmetry,
    } = match relatable.parse_args::<RelatableAttributes>() {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    let syn::Data::Struct(data_struct) = &input.data else {
        return syn::Error::new_spanned(ty, "expected struct")
            .to_compile_error()
            .into();
    };

    let mut fields = data_struct.fields.iter();
    let Some(container) = fields.next().map(|field| &field.ty) else {
        return syn::Error::new_spanned(ty, "expected exactly one field")
            .to_compile_error()
            .into();
    };
    if fields.next().is_some() {
        return syn::Error::new_spanned(ty, "expected exactly one field")
            .to_compile_error()
            .into();
    }

    quote! {
        #[automatically_derived]
        impl ::evergreen_relations::relation::Relatable for #ty {
            type Relation = #relation;
            type Opposite = #opposite;
            type Symmetry = #symmetry;
            type Container = #container;
        }
    }
    .into()
}

struct RelatableAttributes {
    relation: syn::Type,
    opposite: syn::Type,
    symmetry: proc_macro2::TokenStream,
}

impl Parse for RelatableAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let relation = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let mut opposite = None;

        let fields: Punctuated<TypeField, syn::Token![,]> =
            input.parse_terminated(Parse::parse, syn::Token![,])?;

        for field in fields {
            match field.name.to_string().as_str() {
                "opposite" => opposite = Some(field.ty),
                _ => return Err(syn::Error::new_spanned(field.name, "unknown attribute")),
            }
        }

        let opposite =
            opposite.ok_or_else(|| syn::Error::new(span, "missing `opposite` attribute"))?;

        let symmetry = if is_self_type(&opposite) {
            quote! { ::evergreen_relations::relation::Symmetric }
        } else {
            quote! { ::evergreen_relations::relation::Asymmetric }
        };

        Ok(Self {
            relation,
            opposite,
            symmetry,
        })
    }
}

struct TypeField {
    name: syn::Ident,
    ty: syn::Type,
}

impl Parse for TypeField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let ty = input.parse()?;
        Ok(Self { name, ty })
    }
}

fn is_self_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(path) = ty else {
        return false;
    };
    if path.path.segments.len() != 1 {
        return false;
    }
    let segment = &path.path.segments[0];

    segment.ident == "Self" && matches!(segment.arguments, syn::PathArguments::None)
}
