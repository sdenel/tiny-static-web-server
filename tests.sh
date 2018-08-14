#!/usr/bin/env bash
set -e

echo "Building"
./build.sh

echo "Starting tiny-static-web-server."
rm -rf tmp
mkdir tmp
echo "Hello world!" > tmp/index.html
echo "Hello world!" > tmp/index2.html
gzip -9 -k -f -c "tmp/index2.html" > "tmp/index2.html.gz";
./target/release/tiny-static-web-server tmp/ &
PID=$!
sleep 1

#
# index.html
#
echo "curl http://localhost:8080/..."
DOWNLOADED=`curl http://localhost:8080/`
echo "$DOWNLOADED"
if [[ "$DOWNLOADED" != "Hello world!" ]]
then
    echo -e "\"$DOWNLOADED\" is not equal to \"Hello world!\""
    echo "Killing the server..."
    kill $PID
    exit 1
else
    echo "index.html: success"
fi

#
# index2.html (gzipped)
#
echo "curl http://localhost:8080/index2.html..."
DOWNLOADED=`curl -v -H "Accept-Encoding: gzip" http://localhost:8080/index2.html | zcat`
echo "$DOWNLOADED"
if [[ "$DOWNLOADED" != "Hello world!" ]]
then
    echo -e "\"$DOWNLOADED\" is not equal to \"Hello world!\""
    echo "Killing the server..."
    kill $PID
    exit 1
else
    echo "index2.html: success"
fi

echo "All good!"
echo "Killing the server..."
kill $PID
exit 0
