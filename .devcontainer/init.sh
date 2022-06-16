#!/bin/sh

echo "init sh script"

yum update -y && yum upgrade -y
yum install -y git glibc-devel.x86_64 glibc-headers.x86_64 gcc \
    vim curl

curl https://sh.rustup.rs -sSf | sh -s -- -y --profile complete

echo "init sh script end"
