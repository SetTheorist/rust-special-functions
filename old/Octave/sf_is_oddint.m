## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_is_oddint (@var{x})
## True if $x$ represents an odd integer
## @end deftypefn
function res = sf_is_oddint(x)
  if (nargin < 1) print_usage; endif
  res = sf_is_int(x) & (rem(x,2)==1);
endfunction
