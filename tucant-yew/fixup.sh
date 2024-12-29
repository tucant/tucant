#!/usr/bin/env bash
set -ex

sed -n '/<script type="module"/,/<\/script>/p' $TRUNK_STAGING_DIR/index.html > $TRUNK_STAGING_DIR/test.js
sed -i.bak '/script/d' $TRUNK_STAGING_DIR/test.js
sed -i.bak '/<script type="module"/,/<\/script>/d' $TRUNK_STAGING_DIR/index.html
sed -i.bak "s@</title>@</title><script type=\"module\" src=\"${TRUNK_PUBLIC_URL}test.js\"></script>@" $TRUNK_STAGING_DIR/index.html
rm $TRUNK_STAGING_DIR/test.js.bak
rm $TRUNK_STAGING_DIR/index.html.bak
