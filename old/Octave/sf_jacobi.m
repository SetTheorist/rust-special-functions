## -*- texinfo -*-
## @deftypefn {Function File} {[@var{sn},@var{cn},@var{dn}] =} sf_jacobi (@var{x}, @var{k})
## Compute Jacobi elliptic functions sn, cn, dn
## @end deftypefn
function [sn,cn,dn] = sf_jacobi(x, k)
  if (nargin < 2) print_usage; endif
  [am,bm,cm] = sf_agm(1, sqrt(1-k^2), k);
  N = length(am);
  phi_0 = 2^(N-1) * am(N)*x;
  for m = (N-1):(-1):1
    phi_1 = phi_0;
    phi_0 = (phi_1 + sf_asin(sf_sin(phi_1) * cm(m+1)/am(m+1)))/2;
  endfor
  sn = sf_sin(phi_0);
  cn = sf_cos(phi_0);
  #dn = cn / sf_cos(phi_1 - phi_0);
  dn = sqrt(1.0 - k^2 * sn^2);
endfunction
