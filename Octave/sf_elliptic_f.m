## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_f (@var{phi}, @var{k})
## Compute the (incomplete) elliptic integral of the first kind
##
## $ F(phi, k) = \int_0^\phi d\theta / sqrt(1-k^2*\sin^2(theta)) $
##
## @end deftypefn
function res = sf_elliptic_f(phi, k)
  if (nargin < 2) print_usage; endif
  if (any(size(phi)!=size(k)))
    if (isscalar(phi)) phi*=ones(size(k));
    elseif (isscalar(k)) k*=ones(size(phi));
    else error("sf_elliptic_f: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(k));
  for kk = 1:prod(size(k))
    res(kk) = sf_elliptic_f__1(phi(kk), k(kk));
  endfor
endfunction

function res = sf_elliptic_f__1(phi, k)
  #qua = quad(@(t)(1/sqrt(1-k^2*sin(t)^2)), 0, phi)
  if (k==1)
    res = sf_log((1 + sf_sin(phi)) / (1 - sf_sin(phi))) / 2;
  elseif (k==0)
    res = phi;
  else
    if (phi==0)
      res = 0;
    else
      res = ascending_landen(phi, k);
    endif
  endif
endfunction

function res = agm_method(phi, k)
  [an,bn,cn,phin] = sf_agm(1.0, sqrt(1 - k^2), phi, k);
  res = phin(end) / (2^(length(phin)-1) * an(end));
endfunction

function res = ascending_landen(phi, k)
  res = 1;
  n = 0;
  do
    k2 = 2 * sf_sqrt(k) / (1 + k);
    phi2 = (phi + asin(k*sin(phi)))/2;
    res *= 2 / (1 + k);
    k = k2;
    phi = phi2;
    ++n; if (n>999) break; endif
  until (k==1)
  res *= sf_log((1 + sf_sin(phi)) / (1 - sf_sin(phi))) / 2;
endfunction
