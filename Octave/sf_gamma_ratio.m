## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_gamma_ratio (@var{aa}, @var{bb})
## Compute ratio of gamma functions $\Gamma(a_1) ... \Gamma(a_p) / \Gamma(b_1) ... \Gamma(b_q)$
## Cancels poles appropriately.
## TODO: currently prone to overflow issues, fix this
## @end deftypefn

function res = sf_gamma_ratio(aa, bb)
  if (nargin < 2) print_usage; endif
  anegs = sf_is_nonposint(aa);
  bnegs = sf_is_nonposint(bb);
  num_anegs = sum(anegs);
  num_bnegs = sum(bnegs);
  if (num_anegs != num_bnegs)
    if (num_anegs > num_bnegs)
      res = Inf;
    else
      res = 0;
    endif
    return;
  endif
  if (num_anegs != 0)
    mult = (-1)^(rem(sum(aa(anegs))+sum(bb(anegs)),2)) * prod(sf_factorial(-bb(bnegs))) / prod(sf_factorial(-aa(anegs)));
  else
    mult = 1.0;
  endif
  res = mult * prod(sf_gamma(aa(!anegs))) / prod(sf_gamma(bb(!bnegs)));
endfunction
