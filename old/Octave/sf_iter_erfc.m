## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_iter_erfc (@var{z}, @var{n})
## @deftypefnx {Function File} {[@var{res},@var{vecres}] =} sf_iter_erfc (@var{z}, @var{n})
## Compute iterated integral of the complementary error-function, n=-1,0,1,...
## @end deftypefn

function [res,vec] = sf_iter_erfc(z, n)
  if (n<-1 || n!=fix(n)) res = nan; return; endif
  if (n == -1) res = exp(-z^2)*2/sqrt(pi); return; endif
  if (n == 0) res = sf_erfc(z); return; endif
  x = zeros(1,n+2);
  x(-1+2) = exp(-z^2)*2/sqrt(pi);
  x( 0+2) = sf_erfc(z);
  for k=1:n
    x(k+2) = -(z/k)*x(k-1+2) + (1/(2*k))*x(k-2+2);
  endfor
  res = x(n+2);
  if (nargout>1)
    vec = x;
  endif
endfunction
