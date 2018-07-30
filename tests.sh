#!/usr/bin/env bash
set -e

echo "Building"
./build.sh

echo "Starting tiny-static-web-server ."
rm -rf tmp
mkdir tmp
echo "Hello world!" > tmp/index.html
./target/release/tiny-static-web-server tmp/ &
PID=$!
sleep 1

echo "curl http://localhost:8080/..."
DOWNLOADED=`curl http://localhost:8080/`
if [[ "$DOWNLOADED" != "Hello world!" ]]
then
    echo -e "\"$DOWNLOADED\" is not equal to \"Hello world!\""
    echo "Killing the server..."
    kill $PID
    exit 1
else
    echo "Killing the server..."
    kill $PID
fi



