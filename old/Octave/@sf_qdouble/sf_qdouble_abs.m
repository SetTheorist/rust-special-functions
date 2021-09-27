function s = sf_qdouble_abs(q)
  if (q.hi_ >= 0.0)
    s = q;
  else
    s = -q;
  endif
endfunction
