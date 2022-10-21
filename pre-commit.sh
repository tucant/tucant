#!/bin/sh

# SPDX-FileCopyrightText: The tucant Contributors
#
# SPDX-License-Identifier: AGPL-3.0-or-later

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH"

./frontend-react/node_modules/.bin/prettier --ignore-unknown --write .
./frontend-react/node_modules/.bin/eslint --fix frontend-react

FILES=$(git diff --name-only --cached)
echo "$FILES" | xargs git add

exit 0