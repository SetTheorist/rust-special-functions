## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_i (@var{nu}, @var{z})
## Compute Bessel I_nu(z) function -- just a wrapper of built-in besseli()
## @end deftypefn

function res = sf_bessel_i(nu,z)
  res = besseli(nu,z);
endfunction
