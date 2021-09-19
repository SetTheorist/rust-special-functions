## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_fibonacci_number (@var{n})
## Compute Fibonacci number
## @end deftypefn

function res = sf_fibonacci_number(n)
  if (nargin < 1) print_usage; endif
  res = ones(size(n));
  for k = 1:prod(size(n))
    res(k) = fibonacci_1(n(k));
  endfor
endfunction

function res = fibonacci_1(n)
  persistent cache = [1]
  if (!sf_is_int(n)) res = nan; return; endif
  if (n==0 || n==1) res = n; return; endif
  if (n<0) res = (-1)^(rem(n,2)) * fibonacci_1(-n); return; endif
  if (length(cache) >= n) && (cache(n)!=0) res = cache(n); return; endif
  if (n>1500) res = inf; return; endif
  persistent qq = (1 + sf_sqrt(5))/2;
  persistent zz = 1/sf_sqrt(5);
  res = round(qq^n * zz);
  cache(n) = res;
endfunction
