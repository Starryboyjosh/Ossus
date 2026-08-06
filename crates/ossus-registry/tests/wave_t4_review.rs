//! Regressions for defects found during the Opus 5 review of WAVE-002 T4.
//!
//! Each test here failed before its corresponding fix. They are kept separate
//! from `wave_t4.rs` so the review evidence stays attributable.

use ossus_registry::{
    Budget, Diagnostic, ManifestValidation, Taxonomy, parse_canonical_manifest_with_taxonomy,
};

const EXAMPLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../specs/examples/canonical-manifest.example.toml"
));

fn parse(source: &str) -> Result<ManifestValidation, Vec<Diagnostic>> {
    let taxonomy = match Taxonomy::load_builtin(&Budget::default()) {
        Ok(taxonomy) => taxonomy,
        Err(diagnostics) => panic!("built-in taxonomy failed to load: {diagnostics:?}"),
    };
    parse_canonical_manifest_with_taxonomy(source, &Budget::default(), &taxonomy)
}

fn assert_code(source: &str, code: &str) {
    match parse(source) {
        Ok(_) => panic!("expected `{code}`, but the manifest was accepted"),
        Err(diagnostics) => {
            let found: Vec<&str> = diagnostics
                .iter()
                .map(|diagnostic| diagnostic.reason_code.as_str())
                .collect();
            assert!(found.contains(&code), "expected `{code}`, got {found:?}");
        }
    }
}

fn assert_valid(source: &str) {
    if let Err(diagnostics) = parse(source) {
        panic!("expected a valid manifest, got {diagnostics:?}");
    }
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "fixture substring not found: {from}");
    source.replacen(from, to, 1)
}

/// The string budget is stated in KiB, so it must count bytes.
///
/// The original check counted `chars`, which let an astral-plane string occupy
/// four bytes per counted unit — 128 KiB of memory against a 32 KiB budget. The
/// pre-existing budget test used `"x".repeat(32_769)`, where chars and bytes are
/// equal, so ASCII alone could never expose the divergence.
#[test]
fn string_budget_counts_bytes_not_code_points() {
    // 8_193 four-byte characters = 32_772 bytes: just past the budget, but only
    // 8_193 code points, so a char-based check accepts it.
    let oversized: String = core::iter::repeat_n('\u{1D518}', 8_193).collect();
    assert!(
        oversized.chars().count() < 32_768,
        "must pass a char-based check"
    );
    assert!(oversized.len() > 32_768, "must fail a byte-based check");

    let source = replace_once(
        EXAMPLE,
        "description = \"Reviews visual quality, responsive layout, and accessibility boundaries.\"",
        &format!("description = \"{oversized}\""),
    );
    assert_code(&source, "budget.string-length.exceeded");
}

/// The depth diagnostic must report the configured limit, not the observed depth.
///
/// It previously interpolated the depth that tripped the gate, so a manifest
/// nested 33 levels deep was told the limit was 33 — the one number guaranteed
/// not to be the limit.
#[test]
fn nesting_depth_diagnostic_reports_the_configured_limit() {
    let budget = Budget::default();
    let mut nested = String::from("a = ");
    for _ in 0..=budget.max_nesting_depth {
        nested.push_str("{ a = ");
    }
    nested.push_str("\"x\"");
    for _ in 0..=budget.max_nesting_depth {
        nested.push('}');
    }

    let Err(diagnostics) = parse(&nested) else {
        panic!("expected the nesting budget to reject this input");
    };
    let Some(depth_diagnostic) = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.reason_code == "budget.nesting-depth.exceeded")
    else {
        panic!("expected a nesting-depth diagnostic, got {diagnostics:?}");
    };
    assert!(
        depth_diagnostic
            .message
            .contains(&budget.max_nesting_depth.to_string()),
        "message must name the configured limit, got: {}",
        depth_diagnostic.message
    );
}

/// Nesting exactly at the budget is permitted; one level beyond is not.
#[test]
fn nesting_depth_boundary_is_inclusive() {
    let budget = Budget::default();
    let nest = |depth: usize| {
        let mut source = String::from("a = ");
        for _ in 0..depth {
            source.push_str("{ a = ");
        }
        source.push_str("\"x\"");
        for _ in 0..depth {
            source.push('}');
        }
        source
    };

    // Both are invalid manifests; only the deeper one may trip the budget gate,
    // which short-circuits before any field-level diagnostic is produced.
    let at_limit = parse(&nest(budget.max_nesting_depth));
    let Err(at_limit) = at_limit else {
        panic!("a manifest with no required fields cannot be valid");
    };
    assert!(
        !at_limit
            .iter()
            .any(|diagnostic| diagnostic.reason_code == "budget.nesting-depth.exceeded"),
        "depth {} is at the budget and must not trip it",
        budget.max_nesting_depth
    );

    assert_code(
        &nest(budget.max_nesting_depth + 1),
        "budget.nesting-depth.exceeded",
    );
}

/// R1 requires "complete human reading" in `RISK_TIERS.md`, the same phrase as R2.
///
/// The floor table originally grouped R1 with R0 at `light-human`, which let an
/// R1 resource — one that may read project files through host tools — ship with
/// the review depth intended for instruction-only content.
#[test]
fn r1_requires_full_human_review() {
    let r1_light = replace_once(EXAMPLE, "tier = \"R0\"", "tier = \"R1\"");
    assert_code(&r1_light, "review.tier.insufficient-for-risk");

    let r1_full = replace_once(
        &replace_once(EXAMPLE, "tier = \"R0\"", "tier = \"R1\""),
        "tier = \"light-human\"",
        "tier = \"full-human\"",
    );
    assert_valid(&r1_full);
}

/// R0 keeps the lighter floor, so the correction did not simply raise everything.
#[test]
fn r0_still_accepts_light_human_review() {
    assert_valid(EXAMPLE);
}

/// Diagnostics are read in a terminal while deciding whether to admit a file.
///
/// Bounding only the length of an offending value let a manifest key carry ANSI
/// escape sequences and newlines straight into human-format CLI output, where a
/// hostile manifest could repaint the screen or forge extra diagnostic lines.
/// The JSON writer already escaped control characters; the values themselves did
/// not, so every other consumer of `Diagnostic` was exposed.
#[test]
fn hostile_manifest_keys_cannot_inject_terminal_escapes() {
    // The escapes are written in TOML form: a raw control character is not legal
    // inside a TOML basic string, so a hostile author reaches the same bytes
    // through the parser's own unicode escapes, which are decoded into the key
    // before any Ossus code sees it.
    let source = concat!(
        "schema_version = \"1.0.0\"\n",
        "\"evil\\u001B[31m\\nSTATUS: approved\\u001B[0m\" = true\n"
    );

    let Err(diagnostics) = parse(source) else {
        panic!("a manifest with an unknown field cannot be valid");
    };
    for diagnostic in &diagnostics {
        for text in [&diagnostic.field_path, &diagnostic.message] {
            assert!(
                !text.chars().any(char::is_control),
                "control character reached a diagnostic: {text:?}"
            );
        }
    }

    let Some(unknown) = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.reason_code == "manifest.field.unknown")
    else {
        panic!("expected an unknown-field diagnostic, got {diagnostics:?}");
    };
    assert!(
        unknown.field_path.contains("\\u{1b}"),
        "the escape must remain visible in escaped form, got {:?}",
        unknown.field_path
    );
}

/// The budget walk built its field path from raw keys, bypassing the bounding.
///
/// That path was both unescaped and unbounded, so a single long key produced a
/// field path far past the 64-character diagnostic limit.
#[test]
fn budget_field_paths_are_bounded_and_escaped() {
    let source = format!(
        "schema_version = \"1.0.0\"\n\n[source]\n\"{}\" = \"{}\"\n\"boom\\u001B[31m\" = \"{}\"\n",
        "L".repeat(400),
        "y".repeat(40_000),
        "z".repeat(40_000)
    );

    let Err(diagnostics) = parse(&source) else {
        panic!("oversized strings must be rejected");
    };
    let budget_paths: Vec<&str> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.reason_code == "budget.string-length.exceeded")
        .map(|diagnostic| diagnostic.field_path.as_str())
        .collect();
    assert!(
        !budget_paths.is_empty(),
        "expected budget diagnostics, got {diagnostics:?}"
    );
    for path in &budget_paths {
        assert!(
            !path.chars().any(char::is_control),
            "control character reached a budget field path: {path:?}"
        );
        // "source." plus one bounded 64-character segment plus the ellipsis.
        assert!(
            path.chars().count() <= 128,
            "budget field path is unbounded ({} chars): {path:?}",
            path.chars().count()
        );
    }
}

/// `capability.alias.invalid-format` only checked the alias length.
///
/// Any character was accepted, so two aliases could be visually
/// indistinguishable in a terminal while resolving to different capabilities.
#[test]
fn alias_format_is_validated_not_only_bounded() {
    let capabilities = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specs/taxonomy/capabilities-v1.toml"
    ));
    let deprecations = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../specs/taxonomy/deprecations-v1.toml"
    ));

    // Written in TOML source form, so the parser decodes the escape into the key.
    for hostile in [
        "A11Y",
        "visual review",
        "-leading-hyphen",
        "review\\u001B[31m",
        "review\\u200B",
        "revie\u{0301}w",
    ] {
        let aliases = format!(
            "schema_version = \"1.0.0\"\n\n[aliases]\n\"{hostile}\" = \"frontend.accessibility\"\n"
        );
        let result =
            Taxonomy::from_toml_sources(capabilities, &aliases, deprecations, &Budget::default());
        let Err(diagnostics) = result else {
            panic!("alias `{hostile}` must be rejected");
        };
        let found: Vec<&str> = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.reason_code.as_str())
            .collect();
        assert!(
            found.contains(&"capability.alias.invalid-format"),
            "alias `{hostile}` produced {found:?}"
        );
    }
}

/// The pre-parse depth scan must survive every legal multi-line string ending.
///
/// The scan exists so a hostile input is rejected *before* the TOML parser can
/// recurse. It consumed a fixed three bytes when closing a multi-line string and
/// treated `\` as ordinary content, so three legal endings — `"""\""""`,
/// `"""a""""` and `'''a''''` — left one delimiter byte behind. The scanner read
/// that byte as opening a new string and skipped every bracket in the rest of the
/// file, silently disabling the gate. Only the post-parse walk still caught the
/// nesting, which is the layer this scan is specifically meant not to rely on.
///
/// The two gates are distinguished by their field path: the pre-parse scan
/// reports `$`, the post-parse walk reports the offending value's path.
#[test]
fn the_preparse_depth_scan_resynchronizes_after_every_string_form() {
    let deep = format!("deep = {}{}\n", "[".repeat(40), "]".repeat(40));
    let budget = Budget::default();
    assert!(
        40 > budget.max_nesting_depth,
        "the input must exceed the budget"
    );

    for prefix in [
        "",
        "description = \"\"\"\\\"\"\"\"\n",
        "description = \"\"\"a\"\"\"\"\n",
        "description = \"\"\"a\"\"\"\"\"\n",
        "description = '''a''''\n",
        "description = '''a'''\n",
        "description = \"a\"\n",
        "description = 'a'\n",
        "# a comment with a lone \" and '\n",
    ] {
        let source = format!("schema_version = \"1.0.0\"\n{prefix}{deep}");
        let Err(diagnostics) = parse(&source) else {
            panic!("depth 40 must be rejected; prefix was {prefix:?}");
        };
        let Some(depth) = diagnostics
            .iter()
            .find(|diagnostic| diagnostic.reason_code == "budget.nesting-depth.exceeded")
        else {
            panic!("no depth diagnostic for prefix {prefix:?}: {diagnostics:?}");
        };
        assert_eq!(
            depth.field_path, "$",
            "the pre-parse scan was bypassed by prefix {prefix:?}"
        );
    }
}

/// Resynchronizing must not make the scan count brackets that live in strings.
#[test]
fn the_preparse_depth_scan_still_ignores_bracket_bearing_strings() {
    let source = format!(
        "schema_version = \"1.0.0\"\ndescription = \"\"\"{}\"\"\"\nother = '''{}'''\n",
        "[".repeat(200),
        "{".repeat(200)
    );

    let Err(diagnostics) = parse(&source) else {
        panic!("a manifest with no required fields cannot be valid");
    };
    assert!(
        !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.reason_code == "budget.nesting-depth.exceeded"),
        "brackets inside strings must not be counted: {diagnostics:?}"
    );
}

/// The governed alias set must keep loading, so the rule was not over-tightened.
#[test]
fn the_governed_alias_set_still_loads() {
    let taxonomy = match Taxonomy::load_builtin(&Budget::default()) {
        Ok(taxonomy) => taxonomy,
        Err(diagnostics) => panic!("built-in taxonomy failed to load: {diagnostics:?}"),
    };

    assert_eq!(taxonomy.aliases().len(), 48);
    assert_eq!(taxonomy.capabilities().len(), 44);
}
