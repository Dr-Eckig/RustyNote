use std::{fmt, str::FromStr};

use comrak::{ComrakOptions, ExtensionOptions, ParseOptions, RenderOptions, markdown_to_html};

/// Selects which markdown dialect should be used for parsing.
#[derive(PartialEq, Clone, Debug, Default)]
pub enum Dialect {
    #[default]
    Common,
    GitHub,
}

impl Dialect {
    /// Parses markdown input into HTML using the configured dialect.
    ///
    /// ```rust
    /// use markdown::api::parser::Parser;
    ///
    /// let html = Parser::Common.parse_markdown_to_html("# Title");
    /// assert!(html.contains("<h1>Title</h1>"));
    /// ```
    pub fn parse_markdown_to_html(&self, input: &str) -> String {
        match self {
            Self::Common => {
                let options = ComrakOptions::default();
                markdown_to_html(input, &options)
            }
            Self::GitHub => {
                let options = ComrakOptions {
                    extension: ExtensionOptions {
                        strikethrough: true,
                        table: true,
                        autolink: true,
                        tasklist: true,
                        tagfilter: true,
                        ..Default::default()
                    },
                    parse: ParseOptions {
                        ..Default::default()
                    },
                    render: RenderOptions {
                        github_pre_lang: true,
                        gfm_quirks: true,
                        ..Default::default()
                    },
                };

                markdown_to_html(input, &options)
            }
        }
    }
}

impl fmt::Display for Dialect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dialect::Common => "Common",
                Dialect::GitHub => "GitHub",
            }
        )
    }
}

impl FromStr for Dialect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "common" => Ok(Dialect::Common),
            "github" => Ok(Dialect::GitHub),
            _ => Err(()),
        }
    }
}
