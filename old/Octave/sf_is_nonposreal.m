## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_is_nonposreal (@var{x})
## True if $x$ represents a real <= 0
## @end deftypefn
function res = sf_is_nonposreal(x)
  if (nargin < 1) print_usage; endif
  res = isreal(x) & (x<=0);
endfunction
