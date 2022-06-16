# AlmaLinux8 Rust Code Space

```sh
#!/bin/sh

echo "init sh script"

dnf update -y && dnf upgrade -y
dnf install -y git glibc-devel.x86_64 glibc-headers.x86_64 \
    vim curl

curl https://sh.rustup.rs -sSf | sh -s -- -y --profile complete

echo "init sh script end"
```
