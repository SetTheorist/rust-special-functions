function s = mpower(c, e)
  if (isnumeric(c)) c=sf_qdouble(c); endif % promote double to sf_qdouble
  if (isnumeric(e)) e=sf_qdouble(e); endif % promote double to sf_qdouble
  if (e == sf_qdouble_floor(e))
    s = mpower_int(c, e);
  else
    s = sf_qdouble_exp(e * sf_qdouble_log(c));
  endif
endfunction
function s = mpower_int(u, c)
  if (c < 0)
    s = 1.0/mpower_int(u, -c);
    return;
  endif
  switch (c)
  case 0
    s = sf_qdouble(1.0);
  case 1
    s = sf_qdouble(u);
  case 2
    s = mpower_sqr(u);
  case 3
    s = mpower_sqr(u)*u;
  case 4
    s = mpower_sqr(mpower_sqr(u));
  otherwise # binary method
    n = sf_qdouble_abs(c);
    y = sf_qdouble(1.0);
    z = sf_qdouble(u);
    while (true)
      m = n;
      n = sf_qdouble_floor(n/2);
      if (n+n != m)
        y *= z;
        if (n == 0)
          s = y;
          return;
        endif
      endif
      z = mpower_sqr(z);
    endwhile
  endswitch
endfunction
function s = mpower_sqr(x)
 C = 134217729.0 * x.hi_;
 hx = C - x.hi_;
 hx = C - hx;
 tx = x.hi_ - hx;
 C = x.hi_^2;
 c = ((((hx^2 - C) + 2.0*hx*tx)) + tx^2) + 2.0*x.hi_*x.lo_;
 hx = C + c;
 Clo = C - hx;
 s = sf_qdouble(hx, c + Clo);
endfunction
