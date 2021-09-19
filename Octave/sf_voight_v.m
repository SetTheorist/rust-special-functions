## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_voight_v (@var{x}, @var{t})
## Compute the Voight function $V(x,t) = ...$
## @end deftypefn

function res = sf_voight_v(x,t)
  z = (1 - x*I) ./ (2*sqrt(t));
  res = imag( sqrt(pi./(4*t)) .* sf_exp(z.^2) .* sf_erfc(z) );
endfunction
