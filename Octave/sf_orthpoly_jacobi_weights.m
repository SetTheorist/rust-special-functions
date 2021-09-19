## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_jacobi_weights (@var{n}, @var{a}, @var{b})
## Compute the Gaussian quadrature weights of the $n$'th Jacobi polynomial: $P^(a,b)_n(z)$
## $n=0, 1, 2, ...$, $a>-1$, $b>-1$
## @end deftypefn

function res = sf_orthpoly_jacobi_weights(n, a, b)
  if (nargin < 3) print_usage; endif
  if (!sf_is_nonnegint(n) || a<=-1 || b<=-1) print_usage; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [1];
  else
    zs = sf_orthpoly_jacobi_zeros(n, a, b);
    # need ortho_normal_ polynomials here
    nrm = sf_orthpoly_jacobi_scale(0:(n-1), a, b);
    res = zeros(n, 1);
    for jj = 0:(n-1)
      res += (sf_orthpoly_jacobi_value(jj, a, b, zs)*nrm(jj+1)).^2;
    endfor
    res = 1./res;
  endif
endfunction
