## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_voight_u (@var{x}, @var{t})
## Compute the Voight function $U(x,t) = ...$
## @end deftypefn

function res = sf_voight_u(x,t)
  z = (1 - x*I) ./ (2*sqrt(t));
  res = real( sqrt(pi./(4*t)) .* sf_exp(z.^2) .* sf_erfc(z) );
endfunction
