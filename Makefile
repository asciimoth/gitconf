PKG_NAME = $(shell basename -s .git $$(git remote get-url origin))
PKG_VERSION = $(shell echo "$$(cargo pkgid | cut -d# -f2-)-$$(git rev-list --all --count)")
BRANCH = $(shell echo "-$$(git rev-parse --abbrev-ref HEAD)")

ifeq "$(BRANCH)" "-main"
BRANCH = ""
endif

ifeq "$(BRANCH)" "-master"
BRANCH = ""
endif

PKG_NAME := "$(PKG_NAME)$(BRANCH)"

clear:
	rm -rf ./out
	rm -rf ./target

build-man:
	mkdir -p ./out
	gzip -cf contrib/man/gitconf.1 > ./out/gitconf.1.gz

build-native:
	cargo build --release --offline
	mkdir -p ./out
	cp target/release/gitconf out/gitconf

build-x64:
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --offline --target=x86_64-unknown-linux-musl
	mkdir -p ./out
	cp target/x86_64-unknown-linux-musl/release/gitconf out/gitconf

build-arm7:
	rustup target add armv7-unknown-linux-musleabi
	cargo build --release --offline --target=armv7-unknown-linux-musleabi
	mkdir -p ./out
	cp target/armv7-unknown-linux-musleabi/release/gitconf out/gitconf

build: build-man build-native

create-deb-template: build-man
	mkdir -p ./out/deb/DEBIAN
	mkdir -p ./out/deb/usr/bin/
	mkdir -p ./out/deb/usr/share/man/man1/
	mkdir -p ./out/deb/etc/.gitconf/profiles
	mkdir -p ./out/deb/etc/.gitconf/current
	cp contrib/deb/control out/deb/DEBIAN/control
	echo "Package: $(PKG_NAME)" >> out/deb/DEBIAN/control
	echo "Version: $(PKG_VERSION)" >> out/deb/DEBIAN/control
	cp out/gitconf.1.gz out/deb/usr/share/man/man1/gitconf.1.gz
	cp contrib/config/DEFAULT out/deb/etc/.gitconf/profiles/DEFAULT
	cp contrib/config/DEFAULT out/deb/etc/.gitconf/current/DEFAULT

build-deb-x64: build-x64 create-deb-template
	cp -r out/deb out/$(PKG_NAME)-deb-x64
	echo "Architecture: amd64" >> out/$(PKG_NAME)-deb-x64/DEBIAN/control
	cp out/gitconf out/$(PKG_NAME)-deb-x64/usr/bin/gitconf
	dpkg-deb --build out/$(PKG_NAME)-deb-x64

build-deb-arm7: build-arm7 create-deb-template
	cp -r out/deb out/$(PKG_NAME)-deb-arm7
	echo "Architecture: armhf" >> out/$(PKG_NAME)-deb-arm7/DEBIAN/control
	cp out/gitconf out/$(PKG_NAME)-deb-arm7/usr/bin/gitconf
	dpkg-deb --build out/$(PKG_NAME)-deb-arm7

install:
	cp out/gitconf /usr/bin/gitconf
	cp out/gitconf.1.gz /usr/share/man/man1/gitconf.1.gz
	mkdir -p /etc/.gitconf/profiles
	mkdir -p /etc/.gitconf/current
	cp contrib/config/DEFAULT /etc/.gitconf/profiles/DEFAULT
	cp contrib/config/DEFAULT /etc/.gitconf/current/DEFAULT

uninstall:
	rm /usr/bin/gitconf
	rm /usr/share/man/man1/gitconf.1.gz
