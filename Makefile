clear:
	rm -rf ./out

build-man:
	mkdir -p ./out
	gzip -cf contrib/man/gitconf.1 > ./out/gitconf.1.gz
	
