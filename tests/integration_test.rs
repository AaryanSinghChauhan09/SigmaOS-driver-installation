// Integration test for SigmaOS-driver-installation
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use driver_installation as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
