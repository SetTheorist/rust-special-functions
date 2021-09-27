## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_cos (@var{z})
## Compute cosine
##  -- use high-precision range-reduction then series for real
## @end deftypefn

function res = sf_cos(z)
  res = ones(size(z));
  for n = 1:prod(size(z));
    res(n) = sf_cos_1(z(n));
  endfor
endfunction

function res = sf_cos_1(z)
  persistent qpi = sf_qdouble("3.1415926535897932384626433832795028841971693993751");

  # Recurse for complex
  if (imag(z)!=0)
    x = real(z); y = imag(z);
    res = sf_cos(x)*sf_cosh(y) - I*sf_sin(x)*sf_sinh(y);
    return;
  endif

  z = abs(z);
  # range-reduction (if necessary) with increased precision
  # (theoretically not enough but almost)
  if (z>3.1415)
    qz = sf_qdouble(z);
    nper = floor((z/qpi).hi);
    zfrac = (qz - nper*qpi)^2;
    lo = zfrac.lo;
    zz2 = zfrac.hi;
    # some correction terms
    e_ = -lo/2 + lo^2/24 + 2*zz2*lo/24;
  else
    nper = 0;
    zz2 = z^2;
    e_ = 0;
  endif
  # vectorized summation
  NN = 24; # empirically determined number of terms
  terms = (-zz2)*ones(1,NN) ./ (2*(1:NN) .* (2*(1:NN)-1));
  res = sum([1, cumprod(terms), e_], 'extra');
  res *= (-1)^(rem(nper,2));
endfunction

%!test assert(sf_cos(1:10).^2 + sf_sin(1:10).^2, 1+0*(1:10), -5e-15);
%!test assert(sf_cos(0), 1, -5e-15);
%!test assert(sf_cos(pi/2), 0, -5e-15);
%!test assert(sf_cos(pi), -1, -5e-15);
%!test assert(sf_cos((2:2:20)*pi), +1+0*(2:2:20), -5e-15);
%!test assert(sf_cos((1:2:20)*pi), -1+0*(1:2:20), -5e-15);

