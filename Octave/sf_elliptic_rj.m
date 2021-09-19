## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_rj (@var{x}, @var{y}, @var{z}, @var{p})
## Compute the symmetric elliptic integral $R_J(x,y,z,p)$
## x,y,z>0
## @end deftypefn
function res = sf_elliptic_rj(x,y,z,p)
  persistent two23 = 0.62996052494743658238;
  if (nargin != 4) print_usage; endif
  x_=x; y_=y; z_=z; p_=p;
  n = 0;
  smm = 0;
  scale = 1.0;
  do
    lam = sqrt(x*y) + sqrt(y*z) + sqrt(z*x);
    alpha = p*(sqrt(x)+sqrt(y)+sqrt(z)) + sqrt(x*y*z);
    beta = sqrt(p)*(p+lam);
    old_smm = smm;
    if (abs(1 - alpha^2/beta^2) < 5e-16)
      # optimization to reduce external calls
      smm += scale*3/alpha;
    else
      smm += scale*3*sf_elliptic_rc(alpha^2, beta^2);
    endif
    mu = (x+y+z+p)/4;
    xyzp_old = [x,y,z,p];
    XYZP = 1 - [x,y,z,p]/mu;
    x = (x+lam)*two23/mu;
    y = (y+lam)*two23/mu;
    z = (z+lam)*two23/mu;
    p = (p+lam)*two23/mu;
    scale *= mu^(-3/2);
    eps = max(abs(XYZP));
    ++n; if (n>999) break; endif
  until (abs(eps)<1e-16) || all(xyzp_old == [x,y,z,p]) || (smm==old_smm)
  if (n>999)
    warning('sf:convergence', "sf_elliptic_rj(%g,%g,%g,%g) failed to converge", x_, y_, z_, p_);
  endif
  res = scale*x^(-3/2) + smm;
endfunction
