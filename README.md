**Work in progress. It works, but it's ugly and badly documented!**

[![Build Status](https://travis-ci.com/sdenel/tiny-static-web-server.svg?branch=master)](https://travis-ci.com/sdenel/tiny-static-web-server)

A particularly small and fast static web server, written in Rust, aimed for containerization.
* Aimed to be used with a light Docker image (for example with a [distroless image](https://github.com/GoogleContainerTools/distroless), the size of the full image is around ~10Mb)
* Aimed to be fast:
    * Files are automatically put in RAM during the first request, once and for all (served files are expected to be immutable!)
    * Still to do:
        * Use .gz files when available
        * Compute hash and respond 301 if there was not change
        * Send specific cache headers when the file is expected to never change (files with a hash in their name)
    
# Usage
I am using this image to serve a webapp written in Angular. The purpose is to have an immutable and light container.