# Claim: at the G-study counts of a balanced crossed p×i×r random design,
#   1 - G = (EMS_pi + EMS_pr - EMS_pir) / EMS_p,
# so under normality (1 - Ĝ)/(1 - G) = [L/E(L)] / [MS_p/EMS_p] with
# L = MS_pi + MS_pr - MS_pir, which is approximately F(ν, n_p - 1) with the
# Satterthwaite ν = L² / Σ_j (c_j MS_j)²/df_j. This supports the interval
#   G ∈ [1 - (1-Ĝ)/F_{α/2}, 1 - (1-Ĝ)/F_{1-α/2}],  F ~ F(ν̂, n_p - 1),
# whose calibration the coverage study must establish (the F law is approximate).
# Plan: salib journal/2026-10-23-g-theory-bootstrap-plan.md.
# Derivation source: EMS from Brennan (2001) as reproduced in CASMA 21 Eq. 71;
# G and δ from their definitions (not from salib's code).
var('np ni nr s_p s_i s_r s_pi s_pr s_ir s_pir')
ems = {
    'p':   s_pir + nr*s_pi + ni*s_pr + ni*nr*s_p,
    'pi':  s_pir + nr*s_pi,
    'pr':  s_pir + ni*s_pr,
    'pir': s_pir,
}
delta = s_pi/ni + s_pr/nr + s_pir/(ni*nr)          # relative error at observed counts
G = s_p / (s_p + delta)
claim = (ems['pi'] + ems['pr'] - ems['pir']) / ems['p']
assert bool((1 - G - claim).simplify_full() == 0), "1 - G identity fails"

# MS_e ~ EMS_e χ²_{df}/df independently: E(L) and Var(L) give the Satterthwaite df.
df = {'p': np-1, 'pi': (np-1)*(ni-1), 'pr': (np-1)*(nr-1), 'pir': (np-1)*(ni-1)*(nr-1)}
EL = ems['pi'] + ems['pr'] - ems['pir']
VarL = 2*ems['pi']^2/df['pi'] + 2*ems['pr']^2/df['pr'] + 2*ems['pir']^2/df['pir']
nu = (2*EL^2/VarL).simplify_full()
nu_hat_form = EL^2 / (ems['pi']^2/df['pi'] + ems['pr']^2/df['pr'] + ems['pir']^2/df['pir'])
assert bool((nu - nu_hat_form).simplify_full() == 0), "Satterthwaite nu form fails"
print("ok: 1-G = (EMS_pi+EMS_pr-EMS_pir)/EMS_p; nu = L^2 / sum MS_j^2/df_j")
