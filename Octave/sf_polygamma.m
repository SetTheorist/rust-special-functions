## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_gamma (@var{n}, @var{z})
## Compute the polygamma function $\psi^(n)(z) = d^n/dz^n \psi(z)$
## @end deftypefn

function res = sf_polygamma(n, z)
  if (nargin < 2) print_usage; endif
  if (any(size(n)!=size(z)))
    if (isscalar(n))
      n *= ones(size(z));
    elseif (isscalar(z))
      z *= ones(size(n));
    else
      error("sf_polygamma: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    if (!sf_is_nonnegint(n(kk)))
      res(kk) = NaN;
    elseif (n==0)
      res(kk) = sf_digamma(z(kk));
    else
      res(kk) = series(n(kk), z(kk));
    endif
  endfor
endfunction

function res = sf_polygamma_1(n, z)
  #res = series2(n, z);
endfunction

# series with Euler-Maclaurin correction terms
# valid for z != 0, -1, -2, ...
function res = series(n, z)
  persistent bns = sf_bernoulli_number_scaled([2,4,6]);
  res = sm = 0.0;
  k = 0;
  do
    sm += (z+k)^(-n-1);
    old_res = res;
    res = sm ...
      +(z+k)^(-n)/n -(z+k)^(-n-1)/2 ...
      -bns(1)*(z+k)^(-n-2)*(-n-1) ...
      -bns(2)*(z+k)^(-n-4)*(-n-1)*(-n-2)*(-n-3) ...
      -bns(3)*(z+k)^(-n-6)*(-n-1)*(-n-2)*(-n-3)*(-n-4)*(-n-5) ;
    ++k;
    if (k>999) break; endif
  until (res == old_res)
  #res *= (-1)^(n+1) * sf_factorial(n);
  res *= (-1)^(n+1) * prod(1:n);
endfunction

