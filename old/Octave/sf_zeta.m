## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_zeta (@var{z})
## Compute Riemann zeta function.
## @end deftypefn
function res = sf_zeta(z)
  if (nargin < 1) print_usage; endif
  res = zeros(size(z));
  for k = 1:prod(size(z))
    if (z(k)==1.0)
      res(k) = Inf;
    elseif (real(z(k))<0.0)
      res(k) = 2 * (2*pi)^(z(k)-1)*sf_sin(pi*z(k)/2)*sf_gamma(1-z(k))*sf_zeta(1-z(k));
    else
      res(k) = sf_zeta_1(z(k));
    endif
  endfor
endfunction

function res = sf_zeta_1(z)
  oldold_res = old_res = res = 0.0;
  smm = 1.0;
  e_ = 0;
  n = 2;
  zz1 = z/12;
  zz2 = z*(z+1)*(z+2)/720.0;
  zz3 = z*(z+1)*(z+2)*(z+3)*(z+4)/30240.0;
  zz4 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)/1209600.0;
  zz5 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)/239500800.0;
  do
    #smm += n^(-z);
    t_ = smm;
    y_ = n^(-z) + e_;
    smm = t_ + y_;
    e_ = (t_ - smm) + y_;
    oldold_res = old_res;
    old_res = res;
    res = smm ...
        + n^(1-z)/(z-1) - n^(-z)/2   ...
        + n^(-z-1)*zz1 - n^(-z-3)*zz2 + n^(-z-5)*zz3 - n^(-z-7)*zz4 + n^(-z-9)*zz5 ...
        + e_;
    ++n;
    if (n>999) break; endif
  until ((res == old_res) && (oldold_res == res))
endfunction
