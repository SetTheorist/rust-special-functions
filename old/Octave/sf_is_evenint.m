## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_is_evenint (@var{x})
## True if $x$ represents an even integer
## @end deftypefn
function res = sf_is_evenint(x)
  if (nargin < 1) print_usage; endif
  res = sf_is_int(x) & (rem(x,2)==0);
endfunction
