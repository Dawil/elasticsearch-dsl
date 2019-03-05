with import <nixpkgs> {};

stdenv.mkDerivation rec {
	name = "es-dsl";
	env = buildEnv { name = name; paths = buildInputs; };
	buildInputs = [
		python36
		pipenv
		cargo
		ruby_2_5
	];
}
