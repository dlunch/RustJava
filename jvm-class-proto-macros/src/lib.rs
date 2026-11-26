use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    Attribute, Error, Fields, FnArg, Ident, ImplItem, ItemImpl, ItemStruct, LitStr, Meta, Result, Token, TraitItemFn, Type, ext::IdentExt,
    meta::ParseNestedMeta, parenthesized, parse_macro_input, punctuated::Punctuated,
};

/// Declares a Java class from a struct, which becomes the unit marker type of the class.
///
/// Class metadata comes from the attribute arguments, with bare arguments being access flags written as in Java. A class extends
/// `java/lang/Object` unless `extends = ".."` names another superclass. Each struct field declares a Java field of the same name whose
/// descriptor is derived from the field type unless `#[field(descriptor = "..")]` is given.
///
/// ```ignore
/// #[java_class(name = "java/lang/Boolean", implements("java/io/Serializable"), public, final)]
/// pub struct Boolean {
///     #[field(public, static, final)]
///     TRUE: ClassInstanceRef<Self>,
///     #[field(private, final)]
///     value: bool,
/// }
/// ```
#[proc_macro_attribute]
pub fn java_class(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut class = Class::default();
    let parser = syn::meta::parser(|meta| class.parse(meta));
    parse_macro_input!(args with parser);
    let item = parse_macro_input!(input as ItemStruct);

    expand_class(class, item).unwrap_or_else(Error::into_compile_error).into()
}

/// Generates `as_proto()` for a `#[java_class]` type from its `impl` block.
///
/// Every function marked with `#[method]` becomes a method of the class. A method name defaults to the camelCase form of the function
/// name, and its descriptor is derived from the function signature unless `descriptor` is given. A function without a body is an abstract
/// method. A function with several `#[method]` attributes is registered once per attribute.
///
/// ```ignore
/// #[java_methods]
/// impl InputStream {
///     #[method(constructor, public)]
///     async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> { .. }
///
///     #[method(public)]
///     async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;
/// }
/// ```
#[proc_macro_attribute]
pub fn java_methods(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemImpl);

    expand_methods(item).unwrap_or_else(Error::into_compile_error).into()
}

#[derive(Default)]
struct Member {
    name: Option<LitStr>,
    descriptor: Option<LitStr>,
    flags: Vec<Ident>,
}

impl Member {
    // Takes the member attribute out of `attrs`; a member without arguments keeps every default.
    fn take(attrs: &mut Vec<Attribute>, attribute: &str) -> Result<Option<Self>> {
        let Some(index) = attrs.iter().position(|attr| attr.path().is_ident(attribute)) else {
            return Ok(None);
        };
        let attr = attrs.remove(index);

        let mut member = Self::default();
        if !matches!(attr.meta, Meta::Path(_)) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    member.name = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("descriptor") {
                    member.descriptor = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("constructor") {
                    member.name = Some(LitStr::new("<init>", meta.path.require_ident()?.span()));
                } else if meta.path.is_ident("static_initializer") {
                    let span = meta.path.require_ident()?.span();
                    member.name = Some(LitStr::new("<clinit>", span));
                    member.flags.push(Ident::new("STATIC", span));
                } else {
                    member.flags.push(flag(&meta)?);
                }

                Ok(())
            })?;
        }

        Ok(Some(member))
    }
}

#[derive(Default)]
struct Class {
    name: Option<LitStr>,
    extends: Option<LitStr>,
    implements: Vec<LitStr>,
    flags: Vec<Ident>,
}

impl Class {
    fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("name") {
            self.name = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("extends") {
            self.extends = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("implements") {
            let content;
            parenthesized!(content in meta.input);
            self.implements.extend(Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?);
        } else {
            self.flags.push(flag(&meta)?);
        }

        Ok(())
    }
}

// Java modifiers map onto the access flag constants of the same name, so an unknown modifier is reported by the compiler at its own span.
fn flag(meta: &ParseNestedMeta) -> Result<Ident> {
    let ident = meta.path.require_ident()?;

    Ok(Ident::new(&ident.to_string().to_uppercase(), ident.span()))
}

fn camel_case(name: &str) -> String {
    let mut result = String::new();
    let mut upper = false;
    for c in name.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            result.extend(c.to_uppercase());
            upper = false;
        } else {
            result.push(c);
        }
    }

    result
}

fn expand_class(class: Class, item: ItemStruct) -> Result<proc_macro2::TokenStream> {
    let private = quote!(::jvm_class_proto::__private);
    let ItemStruct {
        attrs, vis, ident, fields, ..
    } = item;
    let class_name = class.name.ok_or_else(|| Error::new_spanned(&ident, "java_class requires `name`"))?;
    if matches!(fields, Fields::Unnamed(_)) {
        return Err(Error::new_spanned(&fields, "java_class fields must be named"));
    }

    let mut field_protos = Vec::new();
    for mut field in fields {
        let member = Member::take(&mut field.attrs, "field")?.unwrap_or_default();
        // A field keeps its Java name verbatim, since constants such as `MAX_VALUE` contain underscores.
        let ident = field.ident.unwrap();
        let name = member.name.unwrap_or_else(|| LitStr::new(&ident.unraw().to_string(), ident.span()));
        let ty = field.ty;
        let descriptor = match member.descriptor {
            Some(descriptor) => quote!(#descriptor),
            None => quote!(&<#ty as ::jvm_class_proto::JavaDescriptor>::descriptor()),
        };
        let flags = member.flags;
        field_protos.push(quote! {
            ::jvm_class_proto::JavaFieldProto::new(#name, #descriptor, #private::FieldAccessFlags::empty() #(| #private::FieldAccessFlags::#flags)*)
        });
    }

    let descriptor = format!("L{};", class_name.value());
    // Only `java/lang/Object` and interfaces have no superclass in a prototype.
    let parent_class = match class.extends {
        Some(extends) => quote!(::core::option::Option::Some(#extends)),
        None if class_name.value() == "java/lang/Object" || class.flags.iter().any(|flag| flag == "INTERFACE") => {
            quote!(::core::option::Option::None)
        }
        None => quote!(::core::option::Option::Some("java/lang/Object")),
    };
    let interfaces = class.implements;
    let flags = class.flags;

    Ok(quote! {
        #(#attrs)*
        #vis struct #ident;

        impl ::jvm_class_proto::JavaDescriptor for #ident {
            fn descriptor() -> #private::String {
                #private::String::from(#descriptor)
            }
        }

        impl #ident {
            // Consumed by the `as_proto()` that `java_methods` generates.
            #[doc(hidden)]
            pub(crate) fn __class_proto<C>(methods: #private::Vec<::jvm_class_proto::JavaMethodProto<C>>) -> ::jvm_class_proto::JavaClassProto<C>
            where
                C: ?Sized + ::core::marker::Send,
            {
                ::jvm_class_proto::JavaClassProto {
                    name: #class_name,
                    parent_class: #parent_class,
                    interfaces: #private::Vec::from([#(#interfaces),*]),
                    methods,
                    fields: #private::Vec::from([#(#field_protos),*]),
                    access_flags: #private::ClassAccessFlags::empty() #(| #private::ClassAccessFlags::#flags)*,
                }
            }
        }
    })
}

fn expand_methods(mut item: ItemImpl) -> Result<proc_macro2::TokenStream> {
    let private = quote!(::jvm_class_proto::__private);

    let mut context = None;
    let mut methods = Vec::new();
    for impl_item in &mut item.items {
        // A function without a body is not a valid impl item, so it arrives unparsed; it only declares an abstract method and is dropped.
        let mut declaration = None;
        if let ImplItem::Verbatim(tokens) = impl_item
            && let Ok(function) = syn::parse2::<TraitItemFn>(tokens.clone())
        {
            *impl_item = ImplItem::Verbatim(proc_macro2::TokenStream::new());
            declaration = Some(function);
        }
        let is_abstract = declaration.is_some();
        let (attrs, sig) = match (&mut declaration, impl_item) {
            (Some(function), _) => (&mut function.attrs, &function.sig),
            (None, ImplItem::Fn(function)) => (&mut function.attrs, &function.sig),
            _ => continue,
        };
        let registered = methods.len();
        while let Some(mut method) = Member::take(attrs, "method")? {
            let mut parameters = Vec::new();
            for input in &sig.inputs {
                let FnArg::Typed(argument) = input else {
                    return Err(Error::new_spanned(input, "method does not take `self`"));
                };
                parameters.push(&argument.ty);
            }
            if context.is_none() {
                let Some(Type::Reference(reference)) = parameters.get(1).map(|ty| &***ty) else {
                    return Err(Error::new_spanned(sig, "method requires `jvm` and mutable context parameters"));
                };
                context = Some(reference.elem.clone());
            }

            let ident = &sig.ident;
            let name = method
                .name
                .unwrap_or_else(|| LitStr::new(&camel_case(&ident.unraw().to_string()), ident.span()));
            if let Some(flag) = method.flags.iter().find(|flag| *flag == "ABSTRACT") {
                return Err(Error::new_spanned(flag, "an abstract method is declared by omitting the function body"));
            }
            if is_abstract {
                method.flags.push(Ident::new("ABSTRACT", ident.span()));
            }
            let flags = method.flags;
            let output = &sig.output;
            // Choosing the trait by the `static` modifier makes a missing receiver a type error instead of a wrong descriptor.
            let signature = if flags.iter().any(|flag| flag == "STATIC") {
                quote!(StaticMethodSignature)
            } else {
                quote!(InstanceMethodSignature)
            };
            let descriptor = match method.descriptor {
                Some(descriptor) => quote!(#descriptor),
                // Spanned at the function name so that a signature error points at the method instead of the whole impl block.
                None => quote_spanned!(ident.span()=> &<fn(#(#parameters),*) #output as ::jvm_class_proto::#signature>::descriptor()),
            };
            let proto = if is_abstract {
                quote!(::jvm_class_proto::JavaMethodProto::new_abstract(#name, #descriptor, flags))
            } else {
                quote!(::jvm_class_proto::JavaMethodProto::new(#name, #descriptor, Self::#ident, flags))
            };
            methods.push(quote!({
                let flags = #private::MethodAccessFlags::empty() #(| #private::MethodAccessFlags::#flags)*;
                #proto
            }));
        }
        if is_abstract && methods.len() == registered {
            return Err(Error::new_spanned(sig, "a function without a body must be marked `#[method]`"));
        }
    }

    let self_ty = &item.self_ty;
    // A class without methods does not depend on the context, so its prototype fits any of them.
    let as_proto = match context {
        Some(context) => quote! {
            pub fn as_proto() -> ::jvm_class_proto::JavaClassProto<#context> {
                Self::__class_proto(#private::Vec::from([#(#methods),*]))
            }
        },
        None => quote! {
            pub fn as_proto<C>() -> ::jvm_class_proto::JavaClassProto<C>
            where
                C: ?Sized + ::core::marker::Send,
            {
                Self::__class_proto(#private::Vec::new())
            }
        },
    };

    Ok(quote! {
        #item

        impl #self_ty {
            #as_proto
        }
    })
}
