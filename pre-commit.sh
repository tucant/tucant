#!/bin/sh

# SPDX-FileCopyrightText: The tucant Contributors
#
# SPDX-License-Identifier: AGPL-3.0-or-later

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH"

cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features

cd frontend-react
npm run check-fix
cd ..

FILES=$(git diff --name-only --cached)
echo "$FILES" | xargs git add

exit 0