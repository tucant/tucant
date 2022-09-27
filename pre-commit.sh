#!/bin/sh

cd $(dirname "$0")

FILES=$(git diff  --name-only --cached --diff-filter=ACMR | sed 's| |\\ |g')
[ -z "$FILES" ] && exit 0

# Prettify all selected files
echo "$FILES" | xargs ./tucant/node_modules/.bin/prettier --ignore-unknown --write

# Add back the modified/prettified files to staging
echo "$FILES" | xargs git add

./tucant/node_modules/.bin/eslint --cache --fix

exit 0