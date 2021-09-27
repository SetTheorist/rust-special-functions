## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_y_dz (@var{nu}, @var{z})
## Compute derivative of Bessel Y_nu(z) function with respect to argument z
## @end deftypefn

function res = sf_bessel_y_dz(nu, z)
  # quick hack
  res = (sf_bessel_y(nu-1, z) - sf_bessel_y(nu+1, z))/2;
endfunction
