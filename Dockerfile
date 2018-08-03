FROM gcr.io/distroless/cc
COPY target/release/tiny-static-web-server www /
WORKDIR /
ENTRYPOINT ["/tiny-static-web-server"]
CMD ["/www/"]
# You will have to bind /www to a volume or copy something in it!