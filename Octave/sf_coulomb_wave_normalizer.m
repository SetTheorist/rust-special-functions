## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_coulomb_wave_normalizer (@var{ell}, @var{eta})
## Compute ...
## @end deftypefn
function res = sf_coulomb_wave_normalizer(ell, eta)
  if (nargin < 2) print_usage; endif

  #if (eta==0) res = 2^ell * exp(sf_ln_gamma(ell+1) - sf_ln_gamma(2*ell+2)); return; endif
  if (eta==0) res = 1/prod(1:2:(2*ell+1)); return; endif

  #res = 2^ell * sf_exp(-pi*eta/2) * abs(sf_gamma(ell + 1 + I*eta)) / sf_factorial(2*ell+1)
  #res = 2^ell * sqrt( (2*pi*eta / (sf_exp(2*pi*eta) - 1)) * prod((1:ell).^2 + eta^2)) / sf_factorial(2*ell+1)
  # this avoids spurious blowup for large values (though likely loses a few digits of precision)
  res = sf_exp(ell*sf_log(2) ...
               + 1/2*(log(2*pi*eta / sf_exp_m1(2*pi*eta)) + sum(log((1:ell).^2 + eta^2))) ...
               - sf_ln_factorial(2*ell+1));
endfunction
