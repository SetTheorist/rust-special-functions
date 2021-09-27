## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_lambert_w (@var{z})
## @deftypefnx {Function File} {@var{res} =} sf_lambert_w (@var{z}, @var{b})
## Compute Lambert's W function $W(z) \exp(W(z)) = z$ with optional specification
## of the branch to return.
## (Currently only supports the two real branches b=0, b=-1; and only supports real z).
## @end deftypefn
function res = sf_lambert_w(z, b)
  if (nargin < 1) print_usage; endif
  if (nargin < 2) b = 0; endif
  if (imag(z)!=0) res = NaN; return; endif
  if (b==0)
    # positive real branch
    if (z<-sf_exp(-1)) res = NaN; return; endif
    if (z==0) res = 0; return; endif
    if (z<0)
      w = -0.1;
    else
      w = sf_log(z/sf_log_p1(z));
    endif
  elseif (b==-1)
    # negative real branch
    if (z<-sf_exp(-1) || z>0) res = NaN; return; endif
    if (z==0) res = 0; return; endif
    if (z==-sf_exp(-1)) res = -1; return; endif
    # Initial approximation
    if (z < -0.183939)
      # series expansion (for z near -1/e)
      p = -sqrt(2*(exp(1)*z + 1));
      w = -1 + p - p^2/3 + p^3*11/72 - 43*p^4/540;
    else
      # asymptotic near 0^-
      L1 = sf_log(-z);
      L2 = sf_log(-sf_log(-z));
      w = L1 - L2 + L2/L1 + ((-2+L2)*L2)/(2*L1^2) + ((6-9*L2+2*L2^2)*L2)/(6*L1^3);
    endif
  else
    res = NaN;
  endif

  # Halley iteration
  n = 0;
  old_w = w+1;
  do
    ew = sf_exp(w);
    old_old_w = old_w;
    old_w = w;
    w -= (w*ew - z) / ((w+1)*ew - (w+2)*(w*ew-z)/(2*w+2));
    ++n;
    if (n>999) break; endif
  until (w == old_w) || (w == old_old_w)
  res = w;
endfunction
