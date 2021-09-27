## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_debye_scaled (@var{n}, @var{z})
## Compute scaled Debye function $\O{D}_n(z) \frac{n}{z^n}$
## @end deftypefn

function res = sf_debye_scaled(n,z)
  if (nargin < 2) print_usage; endif
  if (any(!sf_is_posint(n))) error("sf_debye: n must be positive integer"); endif
  if (any(size(z) != size(n)))
    if (isscalar(z)) z *= ones(size(n));
    elseif (isscalar(n)) n *= ones(size(z));
    else error("sf_debye_scaled: mismatched parameter sizes");
    endif
  endif
  res = ones(size(z));
  for k = 1:prod(size(z))
    if (real(z(k))<0)
      zz = -z(k);
      res(k) = zz*n(k)/(n(k)+1);
    else
      zz = z(k);
      res(k) = 0;
    endif
    if (abs(zz) < 2)
      res(k) += sf_debye_1(n(k),zz);
    else
      res(k) += (sf_factorial(n(k))*sf_zeta(n(k)+1)*n(k)/zz^(n(k)) - coint(n(k), zz));
    endif
  endfor
endfunction

function res = sf_debye_1(n,z)
  # cache these numbers for big speed-up (avoiding function calls)
  persistent NBNS = 299;
  persistent bns = sf_bernoulli_number_scaled(2*[1:NBNS]);
  res = 1.0/n - z/(2*(n+1));
  z2 = z*z;
  zpow = 1.0;
  k = 1;
  do
    zpow *= z2;
    old_res = res;
    #res += sf_bernoulli_number_scaled(2*k) * zpow / (2*k + n);
    res += bns(k) * zpow / (2*k + n);
    ++k; if (k>NBNS) break; endif
  until (res == old_res);
  if (k>NBNS)
    warning('sf:convergence', "sf_debye_scaled(%g,%g) series failed to converge after %g iterations", n, z, k);
  endif
  res *= n;
endfunction

# \int_x^\infty
function res = coint(n, z)
  res = 0;
  k = 1;
  do
    old_res = res;
    res += sf_exp(-k*z)*coterm(n, z, k);
    ++k; if (k>999) break; endif
  until (res == old_res)
endfunction
function res = coterm(n, z, k)
  res = term = n/k;
  for j = (n-1):(-1):0
    term *= (j+1)/(k*z);
    res += term;
  endfor
endfunction
