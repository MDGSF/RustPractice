## prod image on FC
FROM debian:stretch-slim
COPY ./hello/target/debug/hello /root/hello
WORKDIR /root
ENTRYPOINT ["/root/hello"]