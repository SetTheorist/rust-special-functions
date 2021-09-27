## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_t_coeffs (@var{n})
## Compute the coefficients of the $n$'th Chebyshev polynomial of the first kind: $T_n(z) = a_1 + a_2*x + ... + a_(n+1)*x^n$,
## $n=0, 1, 2, ...$
## @end deftypefn

function res = sf_orthpoly_chebyshev_t_coeffs(n)
  if (nargin < 1) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  switch (n)
  case 0
    res = [1];
  case 1
    res = [0,1];
  otherwise
    persistent cache = {};
    if (n<=length(cache) && !isempty(cache{n}))
      res = cache{n};
      return;
    endif
    rm1 = zeros(1,n+1); rm1(1) = 1;
    rm0 = zeros(1,n+1); rm0(2) = 1;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = 2*shift(rm1,1) - rm2;
    endfor
    res = rm0;
    if (n<1000) cache{n} = res; endif
  endswitch
endfunction
