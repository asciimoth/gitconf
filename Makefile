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
	cp target/release/gitconf out/gitconf

build-arm7:
	rustup target add armv7-unknown-linux-musleabi
	cargo build --release --offline --target=armv7-unknown-linux-musleabi
	mkdir -p ./out
	cp target/release/gitconf out/gitconf

build: build-man build-native

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
