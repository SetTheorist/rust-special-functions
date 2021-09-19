## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_rd (@var{x}, @var{y}, @var{z})
## Compute the symmetric elliptic integral $R_D(x,y,z)$
## x,y,z>0
## @end deftypefn
function res = sf_elliptic_rd(x,y,z)
  persistent two23 = 0.62996052494743658238;
  if (nargin != 3) print_usage; endif
  x_ = x; y_ = y; z_ = z;
  n = 0;
  smm = 0;
  do
    lam = sqrt(x*y) + sqrt(y*z) + sqrt(z*x);
    smm += 3/sqrt(z)/(z+lam);
    mu = (x+y+z)/3;
    xyz_old = [x,y,z];
    XYZ = 1 - [x,y,z]/mu;
    x = (x+lam)*two23;
    y = (y+lam)*two23;
    z = (z+lam)*two23;
    eps = max(abs(XYZ));
    ++n; if (n>999) break; endif
  until (abs(eps)<1e-16) || all(xyz_old == [x,y,z])
  if (n>999)
    warning('sf:convergence', "sf_elliptic_rd(%g,%g,%g) failed to converge", x_, y_, z_);
  endif
  res = x^(-3/2) + smm;
endfunction

%% symmetries
%!test assert(sf_elliptic_rd(3,4,5), sf_elliptic_rd(4,3,5), -1e-15)
%
%% scaling
%!test assert(sf_elliptic_rd(7*3,7*4,7*5), sf_elliptic_rd(3,4,5)/7^(3/2), -1e-15)
%!test assert(sf_elliptic_rd(11*3,11*4,11*5), sf_elliptic_rd(3,4,5)/11^(3/2), -1e-15)
