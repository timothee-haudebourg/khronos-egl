with import <nixpkgs> {};

runCommand "dummy" {
	buildInputs = [
		libGL
		pkg-config
	];
} ""
