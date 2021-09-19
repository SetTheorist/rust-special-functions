## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_elliptic_pi ([@var{phi}], @var{c}, @var{k})
## Compute the (in)complete elliptic integral of the third kind $\Pi(c, k)=\Pi(\pi/2, c, k)$
##  or $\Pi(\phi, c, k) = \int_0^\phi dt/( (1-c\sin^2(t))\sqrt(1-k^2\sin^2(t)) )$
## (c=\alpha^2 in DLMF notation)
## for real values only 0<k<1, 0<c<1
## @end deftypefn
function res = sf_elliptic_pi(phi, c, k)
  if (nargin < 2) print_usage; endif
    
  if (nargin < 3)
    k=c; c=phi; phi=pi/2;
    if (any(size(k)!=size(c)))
      if (isscalar(k)) k*=ones(size(c));
      elseif (isscalar(c)) c*=ones(size(k));
      else error("sf_elliptic_pi: mismatched parameter sizes");
      endif
    endif
    res = zeros(size(k));
    for kk = 1:prod(size(k))
      res(kk) = complete_agm(k(kk), c(kk));
    endfor
  else
    siz = max([size(k); size(c); size(phi)]);
    if (any(siz!=size(k))) if (isscalar(k)) k*=ones(siz); else error("sf_elliptic_pi: mismatched parameter sizes"); endif; endif
    if (any(siz!=size(c))) if (isscalar(c)) c*=ones(siz); else error("sf_elliptic_pi: mismatched parameter sizes"); endif; endif
    if (any(siz!=size(phi))) if (isscalar(phi)) phi*=ones(siz); else error("sf_elliptic_pi: mismatched parameter sizes"); endif; endif
    res = zeros(size(k));
    for kk = 1:prod(size(k))
      if (phi(kk)==0)
        res(kk) = 0.0;
      else
        res(kk) = gauss_transform(k(kk), c(kk), phi(kk));
      endif
    endfor
  endif

  #qua = quad(@(t)(1.0/(1-c*sf_sin(t)^2)/sqrt(1.0 - k^2*sf_sin(t)^2)), 0, phi)
endfunction

# -\infty < k^2 < 1
# -\infty < c < 1
function res = complete_agm(k, c)
  [an, gn, ~] = sf_agm(1, sf_sqrt(1.0-k^2), 0);
  N = length(an);
  pn = zeros(1, N); pn(1) = sf_sqrt(1 - c);
  Qn = zeros(1, N); Qn(1) = 1;
  en = zeros(1, N); en(1) = (pn(1)^2 - an(1)*gn(1)) / (pn(1)^2 + an(1)*gn(1));
  for n = 2:N
    pn(n) = (pn(n-1)^2 + an(n-1)*gn(n-1)) / (2*pn(n-1));
    en(n) = (pn(n)^2 - an(n)*gn(n)) / (pn(n)^2 + an(n)*gn(n));
    Qn(n) = Qn(n-1)*en(n-1)/2;
  endfor
  qsum = sum(Qn, 'extra');
  res = pi/(4*an(end)) * (2 + c/(1-c)*qsum);
endfunction


function res = gauss_transform(k, c, phi)
  kp = sf_sqrt(1-k^2);
  if (kp==1)
    cp = sf_sqrt(1-c);
    res = atan(cp * sf_tan(phi)) / cp;
    return;
  elseif (1-(k^2/c)==0)
    # special case else rho below is zero...
    res = (sf_elliptic_e(phi, k) - c*cos(phi)*sin(phi)/sqrt(1-c*sin(phi)^2))/(1-c);
    return;
  endif

  k1 = (1 - kp) / (1 + kp);
  delta = sf_sqrt(1-k^2*sf_sin(phi)^2);
  psi1 = asin((1+kp)*sf_sin(phi) / (1+delta));
  rho = sf_sqrt(1 - (k^2/c));
  c1 = c*(1+rho)^2/(1+kp)^2;
  xi = csc(phi)^2;
  #[k1, c1, psi1]
  newgt = gauss_transform(k1, c1, psi1);
  res = ( 4/(1+kp)*newgt + (rho-1)*sf_elliptic_f(phi, k) - sf_elliptic_rc(xi-1, xi-c) )/rho;
endfunction

