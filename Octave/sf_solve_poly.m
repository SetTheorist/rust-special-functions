## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_solve_poly (@var{a}, [@var{nr}])
## Compute the root(s) of the polynomial $a_1 + a_2*x + ... + a_n*x^(n-1)$.
## returns up to nr roots.
## @end deftypefn
function res = sf_solve_poly(a, nr)
  if (nargin < 1) print_usage; endif
  #if (nargin < 2) nr = 1; endif
  m = length(a)-1; # degree

  #nr = min(nr, m);
  #res = ones(nr);
  #res(1) = laguerre(a, 1);

  res = ones(m,1);
  ad = a;
  for jj = (m-1):(-1):0
    x = 0.0;
    ad_v = ad(1:(jj+2));
    x = laguerre(ad_v, x);
    if (abs(imag(x)) <= 1e-16*abs(real(x)))
      x = real(x);
    endif
    res(jj+1) = x;
    # forward deflation:
    b = ad(jj+2);
    for jjj = jj:(-1):0
      c = ad(jjj+1);
      ad(jjj+1) = b;
      b = x*b + c;
    endfor
  endfor

  # polish the roots
  da = (1:m) .* a(2:end);
  for jj = 1:m
    res(jj) = polish(a, da, res(jj));
  endfor
  res = sort(res);
endfunction

function res = polish(a, da, x)
  # try one Newton step, else Laguerre with full coeffs
  fx = sf_polynomial_value(a, x);
  dfx = sf_polynomial_value(da, x);
  new_x = x - fx/dfx;
  f_newx = sf_polynomial_value(a, new_x);
  if (abs(f_newx) < abs(fx))
    res = new_x;
  else
    res = laguerre(a, x);
  endif
endfunction

# Laguerre's method, adapted from NR
function res = laguerre(a, x)
  persistent MR=8;
  persistent MT=12;
  persistent MAXIT=MR*MT;
  persistent frac = [0.5, 0.25, 0.75, 0.13, 0.38, 0.62, 0.88, 1.0];
  persistent eps = 1e-17;
  m = length(a);
  for it = 1:MAXIT
    b = a(m);
    err = abs(b);
    d = f = 0;
    abx = abs(x);
    for jj = m-1 : (-1) : 1
      f = x*f + d;
      d = x*d + b;
      b = x*b + a(jj);
      err = abs(b) + abx*err;
    endfor
    err *= eps;
    if (abs(b) <= err)
      #on a root
      break;
    endif
    g = d/b;
    g2 = g*g;
    h = g2 - 2*f/b;
    sq = sqrt((m-1)*m*h - g2);
    gp = g + sq;
    gm = g - sq;
    abp = abs(gp);
    abm = abs(gm);
    if (abp < abm) gp = gm; endif
    if (max(abp,abm) > 0)
      dx = m/gp;
    else
      dx = (1+abx)*exp(I*it);
    endif
    x1 = x - dx;
    if (x == x1)
      #converged
      break;
    endif
    if (rem(it, MT)!=0) x = x1;
    else x -= frac(it/MT) * dx;
    endif
  endfor
  res = x;
endfunction

