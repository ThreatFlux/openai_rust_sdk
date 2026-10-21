# Stable dependency and code-scanning remediation

Reviewed 2026-10-21 against main `ad3bd0c5f9d281c9342732960f80f117890a8928`.
All **31 open alerts** are accounted for: **13 valid package detections**,
**10 scoped FPs**, **one undetermined zlib mapping**, and **seven Scorecard
process/aggregate signals**. Package detection does not prove SDK exploitation.

## Original artifact verification

Inspected immutable published index
`sha256:554c5ae4158a08228c12d849264a23875a24e2aac043df5d2de846bbc82c8e9d`:

- arm64: `sha256:2ff3102e78a6fa9ccb43ae90ec04a3dc298a257e52f794b782d931c0c0d9d928`
- amd64: `sha256:b6ec73d28f127de6fd72934376dcdf8959246fb23c48c2949816413233c58634`

Both original manifests were pulled and exported without running their code.
Neither filesystem contains nscd. Only amd64/arm64 applications are published;
Power8 implementation findings do not apply to these artifacts.
The installed libstdc++6 files are byte-identical to the independently
[disassembled review artifacts](https://github.com/ThreatFlux/vertex_rust_sdk/blob/b169c1e27ecc8c59f7592f268154b4f112db4db2/docs/security/gcc-zlib-image-evidence.txt):

| Architecture | libstdc++.so.6.0.33 SHA256 |
| --- | --- |
| arm64 | `fff1f0050a33ddf29c44634c7639ac16e2573a61e3daab4fcc4c007b432ccf19` |
| amd64 | `972bb2a18b71140dab0240f8a1f68ab3fb1d56bcd4c4f824a91b70888faf5a00` |

Those exact allocator implementations call unaffected `posix_memalign` without
vulnerable alignment rounding. PBDS template defects do not belong to the
runtime/metadata packages. See the
[upstream allocator fix](https://github.com/gcc-mirror/gcc/commit/200183d4ab5d57575f3dfebc6af3baf649910b9e)
and [PBDS fix](https://github.com/gcc-mirror/gcc/commit/aaa8351f4d2e636f9680a1f0a8ebc2f0a60611e6).
Each FP is dismissed individually with original-artifact evidence; package
removal alone is not the reason for any FP disposition.

## Implemented package remediation

All Cargo capabilities remain enabled in Docker, including YARA-X compilation.
Build static PIE against maintained Alpine musl 1.2.6-r2, explicitly exclude
Rust's older bundled CRT, prove system libc.a selection from the linker map,
and retain native archive/init provenance. Static tini and a minimal runtime
remove glibc, zlib and GCC shared libraries. Trust data and its actual fixed
Debian package record move together, preventing stale CA provenance.

Validate exported ELF files, PIE, architecture, nonroot account, root-owned
executables, CA roots, SBOM and actual system-musl linkage. Execute the real
YARA compiler against valid and invalid synthetic rules, plus batch generation
with the chosen model, under read-only nonroot execution. Native CI builds
and tests both architectures; no Cargo features or auth modes are dropped.

The old `.trivyignore` blanket/history list is removed. Scans report all
severities and unfixed issues with Trivy 0.76.0. Existing historical glibc
entries hidden by that file are removed from the runtime too; removing ignore
rules is reporting visibility, not itself a fix.

## Additional baseline finding beyond the existing alert set

A fresh unfiltered Trivy 0.76.0 scan of the original ARM64 Docker image also
reports **DSA-6572-1** for `ca-certificates` 20250419. Both original platform
bundles have SHA256
`714d457d580922dbf1d0be8bd35ba236a842b50b0072ae791582a19adef772a5`, the same
[verified distrusted emSign root bundle](https://github.com/ThreatFlux/vertex_rust_sdk/blob/b169c1e27ecc8c59f7592f268154b4f112db4db2/docs/security/ca-alert-review.md).
This is an additional **TP**, not an existing OpenAI alert among the 31 rows
below. The replacement copies the fixed CA bundle and its real supplier
package record together; the final ARM64 all-severity scan has zero OS/Cargo
findings. The baseline also exposes historical glibc detections previously
hidden by `.trivyignore`; that entire affected component is removed.

## Complete per-alert decisions and plans

| Alert | Rule | Decision | Why and remediation/validation |
| --- | --- | --- | --- |
| [12814](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12814) | [CVE-2026-85091](https://security-tracker.debian.org/tracker/CVE-2026-85091) | Undetermined version range | Upstream and Debian affected-version evidence conflict; no FP dismissal. Remove zlib and verify the final image. |
| [12812](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12812) | [CVE-2026-95619](https://security-tracker.debian.org/tracker/CVE-2026-95619) | FP: scoped component/architecture | The actual Linux libstdc++ allocator uses safe posix_memalign; other mapped GCC packages do not contain that allocator. |
| [12811](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12811) | [CVE-2026-102010](https://security-tracker.debian.org/tracker/CVE-2026-102010) | FP: scoped component/architecture | PBDS header/template flaw mapped to shared runtime/metadata packages; affected development component is absent. |
| [12804](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12804) | [CVE-2026-95619](https://security-tracker.debian.org/tracker/CVE-2026-95619) | FP: scoped component/architecture | The actual Linux libstdc++ allocator uses safe posix_memalign; other mapped GCC packages do not contain that allocator. |
| [12803](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12803) | [CVE-2026-102010](https://security-tracker.debian.org/tracker/CVE-2026-102010) | FP: scoped component/architecture | PBDS header/template flaw mapped to shared runtime/metadata packages; affected development component is absent. |
| [12802](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12802) | [CVE-2026-95619](https://security-tracker.debian.org/tracker/CVE-2026-95619) | FP: scoped component/architecture | The actual Linux libstdc++ allocator uses safe posix_memalign; other mapped GCC packages do not contain that allocator. |
| [12801](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12801) | [CVE-2026-102010](https://security-tracker.debian.org/tracker/CVE-2026-102010) | FP: scoped component/architecture | PBDS header/template flaw mapped to shared runtime/metadata packages; affected development component is absent. |
| [12800](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12800) | [CVE-2026-97399](https://security-tracker.debian.org/tracker/CVE-2026-97399) | FP: scoped component/architecture | Defect is specifically Power8-optimized `strncasecmp`; amd64 and arm64 do not execute that implementation. Debian's source-level vulnerable status does not distinguish the architecture. |
| [12799](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12799) | [CVE-2026-95619](https://security-tracker.debian.org/tracker/CVE-2026-95619) | FP: scoped component/architecture | The actual Linux libstdc++ allocator uses safe posix_memalign; other mapped GCC packages do not contain that allocator. |
| [12798](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12798) | [CVE-2026-102010](https://security-tracker.debian.org/tracker/CVE-2026-102010) | FP: scoped component/architecture | PBDS header/template flaw mapped to shared runtime/metadata packages; affected development component is absent. |
| [12796](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12796) | [CVE-2026-95818](https://security-tracker.debian.org/tracker/CVE-2026-95818) | TP package; exploit path not established | Loader overflow requires an AT_SECURE setuid/setgid executable with qualifying `$ORIGIN` RPATH/RUNPATH. Inspected base has no such executables; normal nonroot CLI startup does not meet those prerequisites. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12795](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12795) | [CVE-2026-86805](https://security-tracker.debian.org/tracker/CVE-2026-86805) | TP package; exploit path not established | Loader TOCTOU requires a qualifying setuid/setgid `$ORIGIN/..` program, hard-link capability and disabled `fs.protected_hardlinks`. No setuid/setgid executable exists in inspected base. Host sysctl and final ELF metadata were not measured. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12794](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12794) | [CVE-2026-8674](https://security-tracker.debian.org/tracker/CVE-2026-8674) | TP package; exploit path not established | Oversized resolver search domain in `/etc/resolv.conf` or `LOCALDOMAIN` can abort a name-resolving process. SDK/auth use reqwest clients with default system DNS, so DNS initialization is a real route. Attacker control of configuration depends on DHCP/VPN/orchestrator and is not established. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12793](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12793) | [CVE-2026-89092](https://security-tracker.debian.org/tracker/CVE-2026-89092) | FP: scoped component/architecture | Vulnerability requires the nscd daemon and large malicious DNS responses; nscd is a separate Debian executable package and is absent from the inspected base and both original final platform images. This is not evidence that ordinary reqwest DNS is vulnerable to the same bug. Source-package association alone is too broad for container exploitability. |
| [12777](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12777) | [CVE-2026-80489](https://security-tracker.debian.org/tracker/CVE-2026-80489) | TP package; exploit path not established | EUC_JISX0213 converter can loop on split output. Module is present. No explicit iconv path found in SDK source; Rust strings and JSON do not by themselves demonstrate use of glibc conversion. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12776](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12776) | [CVE-2026-18374](https://security-tracker.debian.org/tracker/CVE-2026-18374) | TP package; exploit path not established | Requires attacker-controlled `fopen` mode with effectively empty `,ccs=`. No explicit `fopen`/C mode handling found in SDK source; Rust file paths are not fopen mode strings. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12775](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12775) | [CVE-2026-77117](https://security-tracker.debian.org/tracker/CVE-2026-77117) | TP package; exploit path not established | SHIFT_JISX0213 pending-state infinite conversion issue. Module exists; no explicit SDK iconv call was found. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12774](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12774) | [CVE-2026-19499](https://security-tracker.debian.org/tracker/CVE-2026-19499) | TP package; exploit path not established | `strfmon`/`strfmon_l` right-padding overflow; no corresponding SDK calls found. Rust formatting does not establish this monetary-formatting path. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12772](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12772) | [CVE-2026-19542](https://security-tracker.debian.org/tracker/CVE-2026-19542) | TP package; exploit path not established | `tdelete` deep-tree stack overflow requires a large C `tsearch` tree; no corresponding SDK calls found. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12770](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12770) | [CVE-2026-6368](https://security-tracker.debian.org/tracker/CVE-2026-6368) | TP package; exploit path not established | `wordexp(WRDE_APPEND)` can yield invalid memory; no SDK wordexp calls found. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12769](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12769) | [CVE-2026-6791](https://security-tracker.debian.org/tracker/CVE-2026-6791) | TP package; exploit path not established | Oversized tilde username causes wordexp stack exhaustion; no explicit SDK wordexp calls found. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12717](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12717) | SASTID | Historical coverage signal | Security/CodeQL already runs for all main pushes and PRs. Historical skipped commits remain in the score window; retain until fresh analysis ages them out. |
| [12716](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12716) | [CVE-2026-27171](https://security-tracker.debian.org/tracker/CVE-2026-27171) | TP package; exploit path not established | Affected source/version detected in the original image; a fixed minimal runtime removes the implicated component. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12617](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12617) | [CVE-2026-6238](https://security-tracker.debian.org/tracker/CVE-2026-6238) | TP package; exploit path not established | Deprecated DNS debug printers can over-read malformed records. These printers are not called by the normal glibc resolver; no SDK printer calls found. Remove affected runtime component; verify both published architectures and fresh scans. |
| [12614](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/12614) | [CVE-2026-5435](https://security-tracker.debian.org/tracker/CVE-2026-5435) | TP package; exploit path not established | TSIG output overflow is in the same deprecated debug printers, not default hostname resolution; no SDK printer calls found. Remove affected runtime component; verify both published architectures and fresh scans. |
| [4619](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/4619) | CITestsID | Coverage gap addressed prospectively | Remove documentation-only CI skips so each future main/PR commit receives tests; preserve genuine historical counts until new runs replace the window. |
| [1985](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/1985) | VulnerabilitiesID | Dependency remediation plus guarded residuals | Stable YARA-X 1.22 selects patched Wasmtime 48.0.7, removing four exceptions and earlier CVEs. The unfiltered graph retains public-only RSA timing applicability and Bincode maintenance metadata; see configuration review. |
| [1984](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/1984) | FuzzingID | Fuzz integration added | Add a seeded libFuzzer target for API response/structured-output parsing and scheduled/PR sanitizer runs. Validate harness and publish results; do not dismiss a valid historical lack of fuzzing. |
| [1982](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/1982) | CIIBestPracticesID | Owner enrollment required | The absent OpenSSF badge is a valid process signal, not a source-code vulnerability. Earning it requires organization enrollment and truthful project attestations; no badge or self-certification is fabricated by this update. |
| [1981](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/1981) | CodeReviewID | Historical review signal | Earlier merged changes lack approvals. This PR supplies evidence for review; require genuine reviewer approval prospectively. Do not manufacture approvals or rewrite historical records. |
| [1957](https://github.com/ThreatFlux/openai_rust_sdk/security/code-scanning/1957) | BranchProtectionID | Verified configuration gap; owner policy decision | GitHub API confirms no required human approval and no required status contexts; active Copilot review rulesets do not supply those protections. Proposed remedy: require PRs, one human approval, Code Owners where configured, and successful CI/security contexts on main. Remote settings are not silently changed by this code PR. |

## Validation record

- `make ci` prerequisites completed successfully inside `make all`: format,
  strict Clippy, package/docs contracts, 1,206 tests (3 ignored), all three
  feature configurations, rustdoc, guarded Cargo audit and cargo-deny.
- Rust **1.97.1** locked all-target/all-feature check passed; build toolchain is
  latest stable **1.99.1**.
- Nightly/libFuzzer AddressSanitizer run completed **1,000 cases** using the
  committed seed and lockfile, without crashes. Production builds use stable.
- Actual ARM64 image passed static PIE/native-provenance/CA checks and real
  YARA valid/invalid compilation plus batch generation. Exported native
  evidence: [native-build-arm64.txt](native-build-arm64.txt).
- Fresh Trivy 0.76.0 database updated 2026-10-21T01:19:53Z: **0 raw OS/Cargo
  findings** on that final image. An independent unfiltered `cargo audit`
  still reports the reviewed public-only RSA timing applicability and Bincode
  maintenance warning; a zero Trivy report does not erase these facts.
- Python syntax, workflow actionlint and staged diff checks passed. Hosted
  native AMD64 build/tests remain required; no local AMD64 build is claimed.
- Optional local coverage/benchmark stages continue after the completed
  standard CI gate; final hosted results are visible on the PR.

Current source/build fixes do not rewrite old main artifacts. Valid alerts and
historical process signals remain visible until replacement scans or real
owner/process actions establish closure. Nothing is merged or deployed by
this PR, and no Jira ticket is required.

Final local ARM64 image: `sha256:b3a597690ae4fff84e95de696b945bffe33328c1454e11f8d6ec54579e232f2b`.
