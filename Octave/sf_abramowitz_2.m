## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_abramowitz_2 (@var{z})
## Compute integral $f(x)=\int_0^\infty e^(-t^2)/(t+x) dx$ from Abramowitz & Stegun
## @end deftypefn
function res = sf_abramowitz_2(z,opt)
  if (nargin < 1)
    print_usage;
  endif
  if (nargin < 2) opt = -1; endif
  # not clear what appropriate value for real negative z should be ...
  # (integral as such diverges)
  if (((opt<0)&&((imag(z)!=0)||(0<z)&&(z<1))) || (opt==0))
    # use for small positive z or complex z
    # unfortunately, blows up for large |z| ...
    res = sqrt(pi)*z;
    k = 1;
    do
      old_res = res;
      res += sqrt(pi)*(z^(2*k+1)/(factorial(k)*(2*k+1))) - z^(2*k)/(factorial(k)*(2*k));
      ++k;
      if (k>999) break; endif
    until (res == old_res)
    k
    res = sf_exp(-z^2)*(res - 0.57721566490153286061/2) - sf_exp(-z^2)*sf_log(z);
  elseif (((opt<0)&&(z<10)) || (opt==1))
    res = (-1/2)*sf_exp(-z^2)*sf_expint_ei(z^2) + sqrt(pi)*sf_dawson(z);
  else
    res = quad(@(t)(exp(-t^2)/(t+z)), 0, Inf);
  endif
endfunction
