// cbindgen --config cbindgen.toml --crate sfc --output sfc.h
// gcc test.c -lm -lsfc -L ./target/debug/

#[no_mangle]#[must_use]
pub extern "C" fn sf_exp(x:f64) -> f64 {
    sf::exp::sf_exp(x)
}
