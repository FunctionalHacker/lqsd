#!/bin/bash

if (( $# == 1 )); then
	TAG=$1
else
	TAG=$(git tag | tail -n1)
fi

VERSION_NR=$(printf $TAG | cut -c2-)

printf "\e[34m Building release version $VERSION_NR\e[0m\n\n"


printf "\e[34m Checking out $TAG\e[0m\n\n"
git checkout tags/$TAG > /dev/null 2>&1

printf "\e[34m Building with cargo\e[0m\n"
cargo build --release --locked

printf "\n\n"

printf "\e[34m Copying needed files for release\e[0m\n\n"
mkdir release
cp target/release/lqsd release/
cp LICENSE release/
cd release

printf "\e[34m Signing binary with GPG\e[0m\n"
gpg --detach-sign --armor lqsd

printf "\n"

printf "\e[34m Calculating checksums\e[0m\n\n"
sha256sum lqsd  > lqsd.sha256
md5sum lqsd  > lqsd.md5

printf "\e[34m Verifying GPG signature\e[0m\n"
gpg --verify lqsd.asc

printf "\n\n"

printf "\e[34m Validating checksums\e[0m\n"
sha256sum -c lqsd.sha256
md5sum -c lqsd.md5

printf "\n\n"


printf "\e[34m Compressing to tar.zst\e[0m\n\n"
tar cf lqsd_${VERSION_NR}_x86_64.tar.zst * --zstd

mv *.tar.zst ..

cd ..

printf "\e[34m removing leftover files\e[0m\n\n"
rm -r release

printf "\e[34m Returning to master\e[0m\n\n"
git checkout master > /dev/null 2>&1
