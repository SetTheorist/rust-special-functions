## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sin (@var{z})
## Compute sine
##  -- use high-precision range-reduction then series for real
## @end deftypefn

function res = sf_sin(z)
  res = zeros(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_sin_1k(z(n));
  endfor
endfunction

function res = sf_sin_1k(z)
  persistent qpi = sf_qdouble("3.1415926535897932384626433832795028841971693993751");

  # Recurse for complex
  if (imag(z)!=0)
    x = real(z); y = imag(z);
    res = sf_sin(x)*sf_cosh(y) + I*sf_cos(x)*sf_sinh(y);
    return;
  endif

  if (z<0) sgn = -1; else sgn = +1; endif
  z = abs(z);
  # range-reduction (if necessary) with increased precision
  # (theoretically not enough but almost)
  if (z>3.1415)
    qz = sf_qdouble(z);
    nper = floor((z/qpi).hi);
    zfrac = qz - nper*qpi;
    zz2 = (zfrac^2).hi;
    e_ = zfrac.lo;
    fst = zfrac.hi;
  else
    nper = 0;
    zz2 = z^2;
    e_ = 0;
    fst = z;
  endif
  # vectorized summation
  NN = 24; # empirically determined number of terms
  terms = (-zz2)*ones(1,NN) ./ (2*(1:NN) .* (1+2*(1:NN)));
  res = sum([cumprod([fst, terms]),e_], 'extra');
  res *= sgn * (-1)^(rem(nper,2));
endfunction


%!test assert(sf_cos(0:.01:pi).^2 + sf_sin(0:.01:pi).^2, 1+0*(0:.01:pi), -5e-15);
%!test assert(sf_sin(0), 0, -5e-15);
%!test assert(sf_sin(pi/2), 1, -5e-15);
%!test assert(sf_sin(pi), 0, -5e-15);
%!test assert(sf_sin(3*pi/2), -1, -5e-15);
%!test assert(sf_sin((2:20)*pi), 0*(2:20), -1e-14);
%!test assert(sf_sin((1/2+(2:2:20))*pi), +1+0*(2:2:20), -5e-15);
%!test assert(sf_sin((1/2+(1:2:20))*pi), -1+0*(1:2:20), -5e-15);

