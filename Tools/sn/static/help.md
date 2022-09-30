# sn document

- curl: `https://curl.se/`
- httpie: `https://github.com/httpie/httpie`
- mdcat: `https://github.com/lunaryorn/mdcat`

```sh
# Get help document.
curl http://127.0.0.1:8080/help
# Get help document, and use mdcat to render.
curl http://127.0.0.1:8080/help | mdcat
```

## URL

- /help: Show this help document.
- /upload: Upload file (multipart/form-data).
- /upload/binary: Upload file (binary).
- /download: Download file.
- /ws: Websocket connection.
- /exec1: Execute command until finish, return stdout.
- /exec2: Execute command until finish, return stderr.
- /exec3: Execute command in background, return immediately.
          This mode redirect stdin, stdout, stderr all to /dev/null.
- /exec4: Execute command in background, return immediately.
          This mode stdin, stdout, stderr all inherit from parent.


- /exec1, /exec2, /exec3
    `bash -c "command string"` or
    `sh -c "command string"`
    If bash found use bash, else use sh.
- /exec4
    Execute `"command string"` directly, no use `bash -c` or `sh -c`.


## Examples:

```sh
# Upload file (multipart/form-data)
# (httpie) Upload one local file.
# Upload local file `one.jpg` to remote directory `/home/mdgsf/aa`.
http \
  --form POST \
  "http://127.0.0.1:8080/upload?directory=/home/mdgsf/aa" \
  one@/home/mdgsf/bb/one.jpg

# Upload file (multipart/form-data)
# (httpie) Upload multiple local file.
http \
  --form POST \
  "http://127.0.0.1:8080/upload?directory=/home/mdgsf/aa" \
  one@/home/mdgsf/bb/one.jpg
  two@/home/mdgsf/bb/two.jpg
```

```sh
# Upload file (binary)
curl --location --request POST \
  'http://127.0.0.1:8080/upload/binary?filename=/home/mdgsf/one.jpg' \
  --data-binary '@/home/mdgsf/bb/one.jpg'

# Content-Type (option)
# --header 'Content-Type: image/jpeg'
# --header 'Content-Type: application/octet-stream'
```

```sh
# Download file from remote.
# Download remote file `/home/mdgsf/foo.jpg` to local.
curl \
  -o foo.jpg \
  "http://127.0.0.1:8080/download?filename=/home/mdgsf/foo.jpg"
```

```sh
# Excute `bash -c "ls -lh"`, and return stdout.
curl -G \
  --data-urlencode "cmd=ls -lh" \
  http://127.0.0.1:8080/exec1
```

```sh
# Excute `bash -c "ls -lh"`,
# and set working_directory to `/home/mdgsf`,
# and return stdout.
curl -G \
  --data-urlencode "cmd=ls -lh" \
  --data-urlencode "working_directory=/home/mdgsf" \
  http://127.0.0.1:8080/exec1
```

