## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_rf (@var{x}, @var{y}, @var{z})
## Compute the symmetric elliptic integral of the first kind $R_F(x,y,z)$
## x,y,z>0
## @end deftypefn
function res = sf_elliptic_rf(x,y,z)
  if (nargin != 3) print_usage; endif
  x_ = x; y_ = y; z_ = z;
  #if (x>y) t=x;x=y;y=t; endif
  #if (x>z) t=x;x=z;z=t; endif
  n = 0;
  do
    lam = sqrt(x*y) + sqrt(y*z) + sqrt(z*x);
    mu = (x+y+z)/3;
    xyz_old = [x,y,z];
    XYZ = 1 - [x,y,z]/mu;
    x = (x+lam)/4;
    y = (y+lam)/4;
    z = (z+lam)/4;
    eps = max(abs(XYZ));
    ++n; if (n>999) break; endif
  until (abs(eps)<1e-16) || all(xyz_old == [x,y,z])
  if (n>999)
    warning('sf:convergence', "sf_elliptic_rf(%g,%g,%g) failed to converge", x_, y_, z_);
  endif
  res = x^(-1/2);
  #s2 = sum(XYZ.^2)/4;
  #s3 = sum(XYZ.^3)/6;
  #res = mu^(-1/2) * (1 + s2/5 + s3/7 + s2^2/6 + s2*s3*3/11);
endfunction

%% symmetries
%!test assert(sf_elliptic_rf(3,4,5), sf_elliptic_rf(3,5,4), -1e-15)
%!test assert(sf_elliptic_rf(3,4,5), sf_elliptic_rf(5,4,3), -1e-15)
%!test assert(sf_elliptic_rf(3,4,5), sf_elliptic_rf(5,3,4), -1e-15)
%!test assert(sf_elliptic_rf(3,4,5), sf_elliptic_rf(4,5,3), -1e-15)
%!test assert(sf_elliptic_rf(3,4,5), sf_elliptic_rf(4,3,5), -1e-15)
%
%% scaling
%!test assert(sf_elliptic_rf(7*3,7*4,7*5), sf_elliptic_rf(3,4,5)/sqrt(7), -1e-15)
%!test assert(sf_elliptic_rf(11*3,11*4,11*5), sf_elliptic_rf(3,4,5)/sqrt(11), -1e-15)
%
%% special constant
%!test assert(sf_elliptic_rf(0,1,2), sf_gamma(1/4)^2/(4*sqrt(2*pi)), -1e-15)
%
%% special case
%!test assert(sf_elliptic_rf(0,3,3), pi/(2*sqrt(3)), -1e-15)
%!test assert(sf_elliptic_rf(0,9,9), pi/6, -1e-15)
%
%% special case
%!test assert(sf_elliptic_rf(2,2,2), 1/sqrt(2), -1e-15)
%!test assert(sf_elliptic_rf(17,17,17), 1/sqrt(17), -1e-15)
%
%% direct quadrature
%!test assert(sf_elliptic_rf(1,2,3), quad(@(t)(1/sqrt((t+1)*(t+2)*(t+3))), 0, inf)/2, -1e-12)
%!test assert(sf_elliptic_rf(12,23,37), quad(@(t)(1/sqrt((t+12)*(t+23)*(t+37))), 0, inf)/2, -1e-12)
