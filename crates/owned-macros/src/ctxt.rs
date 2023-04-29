use std::cell::RefCell;
use std::fmt;

use proc_macro2::{Span, TokenStream};

pub(crate) struct Ctxt {
    errors: RefCell<Vec<syn::Error>>,
    pub(crate) owned_to_owned: syn::Path,
    pub(crate) owned_borrow: syn::Path,
    pub(crate) clone: syn::Path,
    pub(crate) borrow: syn::Path,
    pub(crate) to_owned: syn::Path,
}

impl Ctxt {
    pub(crate) fn new(span: Span) -> Self {
        Self {
            errors: RefCell::new(Vec::new()),
            owned_to_owned: path(span, ["owned", "ToOwned"]),
            owned_borrow: path(span, ["owned", "Borrow"]),
            clone: path(span, ["core", "clone", "Clone", "clone"]),
            borrow: path(span, ["owned", "Borrow", "borrow"]),
            to_owned: path(span, ["owned", "ToOwned", "to_owned"]),
        }
    }

    /// Convert context into any registered errors.
    pub(crate) fn into_errors(self) -> TokenStream {
        let errors = self.errors.into_inner();

        let mut stream = TokenStream::new();

        for error in errors {
            stream.extend(error.to_compile_error());
        }

        stream
    }

    /// Record an error.
    pub(crate) fn error(&self, error: syn::Error) {
        self.errors.borrow_mut().push(error);
    }

    /// Record a spanned error.
    pub(crate) fn span_error<T>(&self, span: Span, message: T)
    where
        T: fmt::Display,
    {
        self.error(syn::Error::new(span, message));
    }

    /// Check if context has errors.
    pub(crate) fn has_errors(&self) -> bool {
        !self.errors.borrow().is_empty()
    }
}

pub(crate) fn path<I>(span: Span, parts: I) -> syn::Path
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut path = syn::Path {
        leading_colon: Some(<syn::Token![::]>::default()),
        segments: syn::punctuated::Punctuated::default(),
    };

    for part in parts {
        path.segments.push(syn::PathSegment {
            ident: syn::Ident::new(part.as_ref(), span),
            arguments: syn::PathArguments::None,
        });
    }

    path
}