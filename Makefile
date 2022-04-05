docker:
	docker run --volume $(PWD):/dgen -w /dgen -t joshcody/dgen-build .