## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_chebyshev_u_value (@var{n}, @var{z})
## Compute the value of the $n$'th Chebyshev polynomial of the second kind at $z$: $U_n(z)$,
## $n=0, 1, 2, ...$, typically $z\in[-1,1]$
## @end deftypefn

function res = sf_orthpoly_chebyshev_u_value(n, z)
  if (nargin < 2) print_usage; endif
  if (!sf_is_nonnegint(n)) print_usage; endif
  switch (n)
  case 0
    res = ones(size(z));
  case 1
    res = 2*z;
  otherwise
    rm1 = ones(size(z));
    rm0 = 2*z;
    for k=2:n
      rm2 = rm1;
      rm1 = rm0;
      rm0 = 2*z.*rm1 - rm2;
    endfor
    res = rm0;
  endswitch
endfunction
