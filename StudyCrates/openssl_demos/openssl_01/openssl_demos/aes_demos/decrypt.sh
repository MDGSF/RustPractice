#!/usr/bin/env bash

EncKey="EncKey.txt"
CryptFile="CryptFile.txt"
DecryptedFile="DecryptedFile.txt"

openssl enc -d -aes-256-cbc \
    -pbkdf2 -iter 1000 -md sha256 -base64 \
    -in "$CryptFile" \
    -out "$DecryptedFile" \
    -pass "file:$EncKey" -p
