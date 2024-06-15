#!/usr/bin/env bash

PlainFile="PlainFile.txt"
CryptFile="CryptFile.txt"
EncKey="EncKey.txt"

openssl enc -aes-256-cbc \
    -pbkdf2 -iter 1000 -md sha256 -salt -S "4469646144696461" -base64 \
    -in "$PlainFile" -out "$CryptFile" \
    -pass "file:$EncKey"
