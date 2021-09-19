## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_erf_inv (@var{z})
## Compute the (real) inverse of the error-function sf_erf() for $-1<z<1$
## @end deftypefn

function res = sf_erf_inv(z,opt)
  if (nargin < 1)
    print_usage;
  endif
  if (imag(z)!=0 || z<-1 || z>1) res = NaN; return; endif
  if (z==-1) res = -Inf; return; endif
  if (z==+1) res = +Inf; return; endif
  persistent twoqpi = 2/sqrt(pi);
  # quick approximation
  # - could do much better but Halley works good enough to get the job done,
  #   though less efficiently (many iterations)
  t = z / twoqpi;
  res = t + t^3/3 + t^5*7/30 + t^7*127/630;
  # now halley's
  iters = 0;
  do
    old_res = res;
    f = sf_erf(res) - z;
    df = sf_exp(-res^2)*twoqpi;
    res -= f / (df + res*f);
    ++iters;
    if (iters>19) break; endif
  until (res == old_res)
endfunction
