#!/usr/bin/env bash
set -e

echo "Building"
../build.sh

echo "Starting tiny-static-web-server."
rm -rf tmp
mkdir tmp
echo "Hello world!" > tmp/index.html
echo "Hello world!" > tmp/index2.html
gzip -9 -k -f -c "tmp/index2.html" > "tmp/index2.html.gz";
../target/release/tiny-static-web-server tmp/ &
PID=$!
sleep 1

./node_modules/newman/bin/newman.js run tiny-static-web-server.postman_collection.json && { kill $PID; true; } || { kill $PID; false; }