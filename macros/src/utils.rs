use syn::{Attribute, Expr, Token};

/// 解析 builder 属性
pub fn parse_builder_attrs(attrs: &[Attribute]) -> syn::Result<(bool, Option<Expr>)> {
    let mut skip = false;
    let mut default = None;

    for attr in attrs {
        if !attr.path().is_ident("builder") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            // #[builder(skip)]
            if meta.path.is_ident("skip") {
                skip = true;
                return Ok(());
            }

            // #[builder(default)] or #[builder(default = 123)]
            if meta.path.is_ident("default") {
                if meta.input.peek(Token![=]) {
                    let value: Expr = meta.value()?.parse()?;
                    default = Some(value);
                } else {
                    default = Some(syn::parse_quote!(::std::default::Default::default()));
                }
                return Ok(());
            }

            Ok(())
        })?;
    }

    Ok((skip, default))
}
