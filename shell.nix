with import <nixpkgs> {};

runCommand "dummy" {
	buildInputs = [
		gcc
		libGL
		pkg-config
	];
} ""
