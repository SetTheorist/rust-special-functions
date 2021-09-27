function p = sf_qdouble_exp(x)
# Code borrowed... (and ultimately ideas from Alan Miller)
  if (x.hi_<-745.0)
    p = sf_qdouble(0.0);
    return;
  endif
  y = x/log2();

  iy=fix(rint(y).hi_);
  temp=sf_qdouble(iy);
  y = (y-temp)*log2();
  y = sf_qdouble_ldexp(y,-1);
  ysq = y^2;
  sum1 = y*((((ysq + 3960.0)*ysq + 2162160.0)*ysq + 302702400.0)*ysq + 8821612800.0);
  sum2 = (((90.0*ysq + 110880.0)*ysq + 30270240.0)*ysq + 2075673600.0)*ysq + 17643225600.0;

  temp = sum1/(sum2-sum1);
  y = temp*(temp+1);
  y = sf_qdouble_ldexp(y,2);
  p = sf_qdouble_ldexp(y+1,iy);
endfunction
function z = rint(x)
  z = sf_qdouble_floor(x + sf_qdouble(0.5));
endfunction
function z = log2()
  persistent cache = nan;
  if (cache == cache)
    z = cache;
    return
  endif
  # use series log(2) = (2/3) * \sum_{k>=1} 1/((2k+1)9^k)
  k = 1;
  res = sf_qdouble(1.0);
  term = sf_qdouble(1.0);
  do
    term /= 9.0;
    old_res = res;
    res += term / (2*k+1);
    ++k;
  until (res == old_res)
  z = 2*res/3;
  cache = z;
endfunction

