## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_spence (@var{z})
## Compute Spence's integral.
## @end deftypefn

function res = sf_spence(z)
  if (nargin < 1) print_usage; endif
  res = zeros(size(z));
  for j = 1:prod(size(z))
    res(j) = spence_single(z(j));
  endfor
endfunction

function res = spence_single(z)
  persistent pi2_6 = pi^2/6;

  if (isnan(z)) res = z; return; endif
  if (z < 0) res = nan; return; endif
  if (z == 0) res = pi2_6; return; endif

  if (z < 0.5) method = 0;
  elseif (z < 1.0) method = 1;
  elseif (z < 2.5) method = 3;
  else method = 2;
  endif

  # different reflections
  switch (method)
  case 0
    z_mult = z;
    [res,e_] = series(z);
    term = pi2_6 - sf_log(z)*sf_log(1-z);
  case 1
    [res,e_] = series(1 - z);
     res = -res; e_ = -e_;
     term = 0.0;
  case 2
    [res,e_] = series(1/(1-z));
    term = -pi2_6 - sf_log(z-1)^2/2;
  case 3
    #if (abs((z-1)/z)<1)
      [res,e_] = series((z - 1)/z);
      #[res,e_] = series(1 - 1/z);
    #else
    #  [res,e_] = series(z/(z-1));
    # TODO: - finish this!  need this inversion
    term = -sf_log(z)^2/2;
  endswitch
  res = (res + term) + e_;
endfunction

function [res,e_] = series(z)
  res = old_res = 0.0;
  zk = 1.0;
  k = 1;
  e_ = 0.0;
  do
    zk *= z;
    old_res = res;
    #res += zk/k^2;
      t_ = res;
      y_ = (-zk/k^2) + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
    ++k; if (k>999) break; endif
  until (res == old_res)
  if (k>999) warning('sf:convergence', "sf_spence: series(%g) failed to converge", z); endif
endfunction

