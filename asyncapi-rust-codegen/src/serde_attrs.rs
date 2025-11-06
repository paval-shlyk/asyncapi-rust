//! Utilities for parsing serde attributes

use syn::Attribute;

/// Extract the value from `#[serde(rename = "...")]`
pub fn extract_serde_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut rename_value = None;

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                rename_value = Some(s.value());
            }
            Ok(())
        });

        if rename_value.is_some() {
            return rename_value;
        }
    }
    None
}

/// Extract the value from `#[serde(tag = "...")]`
pub fn extract_serde_tag(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut tag_value = None;

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                tag_value = Some(s.value());
            }
            Ok(())
        });

        if tag_value.is_some() {
            return tag_value;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_extract_serde_rename() {
        let attrs: Vec<Attribute> = vec![parse_quote! {
            #[serde(rename = "custom_name")]
        }];

        assert_eq!(
            extract_serde_rename(&attrs),
            Some("custom_name".to_string())
        );
    }

    #[test]
    fn test_extract_serde_rename_none() {
        let attrs: Vec<Attribute> = vec![parse_quote! {
            #[derive(Debug)]
        }];

        assert_eq!(extract_serde_rename(&attrs), None);
    }

    #[test]
    fn test_extract_serde_tag() {
        let attrs: Vec<Attribute> = vec![parse_quote! {
            #[serde(tag = "type")]
        }];

        assert_eq!(extract_serde_tag(&attrs), Some("type".to_string()));
    }

    #[test]
    fn test_extract_serde_tag_none() {
        let attrs: Vec<Attribute> = vec![parse_quote! {
            #[serde(rename = "foo")]
        }];

        assert_eq!(extract_serde_tag(&attrs), None);
    }
}
