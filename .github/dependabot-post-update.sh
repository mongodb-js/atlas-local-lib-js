#!/usr/bin/env bash
# Runs from the apix-action Dependabot workflow after a dependency bump.
# Only edit files here: the workflow stages the working tree and creates the
# signed commit itself.
set -Eeou pipefail

# Pinned to the version CI uses to verify the notice, so the regenerated file
# matches the checked-in copy. Bump both together.
CARGO_ABOUT_VERSION="0.8.4"

if ! command -v cargo-about >/dev/null 2>&1; then
  cargo install --locked --version "${CARGO_ABOUT_VERSION}" cargo-about
fi

# Keep Cargo.lock stable without forcing offline metadata resolution; fresh
# Dependabot runners may not have every registry index entry cached.
cargo about generate --locked about.hbs > LICENSE-3RD-PARTY.txt
