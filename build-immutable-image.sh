#!/usr/bin/env bash
set -e

# This script does not require to launch the Docker daemon.
# It creates an image in . named tiny-static-web-server-immutable
if [ -z "$1" ]
  then
    echo "./build-immutable-image.sh THE_DIRECTORY_WITH_YOUR_FILES"
    exit 1
fi
STATIC_DIR=$1

# First export $STATIC_DIR
TEMP_DIR="$(mktemp -d /tmp/tiny-static-web-server.XXXXXX)"
echo "Working in $TEMP_DIR (will be deleted afterward if exits 0)"

curl --silent https://raw.githubusercontent.com/sdenel/docker-pull/master/docker-pull -o "$TEMP_DIR/docker-pull" && chmod +x "$TEMP_DIR/docker-pull"
curl --silent https://raw.githubusercontent.com/sdenel/docker-add-layer/master/docker-add-layer -o "$TEMP_DIR/docker-add-layer" && chmod +x "$TEMP_DIR/docker-add-layer"


mkdir -p "$TEMP_DIR/www/www/"
echo "Copying $STATIC_DIR to $TEMP_DIR/www/www/"
cp -r "$STATIC_DIR"/* "$TEMP_DIR/www/www/"
for file in `find "$TEMP_DIR/www/www/" -type f | grep -v .gz`; do
    gzip -9 -k -f -c "$file" > "$file.gz";
    OLD_SIZE=`stat --printf="%s" "$file"`
    NEW_SIZE=`stat --printf="%s" "$file.gz"`
    echo "Compressed $file: $OLD_SIZE -> $NEW_SIZE"
done

# Downloading tiny-static-web-server
$TEMP_DIR/docker-pull index.docker.io/sdenel/tiny-static-web-server "$TEMP_DIR/base-image"
$TEMP_DIR/docker-add-layer "$TEMP_DIR/base-image" "$TEMP_DIR/www/" tiny-static-web-server-immutable

rm -rf "$TEMP_DIR"
