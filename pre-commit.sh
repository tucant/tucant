#!/bin/sh

# SPDX-FileCopyrightText: The tucant Contributors
#
# SPDX-License-Identifier: AGPL-3.0-or-later

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH"

FILES=$(git diff  --name-only --cached --diff-filter=ACMR | sed 's| |\\ |g')
[ -z "$FILES" ] && exit 0

# Prettify all selected files
echo "$FILES" | xargs ./frontend-react/node_modules/.bin/prettier --ignore-unknown --write

./frontend-react/node_modules/.bin/eslint --cache --fix frontend-react

# Add back the modified/prettified files to staging
echo "$FILES" | xargs git add

exit 0