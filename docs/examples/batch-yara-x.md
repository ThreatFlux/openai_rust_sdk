# Batch API example: generate and validate YARA-X rules

This optional, security-oriented example shows how to prepare JSONL requests for
the OpenAI Batch API and validate the generated YARA rules locally with
[`yara-x`](https://crates.io/crates/yara-x). It is a focused Batch API case study,
not a core requirement or primary use case of the SDK.

The example tooling reports a small set of structural rule features, scans two
built-in test samples, runs three built-in validator cases, and generates Batch
API JSONL containing YARA-rule prompts.

None of the CLI commands sends a network request, and none requires
`OPENAI_API_KEY`. Batch generation does require an explicit model ID; the
commands below use `OPENAI_MODEL` as shell configuration and pass it to the CLI.
The generated JSONL must be uploaded and submitted separately if you want
OpenAI to process it.

## Enable the example tooling

For a library dependency from crates.io:

```console
cargo add openai_rust_sdk --features yara
```

For the installed CLI from crates.io:

```console
cargo install openai_rust_sdk --features yara
openai_rust_sdk --help
```

`cargo install` installs the latest published crate, which can lag behind the
default branch. These docs describe the current source tree. To run that exact
code from a checkout, use:

```console
git clone https://github.com/ThreatFlux/openai_rust_sdk.git
cd openai_rust_sdk
cargo run --features yara -- --help
```

The default feature set does not compile `yara-x`. The related Cargo features
are:

| Feature | Effect |
| --- | --- |
| `yara` | Enables YARA-X compilation, validation, and built-in validator suites. |
| `testing` | Reserved compatibility feature; it does not enable YARA-X. |
| `full` | Enables all optional capabilities, including `yara`. |

The batch JSONL generator is also available as a library API without optional
features. The packaged CLI intentionally requires `yara` for every command,
including batch generation.

## Generate Batch API JSONL

```console
export OPENAI_MODEL="gpt-5.6-luna"
cargo run --features yara -- generate-batch \
  --output-dir ./batch-output \
  --suite comprehensive \
  --model "$OPENAI_MODEL"
```

With the installed binary:

```console
openai_rust_sdk generate-batch \
  --output-dir ./batch-output \
  --suite basic \
  --model "$OPENAI_MODEL"
```

The exact options are:

```text
Usage: openai_rust_sdk generate-batch [OPTIONS] --output-dir <OUTPUT_DIR> --model <MODEL>

Options:
  -o, --output-dir <OUTPUT_DIR>
  -s, --suite <SUITE>            [default: comprehensive]
  -m, --model <MODEL>            Model ID written to every Batch API request
  -h, --help                     Print help
```

Available suites are:

| Suite | Requests | Focus |
| --- | ---: | --- |
| `basic` | 3 | Literal text, PE headers, and diagnostic strings. |
| `malware` | 3 | UPX, ransomware indicators, and keylogger APIs. |
| `comprehensive` | 10 | Basic, malware, regex, cryptocurrency, size, loops, modules, and obfuscated JavaScript prompts. |

The output path is
`<OUTPUT_DIR>/<SUITE>_batch_jobs.jsonl`. Each line targets
`POST /v1/chat/completions`. The CLI never chooses a model implicitly. The
library API supports the same explicit selection:

```rust
use openai_rust_sdk::BatchJobGenerator;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generator = BatchJobGenerator::with_model("gpt-5.6-luna")?;
    generator.generate_test_suite(Path::new("basic_batch_jobs.jsonl"), "basic")?;
    Ok(())
}
```

An unknown suite produces an error. The generator only writes JSONL; it does
not upload a file, create a Batch API job, poll it, or validate model output.
Those operations require a separately configured API client and
`OPENAI_API_KEY`.

## Run the end-to-end Batch workflow

The runnable [`batch_processing_demo.rs`](../../examples/batch_processing_demo.rs)
connects the example-specific generator to the SDK's general Batch client. It
uploads the JSONL input, creates a Batch job, polls its status, downloads the
result files, and applies the local YARA extraction and reporting helpers.

```console
export OPENAI_API_KEY="your_api_key_here"
export OPENAI_MODEL="gpt-5.6-luna"
cargo run --example batch_processing_demo
```

This command makes live API requests and may incur usage charges. It polls for
up to five minutes; a Batch job can continue after the example stops polling.

## Validate a rule

From a source checkout:

```console
cargo run --features yara -- validate-rule --file rules/example.yar
cargo run --features yara -- validate-rule --file rules/example.yar --verbose
```

With the installed binary:

```console
openai_rust_sdk validate-rule --file rules/example.yar --verbose
```

The exact options are:

```text
Usage: openai_rust_sdk validate-rule [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>
  -v, --verbose
  -h, --help         Print help
```

Normal output includes validity, an extracted rule name when one is found, and
compilation time. Compilation diagnostics are printed for an invalid rule.
`--verbose` additionally prints the detected `RuleFeatures` structure.

The validator returns a `ValidationResult` with:

- `is_valid`, the extracted `rule_name`, compilation `errors`, and `warnings`;
- lexical feature flags for strings, hex patterns, regular expressions,
  metadata, imports, `filesize`, and `any of`/`all of` usage;
- a string count and a simple complexity score of `min(string_count + 1, 10)`;
- compilation time in milliseconds and source size in bytes; and
- scan results for the built-in PE-header and email-text byte samples.

Feature detection is intentionally lightweight source inspection, not semantic
analysis. `warnings` is currently empty, and `metrics.pattern_count` is reserved
but currently remains `0`; use `features.string_count` for the detected source
pattern count.

Library example:

```rust
use openai_rust_sdk::YaraValidator;

fn main() {
    let rule = r#"
rule detect_pe_header {
    strings:
        $mz = { 4D 5A }
    condition:
        $mz at 0
}
"#;

    let result = YaraValidator::new()
        .validate_rule(rule)
        .expect("YARA validation failed");
    println!("valid: {}", result.is_valid);
    println!("compile time: {} ms", result.metrics.compilation_time_ms);
    println!("sample scans: {:#?}", result.pattern_tests);
}
```

The CLI currently reports an invalid compiled rule as `Valid: false` but exits
successfully. If an automated workflow requires a failing exit status, call the
library API and fail explicitly when `result.is_valid` is false.

## Run the built-in validator cases

```console
cargo run --features yara -- run-tests
# Installed binary:
openai_rust_sdk run-tests
```

`run-tests` executes three local cases: a valid string rule, a valid hex-pattern
rule, and a deliberately invalid rule. It reports totals, passed and failed
cases, and a success percentage. This command does not use the named batch
generation suites above.

## Security guidance

Treat both untrusted rule source and model-generated rules as untrusted input.
YARA validation confirms that a rule compiles; it does not establish that the
rule is safe, correct, performant, or free of false positives and false
negatives.

- Review generated rules and test them against representative benign and
  malicious corpora before deployment.
- Compile untrusted rules in an isolated worker with CPU, memory, and wall-clock
  limits. The current validator compiles in-process and provides no timeout.
- Keep `yara-x` and this crate updated, especially when processing rules from
  outside your trust boundary.
- Do not put secrets, credentials, customer data, or sensitive indicators in
  batch prompts unless your data-handling policy permits it.
- Never embed an API key in a rule or generated JSONL. Supply credentials only
  to the separate process that submits the batch.

The built-in validator scans only two small in-memory samples. It does not scan
files from disk, recursively inspect directories, or provide a malware sandbox.
