## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_el3 (@var{x}, @var{kc}, @var{p})
## Compute the Burlisch's elliptic integral $el_3(x,k_c,p)$
## @end deftypefn
function res = sf_elliptic_el3(x, kc, p)
  if (nargin != 3) print_usage; endif
  #res = sf_elliptic_pi(atan(x), 1-p, sf_sqrt(1-kc.^2));
  r = 1/x^2;
  res = sf_elliptic_el1(x, kc) + (1-p)/3 * sf_elliptic_rj(r, r+kc^2, r+1, r+p);
endfunction
