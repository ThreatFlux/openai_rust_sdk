# Releasing

Releases are cut by the Auto Release workflow after CI and Security pass on `main`. The `v*` tag
it pushes starts `release.yml` (release assets and the crates.io publish) and `docker.yml` (the
container image). This page covers where `docker.yml` publishes the container image and how to
turn Docker Hub publishing on or off.

## Container images

GitHub Container Registry (GHCR) is the primary registry and is always published:

- Image: `ghcr.io/threatflux/openai_rust_sdk`, for `linux/amd64` and `linux/arm64`.
- Tags: the branch name, the short commit SHA, `latest` on `main`, and `X.Y.Z`, `X.Y` and `X`
  for `vX.Y.Z` tags.
- Signature: runs on `main` and on `v*` tags sign the multi-arch index with keyless cosign once
  the Trivy scan and the startup tests on both platforms have succeeded.
- SBOM: the image carries a CycloneDX SBOM at `/usr/share/doc/openai-rust-sdk/sbom.cdx.json`, and
  every run that publishes the image keeps an SPDX SBOM as a workflow artifact.

Verify an image before you use it:

```bash
cosign verify ghcr.io/threatflux/openai_rust_sdk:latest \
  --certificate-identity-regexp '^https://github\.com/ThreatFlux/openai_rust_sdk/\.github/workflows/docker\.yml@refs/(heads/main|tags/v.+)$' \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
```

## Docker Hub (off by default)

Docker Hub publishing is opt-in. It uses the same switch as the other ThreatFlux Rust
repositories: the `RUST_TEMPLATE_PUBLISH_DOCKERHUB` repository or organization variable.

| `RUST_TEMPLATE_PUBLISH_DOCKERHUB` | Result |
| --- | --- |
| unset or `false` | GHCR only. No Docker Hub login step runs, the Docker Hub secrets are not read, and no `docker.io` tags are created. |
| `true`, with `DOCKERHUB_USERNAME` and `DOCKERHUB_TOKEN` available | GHCR and Docker Hub. |
| `true`, with either secret missing | GHCR only, with a warning in the run. |

When it is on, `docker.yml` pushes the same multi-arch index, with the same digest and tags, to
`docker.io/<namespace>/openai-rust-sdk`, along with the `base-rust-*` base image tags it also
pushes to GHCR. The namespace is the `RUST_TEMPLATE_DOCKERHUB_NAMESPACE` variable, or `threatflux`
when that is unset. On `main` and `v*` tags, the Docker Hub image is signed in the same job and
with the same keyless identity as the GHCR image, so the `cosign verify` command above works with
the `docker.io` reference too. Pull request builds never push to either registry.

Images already published to Docker Hub stay where they are. Turning publishing off only stops new
pushes.

### Turn Docker Hub publishing on

1. Create a Docker Hub access token that can push only to the repositories this workflow
   publishes (`<namespace>/openai-rust-sdk`):
   - Preferred (Docker Team or Business): an organization access token for the Docker Hub
     organization, with repository access limited to `openai-rust-sdk` and the image push scope
     (`scope-image-push`, which includes pull). Grant no delete, repository-admin or
     organization scopes. Its login username is the organization name.
   - Otherwise: a personal access token with **Read & Write** access permissions and no
     **Delete**, created on a dedicated account that has write access only to that repository.
     Its login username is that account's Docker ID.
2. Store the token as the `DOCKERHUB_TOKEN` Actions secret and the matching login username as
   `DOCKERHUB_USERNAME`. Use organization secrets shared with this repository, or repository
   secrets.
3. If the images belong in a namespace other than `threatflux`, set the
   `RUST_TEMPLATE_DOCKERHUB_NAMESPACE` variable.
4. Set the `RUST_TEMPLATE_PUBLISH_DOCKERHUB` variable to `true`, at repository level for this
   repository alone or at organization level for every repository that reads it.
5. Run `docker.yml` on `main` (push or **Run workflow**). Confirm that the **Log in to Docker
   Hub** and **Sign Docker Hub image** steps ran and that the run summary lists the `docker.io`
   image, then run `cosign verify` against the `docker.io` reference.

### Turn Docker Hub publishing off

Delete the `RUST_TEMPLATE_PUBLISH_DOCKERHUB` variable or set it to `false`. Revoke the Docker Hub
token, and remove the two secrets, once no repository needs them.
