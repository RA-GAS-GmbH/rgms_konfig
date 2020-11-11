#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("rgms_konfig.ico");
    #[cfg(features = "ra-gas")]
    res.set_icon("rgms_konfig-ra-gas.ico");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}