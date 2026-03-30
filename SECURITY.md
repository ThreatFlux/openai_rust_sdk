# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.5.x   | :white_check_mark: |
| < 1.5   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT open a public GitHub issue.**
2. Email **wyattroersma@gmail.com** with:
   - A description of the vulnerability
   - Steps to reproduce
   - Potential impact
3. You will receive an acknowledgment within 48 hours.
4. A fix will be developed and released as soon as possible, depending on severity.

## Security Scanning

This project uses automated security scanning:

- **Dependabot** for dependency vulnerability alerts
- **CodeQL** for static analysis
- **Trivy** and **Grype** for container image scanning
- **cargo audit** for Rust advisory database checks
- **Semgrep** for security pattern matching
- **OSSF Scorecard** for supply chain security scoring
