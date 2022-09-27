#!/bin/sh

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH"

FILES=$(git diff  --name-only --cached --diff-filter=ACMR | sed 's| |\\ |g')
[ -z "$FILES" ] && exit 0

# Prettify all selected files
echo "$FILES" | xargs ./tucant/node_modules/.bin/prettier --ignore-unknown --write

./tucant/node_modules/.bin/eslint --cache --fix tucant

# Add back the modified/prettified files to staging
echo "$FILES" | xargs git add

exit 0