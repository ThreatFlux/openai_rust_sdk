# Branch protection for `main`

Both repository maintainers are listed in [CODEOWNERS](../.github/CODEOWNERS).
Require one approving code-owner review so a pull request author needs the
other maintainer's approval. Dismiss approvals after a new commit and require
approval of the most recent push.

Configure the `main` branch protection rule with:

- Require a pull request, one approving review from a code owner, and resolution
  of review conversations.
- Apply the rule to administrators. Do not allow force pushes or branch deletion.
- Require the branch to be up to date before merging and these current check
  names (without a workflow-name prefix):
  - `Test Suite (ubuntu-latest, stable)`
  - `Test Suite (macos-latest, stable)`
  - `Test Suite (windows-latest, stable)`
  - `Security Audit`
  - `CodeQL Analysis`
  - `Quality Gate`

The check names above are the job names emitted by the CI, Security, and Code
Quality workflows. Keep this list aligned with those workflows when renaming
jobs. The previous list included names that are not emitted as checks, which
would leave pull requests permanently pending if required.

The auto-release workflow creates a release pull request and tags the merged
main commit. It does not require a direct push to `main`; release pull requests
must satisfy the same protection rule.

Repository administrators can inspect the live rule at **Settings → Branches**
or with `gh api repos/ThreatFlux/openai_rust_sdk/branches/main/protection`.
