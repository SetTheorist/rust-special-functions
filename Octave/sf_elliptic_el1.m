## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_el1 (@var{x}, @var{kc})
## Compute the Burlisch's elliptic integral $el_1(x,k_c)$
## @end deftypefn
function res = sf_elliptic_el1(x, kc)
  if (nargin != 2) print_usage; endif
  #res = sf_elliptic_f(atan(x), sf_sqrt(1-kc.^2));
  r = 1/x^2;
  res = sf_elliptic_rf(r, r+kc^2, r+1);
endfunction
