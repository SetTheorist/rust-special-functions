## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_exsec (@var{z})
## Compute the exsecant function
## @end deftypefn

function res = sf_exsec(z)
  res = 2 * sf_sin(z / 2).^2 .* sf_sec(z);
endfunction
