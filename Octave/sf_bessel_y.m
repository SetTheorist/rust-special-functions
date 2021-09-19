## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_y (@var{nu}, @var{z})
## Compute Bessel Y_nu(z) function -- just a wrapper of built-in bessely()
## @end deftypefn

function res = sf_bessel_y(nu, z)
  res = bessely(nu, z);
endfunction
