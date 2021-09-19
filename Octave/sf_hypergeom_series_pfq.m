## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hypergeom_series_pfq (@var{a_arr}, @var{b_arr}, @var{z})
## Compute the hypergeometric pFq *series*.  Direct (naive) implementation of summation.
## @end deftypefn

function res = sf_hypergeom_series_pfq(aa, bb, z)
  if (nargin != 3) print_usage; endif
  [aa,bb] = cleanup(aa,bb);
  res = zeros(size(z));
  for k = 1:prod(size(z))
    res(k) = sf_hypergeom_series_pfq_1(aa, bb, z(k));
  endfor
endfunction

function res = sf_hypergeom_series_pfq_1(aa, bb, z)
  res = 1.0;
  term = 1.0;
  n = 1;
  e_ = 0;
  do
    term *= z / n * prod(aa) / prod(bb);
    old_res = res;

    #res += term;
      t_ = res;
      y_ = term + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;

    aa += 1;
    bb += 1;
    ++n; if (n>999) break; endif
  until (res == old_res)
  res += e_;
endfunction

# remove common elements
function [ao,bo] = cleanup(a, b)
  ao=sort(a); bo=sort(b);
  ai=1; bi=1;
  while (ai<=length(ao) && bi<=length(bo))
    if (ao(ai)==bo(bi))
        ao(ai) = bo(bi) = nan;
    elseif (ao(ai)<bo(bi))
        ++ai;
    elseif (ao(ai)>bo(bi))
        ++bi;
    else
      ++ai;
      ++bi;
    endif
  endwhile
  ao=ao(!isnan(ao)); bo=bo(!isnan(bo));
endfunction
