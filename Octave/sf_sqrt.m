## -*- texinfo -*-
## @deftypefn {Function File} {[@var{r1},@var{r2}] =} sf_sqrt (@var{z})
## Compute the square-root(s) of $z$
## @end deftypefn
function [r1,r2] = sf_sqrt(z)
  if (nargin < 1) print_usage; endif
  r1 = ones(size(z));
  for kk = 1:prod(size(z))
    r1(kk) = sf_sqrt_1(z(kk));
  endfor
  if (nargout > 1) r2 = -r1; endif
endfunction

function res = sf_sqrt_1(z)
  if (z==0.0)
    res = 0.0;
  elseif (imag(z)==0)
    # real z
    res = real_newton(abs(z));
    if (z<0) res *= I; endif
  else
    # fully complex z
    x = real(z); y = imag(z);
    r = real_newton(x^2 + y^2);
    res = real_newton((r + x)/2) + I * sign(y)*real_newton((r - x)/2);
  endif
endfunction

function res = real_newton(x)
  res = (0.5 + sf_exp(-pi/x)) / (0.5 + sf_exp(-pi*x));
  n = 0;
  old_res = 0;
  do
    old_old_res = old_res;
    old_res = res;
    res = (res + x/res)/2;
    ++n;
    if (n>999) break; endif
  until (res == old_res) || (res == old_old_res);
  # cleanup with one high-precision step?
  # increases time by 150%!!
  # (0.9ms --> 2.6ms on one machine)
  # but achieves full accuracy
  qres = sf_qdouble(res);
  res = (qres + x/qres).hi/2;
endfunction

# interesting approach, but gets worse
# for smaller or larger x...
# seems to diverge for pure imaginary
function res = exp_series(x)
  nm = 0.5;
  j = 1;
  do
    old_nm = nm;
    nm += sf_exp(-j^2*pi/x);
    ++j;
    if (j>999) break; endif
  until (nm == old_nm);

  dn = 0.5;
  j = 1;
  do
    old_dn = dn;
    dn += sf_exp(-j^2*pi*x);
    ++j;
    if (j>999) break; endif
  until (dn == old_dn);

  res = nm/dn;
endfunction

%!test assert(sf_sqrt(-10:10).^2, -10:10, -5e-15)
%!test [x1,x2] = sf_sqrt(13);
%!     assert(x1^2, 13, -5e-15);
%!     assert(x2^2, 13, -5e-15);
