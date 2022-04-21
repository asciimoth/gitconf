clear:
	rm -rf ./out
	rm -rf ./target

build-man:
	mkdir -p ./out
	gzip -cf contrib/man/gitconf.1 > ./out/gitconf.1.gz

build-bin:
	cargo build --release --offline
	cp target/release/gitconf out/gitconf

build: build-man build-bin

install:
	cp out/gitconf /usr/bin/gitconf
	cp out/gitconf.1.gz /usr/share/man/man1/gitconf.1.gz
