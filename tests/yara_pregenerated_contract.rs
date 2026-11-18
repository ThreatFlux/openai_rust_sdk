//! Exercise YARA-X's shipped protobuf sources with the existing runtime features.
#![cfg(feature = "yara")]

#[test]
fn shipped_yara_schema_still_compiles_and_scans_with_default_modules() {
    let mut compiler = yara_x::Compiler::new();
    compiler
        .add_source(
            r#"import "math"
rule maintained_schema { condition: math.entropy(0, filesize) > 0.0 }"#,
        )
        .unwrap();
    let rules = compiler.build();
    let mut scanner = yara_x::Scanner::new(&rules);
    let results = scanner
        .scan(b"Synthetic maintained schema fixture")
        .unwrap();
    assert_eq!(results.matching_rules().count(), 1);
}
