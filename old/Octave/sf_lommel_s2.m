## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_lommel_s2 (@var{mu}, @var{nu}, @var{z})
## Compute the (asymptotic) second Lommel function $S_(\mu,\nu)(z) ~ \sum ...$
## @end deftypefn
function res = sf_lommel_s2(mu, nu, z)
  if (nargin < 3)
    print_usage;
  endif

  # asymptotic series (breaks down when mu+/-nu is odd integer...)
  res = 1.0;
  k = 1;
  uk = 1.0;
  do
    old_uk = uk;
    uk *= -((mu-2*k+1)^2 - nu^2) / z^2;
    if (abs(uk) > abs(old_uk)) break; endif
    old_res = res;
    res += uk;
    ++k;
    if (k>999) break; endif
  until (res == old_res)
endfunction
