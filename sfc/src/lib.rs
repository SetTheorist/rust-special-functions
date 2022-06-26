// cbindgen --config cbindgen.toml --crate sfc --output sfc.h
// gcc test.c -lm -lsfc -L ./target/debug/

#[no_mangle]#[must_use] pub extern "C" fn sf_exp(x:f64) -> f64 { sf::exp::sf_exp(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_m1(x:f64) -> f64 { sf::exp::sf_exp_m1(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_m1vx(x:f64) -> f64 { sf::exp::sf_exp_m1vx(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_expn(n:isize, x:f64) -> f64 { sf::exp::sf_expn(n, x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_men(n:isize, x:f64) -> f64 { sf::exp::sf_exp_men(n, x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_menx(n:isize, x:f64) -> f64 { sf::exp::sf_exp_menx(n, x) }
