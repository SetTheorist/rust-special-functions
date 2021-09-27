## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_zeta_m1 (@var{z})
## Compute Riemann zeta function minus 1.
## @end deftypefn
function res = sf_zeta_m1(z)
  if (nargin < 1) print_usage; endif
  res = ones(size(z));
  for kk = 1:prod(size(z))
    res(kk) = sf_zeta_m1_1(z(kk));
  endfor
endfunction

function res = sf_zeta_m1_1(z)
  if (z==1.0) res = Inf; return endif
# TODO: fixup reflection here
#  if (z < 1.0)
#    res = nan;
#    return;
#  endif
  oldold_res = old_res = res = 0.0;
  sum = 0.0;
  n = 2;
  em1 = z/12.0;
  em2 = z*(z+1)*(z+2)/720.0;
  em3 = z*(z+1)*(z+2)*(z+3)*(z+4)/30240.0;
  em4 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)/1209600.0;
  em5 = z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)/239500800.0;
  do
    sum += n^(-z);
    oldold_res = old_res;
    old_res = res;
    res = sum + n^(1-z)/(z-1) - n^(-z)/2   ...
        + n^(-z-1)*em1 - n^(-z-3)*em2 ...
        + n^(-z-5)*em3 - n^(-z-7)*em4 ...
        + n^(-z-9)*em5 ;
    ++n;
    if (n>999) break; endif
  until ((res == old_res) && (oldold_res == res))
endfunction
