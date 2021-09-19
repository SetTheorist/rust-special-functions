function p = sf_qdouble_log(x)
  if (x.hi_ < 0.0) p=sf_qdouble(nan,nan); return; endif
  if (x.hi_ == 0.0) p=sf_qdouble(-inf,-inf); return; endif
  s = sf_qdouble(log(x.hi_));
  e = sf_qdouble_exp(s);
  % Newton
  p = s + (x - e)/e;
endfunction
