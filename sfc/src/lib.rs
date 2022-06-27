// cbindgen --config cbindgen.toml --crate sfc --output sfc.h
// gcc test.c -lm -lsfc -L ./target/debug/


#[no_mangle]#[must_use] pub extern "C" fn sf_airy_ai(x:f64) -> f64 { sf::airy::sf_airy_ai(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_airy_bi(x:f64) -> f64 { sf::airy::sf_airy_bi(x) }
//#[no_mangle]#[must_use] pub extern "C" fn sf_airy_aibi(x:f64) -> (f64,f64) { sf::airy::sf_airy_aibi(x) }

#[no_mangle]#[must_use] pub extern "C" fn sf_dilog(x:f64) -> f64 { sf::polylog::sf_dilog(x) }

#[no_mangle]#[must_use] pub extern "C" fn sf_erf(x:f64) -> f64 { sf::erf::sf_erf(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_erfc(x:f64) -> f64 { sf::erf::sf_erfc(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_erf_inv(x:f64) -> f64 { sf::erf::sf_erf_inv(x) }

#[no_mangle]#[must_use] pub extern "C" fn sf_exp(x:f64) -> f64 { sf::exp::sf_exp(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_m1(x:f64) -> f64 { sf::exp::sf_exp_m1(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_m1vx(x:f64) -> f64 { sf::exp::sf_exp_m1vx(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_expn(n:isize, x:f64) -> f64 { sf::exp::sf_expn(n, x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_men(n:isize, x:f64) -> f64 { sf::exp::sf_exp_men(n, x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_exp_menx(n:isize, x:f64) -> f64 { sf::exp::sf_exp_menx(n, x) }

#[no_mangle]#[must_use] pub extern "C" fn sf_gamma(x:f64) -> f64 { sf::gamma::sf_gamma(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_lngamma(x:f64) -> f64 { sf::gamma::sf_lngamma(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_digamma(x:f64) -> f64 { sf::gamma::sf_digamma(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_beta(a:f64, b:f64) -> f64 { sf::gamma::sf_beta(a, b) }

#[no_mangle]#[must_use] pub extern "C" fn sf_zeta(x:f64) -> f64 { sf::zeta::sf_zeta(x) }
#[no_mangle]#[must_use] pub extern "C" fn sf_zeta_m1(x:f64) -> f64 { sf::zeta::sf_zeta_m1(x) }