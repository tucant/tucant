#!/bin/sh

# SPDX-FileCopyrightText: The tucant Contributors
#
# SPDX-License-Identifier: AGPL-3.0-or-later

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH"

cd frontend-react
npm run check-fix
cd ..

FILES=$(git diff --name-only --cached)
echo "$FILES" | xargs git add

exit 0