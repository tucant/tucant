#!/bin/sh
set -ex

sed -n '/<script/,/<\/script>/p' $TRUNK_STAGING_DIR/index.html > $TRUNK_STAGING_DIR/test.js
sed -i '/script/d' $TRUNK_STAGING_DIR/test.js
sed -i '/<script/,/<\/script>/d' $TRUNK_STAGING_DIR/index.html
sed -i 's/<\/title>/<\/title><script src="\/test.js"><\/script>/' $TRUNK_STAGING_DIR/index.html