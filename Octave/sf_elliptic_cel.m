## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_cel (@var{kc}, @var{p}, @var{a}, @var{b})
## Compute the Burlisch's elliptic integral $cel(k_c,p,a,b)$
## @end deftypefn
function res = sf_elliptic_cel(kc, p, a, b)
  if (nargin != 4) print_usage; endif
  res = a * sf_elliptic_rf(0, kc^2, 1) + (b-p*a)/3 * sf_elliptic_rj(0, kc^2, 1, p);
endfunction
