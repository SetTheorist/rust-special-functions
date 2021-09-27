## -*- texinfo -*-
## @deftypefn {Function File} {[@var{f},@var{df}] =} sf_coulomb_wave_f (@var{ell}, @var{eta}, @var{rho})
## Compute Coulomb wave function regular at zero, $F_ell(eta,rho)$
## @end deftypefn
function [res,resd] = sf_coulomb_wave_f(ell, eta, rho, opt)
  if (nargin < 3) print_usage; endif
  if (nargin<4) opt=false; endif
  if (!sf_is_nonnegint(ell) || rho<0) res=resd=nan; return; endif
  cc = sf_coulomb_wave_normalizer(ell, eta);
  if (!opt)
    # series in rho
    akm2 = 1.0;
    akm1 = eta/(ell+1);
    res = akm2*rho^(ell+1) + akm1*rho^(ell+2);
    resd = (ell+1)*akm2*rho^(ell) + (ell+2)*akm1*rho^(ell+1);
    k = ell+3;
    term = rho^(ell+1);
    do
      term *= rho;
      ak = (2*eta*akm1 - akm2) / ((k+ell)*(k-ell-1));
      old_res = res;
      res += term * rho * ak;
      old_resd = resd;
      resd += term * ak * k;
      akm2 = akm1;
      akm1 = ak;
      ++k;
      if (k>999) break; endif
    until ((res == old_res) && (resd == old_resd))
    res *= cc;
    resd *= cc;
  endif

  if (opt)
    # downward series in ell
    # (though direct computation by series seems to give just as good of results,
    #  this blows up for huge ell values also...)
    ell_ = ell;
    NN = ell + 100;
    x = zeros(NN,1);
    # accurate starting values give a little better results for large ell
    x(NN  ) = sf_coulomb_wave_normalizer(NN-1,eta) * rho^(NN-1+1);
    x(NN-1) = sf_coulomb_wave_normalizer(NN-2,eta) * rho^(NN-2+1);
    for k = (NN-2):(-1):1
      ell = k-1;
      x(k) = (T(ell+1,eta,rho)*x(k+1,1) - R(ell+2,eta,rho)*x(k+2,1))/R(ell+1,eta,rho);
    endfor
    # renormalize by computing one value
    #x(:) *= (sf_coulomb_wave_f(0,eta,rho)/x(1));
    res = x(ell_+1);
    resd = x(ell_+1);
  endif
endfunction
function res = R(ell,eta,rho)
  res = sqrt(1 + eta^2/ell^2);
endfunction
function res = S(ell,eta,rho)
  res = ell/rho + eta/ell;
endfunction
function res = T(ell,eta,rho)
  res = S(ell,eta,rho) + S(ell+1,eta,rho);
endfunction
