function s = plus(x, y)
  %fprintf(stderr, "plus()");
  if (isnumeric(x))
    s = plus_qd(y,x);
  elseif (isnumeric(y))
    s = plus_qd(x,y);
  else
    s = plus_qq(x,y);
  endif
endfunction
function s = plus_qd(y, x)
  %fprintf(stderr, "plus_qd()");
  S = x + y.hi_;
  e = S - x;
  s = S - e;
  s = (y.hi_ - e) + (x - s);
  H = S + (s + y.lo_);
  h = (s + y.lo_) + (S - H);
  hi = H + h;
  hhi = H - hi;
  s = sf_qdouble(hi, h + hhi);
endfunction
function s = plus_qq(x, y)
  %fprintf(stderr, "plus_qq()");
  S = x.hi_ + y.hi_;
  T = x.lo_ + y.lo_;
  e = S - x.hi_;
  f = T - x.lo_;
  s = S - e;
  t = T - f;
  s = (y.hi_ - e) + (x.hi_ - s);
  t = (y.lo_ - f) + (x.lo_ - t);
  e = s + T;
  H = S + e;
  h = e + (S - H);
  e = t + h;
  hi = H + e;
  hhi = H - hi;
  s = sf_qdouble(hi, e + hhi);
endfunction
