## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_orthpoly_jacobi_zeros (@var{n}, @var{a}, @var{b})
## Compute the zeros of the $n$'th Jacobi polynomial: $P^(a,b)_n(z)$
## $n=0, 1, 2, ...$, $a>-1$, $b>-1$
## @end deftypefn

function res = sf_orthpoly_jacobi_zeros(n, a, b)
  if (nargin < 3) print_usage; endif
  if (!sf_is_nonnegint(n) || a<=-1 || b<=-1) print_usage; endif
  if (n==0)
    res = [];
  elseif (n==1)
    res = [(b-a)/(2+a+b)];
  else
    m = zeros(n);
    for k=1:n-1
      m(k+1,k+1) = (b^2-a^2) / ((2*k+a+b)*(2*k+a+b+2));
      m(k,k+1) = m(k+1,k) = 2/(2*k+a+b) * sf_sqrt((k*(k+a)*(k+b)*(k+a+b)) / ((2*k+a+b+1)*(2*k+a+b-1)));
    endfor
    m(1,1) = (b-a)/(2+a+b);
    res = sort(eig(m));

    # "polish" the results
    fx = sf_orthpoly_jacobi_value(n,a,b,res);
    dfx = (n+a+b+1)/2 * sf_orthpoly_jacobi_value(n-1, a+1, b+1, res);
    nwt = res - fx./dfx;
    #nrs = sf_orthpoly_jacobi_value(n,a,b,nwt)
    res = nwt;
  endif
endfunction
