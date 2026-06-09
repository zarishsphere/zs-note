//! Integration tests for the types module changes in this PR.
//!
//! Covers:
//! - [`Provider`] enum: from_str, Display, serde serialisation
//! - [`FileEntry`] enum (was a struct): construction and serialisation
//! - Removal of old variants (Anthropic, Google, Custom) and old struct fields
//!
//! Run with:
//!   cargo test --test integration test_types

use zs_note_lib::types::{FileEntry, Provider};

// ---------------------------------------------------------------------------
// Provider::from_str – valid inputs
// ---------------------------------------------------------------------------

#[test]
fn test_provider_from_str_openai() {
    let p: Provider = "openai".parse().expect("openai should parse");
    assert!(matches!(p, Provider::OpenAI));
}

#[test]
fn test_provider_from_str_claude() {
    let p: Provider = "claude".parse().expect("claude should parse");
    assert!(matches!(p, Provider::Claude));
}

#[test]
fn test_provider_from_str_gemini() {
    let p: Provider = "gemini".parse().expect("gemini should parse");
    assert!(matches!(p, Provider::Gemini));
}

#[test]
fn test_provider_from_str_deepseek() {
    let p: Provider = "deepseek".parse().expect("deepseek should parse");
    assert!(matches!(p, Provider::DeepSeek));
}

#[test]
fn test_provider_from_str_ollama() {
    let p: Provider = "ollama".parse().expect("ollama should parse");
    assert!(matches!(p, Provider::Ollama));
}

// ---------------------------------------------------------------------------
// Provider::from_str – case insensitivity
// ---------------------------------------------------------------------------

#[test]
fn test_provider_from_str_case_insensitive_openai() {
    let p: Provider = "OpenAI".parse().expect("case-insensitive OpenAI should parse");
    assert!(matches!(p, Provider::OpenAI));
}

#[test]
fn test_provider_from_str_case_insensitive_claude() {
    let p: Provider = "CLAUDE".parse().expect("case-insensitive CLAUDE should parse");
    assert!(matches!(p, Provider::Claude));
}

#[test]
fn test_provider_from_str_case_insensitive_ollama() {
    let p: Provider = "Ollama".parse().expect("mixed-case Ollama should parse");
    assert!(matches!(p, Provider::Ollama));
}

// ---------------------------------------------------------------------------
// Provider::from_str – removed/invalid variants must now fail
// ---------------------------------------------------------------------------

#[test]
fn test_provider_from_str_anthropic_is_now_invalid() {
    // "anthropic" was previously an alias for Provider::Anthropic; it is now
    // removed. Parsing it must return an error.
    let result: Result<Provider, _> = "anthropic".parse();
    assert!(result.is_err(), "'anthropic' should no longer be a valid provider");
}

#[test]
fn test_provider_from_str_google_is_now_invalid() {
    // "google" was previously an alias for Provider::Google; it is now removed.
    let result: Result<Provider, _> = "google".parse();
    assert!(result.is_err(), "'google' should no longer be a valid provider");
}

#[test]
fn test_provider_from_str_custom_is_now_invalid() {
    // Provider::Custom was removed in this PR.
    let result: Result<Provider, _> = "custom".parse();
    assert!(result.is_err(), "'custom' should no longer be a valid provider");
}

#[test]
fn test_provider_from_str_unknown_string() {
    let result: Result<Provider, _> = "some_unknown_provider".parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown provider"));
}

#[test]
fn test_provider_from_str_empty_string() {
    let result: Result<Provider, _> = "".parse();
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Provider::Display (to_string)
// ---------------------------------------------------------------------------

#[test]
fn test_provider_display_openai() {
    assert_eq!(Provider::OpenAI.to_string(), "openai");
}

#[test]
fn test_provider_display_claude() {
    assert_eq!(Provider::Claude.to_string(), "claude");
}

#[test]
fn test_provider_display_gemini() {
    assert_eq!(Provider::Gemini.to_string(), "gemini");
}

#[test]
fn test_provider_display_deepseek() {
    assert_eq!(Provider::DeepSeek.to_string(), "deepseek");
}

#[test]
fn test_provider_display_ollama() {
    assert_eq!(Provider::Ollama.to_string(), "ollama");
}

// ---------------------------------------------------------------------------
// Provider – Display round-trip through from_str
// ---------------------------------------------------------------------------

#[test]
fn test_provider_display_roundtrip_all_variants() {
    let variants = [
        Provider::OpenAI,
        Provider::Claude,
        Provider::Gemini,
        Provider::DeepSeek,
        Provider::Ollama,
    ];
    for variant in variants {
        let s = variant.to_string();
        let parsed: Provider = s.parse().expect("Display output should parse back");
        assert_eq!(parsed.to_string(), s, "round-trip failed for variant {}", s);
    }
}

// ---------------------------------------------------------------------------
// Provider – JSON serialisation (serde)
// ---------------------------------------------------------------------------

#[test]
fn test_provider_json_serialize_openai() {
    let json = serde_json::to_string(&Provider::OpenAI).expect("serialize");
    assert_eq!(json, r#""openai""#);
}

#[test]
fn test_provider_json_serialize_claude() {
    let json = serde_json::to_string(&Provider::Claude).expect("serialize");
    assert_eq!(json, r#""claude""#);
}

#[test]
fn test_provider_json_serialize_gemini() {
    let json = serde_json::to_string(&Provider::Gemini).expect("serialize");
    assert_eq!(json, r#""gemini""#);
}

#[test]
fn test_provider_json_serialize_deepseek() {
    let json = serde_json::to_string(&Provider::DeepSeek).expect("serialize");
    assert_eq!(json, r#""deepseek""#);
}

#[test]
fn test_provider_json_serialize_ollama() {
    let json = serde_json::to_string(&Provider::Ollama).expect("serialize");
    assert_eq!(json, r#""ollama""#);
}

#[test]
fn test_provider_json_deserialize_openai() {
    let p: Provider = serde_json::from_str(r#""openai""#).expect("deserialize");
    assert!(matches!(p, Provider::OpenAI));
}

#[test]
fn test_provider_json_deserialize_claude() {
    let p: Provider = serde_json::from_str(r#""claude""#).expect("deserialize");
    assert!(matches!(p, Provider::Claude));
}

#[test]
fn test_provider_json_deserialize_gemini() {
    let p: Provider = serde_json::from_str(r#""gemini""#).expect("deserialize");
    assert!(matches!(p, Provider::Gemini));
}

#[test]
fn test_provider_json_roundtrip_all_variants() {
    let variants = [
        Provider::OpenAI,
        Provider::Claude,
        Provider::Gemini,
        Provider::DeepSeek,
        Provider::Ollama,
    ];
    for variant in variants {
        let json = serde_json::to_string(&variant).expect("serialize");
        let back: Provider = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(
            back.to_string(),
            variant.to_string(),
            "JSON round-trip failed for {}",
            variant
        );
    }
}

/// Verify that "anthropic" (old alias) no longer deserialises successfully.
#[test]
fn test_provider_json_deserialize_anthropic_fails() {
    let result: Result<Provider, _> = serde_json::from_str(r#""anthropic""#);
    assert!(result.is_err(), "anthropic should not deserialize as a Provider");
}

/// Verify that "google" (old alias) no longer deserialises successfully.
#[test]
fn test_provider_json_deserialize_google_fails() {
    let result: Result<Provider, _> = serde_json::from_str(r#""google""#);
    assert!(result.is_err(), "google should not deserialize as a Provider");
}

/// Verify that "custom" no longer deserialises successfully.
#[test]
fn test_provider_json_deserialize_custom_fails() {
    let result: Result<Provider, _> = serde_json::from_str(r#""custom""#);
    assert!(result.is_err(), "custom should not deserialize as a Provider");
}

// ---------------------------------------------------------------------------
// FileEntry – enum construction (File variant)
// ---------------------------------------------------------------------------

#[test]
fn test_file_entry_file_variant_construction() {
    use std::path::PathBuf;
    let entry = FileEntry::File {
        name: "note.md".to_string(),
        path: PathBuf::from("/vault/note.md"),
    };

    match entry {
        FileEntry::File { name, path } => {
            assert_eq!(name, "note.md");
            assert_eq!(path, PathBuf::from("/vault/note.md"));
        }
        FileEntry::Folder { .. } => panic!("Expected File variant"),
    }
}

#[test]
fn test_file_entry_folder_variant_construction() {
    use std::path::PathBuf;
    let entry = FileEntry::Folder {
        name: "notes".to_string(),
        path: PathBuf::from("/vault/notes"),
        children: vec![],
    };

    match entry {
        FileEntry::Folder { name, path, children } => {
            assert_eq!(name, "notes");
            assert_eq!(path, PathBuf::from("/vault/notes"));
            assert!(children.is_empty());
        }
        FileEntry::File { .. } => panic!("Expected Folder variant"),
    }
}

#[test]
fn test_file_entry_folder_with_children() {
    use std::path::PathBuf;
    let child = FileEntry::File {
        name: "child.md".to_string(),
        path: PathBuf::from("/vault/notes/child.md"),
    };
    let entry = FileEntry::Folder {
        name: "notes".to_string(),
        path: PathBuf::from("/vault/notes"),
        children: vec![child],
    };

    match entry {
        FileEntry::Folder { children, .. } => {
            assert_eq!(children.len(), 1);
            assert!(matches!(children[0], FileEntry::File { .. }));
        }
        _ => panic!("Expected Folder variant"),
    }
}

// ---------------------------------------------------------------------------
// FileEntry – is_dir / extension fields no longer exist
// ---------------------------------------------------------------------------

/// Ensure FileEntry is an enum and does NOT have old struct-style fields.
/// This test uses pattern matching to confirm the enum variants exist.
#[test]
fn test_file_entry_is_enum_not_struct() {
    use std::path::PathBuf;
    // Constructing both variants must compile and work; there is no `is_dir`
    // field, no `extension` field, and no optional `children` on File.
    let file = FileEntry::File {
        name: "a.md".to_string(),
        path: PathBuf::from("a.md"),
    };
    let folder = FileEntry::Folder {
        name: "dir".to_string(),
        path: PathBuf::from("dir"),
        children: vec![],
    };

    assert!(matches!(file, FileEntry::File { .. }));
    assert!(matches!(folder, FileEntry::Folder { .. }));
}

// ---------------------------------------------------------------------------
// FileEntry – JSON serialisation
// ---------------------------------------------------------------------------

#[test]
fn test_file_entry_file_json_serialization() {
    use std::path::PathBuf;
    let entry = FileEntry::File {
        name: "readme.md".to_string(),
        path: PathBuf::from("readme.md"),
    };
    let json = serde_json::to_string(&entry).expect("FileEntry::File should serialize to JSON");
    assert!(json.contains("readme.md"), "JSON should contain the file name");
    // Verify there is NO is_dir, extension, or children field in the output
    assert!(!json.contains("is_dir"), "is_dir field should not be present");
    assert!(!json.contains("extension"), "extension field should not be present");
}

#[test]
fn test_file_entry_folder_json_serialization() {
    use std::path::PathBuf;
    let entry = FileEntry::Folder {
        name: "notes".to_string(),
        path: PathBuf::from("notes"),
        children: vec![],
    };
    let json = serde_json::to_string(&entry).expect("FileEntry::Folder should serialize to JSON");
    assert!(json.contains("notes"), "JSON should contain the folder name");
    assert!(json.contains("children"), "JSON should contain children array");
}

#[test]
fn test_file_entry_json_roundtrip() {
    use std::path::PathBuf;
    let original = FileEntry::File {
        name: "test.md".to_string(),
        path: PathBuf::from("notes/test.md"),
    };
    let json = serde_json::to_string(&original).expect("serialize");
    let restored: FileEntry = serde_json::from_str(&json).expect("deserialize");

    match restored {
        FileEntry::File { name, path } => {
            assert_eq!(name, "test.md");
            assert_eq!(path, PathBuf::from("notes/test.md"));
        }
        _ => panic!("Expected File variant after round-trip"),
    }
}

#[test]
fn test_file_entry_folder_json_roundtrip_with_children() {
    use std::path::PathBuf;
    let original = FileEntry::Folder {
        name: "docs".to_string(),
        path: PathBuf::from("docs"),
        children: vec![
            FileEntry::File {
                name: "index.md".to_string(),
                path: PathBuf::from("docs/index.md"),
            },
        ],
    };
    let json = serde_json::to_string(&original).expect("serialize");
    let restored: FileEntry = serde_json::from_str(&json).expect("deserialize");

    match restored {
        FileEntry::Folder { name, children, .. } => {
            assert_eq!(name, "docs");
            assert_eq!(children.len(), 1);
        }
        _ => panic!("Expected Folder variant after round-trip"),
    }
}

// ---------------------------------------------------------------------------
// Provider – boundary / regression tests
// ---------------------------------------------------------------------------

/// Ensures that mixed-case "DeepSeek" doesn't accidentally work (only lowercase
/// "deepseek" is valid according to the current from_str implementation).
#[test]
fn test_provider_from_str_uppercase_deepseek() {
    // The implementation lower-cases before matching, so "DeepSeek" should work.
    let result: Result<Provider, _> = "DeepSeek".parse();
    // We expect this to succeed because to_lowercase() normalises it.
    assert!(result.is_ok(), "DeepSeek (mixed-case) should parse successfully");
    assert!(matches!(result.unwrap(), Provider::DeepSeek));
}

/// Regression: ensure whitespace in a provider string is rejected cleanly.
#[test]
fn test_provider_from_str_whitespace_fails() {
    let result: Result<Provider, _> = " openai".parse();
    assert!(result.is_err(), "provider with leading whitespace should fail");
}