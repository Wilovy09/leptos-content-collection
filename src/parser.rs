use serde::de::DeserializeOwned;
use crate::error::CollectionError;

const DELIMITER: &str = "---";

/// Splits a raw file into (frontmatter_yaml, body_markdown).
/// Returns `None` if the file doesn't start with `---`.
pub fn split_frontmatter(raw: &str) -> Option<(&str, &str)> {
    let raw = raw.trim_start_matches('\n');

    let rest = raw.strip_prefix(DELIMITER)?;
    let rest = rest.strip_prefix('\n').unwrap_or(rest);

    let end = rest.find(&format!("\n{DELIMITER}"))?;
    let frontmatter = &rest[..end];
    let body = &rest[end + DELIMITER.len() + 1..];
    let body = body.trim_start_matches('\n');

    Some((frontmatter, body))
}

pub fn parse_frontmatter<T: DeserializeOwned>(
    yaml: &str,
    file_path: &str,
) -> Result<T, CollectionError> {
    serde_yml::from_str(yaml).map_err(|source| CollectionError::InvalidFrontmatter {
        path: file_path.to_owned(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_correctly() {
        let raw = "---\ntitle: Hello\n---\n\nContent here";
        let (fm, body) = split_frontmatter(raw).unwrap();
        assert_eq!(fm, "title: Hello");
        assert_eq!(body, "Content here");
    }

    #[test]
    fn returns_none_without_delimiter() {
        assert!(split_frontmatter("no frontmatter here").is_none());
    }
}
