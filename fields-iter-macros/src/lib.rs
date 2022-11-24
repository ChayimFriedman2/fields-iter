#![forbid(unsafe_code, rust_2018_idioms)]

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_derive(FieldsInspect)]
pub fn derive_fields_inspect(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(item as syn::DeriveInput);
    let syn::Data::Struct(strukt) = item.data else {
        return syn::Error::new_spanned(&item, "only structs are supported for `#[derive(FieldsInspect)]`")
            .into_compile_error()
            .into();
    };

    let name = &item.ident;
    let name_string = name.to_string();
    let fields = match strukt.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => unnamed,
        syn::Fields::Unit => syn::punctuated::Punctuated::new(),
    };
    let fields_count = fields.len() as u32;
    let ((field_names, field_refs), field_muts): ((Vec<_>, Vec<_>), Vec<_>) = fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            let idx = idx as u32;
            let (name_ident, name_string) = match &field.ident {
                Some(name) => (name.clone(), name.to_string()),
                None => (format_ident!("{idx}"), idx.to_string()),
            };
            let name = quote!(#idx => #name_string,);
            // Span to the field type for if the it is not `'static`.
            let field_ref = quote_spanned! {field.ty.span() =>
                #idx => &self.#name_ident as &dyn ::core::any::Any,
            };
            let field_mut = quote_spanned! {field.ty.span() =>
                // SAFETY: By precondition, `this` points to a valid `Self`.
                #idx => unsafe {
                    &mut *(
                        ::fields_iter::addr_of_mut!((*this).#name_ident)
                            as *mut dyn ::core::any::Any
                    )
                }
            };
            ((name, field_ref), field_mut)
        })
        .unzip();
    let (impl_generics, type_generics, where_clause) = item.generics.split_for_impl();
    quote! {
        impl #impl_generics ::fields_iter::FieldsInspectImpl for #name #type_generics
        #where_clause
        {
            fn struct_name() -> &'static str { #name_string }
            fn fields_count() -> u32 { #fields_count }

            fn field_name(n: u32) -> &'static str {
                match n {
                    #(#field_names)*
                    _ => ::fields_iter::field_out_of_bounds(#name_string, n),
                }
            }

            fn field(&self, n: u32) -> &dyn ::core::any::Any {
                match n {
                    #(#field_refs)*
                    _ => ::fields_iter::field_out_of_bounds(#name_string, n),
                }
            }

            unsafe fn field_mut(this: *mut (), n: u32) -> &'static mut dyn ::core::any::Any {
                let this = this.cast::<Self>();
                match n {
                    #(#field_muts)*
                    _ => ::fields_iter::field_out_of_bounds(#name_string, n),
                }
            }
        };
    }
    .into()
}
