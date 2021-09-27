function res = sf_qdouble_sqrt(y)
  if (y.hi_ < 0.0) res=sf_qdouble(nan); return; endif
  if (y.hi_ == 0.0) res=sf_qdouble(y); return; endif
  c = sqrt(y.hi_);
  p = 134217729.0 * c;
  hx = c - p;
  hx += p;
  tx = c - hx;
  p = hx^2;
  q = 2.0*hx*tx;
  u = p + q;
  uu = (p-u) + q + tx^2;
  cc = (((y.hi_ - u) - uu) + y.lo_) / (c+c);
  u = c + cc;
  res = sf_qdouble(u, cc+(c-u));
endfunction
