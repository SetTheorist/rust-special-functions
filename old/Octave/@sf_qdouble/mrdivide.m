function s = mrdivide(x, y)
  if (isnumeric(x))
    s = mrdivide_dq(x,y);
  elseif (isnumeric(y))
    s = mrdivide_qd(x,y);
  else
    s = mrdivide_qq(x,y);
  endif
endfunction
function s = mrdivide_qq(x, y)
  C = x.hi_/y.hi_;
  c = 134217729.0*C;
  hc =c-C;
  u = 134217729.0*y.hi_;
  hc = c-hc;
  tc = C-hc;
  hy = u-y.hi_;
  U = C * y.hi_;
  hy = u-hy;
  ty = y.hi_-hy;
  u = (((hc*hy-U)+hc*ty)+tc*hy)+tc*ty;
  c = ((((x.hi_-U)-u)+x.lo_)-C*y.lo_)/y.hi_;
  hi = C + c;
  Clo = C - hi;
  s = sf_qdouble(hi, Clo + c);
endfunction
function s = mrdivide_dq(x, y)
  C = x/y.hi_;
  c = 134217729.0*C;
  hc =c-C;
  u = 134217729.0*y.hi_;
  hc = c-hc;
  tc = C-hc;
  hy = u-y.hi_;
  U = C*y.hi_;
  hy = u-hy;
  ty = y.hi_-hy;
  u = (((hc*hy-U)+hc*ty)+tc*hy)+tc*ty;
  c = ((((x-U)-u))-C*y.lo_)/y.hi_;
  hi = C + c;
  Clo = C - hi;
  s = sf_qdouble(hi, Clo + c);
endfunction
function s = mrdivide_qd(x, y)
  C = x.hi_/y;
  c = 134217729.0*C;
  hc = c-C;
  u = 134217729.0*y;
  hc = c-hc;

  tc = C-hc;
  hy = u-y;
  U = C*y;
  hy = u-hy;
  ty = y-hy;
  u = (((hc*hy-U)+hc*ty)+tc*hy)+tc*ty;
  c = (((x.hi_-U)-u)+x.lo_)/y;
  hi = C + c;
  Clo = C - hi;
  s = sf_qdouble(hi, Clo + c);
endfunction
