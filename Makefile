build: out/joule

clean:
	rm -rf out

out/joule:
	docker buildx build --target export --output out .
