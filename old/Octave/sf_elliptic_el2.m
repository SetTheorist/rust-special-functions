## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_el2 (@var{x}, @var{kc}, @var{a}, @var{b})
## Compute the Burlisch's elliptic integral $el_2(x,k_c,a,b)$
## @end deftypefn
function res = sf_elliptic_el2(x, kc, a, b)
  if (nargin != 4) print_usage; endif
  r = 1/x^2;
  res = a * sf_elliptic_el1(x, kc) + (b-a)/3 * sf_elliptic_rd(r, r+kc^2, r+1);
endfunction
