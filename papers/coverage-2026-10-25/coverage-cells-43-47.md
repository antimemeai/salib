# G-theory coverage study (full), cells 43..90 of 90

n_sim = 1900, B = 999, nominal 0.95. Pass: coverage in [.94, .96] and each tail miss in [.015, .035]; a None interval counts as a miss. Lower miss: the interval lies entirely below the truth; upper miss: entirely above. Conditional coverage counts available intervals only; bias-eliminated coverage targets the mean point estimate.

| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 50×8×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9368 (0.0056) | 0.0558 | 0.0074 | 0.9368 | 0.9368 | 0.9061 | facet-SE ratio 0.994 | fail |
| 50×8×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9616 (0.0044) | 0.0142 | 0.0242 | 0.9616 | 0.9621 | 1.0127 |  | fail |
| 50×8×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8811 (0.0074) | 0.1189 | 0.0000 | 0.8811 | 0.8768 | 2.1106 | facet-SE ratio 0.960 | fail |
| 50×8×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0158 | 0.0263 | 0.9579 | 0.9595 | 3.9625 |  | pass |
| 50×8×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8474 (0.0083) | 0.1526 | 0.0000 | 0.8474 | 0.8653 | 0.0370 | facet-SE ratio 0.929 | fail |
| 50×8×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.878 | 0.8316 (0.0086) | 0.0000 | 0.0468 | 0.9467 | 0.8258 | 2667485801.0498 |  | fail |
| 50×8×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0374 | 0.0163 | 0.9463 | 0.9463 | 0.3584 | facet-SE ratio 1.003 | fail |
| 50×8×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0237 | 0.0242 | 0.9521 | 0.9516 | 0.3658 |  | pass |
| 50×8×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9847 (0.0028) | 0.0000 | 0.0153 | 0.9847 | 0.9842 | 0.0570 | facet-SE ratio 1.012 | fail |
| 50×8×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.758 | 0.6084 (0.0112) | 0.0000 | 0.1495 | 0.8028 | 0.6021 | 21477629190.4863 |  | fail |
| 50×8×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9932 (0.0019) | 0.0005 | 0.0063 | 0.9932 | 0.9937 | 0.0316 | facet-SE ratio 0.927 | fail |
| 50×8×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.899 | 0.8100 (0.0090) | 0.0000 | 0.0895 | 0.9005 | 0.8121 | 4728138448.9501 |  | fail |
| 50×8×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0358 | 0.0195 | 0.9447 | 0.9432 | 0.1489 | facet-SE ratio 0.994 | fail |
| 50×8×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0295 | 0.0237 | 0.9468 | 0.9463 | 0.1500 |  | pass |
| 50×8×5 | near-zero-rater | normal | G | F interval | 0.8681 | 1.000 | 0.9568 (0.0047) | 0.0221 | 0.0211 | 0.9568 | 0.9484 | 0.1171 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×5 | near-zero-rater | normal | Phi | parametric | 0.7817 | 1.000 | 0.9516 (0.0049) | 0.0189 | 0.0295 | 0.9516 | 0.9505 | 0.2277 |  | pass |
| 50×8×5 | near-zero-rater | normal | Phi | parametric, attached | 0.7817 | 0.597 | 0.5689 (0.0114) | 0.0105 | 0.0179 | 0.9524 | 0.5674 | 0.2281 |  | fail |
| 50×8×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9442 (0.0053) | 0.0447 | 0.0111 | 0.9442 | 0.9421 | 3.2991 | facet-SE ratio 0.975 | fail |
| 50×8×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9426 (0.0053) | 0.0226 | 0.0347 | 0.9426 | 0.9426 | 3.6145 |  | pass |
| 50×8×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8800 (0.0075) | 0.1200 | 0.0000 | 0.8800 | 0.8721 | 1.2906 | facet-SE ratio 0.944 | fail |
| 50×8×5 | high-g | normal | i | Satterthwaite | 0.5000 | 0.989 | 0.9389 (0.0055) | 0.0000 | 0.0500 | 0.9494 | 0.9458 | 12575898935.8909 |  | fail |
| 50×8×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8326 (0.0086) | 0.1674 | 0.0000 | 0.8326 | 0.8368 | 1.5168 | facet-SE ratio 0.932 | fail |
| 50×8×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.968 | 0.9347 (0.0057) | 0.0000 | 0.0332 | 0.9657 | 0.9316 | 17324213428.5854 |  | fail |
| 50×8×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0342 | 0.0137 | 0.9521 | 0.9516 | 0.1791 | facet-SE ratio 1.023 | fail |
| 50×8×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0211 | 0.0221 | 0.9568 | 0.9579 | 0.1827 |  | pass |
| 50×8×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9411 (0.0054) | 0.0400 | 0.0189 | 0.9411 | 0.9400 | 0.2214 | facet-SE ratio 0.984 | fail |
| 50×8×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0242 | 0.0305 | 0.9453 | 0.9442 | 0.2281 |  | pass |
| 50×8×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9284 (0.0059) | 0.0637 | 0.0079 | 0.9284 | 0.9279 | 0.5336 | facet-SE ratio 0.979 | fail |
| 50×8×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0200 | 0.0300 | 0.9500 | 0.9463 | 0.6218 |  | pass |
| 50×8×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0337 | 0.0163 | 0.9500 | 0.9500 | 0.0744 | facet-SE ratio 1.027 | pass |
| 50×8×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0305 | 0.0168 | 0.9526 | 0.9537 | 0.0750 |  | pass |
| 50×8×5 | high-g | normal | G | F interval | 0.9581 | 1.000 | 0.9468 (0.0051) | 0.0216 | 0.0316 | 0.9468 | 0.9426 | 0.0367 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×5 | high-g | normal | Phi | parametric | 0.9195 | 1.000 | 0.9474 (0.0051) | 0.0121 | 0.0405 | 0.9474 | 0.9442 | 0.1046 |  | fail |
| 50×8×5 | high-g | normal | Phi | parametric, attached | 0.9195 | 0.957 | 0.9126 (0.0065) | 0.0121 | 0.0326 | 0.9533 | 0.9105 | 0.1060 |  | fail |
| 50×20×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0389 | 0.0105 | 0.9505 | 0.9479 | 1.3226 | facet-SE ratio 1.013 | fail |
| 50×20×2 | all-one | normal | p | Satterthwaite | 1.0000 | 0.999 | 0.9511 (0.0049) | 0.0000 | 0.0484 | 0.9516 | 0.9579 | 5942010197.2984 |  | fail |
| 50×20×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9274 (0.0060) | 0.0663 | 0.0063 | 0.9274 | 0.9274 | 2.0623 | facet-SE ratio 0.961 | fail |
| 50×20×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.981 | 0.9321 (0.0058) | 0.0000 | 0.0484 | 0.9506 | 0.9321 | 64007173529.1816 |  | fail |
| 50×20×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.6279 (0.0111) | 0.3721 | 0.0000 | 0.6279 | 0.6416 | 5.1741 | facet-SE ratio 1.181 | fail |
| 50×20×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.789 | 0.7547 (0.0099) | 0.0000 | 0.0347 | 0.9560 | 0.7505 | 231323771012.5508 |  | fail |
| 50×20×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0321 | 0.0226 | 0.9453 | 0.9432 | 0.2860 | facet-SE ratio 0.968 | pass |
| 50×20×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0211 | 0.0332 | 0.9458 | 0.9458 | 0.2897 |  | pass |
| 50×20×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9295 (0.0059) | 0.0589 | 0.0116 | 0.9295 | 0.9305 | 0.8235 | facet-SE ratio 0.997 | fail |
| 50×20×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0211 | 0.0237 | 0.9553 | 0.9553 | 0.9032 |  | pass |
| 50×20×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9189 (0.0063) | 0.0768 | 0.0042 | 0.9189 | 0.9195 | 1.2795 | facet-SE ratio 0.950 | fail |
| 50×20×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0279 | 0.0337 | 0.9384 | 0.9379 | 1.5983 |  | fail |
| 50×20×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0337 | 0.0211 | 0.9453 | 0.9463 | 0.1810 | facet-SE ratio 0.994 | pass |
| 50×20×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0268 | 0.0226 | 0.9505 | 0.9500 | 0.1826 |  | pass |
| 50×20×2 | all-one | normal | G | F interval | 0.6349 | 1.000 | 0.9479 (0.0051) | 0.0226 | 0.0295 | 0.9479 | 0.9468 | 0.4183 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×2 | all-one | normal | Phi | parametric | 0.4651 | 1.000 | 0.9237 (0.0061) | 0.0042 | 0.0721 | 0.9237 | 0.9426 | 0.4949 |  | fail |
| 50×20×2 | all-one | normal | Phi | parametric, attached | 0.4651 | 0.777 | 0.7389 (0.0101) | 0.0032 | 0.0353 | 0.9506 | 0.7479 | 0.5138 |  | fail |
| 50×20×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9400 (0.0054) | 0.0511 | 0.0089 | 0.9400 | 0.9400 | 0.8474 | facet-SE ratio 1.013 | fail |
| 50×20×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0174 | 0.0284 | 0.9542 | 0.9553 | 0.9353 |  | pass |
| 50×20×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9216 (0.0062) | 0.0768 | 0.0016 | 0.9216 | 0.9247 | 1.2885 | facet-SE ratio 0.986 | fail |
| 50×20×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0216 | 0.0237 | 0.9547 | 0.9547 | 1.6212 |  | pass |
| 50×20×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.6889 (0.0106) | 0.3111 | 0.0000 | 0.6889 | 0.6805 | 0.0626 | facet-SE ratio 1.172 | fail |
| 50×20×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.709 | 0.6626 (0.0108) | 0.0000 | 0.0468 | 0.9340 | 0.6637 | 7044772892.4711 |  | fail |
| 50×20×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9411 (0.0054) | 0.0342 | 0.0247 | 0.9411 | 0.9432 | 0.2855 | facet-SE ratio 0.984 | pass |
| 50×20×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0195 | 0.0316 | 0.9489 | 0.9495 | 0.2898 |  | pass |
| 50×20×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9895 (0.0023) | 0.0000 | 0.0105 | 0.9895 | 0.9900 | 0.0492 | facet-SE ratio 0.971 | fail |
| 50×20×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.786 | 0.6458 (0.0110) | 0.0000 | 0.1405 | 0.8213 | 0.6542 | 12021799553.4838 |  | fail |
| 50×20×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9968 (0.0013) | 0.0000 | 0.0032 | 0.9968 | 0.9968 | 0.0382 | facet-SE ratio 0.981 | fail |
| 50×20×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.851 | 0.7611 (0.0098) | 0.0000 | 0.0895 | 0.8948 | 0.7568 | 5919197916.1972 |  | fail |
| 50×20×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0279 | 0.0184 | 0.9537 | 0.9521 | 0.1805 | facet-SE ratio 1.022 | pass |
| 50×20×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0226 | 0.0242 | 0.9532 | 0.9547 | 0.1822 |  | pass |
| 50×20×2 | near-zero-rater | normal | G | F interval | 0.9259 | 1.000 | 0.9511 (0.0049) | 0.0226 | 0.0263 | 0.9511 | 0.9421 | 0.0675 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×2 | near-zero-rater | normal | Phi | parametric | 0.8809 | 1.000 | 0.9563 (0.0047) | 0.0321 | 0.0116 | 0.9563 | 0.9558 | 0.1204 |  | fail |
| 50×20×2 | near-zero-rater | normal | Phi | parametric, attached | 0.8809 | 0.463 | 0.4416 (0.0114) | 0.0163 | 0.0047 | 0.9545 | 0.4437 | 0.1246 |  | fail |
| 50×20×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9337 (0.0057) | 0.0621 | 0.0042 | 0.9337 | 0.9342 | 3.3630 | facet-SE ratio 0.981 | fail |
| 50×20×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9426 (0.0053) | 0.0295 | 0.0279 | 0.9426 | 0.9416 | 3.7068 |  | pass |
| 50×20×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9205 (0.0062) | 0.0732 | 0.0063 | 0.9205 | 0.9232 | 1.0186 | facet-SE ratio 0.966 | fail |
| 50×20×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.987 | 0.9400 (0.0054) | 0.0000 | 0.0474 | 0.9520 | 0.9347 | 40161359534.3016 |  | fail |
| 50×20×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.6689 (0.0108) | 0.3311 | 0.0000 | 0.6689 | 0.6689 | 2.7014 | facet-SE ratio 1.207 | fail |
| 50×20×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.806 | 0.7721 (0.0096) | 0.0000 | 0.0337 | 0.9582 | 0.7721 | 87030234010.7454 |  | fail |
| 50×20×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0347 | 0.0153 | 0.9500 | 0.9511 | 0.1430 | facet-SE ratio 1.006 | pass |
| 50×20×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0195 | 0.0247 | 0.9558 | 0.9558 | 0.1450 |  | pass |
| 50×20×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9411 (0.0054) | 0.0489 | 0.0100 | 0.9411 | 0.9426 | 0.4103 | facet-SE ratio 0.992 | fail |
| 50×20×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0237 | 0.0205 | 0.9558 | 0.9574 | 0.4498 |  | pass |
| 50×20×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9137 (0.0064) | 0.0837 | 0.0026 | 0.9137 | 0.9132 | 0.6409 | facet-SE ratio 0.977 | fail |
| 50×20×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0232 | 0.0237 | 0.9532 | 0.9537 | 0.8021 |  | pass |
| 50×20×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0279 | 0.0268 | 0.9453 | 0.9442 | 0.0905 | facet-SE ratio 0.975 | pass |
| 50×20×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0205 | 0.0316 | 0.9479 | 0.9484 | 0.0913 |  | pass |
| 50×20×2 | high-g | normal | G | F interval | 0.9329 | 1.000 | 0.9489 (0.0050) | 0.0253 | 0.0258 | 0.9489 | 0.9468 | 0.0790 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×2 | high-g | normal | Phi | parametric | 0.8743 | 1.000 | 0.8547 (0.0081) | 0.0000 | 0.1453 | 0.8547 | 0.8553 | 0.2236 |  | fail |
| 50×20×2 | high-g | normal | Phi | parametric, attached | 0.8743 | 0.797 | 0.7358 (0.0101) | 0.0000 | 0.0611 | 0.9234 | 0.7358 | 0.2563 |  | fail |
