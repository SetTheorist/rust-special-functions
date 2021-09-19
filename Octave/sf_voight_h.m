## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_voight_h (@var{a}, @var{u})
## Compute the "line-broadening function" $H(a,u) = ...$
## @end deftypefn

function res = sf_voight_h(x,t)
  res = sf_voight_u(u./a, 1./(4*a.^2)) ./ (a*sqrt(pi));
endfunction
