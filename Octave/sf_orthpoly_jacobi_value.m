## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_jacobi_value (@var{n}, @var{a}, @var{b}, @var{z})
## Compute the value of the $n$'th Jacobi polynomial: $J^(a,b)_n(z)$,
## $a>-1$, $b>-1$, $n=0, 1, 2, ...$, typically $z\in(-1,1)$
## @end deftypefn

function res = sf_orthpoly_jacobi_value(n, a, b, z)
  if (nargin < 4) print_usage; endif
  if (!sf_is_nonnegint(n) || a<=-1 || b<=-1) print_usage; endif
  switch (n)
  case 0
    res = ones(size(z));
  case 1
    res = (a-b)/2 + (2+a+b)/2 * z;
  otherwise
    rm1 = ones(size(z));
    rm0 = (a-b)/2 + (2+a+b)/2 * z;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      ax = (2*k+a+b-1)*(a^2-b^2);
      bx = (2*k+a+b-2)*(2*k+a+b-1)*(2*k+a+b);
      cx = 2*(k+a-1)*(k+b-1)*(2*k+a+b);
      dx = 2*k*(k+a+b)*(2*k+a+b-2);
      rm0 = ((ax+bx*z).*rm1 - cx*rm2)/dx;
    endfor
    res = rm0;
  endswitch
endfunction
