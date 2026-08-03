# Security Policy

## Supported versions

Security fixes target the latest published release and the current default branch. Older releases
may be assessed when impact and backport risk justify it, but they are not covered by a standing
support commitment.

## Report a vulnerability

Do not open a public issue, discussion, or pull request for a suspected vulnerability.

1. Use this repository's private vulnerability reporting form under **Security → Advisories →
   Report a vulnerability**, when available.
2. Otherwise, email [security@threatflux.ai](mailto:security@threatflux.ai).

Include the affected version or commit, vulnerability type, likely impact, required preconditions,
and a minimal reproduction or proof of concept. Do not send live credentials, personal data,
proprietary samples, or large malware artifacts in the initial report; maintainers can arrange a
safer transfer method when necessary.

Maintainers will assess reproducibility, severity, affected versions, and remediation options.
Please coordinate public disclosure so users have a reasonable opportunity to update. ThreatFlux
does not currently promise an organization-wide response SLA or monetary reward.

## Security scanning

This project uses automated security scanning:

- **Dependabot** for dependency vulnerability alerts
- **CodeQL** for static analysis
- **Trivy** and **Grype** for container image scanning
- **cargo audit** for Rust advisory database checks
- **Semgrep** for security pattern matching
- **OSSF Scorecard** for supply chain security scoring
