function s = mtimes(x, y)
  %fprintf(stderr, "mtimes()");
  if (isnumeric(x))
    s = mtimes_qd(y,x);
  elseif (isnumeric(y))
    s = mtimes_qd(x,y);
  else
    s = mtimes_qq(x,y);
  endif
endfunction
function s = mtimes_qd(y, x)
  %fprintf(stderr, "mtimes_qd(x=%g)", x);
  C = 134217729.0 * x;
  hx = C - x;
  c = 134217729.0 * y.hi_;
  hx = C - hx;
  tx = x - hx;
  hy = c - y.hi_;
  C = x * y.hi_;
  hy = c - hy;
  ty = y.hi_ - hy;
  c = ((((hx*hy - C) + hx*ty) + tx*hy) + tx*ty) + x*y.lo_;
  hi = C + c;
  hic = C - hi;
  s = sf_qdouble(hi, c + hic);
endfunction
function s = mtimes_qq(x, y)
  %fprintf(stderr, "mtimes_qq()");
  C = 134217729.0 * x.hi_;
  hx = C - x.hi_;
  c = 134217729.0 * y.hi_;
  hx = C - hx;
  tx = x.hi_ - hx;
  hy = c - y.hi_;
  C = x.hi_ * y.hi_;
  hy = c - hy;
  ty = y.hi_ - hy;
  c = ((((hx*hy - C) + hx*ty) + tx*hy) + tx*ty) + (x.hi_*y.lo_ + x.lo_*y.hi_);
  hi = C + c;
  hx = C - hi;
  s = sf_qdouble(hi, c+hx);
endfunction
