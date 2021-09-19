## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hurwitz_zeta (@var{z}, @var{a})
## Compute Hurwitz zeta function $\zeta(z,a)$.
## @end deftypefn
function res = sf_hurwitz_zeta(z, a)
  if (nargin < 2) print_usage; endif
  if (z==1.0)
    res = Inf;
    return
  endif
# TODO: VALIDATE REFLECTION RULE FOR HURWITZ ZETA
#  if (real(z) < 1.0)
#    res = 2 * (2*pi)^(z-1)*sin(pi*z/2)*sf_gamma(1-z)*sf_zeta(1-z);
#    return;
#  endif
  oldold_res = old_res = res = 0.0;
  smm = a^(-z) + (1+a)^(-z);
  n = 2;
  em1 = z/12.0;
  em2 = z*(z+1)*(z+2)/720.0;
  em3 = z*(z+1)*(z+2)*(z+3)*(z+4)/30240.0;
  em4 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)/1209600.0;
  em5 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)/239500800.0;
  # NB kahan accumulation seems to hurt accuracy here...
  do
    smm += (n+a)^(-z);
    oldold_res = old_res;
    old_res = res;
    res = smm + (n+a)^(1-z)/(z-1) - (n+a)^(-z)/2   ...
        + (n+a)^(-z-1)*em1 - (n+a)^(-z-3)*em2 ...
        + (n+a)^(-z-5)*em3 - (n+a)^(-z-7)*em4 ...
        + (n+a)^(-z-9)*em5;
    ++n;
    if (n>999) break; endif
  until ((res == old_res) && (oldold_res == res))
endfunction
