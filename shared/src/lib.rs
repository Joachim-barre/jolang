use semver::Version;

pub mod ffi;
pub mod ir;

pub static VERSION_STR : &str = env!("CARGO_PKG_VERSION");

lazy_static::lazy_static!{
    pub static ref VERSION : Version = Version::parse(VERSION_STR).expect("cannot parse package version");
}
