## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_rg (@var{x}, @var{y}, @var{z})
## Compute the symmetric elliptic integral $R_G(x,y,z)$
## x,y,z>0
## @end deftypefn
function res = sf_elliptic_rg(x,y,z)
  if (nargin != 3) print_usage; endif
  x_ = x; y_ = y; z_ = z;
  if (x>y) t=x;x=y;y=t; endif
  if (x>z) t=x;x=z;z=t; endif
  if (y>z) t=y;y=z;z=t; endif

  t0 = tn = sqrt(x);
  cn = sqrt(y - x);
  an = sqrt(z - x);
  h0 = hn = sqrt(z);
  theta = +1;
  cn_sum = cn^2/2;
  hn_sum = 0;
  n = 1;
  do
    an = (an + sqrt(an^2 - cn^2))/2;
    tn = (tn + sqrt(tn^2 + theta*cn^2))/2;
    cn = cn^2/(2*an)/2;
    cn_sum += 2^(n-1)*cn^2;
    hnm1 = hn;
    hn = hn*tn/sqrt(tn^2+theta*cn^2);
    hn_sum += 2^n*(hn - hnm1);
    ++n; if (n>999) break; endif
  until (cn^2==0)
  if (n>999) warning('sf:convergence', "sf_elliptic_rg(%g,%g,%g) failed to converge", x_, y_, z_); endif
  res = ((t0^2 + theta*cn_sum)*sf_elliptic_rc(tn^2+theta*an^2, tn^2) + h0 + hn_sum)/2;
endfunction
