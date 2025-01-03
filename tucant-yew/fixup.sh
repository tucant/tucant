#!/usr/bin/env bash
set -ex

HASH=$(sha256sum $TRUNK_STAGING_DIR/index.html | cut -d " " -f 1)
sed -n '/<script type="module"/,/<\/script>/p' $TRUNK_STAGING_DIR/index.html > $TRUNK_STAGING_DIR/test-$HASH.js
sed -i.bak '/script/d' $TRUNK_STAGING_DIR/test-$HASH.js
sed -i.bak '/<script type="module"/,/<\/script>/d' $TRUNK_STAGING_DIR/index.html
sed -i.bak "s@</title>@</title><script type=\"module\" src=\"${TRUNK_PUBLIC_URL}test-$HASH.js\"></script>@" $TRUNK_STAGING_DIR/index.html
rm $TRUNK_STAGING_DIR/test-$HASH.js.bak
rm $TRUNK_STAGING_DIR/index.html.bak
