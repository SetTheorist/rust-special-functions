## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_incomplete_beta (@var{x}, @var{a}, @var{b})
## Compute incomplete Beta function
##
## $B_x(a,b) = \int_0^x t^(a-1) (1-t)^(b-1) dt$
##
## requires a>0, b>0, 0<=x<=1
## @end deftypefn

function res = sf_incomplete_beta(x, a, b)
  if (nargin < 3) print_usage; endif
  if (x<0 || x>1 || imag(x)!=0 || sf_is_nonposreal(a) || (x==1 && sf_is_nonposreal(b)))
    error('sf:domain', "domain error for sf_incomplete_beta(%g,%g,%g)", x, a, b);
  endif
  if (x==0) res = 0; return; endif
  if (x==1) res = sf_beta(a,b); return; endif
  # quadrature
  #qua = quad(@(t)(t^(a-1)*(1-t)^(b-1)), 0, x)
  # continued fraction
  res = contfrac(x, a, b);
endfunction

function res = contfrac(x, a, b)
  if (a>1 && b>1 && x>(a-1)/(a+b-2))
    res = sf_beta(a, b) - contfrac(1-x, b, a);
    return;
  endif
  zeta = 1e-150;
  eps = 1e-16;
  fj = 1.0;
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    if (rem(j,2)==0)
      n = (j-2)/2;
      aj = x * (n+1)*(b-n-1)/(a+2*n+1)/(a+2*n+2);
    else
      n = (j-1)/2;
      aj = -x * (a+n)*(a+b+n)/(a+2*n)/(a+2*n+1);
    endif
    Dj = 1.0 + aj*Dj;  if (Dj==0) Dj=zeta; endif
    Cj = 1.0 + aj/Cj;  if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j; if (j>999) break; endif
  until (abs(Deltaj - 1)<eps)
  if (j>999) warning('sf:convergence', "sf_incomplete_beta(%g,%g,%g) failed to converge", x, a, b); endif
  res = 1/fj * x^a*(1-x)^b/a;
endfunction
