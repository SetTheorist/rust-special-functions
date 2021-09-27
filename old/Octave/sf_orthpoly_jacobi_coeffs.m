## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_jacobi_coeffs (@var{n}, @var{a}, @var{b})
## Compute the coefficients of the $n$'th Jacobi polynomial:
## $J^(a,b)_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## $a>-1$, $b>-1$, $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_jacobi_coeffs(n, a, b)
  if (nargin < 3) print_usage; endif
  if (!sf_is_nonnegint(n) || a<-1 || b<-1) print_usage; endif
  switch (n)
  case 0
    res = [1];
  case 1
    res = [(a-b)/2,(2+a+b)/2];
  otherwise
    persistent cache = {};
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
    endif
    rm1 = zeros(1,n+1); rm1(1) = 1;
    rm0 = zeros(1,n+1); rm0(1) = (a-b)/2; rm0(2) = (2+a+b)/2;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      ax = (2*k+a+b-1)*(a^2-b^2);
      bx = (2*k+a+b-2)*(2*k+a+b-1)*(2*k+a+b);
      cx = 2*(k+a-1)*(k+b-1)*(2*k+a+b);
      dx = 2*k*(k+a+b)*(2*k+a+b-2);
      rm0 = (ax*rm1 + bx*shift(rm1,1) - cx*rm2) / dx;
    endfor
    res = rm0;
    if (n<1000) cache{n} = res; endif
  endswitch
endfunction
