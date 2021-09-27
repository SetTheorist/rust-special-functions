## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_bessel_k (@var{nu}, @var{z})
## Compute Bessel K_nu(z) function -- just a wrapper of built-in besselk()
## @end deftypefn

function res = sf_bessel_k(nu,z)
  res = besselk(nu,z);
  #k_quad(nu, z);
endfunction

function res = k_quad(nu, z)
  # estimate range needed
  smm = sf_exp(-z);
  mx = 1;
  do
    old_smm = smm;
    smm += abs(cosh(nu*mx)*exp(-z*cosh(mx)));
    ++mx; 
  until (smm == old_smm);
  mx+=3

  res = smm;
  nn = 1;
  do
    h = 2^(-nn);
    np = floor(mx/h);
    xx = h*(0:np);
    yy = cosh(nu*xx) .* exp(-z*cosh(xx));
    yy(1) /= 2;
    old_res = res;
    res = h*sum(yy, 'extra')
    ++nn;
    if (nn>14) break; endif
  until (res == old_res)
  np
endfunction
