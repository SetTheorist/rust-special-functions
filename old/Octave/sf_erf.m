## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_erf (@var{z})
## Compute error-function -- $\erf(z) = \int_0^z ...$
## @end deftypefn

function res = sf_erf(z)
  if (nargin < 1) print_usage; endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    if (abs(z(kk))<1)
      res(kk) = erf_series(z(kk));
    else
      res(kk) = 1 - sf_erfc(z(kk));
    endif
  endfor
endfunction

function res = erf_series(z)
  persistent tqp = 2.0 / sf_sqrt(pi);
  NN = 32 + 48 * floor(abs(z));
  res = sum(sf_exp(-z^2)*tqp*z*cumprod([1, (2*z^2)*ones(1,NN)]./(2*(0:NN)+1)), 'extra');
  #res = sum(sf_exp(-z^2)*tqp*z*cumprod([1, (2*z^2)*ones(1,NN)]./(2*(0:NN)+1)));
endfunction
