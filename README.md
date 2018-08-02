**Work in progress. It works, but it's ugly and badly documented!**

[![Build Status](https://travis-ci.com/sdenel/tiny-static-web-server.svg?branch=master)](https://travis-ci.com/sdenel/tiny-static-web-server)

A particularly small and fast static web server, written in Rust, aimed for containerization.
* Aimed to be used with a light Docker image (for example with a [distroless image](https://github.com/GoogleContainerTools/distroless), the size of the full image is around ~10Mb)
* Aimed to be fast:
    * Files are automatically put in RAM during the first request, once and for all (served files are expected to be immutable!)
    * Send gzip verson when filename + ".gz" exists
    * Compute sha256, put it in ETag and respond 301 if there was not change
    * Still to do: Send specific cache headers when the file is expected to never change (files with a hash in their name)
    
# Usage
I am using this image to serve a webapp written in Angular. The purpose is to have an immutable and light container.
(script not read yet)
```bash
DIR="www"
for file in `find $DIR -type f | grep -v .gz`; do gzip -9 -k -f -c "$file" > "$file.gz"; done
```


# Installation
```bash
curl --silent -L "https://api.github.com/repos/sdenel/tiny-static-web-server/releases/latest" | \
jq '.assets[0].browser_download_url' | \
xargs sudo curl --silent  -L -o /usr/local/bin/tiny-static-web-server --url
sudo chmod +x /usr/local/bin/tiny-static-web-server
```