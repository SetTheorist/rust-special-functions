## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bernoulli_poly (@var{n}, @var{x})
## Compute value of Bernoulli polynomial at a point $B_n(x)$
## @end deftypefn

function res = sf_bernoulli_poly(n, x)
  if (nargin < 2)
    print_usage;
  endif
  if (n<0 || n!=fix(n)) res = NaN; return; endif

  if (n<=50 || x<0 || x>1 || imag(x)!=0)
    # direct summation for small n or x out of range
    res = direct_sum(n, x)
    res = (-1)^n*direct_sum(n, -x) - n*x^(n-1)
  else # (0<=x && x<=1 && n>50)
    if (fix(n/2)==(n/2))
      res = even_fourier(n,x)
    else
      res = odd_fourier(n,x)
    endif
  endif

endfunction

# direct sum
function res = direct_sum(n, x)
  res = 0.0;
  for k = 0:n
    res += bincoeff(n,k) * sf_bernoulli_number(k) * x^(n-k);
  endfor
endfunction

# valid for 0<=x<=1 and n>=2 even
function res = even_fourier(n, x)
  res = 0.0;
  k = 1;
  do
    old_res = res;
    res += sf_cos(2*pi*k*x) / k^n;
    ++k;
    if (k>999) break; endif;
  until (res == old_res)
  res *= (-1)^(1+n/2) * 2 * sf_factorial(n) * (2*pi)^(-n);
endfunction

# valid for 0<=x<=1 and n>=1 odd
function res = odd_fourier(n, x)
  res = 0.0;
  k = 1;
  do
    old_res = res;
    res += sf_sin(2*pi*k*x) / k^n;
    ++k;
    if (k>999) break; endif;
  until (res == old_res)
  res *= (-1)^(1+(n-1)/2) * 2 * sf_factorial(n) * (2*pi)^(-n);
endfunction
