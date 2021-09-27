## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_coulomb_wave_phase_shift (@var{ell}, @var{eta})
## Compute Coulomb wave function phase shift $\sigma_\ell(\eta) = Ph(\Gamma(ell+1+eta*I))$
## @end deftypefn
function res = sf_coulomb_wave_phase_shift(ell, eta)
  if (nargin < 2)
    print_usage;
  endif
  res = arg(sf_gamma(ell+1+I*eta));
endfunction
