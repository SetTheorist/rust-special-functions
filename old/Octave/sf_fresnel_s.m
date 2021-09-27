## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_fresnel_s (@var{z})
## Compute the Fresnel function $S(z) = \int_0^z sin(t^2 pi/2) dt$
## @end deftypefn

function res = sf_fresnel_s(z)
  if (nargin < 1)
    print_usage;
  endif
  if (abs(z)<1)
    # series for small z -- loses precision badly as z grows
    # (useful really only for |z|<1...)
    term = z^3 * (pi/2);
    res = term / 3;
    mlt = - (pi/2)^2 * z^4;
    n = 1;
    do
      term *= mlt / ((2*n)*(2*n+1));
      old_res = res;
      res += term / (4*n+3);
      ++n;
      if (n>999) break; endif
    until (res == old_res)
  elseif (abs(z)<10)
    # series in spherical bessel functions
    # works better for larger z
    res = 0.0;
    n = 0;
    x = z^2 * pi/2;
    do
      old_res = res;
      res += sf_bessel_spher_j(2*n+1, x);
      ++n;
      if (n>999) break; endif
    until (res == old_res)
    # cleanup
    if (real(z)==0) res = 0 + imag(res)*I; endif
  else
    # asymptotic expansion - works for large z
    # (need to verify for complex values...)
    res = sign(z)/2 - (asymp_f(z)*cos(z^2*pi/2) + asymp_g(z)*sin(z^2*pi/2))/(pi*z);
  endif
endfunction
function res = asymp_f(z)
  z2 = (pi*z^2)^2;
  res = 1.0;
  n = 1;
  term = 1.0;
  do
    old_term = term;
    term *= -(4*n-1)*(4*n-3)/z2;
    old_res = res;
    res += term;
    ++n;
    if (n>99) break; endif
  until (abs(term)>abs(old_term) || res==old_res)
endfunction
function res = asymp_g(z)
  z2 = (pi*z^2)^2;
  res = 1/z2;
  n = 1;
  term = 1.0/(pi*z^2);
  do
    old_term = term;
    term *= -(4*n+1)*(4*n-1)/z2;
    old_res = res;
    res += term;
    ++n;
    if (n>99) break; endif
  until (abs(term)>abs(old_term) || res==old_res)
endfunction
