//! Exercises the bindgen-doc-comment-to-status-message extraction used by
//! `build.rs`, against synthetic `bindgen`-shaped source rather than the real
//! (proprietary) Sentinel headers.

#[path = "../build_hasp_error.rs"]
mod build_hasp_error;

use build_hasp_error::{extract_hasp_error_details, generate_hasp_error_messages_rs};

fn generate(bindings_src: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let details = extract_hasp_error_details(bindings_src).expect("extraction failed");
    let out_path = std::env::temp_dir().join(format!(
        "hasp_error_messages_test_{}_{}.rs",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    generate_hasp_error_messages_rs(&details, &out_path).expect("codegen failed");
    let generated = std::fs::read_to_string(&out_path).unwrap();
    let _ = std::fs::remove_file(&out_path);

    // The whole point of code generation is that it produces valid Rust;
    // parse it back to catch malformed output (e.g. bad escaping) instead of
    // only eyeballing string contents.
    syn::parse_file(&generated).unwrap_or_else(|e| {
        panic!("generated code is not valid Rust: {e}\n---\n{generated}\n---")
    });

    generated
}

#[test]
fn documented_variant_gets_name_and_message() {
    let src = r#"
        pub enum hasp_error_codes {
            #[doc = " Request completed successfully"]
            HASP_STATUS_OK = 0,
        }
    "#;
    let generated = generate(src);
    assert!(generated.contains("0 => \"HASP_STATUS_OK\","));
    assert!(generated.contains("0 => \"Request completed successfully\","));
}

/// Regression test for the bug where a `hasp_error_codes` variant with no
/// preceding doc comment in the C header lost its symbolic name entirely —
/// `hasp_status_name()` returned "UNKNOWN_HASP_STATUS" for a perfectly valid,
/// known status code just because the header didn't document it. The name
/// must always come from the enum variant itself; only the message is
/// allowed to fall back to a generic default.
#[test]
fn undocumented_variant_keeps_its_name_but_gets_a_generic_message() {
    let src = r#"
        pub enum hasp_error_codes {
            #[doc = " Request completed successfully"]
            HASP_STATUS_OK = 0,
            HASP_UNDOCUMENTED = 2,
        }
    "#;
    let generated = generate(src);

    // The name mapping must include the undocumented variant.
    assert!(
        generated.contains("2 => \"HASP_UNDOCUMENTED\","),
        "undocumented variant's name was dropped:\n{generated}"
    );

    // Its message must NOT have its own arm - it should fall through to the
    // generic default at runtime rather than being silently omitted, and it
    // must not have been mistaken for "UNKNOWN_HASP_STATUS" the way the name
    // lookup treats truly unrecognized codes.
    let message_fn_start = generated.find("fn hasp_status_message").unwrap();
    let message_fn = &generated[message_fn_start..];
    assert!(
        !message_fn.contains("2 =>"),
        "undocumented variant should not get its own message arm:\n{generated}"
    );
}

#[test]
fn multiline_doc_comment_is_joined_and_whitespace_normalized() {
    let src = r#"
        pub enum hasp_error_codes {
            #[doc = "  multi  "]
            #[doc = " line"]
            #[doc = " doc "]
            HASP_MULTILINE = 5,
        }
    "#;
    let generated = generate(src);
    assert!(generated.contains("5 => \"multi line doc\","));
}

#[test]
fn duplicate_discriminant_keeps_first_declared_variant() {
    let src = r#"
        pub enum hasp_error_codes {
            #[doc = " canonical name"]
            HASP_TS_DETECTED = 27,
            #[doc = " legacy alias, should be ignored"]
            HASP_RDP_DETECTED = 27,
        }
    "#;
    let generated = generate(src);
    assert!(generated.contains("27 => \"HASP_TS_DETECTED\","));
    assert!(generated.contains("27 => \"canonical name\","));
    assert!(!generated.contains("HASP_RDP_DETECTED"));
}

#[test]
fn messages_with_quotes_and_backslashes_are_escaped_into_valid_rust() {
    let src = r#"
        pub enum hasp_error_codes {
            #[doc = " contains \"quotes\" and a backslash \\ here"]
            HASP_WEIRD_TEXT = 9,
        }
    "#;
    // `generate` already parses the output with `syn` to confirm it's valid
    // Rust; here we also check the escaped content round-trips correctly.
    let generated = generate(src);
    let expected = r#"9 => "contains \"quotes\" and a backslash \\ here","#;
    assert!(
        generated.contains(expected),
        "expected escaped literal not found:\n{generated}"
    );
}

#[test]
fn missing_enum_is_a_clear_error_not_a_panic() {
    let src = r#"
        pub struct NotAnEnum;
    "#;
    let err = extract_hasp_error_details(src).unwrap_err();
    assert!(err.to_string().contains("hasp_error_codes"));
}

#[test]
fn unknown_code_falls_back_to_generic_name_and_message() {
    let src = r#"
        pub enum hasp_error_codes {
            #[doc = " Request completed successfully"]
            HASP_STATUS_OK = 0,
        }
    "#;
    let generated = generate(src);
    assert!(generated.contains("_ => \"UNKNOWN_HASP_STATUS\","));
    assert!(generated.contains("_ => \"Unknown HASP status code\","));
}
