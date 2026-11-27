# Releasing

1. Run **Prepare Release** from the `main` branch in GitHub Actions. Enter an
   exact version or a semver bump accepted by `npm version`.
2. Review and approve the generated `release/prepare-v<version>` pull request.
   It updates `package.json`, `Cargo.toml`, `Cargo.lock`, generated bindings,
   the changelog, and third-party license notice. Auto-merge completes it after
   required checks pass.
3. After the PR merges, **Tag Release** tags the merge commit. The tag triggers
   the **Release** workflow to build artifacts, publish the GitHub release, and
   publish the package to npm.
