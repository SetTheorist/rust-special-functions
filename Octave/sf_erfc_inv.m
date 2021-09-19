## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_erfc_inv (@var{z})
## Compute the (real) inverse of the complementary error-function sf_erfc() for $0<z<2$
## @end deftypefn

function res = sf_erfc_inv(z,opt)
  if (nargin < 1)
    print_usage;
  endif
  if (imag(z)!=0 || z<0 || z>2) res = NaN; return; endif
  if (z==0) res = +Inf; return; endif
  if (z==2) res = -Inf; return; endif
  persistent twoqpi = 2/sqrt(pi);
  # quick approximation
  # - could do much better but Halley works good enough to get the job done,
  #   though less efficiently (many iterations)
  t = (1-z) / twoqpi;
  res = t + t^3/3 + t^5*7/30 + t^7*127/630;
  # now halley's
  iters = 0;
  do
    old_res = res;
    f = sf_erfc(res) - z;
    df = -sf_exp(-res^2)*twoqpi;
    res -= f / (df + res*f);
    ++iters;
    if (iters>19) break; endif
  until (res == old_res)
endfunction
