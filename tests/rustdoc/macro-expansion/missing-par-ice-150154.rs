// Ensure rustdoc does not ICE When expanding a macro that
// generates a `PatKind::Missing` pattern.

//@ compile-flags: -Zunstable-options --generate-macro-expansion

#![crate_name = "foo"]

//@ has 'src/foo/missing-par-ice-150154.rs.html'

macro_rules! foo {
    () => {
        fn(())
    }
}

fn bar() {
    let _: foo!();
}
