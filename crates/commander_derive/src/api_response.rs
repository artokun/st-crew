use darling::{
    ast::{self, Style},
    FromDeriveInput, FromField, FromMeta, FromVariant,
};
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parse::ParseStream, parse_quote, parse_quote_spanned, spanned::Spanned, AttrStyle, DeriveInput,
    Expr, ExprLit, Ident, LitStr, Path, PathSegment,
};
use to_snake_case::ToSnakeCase;

use crate::utils::resolve_package_path;

#[derive(FromDeriveInput)]
#[darling(
    attributes(response),
    supports(struct_named, enum_any),
    forward_attrs(error, doc)
)]
struct ApiResponseOps {
    ident: syn::Ident,

    #[darling(default)]
    error: Option<ErrorOptions>,

    status: Option<syn::Path>,

    data: ast::Data<ApiResponseVariant, VariantField>,

    #[darling(default)]
    attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromVariant)]
#[darling(
    attributes(response),
    supports(newtype, unit),
    forward_attrs(error, doc)
)]
pub struct ApiResponseVariant {
    ident: Ident,

    #[darling(default)]
    error: Option<ErrorOptions>,

    status: Option<syn::Path>,

    #[darling(default)]
    transparent: bool,

    fields: darling::ast::Fields<VariantField>,

    #[darling(default)]
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    is_struct: bool,
}

#[derive(Debug, FromMeta)]
enum ErrorOptions {
    Message(String),
    Expanded {
        name: Option<String>,
        message: Option<String>,
    },
}

#[derive(Debug, FromField)]
#[darling(forward_attrs(from))]
struct VariantField {
    #[allow(dead_code)]
    ident: Option<Ident>,
    ty: syn::Type,

    #[darling(default)]
    attrs: Vec<syn::Attribute>,
}

pub fn impl_api_response(input: DeriveInput) -> TokenStream2 {
    let st_commander = resolve_package_path("st_commander");

    let ops = match ApiResponseOps::from_derive_input(&input) {
        Ok(ops) => ops,
        Err(err) => return err.write_errors(),
    };

    let ident = ops.ident;

    let mut apply_components = Vec::<TokenStream2>::new();
    let mut apply_responses = Vec::<TokenStream2>::new();
    let mut from_impls = Vec::<TokenStream2>::new();

    let mut status_code_match = Vec::<TokenStream2>::new();
    let mut serialize_variants = Vec::<TokenStream2>::new();

    let mut errors = Vec::<TokenStream2>::new();

    let variants = match ops.data {
        ast::Data::Enum(enum_data) => enum_data,

        ast::Data::Struct(fields) => {
            vec![ApiResponseVariant {
                ident: ident.clone(),
                error: ops.error,
                status: ops.status,
                transparent: false,
                fields,
                attrs: ops.attrs,

                is_struct: true,
            }]
        }
    };

    for ApiResponseVariant {
        ident: variant_ident,
        fields: variant_fields,
        attrs: variant_attrs,
        ..
    } in &variants
    {
        let has_thiserror_attr = variant_attrs
            .iter()
            .any(|attr| attr.path().is_ident("error"));

        // If it has a `#[error]` attribute, then thiserror will generate the `From` impls for us.
        if has_thiserror_attr {
            continue;
        }

        match variant_fields.style {
            Style::Tuple if variant_fields.fields.len() == 1 => {
                let VariantField {
                    ident: _,
                    ty: field_ty,
                    attrs: field_attrs,
                } = variant_fields.fields.first().unwrap();

                let should_add_from_impl = field_attrs.iter().any(|attr| {
                    attr.style == AttrStyle::Outer
                        && matches!(&attr.meta, syn::Meta::Path(path) if path.is_ident("from"))
                });

                if should_add_from_impl {
                    from_impls.push(parse_quote! {
                        impl From<#field_ty> for #ident {
                            fn from(value: #field_ty) -> Self {
                                Self::#variant_ident(value)
                            }
                        }

                        impl From<Result<#ident, #field_ty>> for #ident {
                            fn from(result: Result<#ident, #field_ty>) -> Self {
                                match result {
                                    Ok(output) => output,
                                    Err(err) => Self::from(err),
                                }
                            }
                        }
                    });
                }
            }

            _ => {}
        }
    }

    for ApiResponseVariant {
        ident: variant_ident,
        status: variant_status,
        transparent: variant_transparent,
        fields: variant_fields,
        error: variant_error,
        attrs: variant_attrs,
        is_struct,
    } in &variants
    {
        let variant_self: TokenStream2 = if *is_struct {
            parse_quote!(Self)
        } else {
            parse_quote!(Self::#variant_ident)
        };

        let thiserror_attr = variant_attrs
            .iter()
            .find(|attr| attr.path().is_ident("error"));

        let is_transparent = *variant_transparent
            || thiserror_attr
                .map(|attr| matches!(&attr.meta, syn::Meta::List(list) if list.path.is_ident("transparent")))
                .unwrap_or(false);

        // If it's transparent, then it should have a single field which also implements `ApiResponse`.
        if is_transparent {
            match variant_fields.style {
                Style::Tuple if variant_fields.fields.len() == 1 => {
                    let VariantField {
                        ident: _,
                        ty: field_ty,
                        ..
                    } = variant_fields.fields.first().unwrap();

                    apply_components.push(parse_quote!(
                        <#field_ty as #st_commander::response::ApiResponse>::apply_components(components);
                    ));

                    apply_responses.push(parse_quote!(
                        <#field_ty as #st_commander::response::ApiResponse>::apply_responses(responses);
                    ));

                    status_code_match.push(parse_quote! {
                        #variant_self(ref __field0) => #st_commander::response::ApiResponse::status(__field0)
                    });

                    serialize_variants.push(parse_quote! {
                        #variant_self(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                    });
                }

                Style::Unit | Style::Tuple | Style::Struct => {
                    errors.push(
                        syn::Error::new_spanned(
                            variant_ident,
                            "expected tuple variant with exactly one field",
                        )
                        .to_compile_error(),
                    );
                }
            }

            continue;
        }

        let status: syn::Path = match variant_status {
            Some(
                path @ Path {
                    leading_colon: None,
                    ..
                },
            ) if path.get_ident().is_some() => {
                parse_quote_spanned!(
                    path.span() =>
                    axum::http::StatusCode::#path
                )
            }

            Some(
                path @ Path {
                    leading_colon: None,
                    segments,
                },
            ) if segments.first()
                == Some(&PathSegment::from(Ident::new("StatusCode", path.span()))) =>
            {
                parse_quote_spanned!(
                    variant_ident.span() =>
                    axum::http::#path
                )
            }

            Some(path) => {
                errors.push(
                    syn::Error::new_spanned(path, "expected a status code").to_compile_error(),
                );

                parse_quote!(axum::http::StatusCode::NOT_IMPLEMENTED)
            }

            None => {
                errors.push(
                    syn::Error::new_spanned(
                        variant_ident,
                        "expected `response(status = ..)` or `response(transparent)` attribute",
                    )
                    .to_compile_error(),
                );

                parse_quote!(axum::http::StatusCode::NOT_IMPLEMENTED)
            }
        };

        match variant_fields.style {
            Style::Unit => status_code_match.push(parse_quote! {
                #variant_self => #status
            }),

            Style::Tuple => status_code_match.push(parse_quote! {
                #variant_self(..) => #status
            }),

            Style::Struct => status_code_match.push(parse_quote! {
                #variant_self { .. } => #status
            }),
        }

        // Parse the doc comment for an operation name and description
        let response_description = variant_attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| {
                if let syn::Meta::NameValue(syn::MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }),
                    ..
                }) = &attr.meta
                {
                    Some(lit.value().trim().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        // Errors are implemented differently
        if variant_error.is_some() || thiserror_attr.is_some() {
            // Either grab the defined error name, or use the varint name in `snake_case`.
            let err_name = variant_error
                .as_ref()
                .and_then(|opts| match opts {
                    ErrorOptions::Message(_) => None,
                    ErrorOptions::Expanded { name, .. } => name.clone(),
                })
                .unwrap_or_else(|| variant_ident.to_string().to_snake_case());

            // The message is either the message field of the `error` attribute or the message
            // from thiserror's `#[error]` attribute if it exists.
            let err_message = match variant_error.as_ref().and_then(|opts| match opts {
                ErrorOptions::Message(message) => Some(message.clone()),
                ErrorOptions::Expanded { message, .. } => message.clone(),
            }) {
                Some(message) => message,
                None => {
                    match thiserror_attr
                        .as_ref()
                        .and_then(|attr| {
                            attr.parse_args_with(|input: ParseStream| input.parse::<LitStr>())
                                .ok()
                        })
                        .map(|lit| lit.value())
                    {
                        Some(message) => message,

                        None => {
                            errors.push(
                                syn::Error::new_spanned(
                                    variant_ident,
                                    r#"expected `#[api(message = "..")]` attribute or `#[error("..")]` attribute"#,
                                )
                                .to_compile_error(),
                            );

                            continue;
                        }
                    }
                }
            };

            let error_name = variant_ident.to_string();

            let err_context: Option<TokenStream2> = if *is_struct {
                Some(parse_quote!(
                    .property(
                        "context",
                        Self::schema().1,
                    )
                    .required("context")
                ))
            } else {
                None
            };

            // We want errors to be homogonous, so we don't want to add a component for every
            // possible error type. Instead we add a single component for ApiError and then
            // serialize all errors as ApiError.
            apply_responses.push(parse_quote!(
                responses.response(
                    #status.as_u16().to_string(),
                    utoipa::openapi::ResponseBuilder::new()
                        .description(#response_description)
                        .content(
                            "application/json",
                            utoipa::openapi::Content::new(utoipa::openapi::schema::Schema::from(
                                utoipa::openapi::AllOfBuilder::new()
                                    .title(Some(#error_name))
                                    .item(utoipa::openapi::Ref::from_schema_name("ApiError"))
                                    .item(
                                        utoipa::openapi::ObjectBuilder::new()
                                            .property(
                                                "error",
                                                utoipa::openapi::ObjectBuilder::new()
                                                    .schema_type(utoipa::openapi::SchemaType::String)
                                                    .example(Some(serde_json::json!(#err_name))),
                                            )
                                            .required("error")
                                            .property(
                                                "message",
                                                utoipa::openapi::ObjectBuilder::new()
                                                    .schema_type(utoipa::openapi::SchemaType::String)
                                                    .example(Some(serde_json::json!(#err_message))),
                                            )
                                            .required("message")
                                            #err_context
                                            .build()
                                    ),
                            )),
                        )
                        .build()
                );
            ));

            match variant_fields.style {
                Style::Unit => {
                    serialize_variants.push(parse_quote! {
                        err @ #variant_self => {
                            _serde::Serialize::serialize(
                                &#st_commander::response::ApiError::new(#status)
                                    .with_name(#err_name)
                                    .with_error(err),
                                __serializer
                            )
                        }
                    });
                }

                Style::Tuple => {
                    serialize_variants.push(parse_quote! {
                        err @ #variant_self(..) => {
                            _serde::Serialize::serialize(
                                &#st_commander::response::ApiError::new(#status)
                                    .with_name(#err_name)
                                    .with_error(err),
                                __serializer
                            )
                        }
                    });
                }

                Style::Struct => {
                    serialize_variants.push(parse_quote! {
                        err @ #variant_self { .. } => {
                            _serde::Serialize::serialize(
                                &#st_commander::response::ApiError::new(#status)
                                    .with_name(#err_name)
                                    .with_message(#err_message),
                                __serializer
                            )
                        }
                    });
                }
            }
        } else {
            match variant_fields.style {
                Style::Unit | Style::Tuple if variant_fields.fields.is_empty() => {
                    serialize_variants.push(parse_quote! {
                        #variant_self => {
                            _serde::Serialize::serialize(&(), __serializer)
                        }
                    });
                }

                Style::Tuple if variant_fields.fields.len() == 1 => {
                    let VariantField {
                        ident: _,
                        ty: field_ty,
                        ..
                    } = variant_fields.fields.first().unwrap();

                    apply_components.push(parse_quote!(
                        components.schema_from::<#field_ty>();
                    ));

                    apply_responses.push(parse_quote!(
                        responses.response(
                            #status.as_u16().to_string(),
                            utoipa::openapi::RefOr::T(
                                utoipa::openapi::ResponseBuilder::new()
                                    .description(#response_description)
                                    .content(
                                        "application/json",
                                        utoipa::openapi::Content::new(utoipa::openapi::RefOr::Ref(
                                            utoipa::openapi::schema::Ref::from_schema_name(<#field_ty as utoipa::ToSchema>::schema().0)
                                        )),
                                    )
                                    .build(),
                            )
                        );
                    ));

                    serialize_variants.push(parse_quote! {
                        #variant_self(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                    });
                }

                Style::Tuple | Style::Struct | Style::Unit => {
                    errors.push(
                        syn::Error::new_spanned(
                            variant_ident,
                            "expected tuple variant with exactly one field",
                        )
                        .to_compile_error(),
                    );
                }
            }
        }
    }

    parse_quote! {
        #[automatically_derived]
        impl axum::response::IntoResponse for #ident {
            fn into_response(self) -> axum::response::Response {
                (<Self as #st_commander::response::ApiResponse>::status(&self), #st_commander::body::EncodeBody(self)).into_response()
            }
        }

        #[automatically_derived]
        impl #st_commander::response::ApiResponse for #ident {
            fn status(&self) -> axum::http::StatusCode {
                match self {
                    #(#status_code_match,)*
                }
            }

            fn apply_components(mut components: utoipa::openapi::schema::ComponentsBuilder) -> utoipa::openapi::schema::ComponentsBuilder {
                #(components = #apply_components)*
                components
            }

            fn apply_responses(mut responses: utoipa::openapi::ResponsesBuilder) -> utoipa::openapi::ResponsesBuilder {
                #(responses = #apply_responses)*
                responses
            }
        }

        #(#from_impls)*

        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for #ident {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match self {
                        #(#serialize_variants)*
                    }
                }
            }
        };

        #(#errors)*
    }
}
