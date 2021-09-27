## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_e ([@var{phi},] @var{k})
## Compute the (in)complete elliptic integral of the second kind $E(k)$ or $E(phi, k)$
## @end deftypefn
function res = sf_elliptic_e(phi, k)
  if (nargin < 1) print_usage; endif
  if (nargin < 2)
    k = phi;
    res = zeros(size(k));
    for kk = 1:prod(size(k))
      # complete case
      [an,bn,cn] = sf_agm(1.0, sqrt(1.0 - k^2), k(kk));
      cn(1) = k(kk);
      res(kk) = 2.0;
      for n = 1:length(cn)
        res(kk) -= 2^(n-1)*cn(n)^2;
      endfor
      res(kk) *= pi/(4*an(end));
    endfor
  else
    if (any(size(phi)!=size(k)))
      if (isscalar(phi)) phi*=ones(size(k));
      elseif (isscalar(k)) k*=ones(size(phi));
      else error("sf_elliptic_e: mismatched parameter sizes");
      endif
    endif
    res = zeros(size(k));
    for kk = 1:prod(size(k))
      #qua = quad(@(t)(sqrt(1 - k(kk)^2*sin(t)^2)), 0, phi(kk))
      # incomplete case
      if (k(kk) == 1.0)
        res(kk) = sf_sin(phi(kk));
      elseif (k(kk) == 0.0)
        res(kk) = phi(kk);
      else
        res(kk) = ascending_landen(phi(kk), k(kk));
      endif
    endfor
  endif
endfunction

function res = ascending_landen(phi, k)
  if (k == 1)
    res = sin(phi);
  else
    k2 = 2 * sqrt(k) / (1 + k);
    phi2 = (phi + asin(k*sin(phi))) / 2;
    res = (1 + k)*ascending_landen(phi2, k2) + (1 - k)*sf_elliptic_f(phi2, k2) - k*sin(phi);
  endif
endfunction

# BUGGY
function res = agm(k, phi)
  [an,bn,cn,phin] = sf_agm(1.0, sqrt(1.0 - k^2),  k, phi);
  N = length(phin) - 1;
  a = an(end); ph = phin(end);
  res = ph / (2^N * a) ...
      + sum(cn(2:end) .* sf_sin(phin(2:end))) ...
      - ((ph)/(2^(N+1)*a)) * sum(2.^(0:N) .* cn.^2);
680
endfunction
