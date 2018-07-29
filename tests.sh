#!/usr/bin/env bash
set -e

echo "Building"
./build.sh

echo "Starting tiny-static-web-server ."
./target/release/tiny-static-web-server . &
PID=$!

echo "curl http://localhost:8080/README.md..."
README_DOWNLOADED=`curl http://localhost:8080/README.md`
if [[ "$README_DOWNLOADED" != "`cat README.md`" ]]
then
  echo -e "\nReadme downloaded was different than Readme.md!"
fi

echo "Killing the server..."
kill $PID

