## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_coulomb_wave_theta (@var{ell}, @var{eta}, @var{rho})
## Compute Coulomb wave utility $\theta_ell(eta,rho) = rho - \eta \ln(2\rho) - \ell\pi/2 + \sigma_\ell(\eta)$
## @end deftypefn
function res = sf_coulomb_wave_theta(ell, eta, rho)
  if (nargin < 3)
    print_usage;
  endif
  res = rho - eta*sf_log(2*rho) - ell*pi/2 + sf_coulomb_wave_phase_shift(ell,eta);
endfunction
