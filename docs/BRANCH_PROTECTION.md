# Branch Protection Rules

This document outlines the recommended branch protection rules for the OpenAI Rust SDK repository. These settings should be configured in GitHub Settings → Branches.

## Main Branch Protection

### Rule Pattern: `main`

#### Required Status Checks
Enable "Require status checks to pass before merging" with these checks:
- [ ] CI / Quick Check
- [ ] CI / Test Suite (ubuntu-latest)
- [ ] CI / Test Suite (macos-latest)
- [ ] CI / Test Suite (windows-latest)
- [ ] CI / CI Success
- [ ] Security / Security Audit
- [ ] Code Quality / Lint
- [ ] Code Quality / Test Coverage
- [ ] Code Quality / Quality Gate
- [ ] Docker / Docker Build Success

#### Pull Request Requirements
- [ ] **Require pull request reviews before merging**
  - Required approving reviews: 1
  - Dismiss stale pull request approvals when new commits are pushed
  - Require review from CODEOWNERS (if configured)
  
- [ ] **Require conversation resolution before merging**

#### Additional Settings
- [ ] **Require branches to be up to date before merging**
- [ ] **Require signed commits** (optional but recommended)
- [ ] **Include administrators** (enforce for everyone)
- [ ] **Restrict who can push to matching branches**
  - Add specific users/teams who can push directly
- [ ] **Allow force pushes**: Disabled
- [ ] **Allow deletions**: Disabled

## Develop Branch Protection (if used)

### Rule Pattern: `develop`

#### Required Status Checks
- [ ] CI / Quick Check
- [ ] CI / Test Suite (ubuntu-latest)
- [ ] Security / Security Audit

#### Pull Request Requirements
- [ ] **Require pull request reviews before merging**
  - Required approving reviews: 1
  
## How to Configure in GitHub

1. Navigate to your repository on GitHub
2. Go to **Settings** → **Branches**
3. Click **Add rule** under "Branch protection rules"
4. Enter the branch name pattern (e.g., `main`)
5. Configure the settings as outlined above
6. Click **Create** or **Save changes**

## Automation with GitHub CLI

You can also set up branch protection using the GitHub CLI:

```bash
# For main branch
gh api repos/threatflux/openai_rust_sdk/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["CI / Quick Check","CI / Test Suite (ubuntu-latest)","CI / CI Success","Security / Security Audit","Code Quality / Lint","Code Quality / Test Coverage"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true}' \
  --field restrictions=null \
  --field allow_force_pushes=false \
  --field allow_deletions=false \
  --field required_conversation_resolution=true
```

## Benefits of These Rules

1. **Code Quality**: Ensures all code passes tests, linting, and security checks
2. **Review Process**: Guarantees code review before merging
3. **Stability**: Prevents direct pushes to protected branches
4. **Security**: Runs security audits on all changes
5. **Consistency**: Enforces coding standards through automated checks

## Exceptions and Overrides

In emergency situations, repository administrators can:
1. Temporarily disable branch protection
2. Use admin override to merge critical fixes
3. Re-enable protection after the emergency

Always document any emergency overrides in the PR description.

## Related Documentation

- [GitHub Branch Protection Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)
- [CODEOWNERS file format](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)