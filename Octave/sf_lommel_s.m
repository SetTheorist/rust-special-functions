## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_lommel_s (@var{mu}, @var{nu}, @var{z})
## Compute the Lommel function $s_(\mu,\nu)(z)$
## @end deftypefn
function res = sf_lommel_s(mu, nu, z)
  if (nargin < 3)
    print_usage;
  endif

  # series (breaks down when mu+/-nu is odd integer...)
  res = 1.0;
  k = 1;
  tk = 1.0;
  do
    tk *= -z^2 / ((mu+2*k+1)^2 - nu^2);
    old_res = res;
    res += tk;
    ++k;
    if (k>999) break; endif
  until (res == old_res)
  res *= z^(mu+1) / ((mu+1)^2 - nu^2);
endfunction
