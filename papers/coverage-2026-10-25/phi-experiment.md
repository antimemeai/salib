# Phi interval experiment

n_sim = 1900, B = 999, nominal 0.95. Method = generating values / interval construction. "true components" is an oracle diagnostic, not a method. Lower miss: interval entirely below the truth; upper miss: entirely above.

| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 100×20×2 | jiang-2022 | normal | Phi | zeroed estimate / percentile | 0.6511 | 1.000 | 0.9600 (0.0045) | 0.0274 | 0.0126 | 0.9600 | 0.9579 | 0.2450 | errors 0 | fail |
| 100×20×2 | jiang-2022 | normal | Phi | zeroed estimate / basic | 0.6511 | 1.000 | 0.9158 (0.0064) | 0.0021 | 0.0821 | 0.9158 | 0.8942 | 0.2450 | errors 0 | fail |
| 100×20×2 | jiang-2022 | normal | Phi | zeroed estimate / bias-corrected | 0.6511 | 1.000 | 0.9437 (0.0053) | 0.0289 | 0.0274 | 0.9437 | 0.9411 | 0.2434 | errors 0 | pass |
| 100×20×2 | jiang-2022 | normal | Phi | true components / percentile | 0.6511 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.2370 | errors 0 | fail |
| 100×20×2 | jiang-2022 | normal | Phi | true components / basic | 0.6511 | 1.000 | 0.6584 (0.0109) | 0.1421 | 0.1995 | 0.6584 | 0.6489 | 0.2368 | errors 0 | fail |
| 100×20×2 | jiang-2022 | normal | Phi | true components / bias-corrected | 0.6511 | 0.999 | 0.6753 (0.0107) | 0.1553 | 0.1684 | 0.6760 | 0.6679 | 0.2008 | errors 0 | fail |
| 200×20×2 | high-g | normal | Phi | zeroed estimate / percentile | 0.8743 | 1.000 | 0.6958 (0.0106) | 0.0000 | 0.3042 | 0.6958 | 0.6974 | 0.1920 | errors 0 | fail |
| 200×20×2 | high-g | normal | Phi | zeroed estimate / basic | 0.8743 | 1.000 | 0.5579 (0.0114) | 0.0011 | 0.4411 | 0.5579 | 0.5605 | 0.1666 | errors 0 | fail |
| 200×20×2 | high-g | normal | Phi | zeroed estimate / bias-corrected | 0.8743 | 1.000 | 0.7063 (0.0104) | 0.0000 | 0.2937 | 0.7063 | 0.7095 | 0.2487 | errors 0 | fail |
| 200×20×2 | high-g | normal | Phi | true components / percentile | 0.8743 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.2308 | errors 0 | fail |
| 200×20×2 | high-g | normal | Phi | true components / basic | 0.8743 | 1.000 | 0.4511 (0.0114) | 0.1111 | 0.4379 | 0.4511 | 0.4553 | 0.1509 | errors 0 | fail |
| 200×20×2 | high-g | normal | Phi | true components / bias-corrected | 0.8743 | 0.998 | 0.6589 (0.0109) | 0.1253 | 0.2137 | 0.6603 | 0.6611 | 0.2027 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | zeroed estimate / percentile | 0.4651 | 1.000 | 0.7979 (0.0092) | 0.0000 | 0.2021 | 0.7979 | 0.8805 | 0.3563 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | zeroed estimate / basic | 0.4651 | 1.000 | 0.6305 (0.0111) | 0.0547 | 0.3147 | 0.6305 | 0.6905 | 0.3439 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | zeroed estimate / bias-corrected | 0.4651 | 1.000 | 0.7942 (0.0093) | 0.0021 | 0.2037 | 0.7942 | 0.8516 | 0.3629 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | true components / percentile | 0.4651 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.4434 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | true components / basic | 0.4651 | 1.000 | 0.5258 (0.0115) | 0.1284 | 0.3458 | 0.5258 | 0.5742 | 0.4183 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | true components / bias-corrected | 0.4651 | 0.998 | 0.6653 (0.0108) | 0.1132 | 0.2195 | 0.6667 | 0.6763 | 0.3329 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | zeroed estimate / percentile | 0.9091 | 1.000 | 0.7984 (0.0092) | 0.0000 | 0.2016 | 0.7984 | 0.8084 | 0.1217 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | zeroed estimate / basic | 0.9091 | 1.000 | 0.6742 (0.0108) | 0.0000 | 0.3258 | 0.6742 | 0.6858 | 0.1137 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | zeroed estimate / bias-corrected | 0.9091 | 1.000 | 0.8163 (0.0089) | 0.0005 | 0.1832 | 0.8163 | 0.8237 | 0.1503 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | true components / percentile | 0.9091 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.1330 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | true components / basic | 0.9091 | 1.000 | 0.5689 (0.0114) | 0.0968 | 0.3342 | 0.5689 | 0.5747 | 0.1045 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | true components / bias-corrected | 0.9091 | 0.998 | 0.6700 (0.0108) | 0.1195 | 0.2089 | 0.6711 | 0.6742 | 0.1188 | errors 0 | fail |
| 200×8×2 | high-g | normal | Phi | zeroed estimate / percentile | 0.8533 | 1.000 | 0.8137 (0.0089) | 0.0000 | 0.1863 | 0.8137 | 0.8389 | 0.1925 | errors 0 | fail |
| 200×8×2 | high-g | normal | Phi | zeroed estimate / basic | 0.8533 | 1.000 | 0.5953 (0.0113) | 0.0011 | 0.4037 | 0.5953 | 0.6305 | 0.1775 | errors 0 | fail |
| 200×8×2 | high-g | normal | Phi | zeroed estimate / bias-corrected | 0.8533 | 1.000 | 0.7458 (0.0100) | 0.0016 | 0.2526 | 0.7458 | 0.7668 | 0.2359 | errors 0 | fail |
| 200×8×2 | high-g | normal | Phi | true components / percentile | 0.8533 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.2386 | errors 0 | fail |
| 200×8×2 | high-g | normal | Phi | true components / basic | 0.8533 | 1.000 | 0.5074 (0.0115) | 0.0921 | 0.4005 | 0.5074 | 0.5247 | 0.1699 | errors 0 | fail |
| 200×8×2 | high-g | normal | Phi | true components / bias-corrected | 0.8533 | 0.999 | 0.6695 (0.0108) | 0.1089 | 0.2211 | 0.6698 | 0.6737 | 0.2061 | errors 0 | fail |
| 50×20×2 | high-g | normal | Phi | zeroed estimate / percentile | 0.8743 | 1.000 | 0.8463 (0.0083) | 0.0011 | 0.1526 | 0.8463 | 0.8447 | 0.2233 | errors 0 | fail |
| 50×20×2 | high-g | normal | Phi | zeroed estimate / basic | 0.8743 | 1.000 | 0.6532 (0.0109) | 0.0021 | 0.3447 | 0.6532 | 0.6521 | 0.1905 | errors 0 | fail |
| 50×20×2 | high-g | normal | Phi | zeroed estimate / bias-corrected | 0.8743 | 1.000 | 0.8174 (0.0089) | 0.0074 | 0.1753 | 0.8174 | 0.8174 | 0.2693 | errors 0 | fail |
| 50×20×2 | high-g | normal | Phi | true components / percentile | 0.8743 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.2582 | errors 0 | fail |
| 50×20×2 | high-g | normal | Phi | true components / basic | 0.8743 | 1.000 | 0.5353 (0.0114) | 0.1026 | 0.3621 | 0.5353 | 0.5358 | 0.1655 | errors 0 | fail |
| 50×20×2 | high-g | normal | Phi | true components / bias-corrected | 0.8743 | 0.997 | 0.6589 (0.0109) | 0.1274 | 0.2105 | 0.6610 | 0.6605 | 0.2271 | errors 0 | fail |
| 20×20×3 | all-one | normal | Phi | zeroed estimate / percentile | 0.5556 | 1.000 | 0.9532 (0.0048) | 0.0195 | 0.0274 | 0.9532 | 0.9521 | 0.5790 | errors 0 | pass |
| 20×20×3 | all-one | normal | Phi | zeroed estimate / basic | 0.5556 | 1.000 | 0.8005 (0.0092) | 0.0600 | 0.1395 | 0.8005 | 0.7821 | 0.5562 | errors 0 | fail |
| 20×20×3 | all-one | normal | Phi | zeroed estimate / bias-corrected | 0.5556 | 1.000 | 0.9463 (0.0052) | 0.0195 | 0.0342 | 0.9463 | 0.9442 | 0.5726 | errors 0 | pass |
| 20×20×3 | all-one | normal | Phi | true components / percentile | 0.5556 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.5913 | errors 0 | fail |
| 20×20×3 | all-one | normal | Phi | true components / basic | 0.5556 | 1.000 | 0.6547 (0.0109) | 0.1426 | 0.2026 | 0.6547 | 0.6474 | 0.4921 | errors 0 | fail |
| 20×20×3 | all-one | normal | Phi | true components / bias-corrected | 0.5556 | 0.997 | 0.6911 (0.0106) | 0.1511 | 0.1547 | 0.6932 | 0.6947 | 0.4759 | errors 0 | fail |
| 50×8×5 | all-one | normal | Phi | zeroed estimate / percentile | 0.5882 | 1.000 | 0.9505 (0.0050) | 0.0221 | 0.0274 | 0.9505 | 0.9489 | 0.3453 | errors 0 | pass |
| 50×8×5 | all-one | normal | Phi | zeroed estimate / basic | 0.5882 | 1.000 | 0.8732 (0.0076) | 0.0226 | 0.1042 | 0.8732 | 0.8674 | 0.3453 | errors 0 | fail |
| 50×8×5 | all-one | normal | Phi | zeroed estimate / bias-corrected | 0.5882 | 1.000 | 0.9432 (0.0053) | 0.0253 | 0.0316 | 0.9432 | 0.9416 | 0.3460 | errors 0 | pass |
| 50×8×5 | all-one | normal | Phi | true components / percentile | 0.5882 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.3487 | errors 0 | fail |
| 50×8×5 | all-one | normal | Phi | true components / basic | 0.5882 | 1.000 | 0.6389 (0.0110) | 0.1547 | 0.2063 | 0.6389 | 0.6358 | 0.3408 | errors 0 | fail |
| 50×8×5 | all-one | normal | Phi | true components / bias-corrected | 0.5882 | 0.998 | 0.6584 (0.0109) | 0.1589 | 0.1805 | 0.6598 | 0.6589 | 0.2802 | errors 0 | fail |
