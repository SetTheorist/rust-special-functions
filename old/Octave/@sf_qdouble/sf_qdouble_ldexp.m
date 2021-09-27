function z = sf_qdouble_ldexp(x, e)
  # hacky implementation
  z = sf_qdouble(x);
  while (e > 0)
    --e;
    z.hi_ *= 2;
    z.lo_ *= 2;
  endwhile
  while (e < 0)
    ++e;
    z.hi_ /= 2;
    z.lo_ /= 2;
  endwhile
endfunction
