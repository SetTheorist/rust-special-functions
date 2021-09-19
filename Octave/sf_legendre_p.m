## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_legendre_p (@var{n}, @var{x})
## @deftypefnx {Function File} {@var{res} =} sf_legendre_p (@var{nu}, @var{x}); |x|<1
## @deftypefnx {Function File} {@var{res} =} sf_legendre_p (@var{m}, @var{n}, @var{x})
## @deftypefnx {Function File} {@var{res} =} sf_legendre_p (@var{m}, @var{nu}, @var{x}); |x|<1
## Evaluate (associated) Legendre functions (polynomials) of the first kind
## of order m (integer), degree nu (real), for real x (|x|<1 for nu non-integer), $P^m_\nu(x)$.
## TODO: extend to non-integer
## TODO: extend to complex
## @end deftypefn
function res = sf_legendre_p(m, nu, x)
  if (nargin < 2) print_usage; endif
  if (nargin < 3) x = nu; nu = m; m = 0; endif
  if (!sf_is_int(m) || !isreal(x) || (!sf_is_int(nu) && abs(x)>1)) print_usage; endif

  if (!sf_is_int(nu))
    res = series_2(m,nu,x);
    return;
  endif

  # symmetry to ensure degree nu>=0 
  if (nu<0)
    nu = -nu-1;
  endif
  # symmetry to ensure order m>=0
  mult = ones(size(x));
  if (m<0)
    m = -m;
    mult *= (-1)^(rem(m,2)) * sf_factorial(nu-m) / sf_factorial(nu+m);
    #mult *= (-1)^(rem(m,2)) * sf_gamma(nu-m + 1) / sf_gamma(nu+m + 1);
  endif
  # symmetry to ensure argument x>=0
  negx = (x<0);
  x(negx) = -x(negx);
  mult(negx) *= (-1)^(rem(nu-m,2));

  res = zeros(size(x));
  for kk = 1:prod(size(x))
    res(kk) = recur_n(m, nu, x(kk));
  endfor

  res .*= mult;
endfunction

function res = recur_n(m, n, x)
  pmm2 = 0.0;
  if (n<m) res = 0; return; endif
  if (abs(x)<=1)
    pmm1 = prod(1:2:(2*m-1))*(1-x^2)^(m/2);
  else
    pmm1 = prod(1:2:(2*m-1))*(x^2-1)^(m/2) * (-1)^(rem(m,2)-1);
  endif
  if (n==m) res = pmm1; return; endif
  for k = (m+1):n
    pmm = ((2*k-1)*x*pmm1 - (k+m-1)*pmm2) / (k-m);
    pmm2 = pmm1;
    pmm1 = pmm;
  endfor
  res = pmm;
endfunction

# for integer m, arbitrary nu, |1-x|<2
function res = series_2(m, nu, x)
  x2 = (1-x)/2;
  res = 0.0;
  k = 0;
  do
    old_res = res;
    res += sf_pochhammer(-nu+m, k) * sf_pochhammer(nu+1+m, k) / sf_factorial(k) / sf_pochhammer(m+1, k) * x2^k;
    ++k;
    if (k>999) break; endif
  until (res == old_res);
  res *= (-1)^(rem(m,2)) / (2^m*sf_factorial(m)) * (1-x^2)^(m/2) * sf_gamma(nu+m+1) / sf_gamma(nu-m+1);
endfunction

# for arbitrary m, n?
function res = series(m, n, x)
  persistent sqp = sf_sqrt(1/pi);
  th = sf_acos(x);
  res = 0.0;
  res2 = 0.0;
  k = 0;
  do
    old_res = res;
    sf_gamma(n+m+k+1)/sf_gamma(n+k+3/2) * sf_pochhammer(m+1/2, k)/sf_factorial(k)
    sf_pochhammer(m+1/2, k)*sf_pochhammer(n+m+1, k) / sf_factorial(k) / sf_pochhammer(n+3/2, k)
    res += sf_gamma(n+m+k+1)/sf_gamma(n+k+3/2) * sf_pochhammer(m+1/2, k)/sf_factorial(k) * sf_sin(th*(n+m+2*k+1));
    ++k;
    if (k>999) break; endif
  until (old_res == res);
  k
  res *= 2^(m+1)*(sf_sin(th))^m * sqpi;
endfunction
