function [s, ser] = sf_qdouble_agm(a,b,return_series)
  if (nargin<2) print_usage(); return; endif
  if (nargin<3) return_series = false; endif
  if (isnumeric(a)) a=sf_qdouble(a); endif
  if (isnumeric(b)) b=sf_qdouble(b); endif
  oa = sf_qdouble(nan); ob = sf_qdouble(nan);
  if (return_series)
    ser = {}; % can't use matrix with classes - use cells
    n = 1;
  endif
  while ((a != b) && !(oa==a && ob==b) && (a==a && b==b))
    if (return_series)
      ser{n,1} = a;
      ser{n,2} = b;
      ++n;
    endif
    oa = a;
    ob = b;
    ta = a;
    a = (a+b)*0.5;
    b = sf_qdouble_sqrt(ta*b);
  endwhile
  s = a;
endfunction
