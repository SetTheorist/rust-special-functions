function res = sf_qdouble_zeta(z, sub1)
  if (nargin<2) sub1 = false; endif
  if (sub1)
    res = sf_qdouble(0.0)
  else
    res = sf_qdouble(1.0)
  endif
  zz = sf_qdouble(z);
  n = 2;
  sm = res;
  old_res = res;
  do
    sm += n^(-zz);
    old_old_res = old_res;
    old_res = res;
    if (1)
    res = sm  ...
        + n^(1-z)/(z-1) - n^(-z)/2    ...
        + n^(-z-1)*z/12.0     ...
        - n^(-z-3)*z*(z+1)*(z+2)/720.0      ...
        + n^(-z-5)*z*(z+1)*(z+2)*(z+3)*(z+4)/30240.0      ...
        - n^(-z-7)*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)/1209600.0      ...
        + n^(-z-9)*z*(z+1)*(z+2)*(z+3)*(z+4)*(z+5)*(z+6)*(z+7)*(z+8)/239500800.0;
    else
    res = sm;
    endif
    ++n;
    if (n>999) break; endif
  until (res == old_res) && (res == old_old_res)
endfunction

