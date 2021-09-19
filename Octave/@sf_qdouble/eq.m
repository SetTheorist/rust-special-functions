function b = eq(p, q)
  if (isnumeric(q))
    b = (p.hi_ == q) && (p.lo_ == 0.0);
  else
    b = (p.hi_ == q.hi_) && (p.lo_ == q.lo_);
  endif
endfunction
