## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_k (@var{k})
## Compute the complete elliptic integral of the first kind $K(k)$
## @end deftypefn
function res = sf_elliptic_k(k)
  if (nargin < 1) print_usage; endif
  res = zeros(size(k));
  for kk = 1:prod(size(k))
    an = sf_agm(1.0, sqrt(1.0 - k(kk)^2));
    res(kk) = pi / (2*an);
  endfor
endfunction
