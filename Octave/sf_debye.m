## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_debye (@var{n}, @var{z})
## Compute Debye function $\O{D}_n(z)$
## @end deftypefn

function res = sf_debye(n,z)
  if (nargin < 2) print_usage; endif
  if (any(!sf_is_posint(n))) error("sf_debye: n must be positive integer"); endif
  if (any(size(z) != size(n)))
    if (isscalar(z)) z *= ones(size(n));
    elseif (isscalar(n)) n *= ones(size(z));
    else error("sf_debye: mismatched parameter sizes");
    endif
  endif
  res = ones(size(z));
  for k = 1:prod(size(z))
    #qua = quad(@(t)(t^n(k)/(exp(t)-1)), 0, z(k), 1e-18)
    if (real(z(k))<0)
      zz = -z(k);
      res(k) = (-1)^(rem(n(k),2))*zz^(n(k)+1)/(n(k)+1);
      mlt = -1;
    else
      zz = z(k);
      mlt = 1;
      res(k) = 0;
    endif
    if (abs(zz) < 2)
      res(k) += mlt*sf_debye_1(n(k), zz);
    else
      if (abs(zz)<n(k))
        res(k) += mlt*co_2(n(k), zz);
      else
        res(k) += mlt*(sf_factorial(n(k))*sf_zeta(n(k)+1) - coint(n(k), zz));
      endif
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
    warning('sf:convergence', "sf_debye(%g,%g) series failed to converge after %g iterations", n, z, k);
  endif
  res *= z^n;
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
  res = term = z^n/k;
  for j = (n-1):(-1):0
    term *= (j+1)/(k*z);
    res += term;
  endfor
endfunction

# alternative formulation of co-rules
function res = co_2(n, z)
  zet = sf_zeta_m1(n+1);
  eee = sf_exp(-z)*sf_exp_men(n+1, z);
  res = zet + eee;
  k = 2;
  do
    old_res = res;
    res -= sf_exp(-k*z)/k^(n+1)*sf_expn(n, z*k);
    ++k; if (k>999) break; endif
  until (res == old_res)
  res *= sf_factorial(n);
endfunction
