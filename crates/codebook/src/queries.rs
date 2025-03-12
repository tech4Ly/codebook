use tree_sitter::Language;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LanguageType {
    Bash,
    C,
    Css,
    Go,
    HTML,
    Javascript,
    Python,
    Ruby,
    Rust,
    TOML,
    Text,
    Typescript,
}

impl LanguageType {
    pub fn from_str(s: &str) -> LanguageType {
        for language in LANGUAGE_SETTINGS.iter() {
            for id in language.ids.iter() {
                if s == *id {
                    return language.type_;
                }
            }
        }
        LanguageType::Text
    }
    pub fn dictionary_ids(&self) -> Vec<String> {
        for language in LANGUAGE_SETTINGS.iter() {
            if self == &language.type_ {
                return language
                    .dictionary_ids
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        vec![]
    }
}

// Use https://intmainreturn0.com/ts-visualizer/ to help with writing grammar queries
// Language ids documented at https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
pub static LANGUAGE_SETTINGS: &[LanguageSetting] = &[
    LanguageSetting {
        type_: LanguageType::Rust,
        ids: &["rust"],
        dictionary_ids: &["rust"],
        query: include_str!("queries/rust.scm"),
        extensions: &["rs"],
    },
    LanguageSetting {
        type_: LanguageType::C,
        ids: &["c"],
        dictionary_ids: &["c"],
        query: include_str!("queries/c.scm"),
        extensions: &["c", "h"],
    },
    LanguageSetting {
        type_: LanguageType::Python,
        ids: &["python"],
        dictionary_ids: &["python"],
        query: include_str!("queries/python.scm"),
        extensions: &["py"],
    },
    LanguageSetting {
        type_: LanguageType::Javascript,
        ids: &["javascript", "javascriptreact"],
        dictionary_ids: &["javascript", "javascriptreact"],
        query: include_str!("queries/javascript.scm"),
        extensions: &["js", "jsx"],
    },
    LanguageSetting {
        type_: LanguageType::Typescript,
        ids: &["typescript", "typescriptreact"],
        dictionary_ids: &["typescript", "typescriptreact"],
        query: include_str!("queries/typescript.scm"),
        extensions: &["ts", "tsx"],
    },
    LanguageSetting {
        type_: LanguageType::HTML,
        ids: &["html"],
        dictionary_ids: &["html"],
        query: include_str!("queries/html.scm"),
        extensions: &["html", "htm"],
    },
    LanguageSetting {
        type_: LanguageType::Css,
        ids: &["css"],
        dictionary_ids: &["css"],
        query: include_str!("queries/css.scm"),
        extensions: &["css"],
    },
    LanguageSetting {
        type_: LanguageType::Go,
        ids: &["go"],
        dictionary_ids: &["go"],
        query: include_str!("queries/go.scm"),
        extensions: &["go"],
    },
    LanguageSetting {
        type_: LanguageType::TOML,
        ids: &["toml"],
        dictionary_ids: &["toml"],
        query: include_str!("queries/toml.scm"),
        extensions: &["toml"],
    },
    LanguageSetting {
        type_: LanguageType::Ruby,
        ids: &["ruby"],
        dictionary_ids: &["ruby"],
        query: include_str!("queries/ruby.scm"),
        extensions: &["rb"],
    },
    LanguageSetting {
        type_: LanguageType::Bash,
        ids: &["bash", "shellscript", "sh", "shell script"],
        dictionary_ids: &["bash"],
        query: include_str!("queries/bash.scm"),
        extensions: &["sh", "bash"],
    },
];

#[derive(Debug)]
pub struct LanguageSetting {
    pub type_: LanguageType,
    pub query: &'static str,
    /// ID from https://code.visualstudio.com/docs/languages/identifiers
    pub ids: &'static [&'static str],
    pub dictionary_ids: &'static [&'static str],
    pub extensions: &'static [&'static str],
}

impl LanguageSetting {
    pub fn language(&self) -> Option<Language> {
        match self.type_ {
            LanguageType::Bash => Some(tree_sitter_bash::LANGUAGE.into()),
            LanguageType::C => Some(tree_sitter_c::LANGUAGE.into()),
            LanguageType::Css => Some(tree_sitter_css::LANGUAGE.into()),
            LanguageType::Go => Some(tree_sitter_go::LANGUAGE.into()),
            LanguageType::HTML => Some(tree_sitter_html::LANGUAGE.into()),
            LanguageType::Javascript => Some(tree_sitter_javascript::LANGUAGE.into()),
            LanguageType::Python => Some(tree_sitter_python::LANGUAGE.into()),
            LanguageType::Ruby => Some(tree_sitter_ruby::LANGUAGE.into()),
            LanguageType::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            LanguageType::TOML => Some(tree_sitter_toml_ng::LANGUAGE.into()),
            LanguageType::Text => None,
            LanguageType::Typescript => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        }
    }
}

pub fn get_language_setting(language_type: LanguageType) -> Option<&'static LanguageSetting> {
    for setting in LANGUAGE_SETTINGS.iter() {
        if setting.type_ == language_type {
            if setting.language().is_some() {
                return Some(setting);
            }
        }
    }
    None
}

pub fn get_language_name_from_filename(filename: &str) -> LanguageType {
    let extension = filename.split('.').last().unwrap();
    for setting in LANGUAGE_SETTINGS.iter() {
        for ext in setting.extensions.iter() {
            if ext == &extension {
                return setting.type_;
            }
        }
    }
    LanguageType::Text
}
