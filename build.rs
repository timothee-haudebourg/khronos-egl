#[cfg(feature="static")]
extern crate pkg_config;


fn main() {
	#[cfg(feature="static")]
	{
		pkg_config::Config::new()
		.atleast_version("1")
		.probe("egl")
		.unwrap();
	}
}