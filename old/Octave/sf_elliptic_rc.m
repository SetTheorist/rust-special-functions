## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_rc (@var{x}, @var{y})
## Compute the symmetric elliptic integral $R_C(x,y)$ for real parameters
## x>=0, y!=0
## @end deftypefn
function res = sf_elliptic_rc(x,y)
  if (nargin != 2) print_usage; endif
  if (any(size(x)!=size(y)))
    if (isscalar(x)) x*=ones(size(y));
    elseif (isscalar(y)) y*=ones(size(x));
    else error("sf_elliptic_rc: mismatched parameter sizes");
    endif
  endif
  res = zeros(size(x));
  for kk = 1:prod(size(x))
    if (0<=x(kk) && x(kk)<y(kk))
      if (x(kk)==0)
        res(kk) = 1/sf_sqrt(y(kk)-x(kk)) * sf_acos(sf_sqrt(x(kk)/y(kk)));
      else
        res(kk) = 1/sf_sqrt(y(kk)-x(kk)) * sf_atan(sf_sqrt((y(kk)-x(kk))/x(kk)));
      endif
    elseif (0<y(kk) && y(kk)<x(kk))
      res(kk) = 1/sf_sqrt(x(kk)-y(kk)) * sf_atanh(sf_sqrt((x(kk)-y(kk))/x(kk)));
      #res(kk) = 1/sf_sqrt(x(kk)-y(kk)) * sf_log((sf_sqrt(x(kk)) + sf_sqrt(x(kk)-y(kk)))/sf_sqrt(y(kk)));
    elseif (y(kk)<0 && 0<=x(kk))
      #res(kk) = 1/sf_sqrt(x(kk)-y(kk)) * sf_atanh(sf_sqrt(x(kk)/(x(kk)-y(kk))));
      res(kk) = 1/sf_sqrt(x(kk)-y(kk)) * sf_log((sf_sqrt(x(kk))+sf_sqrt(x(kk)-y(kk)))/sf_sqrt(-y(kk)));
      #res(kk) = sf_sqrt(x(kk)/(x(kk)-y(kk))) * sf_elliptic_rc(x(kk)-y(kk), -y(kk));
    elseif (x(kk)==y(kk))
      res(kk) = 1/sf_sqrt(x(kk));
    else
      warning("sf_elliptic_rc(%g,%g): unsupported domain", x(kk), y(kk));
      res(kk) = nan;
    endif
  endfor
endfunction
