## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_chvcos (@var{z})
## Compute the cohavercosine/hacovercosine function
## @end deftypefn

function res = sf_chvcos(z)
  res = sf_cvcos(z) / 2;
endfunction
