#!/bin/sh

if [ $# = 1 ]; then
	TAG=$1
else
	TAG=$(git tag | tail -1)
fi

VERSION_NR=$(echo "$TAG" | cut -c2-)

printf "\e[34m Building release version %s\e[0m\n\n" "$VERSION_NR"


printf "\e[34m Checking out %s \e[0m\n\n" "$TAG"
git checkout "tags/$TAG" > /dev/null 2>&1

printf "\e[34m Building with cargo\e[0m\n"
cargo build --release --locked

printf "\n\n"

printf "\e[34m Copying needed files for release\e[0m\n\n"
mkdir release
cp target/release/lqsd release/
cp LICENSE release/
cp manpage.adoc release/
cd release || return

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

printf "\e[34m Building documentation\e[0m\n"
asciidoctor -b manpage manpage.adoc

printf "\n\n"

printf "\e[34m Compressing to tar.zst\e[0m\n\n"
tar cf lqsd_"${VERSION_NR}"_x86_64.tar.zst ./* --zstd

mv ./*.tar.zst ..

cd ..

printf "\e[34m removing leftover files\e[0m\n\n"
rm -r release

printf "\e[34m Returning to main\e[0m\n\n"
git checkout main > /dev/null 2>&1
