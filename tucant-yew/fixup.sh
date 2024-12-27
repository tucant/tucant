#!/bin/sh

sed -n '/<script/,/<\/script>/p' dist/index.html > dist/test.js
sed -i '/script/d' ./dist/test.js
sed -i '/<script/,/<\/script>/d' dist/index.html
sed -i 's/<\/title>/<\/title><script src="\/test.js"><\/script>/' dist/index.html