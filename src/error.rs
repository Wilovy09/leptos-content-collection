use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing frontmatter in '{0}' — wrap it with ---")]
    MissingFrontmatter(String),

    #[error("Invalid frontmatter in '{path}': {source}")]
    InvalidFrontmatter {
        path: String,
        #[source]
        source: serde_yml::Error,
    },
}
