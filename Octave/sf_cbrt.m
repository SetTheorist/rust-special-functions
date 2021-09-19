## -*- texinfo -*-
## @deftypefn {Function File} {[@var{r1},@var{r2},@var{r3}] =} sf_cbrt (@var{z})
## Compute the cube-root(s) of $z$
## @end deftypefn
function [r1,r2,r3] = sf_cbrt(z)
  if (nargin < 1)
    print_usage;
  endif
  if (nargout > 2)
    r1 = ones(size(z));
    r2 = ones(size(z));
    r3 = ones(size(z));
    for kk = 1:prod(size(z))
      [r1(kk),r2(kk),r3(kk)] = sf_cbrt_1(z(kk));
    endfor
  elseif (nargout > 1)
    r1 = ones(size(z));
    r2 = ones(size(z));
    for kk = 1:prod(size(z))
      [r1(kk),r2(kk)] = sf_cbrt_1(z(kk));
    endfor
  else
    r1 = ones(size(z));
    for kk = 1:prod(size(z))
      [r1(kk)] = sf_cbrt_1(z(kk));
    endfor
  endif
endfunction

function [r1,r2,r3] = sf_cbrt_1(z)
  persistent xi1 = -0.5 + sf_sqrt(3)/2*I;
  persistent xi2 = -0.5 - sf_sqrt(3)/2*I;
  if (z==0.0)
    r1 = 0.0;
  elseif (isreal(z))
    # real z
    r1 = real_newton(z);
  else
    # fully complex z
    # copout approach...
    r = abs(z);
    th = arg(z);
    r1 = real_newton(r) * sf_exp(I*th/3);
  endif
  r2 = r1*xi1;
  r3 = r1*xi2;
  # newton cleanup steps
  r1 = 2*r1/3 + z/(3*r1^2);
  r1 = 2*r1/3 + z/(3*r1^2);
  r2 = 2*r2/3 + z/(3*r2^2);
  r2 = 2*r2/3 + z/(3*r2^2);
  r3 = 2*r3/3 + z/(3*r3^2);
  r3 = 2*r3/3 + z/(3*r3^2);
endfunction

# newton method
function res = real_newton(x)
  sgn = sign(x); x = abs(x);
  res = (0.5 + sf_exp(-pi/x)) / (0.5 + sf_exp(-pi*x));
  n = 0;
  old_res = 0;
  do
    old_old_res = old_res;
    old_res = res;
    res = 2*res/3 + x/(3*res^2);
    ++n;
    if (n>999) break; endif
  until (res == old_res) || (res == old_old_res);
  res *= sgn;
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
