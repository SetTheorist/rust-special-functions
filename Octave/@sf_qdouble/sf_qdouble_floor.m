function s = sf_qdouble_floor(q)
  fhi = floor(q.hi_);
  if (fhi != q.hi_)
    s = sf_qdouble(fhi);
  else
    s = sf_qdouble(q.hi_, floor(q.lo_));
  endif
endfunction
