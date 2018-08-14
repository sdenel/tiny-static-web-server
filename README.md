**Work in progress. It works, but it's ugly and badly documented!**

[![Build Status](https://travis-ci.com/sdenel/tiny-static-web-server.svg?branch=master)](https://travis-ci.com/sdenel/tiny-static-web-server)

A particularly small and fast static web server, written in Rust, aimed to be used inside a container.
* Aims to be fast:
    * Files are automatically put in RAM during the first request, once and for all (served files are expected to be immutable!)
    * Sends gzip version when filename + ".gz" exists
    * Compute sha256, put it in ETag and respond 301 if there was no change.
    * Sends specific cache headers when the file is expected to never change (files with a hash in their name).
    * Still to do: always redirect /index.html to /
* Aims to be small: The size of the full Docker image is around ~10Mb.
* Aims to be compatible with webapps: if a path does not map to a known key, and does no contain a dot, /index.html is returned. 
# Usage
I am using this image to serve a webapp written in Angular. The purpose is to have an immutable and light container.
(script not read yet)
```bash
DIR="www"
for file in `find $DIR -type f | grep -v .gz`; do gzip -9 -k -f -c "$file" > "$file.gz"; done
```

# With Docker
```bash
docker run -v WHERE_YOUR_FILES_BELONG:/www/ -p8080:8080 sdenel/tiny-static-web-server
```

# Installation
## Locally
```bash
curl --silent -L "https://api.github.com/repos/sdenel/tiny-static-web-server/releases/latest" | \
jq '.assets[0].browser_download_url' | \
xargs curl --silent  -L -o tiny-static-web-server --url
chmod +x tiny-static-web-server
```
## Globally
```bash
curl --silent -L "https://api.github.com/repos/sdenel/tiny-static-web-server/releases/latest" | \
jq '.assets[0].browser_download_url' | \
xargs sudo curl --silent  -L -o /usr/local/bin/tiny-static-web-server --url
sudo chmod +x /usr/local/bin/tiny-static-web-server
```

# Advanced usages
## To create a full Docker image, containing your files to serve, without launching the daemon
Creating immutable images in production is a good pattern. use this script to create one;
```bash
./build-immutable-image.sh THE_DIRECTORY_WITH_YOUR_FILES
# The result image is saved in . with the name "tiny-static-web-server-immutable" 
# Test it: docker load < tiny-static-web-server-immutable
# docker run -p8080:8080 SHA_GIVEN_BY_THE_PREVIOUS_CMD
```

Even better, you can create an immutable image without cloning this repository:

It's both downloadless and daemonless :)
```bash
curl -sL https://raw.githubusercontent.com/sdenel/tiny-static-web-server/master/build-immutable-image.sh | \
bash /dev/stdin THE_DIRECTORY_WITH_YOUR_FILES
```
 
## To build .gz version of your files
Use the following script to build a .gz version of all your files:
```bash
$STATIC_DIR="YOURDIRECTORY"
for file in `find $DIR -type f | grep -v .gz`; do
    gzip -9 -k -f -c "$file" > "$file.gz";
    OLD_SIZE=`stat --printf="%s" "$file"`
    NEW_SIZE=`stat --printf="%s" "$file.gz"`
    echo "Compressed $file: $OLD_SIZE -> $NEW_SIZE"
done
```

## To build the binary yourself
```bash
./build.sh
# The binary is now in target/release/
```

## To build the Docker image yourself
```bash
./hooks/pre_build
docker build --no-cache . -t aprettyimagename
```
