## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_struve_l (@var{nu}, @var{z})
## Compute the Struve function LL_nu(z)
## @end deftypefn

function res = sf_struve_l(nu, z)
  if (nargin<2) print_usage(); endif
  # quick hack approach
  res = -I*sf_exp(-I*nu*pi/2).*sf_struve_h(nu, I*z);
  reals = (imag(nu)==0) & (imag(z)==0);
  res(reals) = real(res(reals));
endfunction
