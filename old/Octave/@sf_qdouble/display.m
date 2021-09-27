function display(p, raw_dump_only)
  if (nargin < 2)
    raw_dump_only = false;
  endif
  fprintf("%s = ", inputname(1));
  fprintf("<%.17g,%.17g>", p.hi_, p.lo_);
  if (!raw_dump_only)
    fprintf(" = ");
    digits_dump(p);
  endif
  fprintf("\n");
endfunction
function digits_dump(x)
  if (x.hi_ == 0.0) fprintf("0"); return; endif
  if (x.hi_ != x.hi_) fprintf("NaN"); return; endif
  if (x.hi_ == inf) fprintf("Inf"); return; endif
  if (x.hi_ == -inf) fprintf("-Inf"); return; endif
  ten = sf_qdouble(10.0);
  y = sf_qdouble_abs(x);
  q = log10(y.hi_);
  n = floor(q);
  %if (n<0) ++n; endif
  l = ten^n;
  y /= l;
  if (x<0.0) fprintf("-"); endif
  %d = 34; % digits: should be between 3 and 34...
  d = 35; % digits: should be between 3 and 34...
  for i = 1:d
    if (i == 2) fprintf("."); endif
    m = sf_qdouble_floor(y).hi_;
    if ((m < 0) || (m > 9))
      error("internal error in @sf_qdouble/display.digitsdump<%.17g,%.17g>", x.hi_, x.lo_);
    endif
    fprintf("%i", m);
    y = (y - sf_qdouble(m)) * 10.0;
    if (y.hi_ <= 0.0) break; endif # must be integer
  endfor
  if (n != 0) fprintf("e%i", n); endif
endfunction
