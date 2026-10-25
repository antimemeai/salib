# Fiducial GPQ intervals, cells 0..90 of 90

n_sim = 1900, draws = 9999, nominal 0.95. Pass: coverage in [.94, .96] and each tail miss in [.015, .035]. Lower miss: the interval lies entirely below the truth; upper miss: entirely above.

| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 20×5×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0226 | 0.0200 | 0.9574 | 0.9574 | 2.8616 | errors 0 | pass |
| 20×5×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0316 | 0.0142 | 0.9542 | 0.9521 | 12.1153 | errors 0 | fail |
| 20×5×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0226 | 0.0200 | 0.9574 | 0.9626 | 1318.7742 | errors 0 | pass |
| 20×5×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9458 | 1.0627 | errors 0 | pass |
| 20×5×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0263 | 0.0184 | 0.9553 | 0.9542 | 1.8474 | errors 0 | pass |
| 20×5×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0263 | 0.0279 | 0.9458 | 0.9468 | 8.4818 | errors 0 | pass |
| 20×5×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0226 | 0.0305 | 0.9468 | 0.9474 | 0.6698 | errors 0 | pass |
| 20×5×2 | all-one | normal | G | fiducial GPQ | 0.5556 | 1.000 | 0.9579 (0.0046) | 0.0221 | 0.0200 | 0.9579 | 0.9463 | 0.6622 | errors 0 | pass |
| 20×5×2 | all-one | normal | Phi | fiducial GPQ | 0.3846 | 1.000 | 0.9442 (0.0053) | 0.0558 | 0.0000 | 0.9442 | 0.9453 | 0.6301 | errors 0 | fail |
| 20×5×2 | all-one | normal | G | fiducial GPQ, signed | 0.5556 | 1.000 | 0.9579 (0.0046) | 0.0221 | 0.0200 | 0.9579 | 0.9463 | 0.6622 | errors 0 | pass |
| 20×5×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.3846 | 1.000 | 0.9563 (0.0047) | 0.0437 | 0.0000 | 0.9563 | 0.9563 | 0.6991 | errors 0 | fail |
| 20×5×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0263 | 0.0300 | 0.9437 | 0.9437 | 2.0465 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0232 | 0.0237 | 0.9532 | 0.9526 | 8.4581 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9547 (0.0048) | 0.0279 | 0.0174 | 0.9547 | 0.9663 | 23.2248 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9363 (0.0056) | 0.0279 | 0.0358 | 0.9363 | 0.9368 | 1.0686 | errors 0 | fail |
| 20×5×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9579 (0.0046) | 0.0174 | 0.0247 | 0.9579 | 0.9474 | 0.2463 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9495 (0.0050) | 0.0237 | 0.0268 | 0.9495 | 0.9537 | 0.4488 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0216 | 0.0300 | 0.9484 | 0.9463 | 0.6641 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | G | fiducial GPQ | 0.7663 | 1.000 | 0.9511 (0.0049) | 0.0305 | 0.0184 | 0.9511 | 0.9421 | 0.4140 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6618 | 1.000 | 0.9505 (0.0050) | 0.0495 | 0.0000 | 0.9505 | 0.9668 | 0.6825 | errors 0 | fail |
| 20×5×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.7663 | 1.000 | 0.9542 (0.0048) | 0.0232 | 0.0226 | 0.9542 | 0.9342 | 0.4173 | errors 0 | pass |
| 20×5×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6618 | 1.000 | 0.9589 (0.0046) | 0.0411 | 0.0000 | 0.9589 | 0.9726 | 0.6935 | errors 0 | fail |
| 20×5×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9563 (0.0047) | 0.0216 | 0.0221 | 0.9563 | 0.9568 | 6.8950 | errors 0 | pass |
| 20×5×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9584 (0.0046) | 0.0289 | 0.0126 | 0.9584 | 0.9563 | 5.9707 | errors 0 | fail |
| 20×5×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9584 (0.0046) | 0.0232 | 0.0184 | 0.9584 | 0.9579 | 623.3836 | errors 0 | pass |
| 20×5×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0232 | 0.0232 | 0.9537 | 0.9547 | 0.5323 | errors 0 | pass |
| 20×5×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0216 | 0.0332 | 0.9453 | 0.9458 | 0.9446 | errors 0 | pass |
| 20×5×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0226 | 0.0216 | 0.9558 | 0.9558 | 4.1460 | errors 0 | pass |
| 20×5×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9584 (0.0046) | 0.0200 | 0.0216 | 0.9584 | 0.9584 | 0.3334 | errors 0 | pass |
| 20×5×2 | high-g | normal | G | fiducial GPQ | 0.9091 | 1.000 | 0.9526 (0.0049) | 0.0242 | 0.0232 | 0.9526 | 0.9479 | 0.1883 | errors 0 | pass |
| 20×5×2 | high-g | normal | Phi | fiducial GPQ | 0.8333 | 1.000 | 0.9563 (0.0047) | 0.0437 | 0.0000 | 0.9563 | 0.9674 | 0.8106 | errors 0 | fail |
| 20×5×2 | high-g | normal | G | fiducial GPQ, signed | 0.9091 | 1.000 | 0.9526 (0.0049) | 0.0242 | 0.0232 | 0.9526 | 0.9479 | 0.1883 | errors 0 | pass |
| 20×5×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8333 | 1.000 | 0.9716 (0.0038) | 0.0274 | 0.0011 | 0.9716 | 0.9753 | 0.8328 | errors 0 | fail |
| 20×5×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0247 | 0.0232 | 0.9521 | 0.9526 | 2.5435 | errors 0 | pass |
| 20×5×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0379 | 0.0163 | 0.9458 | 0.9463 | 10.5846 | errors 0 | fail |
| 20×5×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0268 | 0.0205 | 0.9526 | 0.9563 | 50.2519 | errors 0 | pass |
| 20×5×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0253 | 0.0216 | 0.9532 | 0.9532 | 0.9047 | errors 0 | pass |
| 20×5×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0232 | 0.0216 | 0.9553 | 0.9547 | 1.1901 | errors 0 | pass |
| 20×5×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0221 | 0.0247 | 0.9532 | 0.9521 | 3.3980 | errors 0 | pass |
| 20×5×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0189 | 0.0279 | 0.9532 | 0.9537 | 0.4619 | errors 0 | pass |
| 20×5×3 | all-one | normal | G | fiducial GPQ | 0.6250 | 1.000 | 0.9568 (0.0047) | 0.0237 | 0.0195 | 0.9568 | 0.9479 | 0.5618 | errors 0 | pass |
| 20×5×3 | all-one | normal | Phi | fiducial GPQ | 0.4545 | 1.000 | 0.9511 (0.0049) | 0.0484 | 0.0005 | 0.9511 | 0.9547 | 0.6230 | errors 0 | fail |
| 20×5×3 | all-one | normal | G | fiducial GPQ, signed | 0.6250 | 1.000 | 0.9568 (0.0047) | 0.0237 | 0.0195 | 0.9568 | 0.9479 | 0.5618 | errors 0 | pass |
| 20×5×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.4545 | 1.000 | 0.9537 (0.0048) | 0.0453 | 0.0011 | 0.9537 | 0.9574 | 0.6359 | errors 0 | fail |
| 20×5×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0253 | 0.0221 | 0.9526 | 0.9532 | 2.0026 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0295 | 0.0242 | 0.9463 | 0.9463 | 8.3150 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9563 (0.0047) | 0.0226 | 0.0211 | 0.9563 | 0.9626 | 0.8809 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0242 | 0.0226 | 0.9532 | 0.9516 | 0.9076 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9516 (0.0049) | 0.0284 | 0.0200 | 0.9516 | 0.9416 | 0.1521 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9579 (0.0046) | 0.0221 | 0.0200 | 0.9579 | 0.9563 | 0.1668 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0253 | 0.0232 | 0.9516 | 0.9532 | 0.4596 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | G | fiducial GPQ | 0.7874 | 1.000 | 0.9495 (0.0050) | 0.0311 | 0.0195 | 0.9495 | 0.9353 | 0.3545 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6784 | 1.000 | 0.9568 (0.0047) | 0.0384 | 0.0047 | 0.9568 | 0.9674 | 0.5396 | errors 0 | fail |
| 20×5×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.7874 | 1.000 | 0.9463 (0.0052) | 0.0284 | 0.0253 | 0.9463 | 0.9332 | 0.3540 | errors 0 | pass |
| 20×5×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6784 | 1.000 | 0.9584 (0.0046) | 0.0363 | 0.0053 | 0.9584 | 0.9684 | 0.5421 | errors 0 | fail |
| 20×5×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9479 (0.0051) | 0.0263 | 0.0258 | 0.9479 | 0.9474 | 6.7673 | errors 0 | pass |
| 20×5×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0337 | 0.0174 | 0.9489 | 0.9468 | 5.2242 | errors 0 | pass |
| 20×5×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0284 | 0.0263 | 0.9453 | 0.9463 | 24.7818 | errors 0 | pass |
| 20×5×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9595 (0.0045) | 0.0216 | 0.0189 | 0.9595 | 0.9595 | 0.4490 | errors 0 | pass |
| 20×5×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0221 | 0.0258 | 0.9521 | 0.9521 | 0.5953 | errors 0 | pass |
| 20×5×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0226 | 0.0274 | 0.9500 | 0.9500 | 1.6827 | errors 0 | pass |
| 20×5×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9526 | 0.2294 | errors 0 | pass |
| 20×5×3 | high-g | normal | G | fiducial GPQ | 0.9302 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9353 | 0.1169 | errors 0 | pass |
| 20×5×3 | high-g | normal | Phi | fiducial GPQ | 0.8696 | 1.000 | 0.9558 (0.0047) | 0.0442 | 0.0000 | 0.9558 | 0.9716 | 0.5487 | errors 0 | fail |
| 20×5×3 | high-g | normal | G | fiducial GPQ, signed | 0.9302 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9353 | 0.1169 | errors 0 | pass |
| 20×5×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.8696 | 1.000 | 0.9605 (0.0045) | 0.0384 | 0.0011 | 0.9605 | 0.9747 | 0.5518 | errors 0 | fail |
| 20×5×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0284 | 0.0268 | 0.9447 | 0.9458 | 2.2365 | errors 0 | pass |
| 20×5×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0232 | 0.0232 | 0.9537 | 0.9558 | 9.9172 | errors 0 | pass |
| 20×5×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0237 | 0.0221 | 0.9542 | 0.9553 | 10.0349 | errors 0 | pass |
| 20×5×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0274 | 0.0247 | 0.9479 | 0.9484 | 0.8017 | errors 0 | pass |
| 20×5×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0305 | 0.0174 | 0.9521 | 0.9553 | 0.7989 | errors 0 | pass |
| 20×5×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0274 | 0.0258 | 0.9468 | 0.9479 | 1.8741 | errors 0 | pass |
| 20×5×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0232 | 0.0326 | 0.9442 | 0.9432 | 0.3232 | errors 0 | pass |
| 20×5×5 | all-one | normal | G | fiducial GPQ | 0.6944 | 1.000 | 0.9468 (0.0051) | 0.0268 | 0.0263 | 0.9468 | 0.9247 | 0.4620 | errors 0 | pass |
| 20×5×5 | all-one | normal | Phi | fiducial GPQ | 0.5319 | 1.000 | 0.9563 (0.0047) | 0.0421 | 0.0016 | 0.9563 | 0.9626 | 0.5653 | errors 0 | fail |
| 20×5×5 | all-one | normal | G | fiducial GPQ, signed | 0.6944 | 1.000 | 0.9468 (0.0051) | 0.0268 | 0.0263 | 0.9468 | 0.9247 | 0.4620 | errors 0 | pass |
| 20×5×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.5319 | 1.000 | 0.9568 (0.0047) | 0.0416 | 0.0016 | 0.9568 | 0.9626 | 0.5673 | errors 0 | fail |
| 20×5×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0221 | 0.0268 | 0.9511 | 0.9511 | 1.9451 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0211 | 0.0268 | 0.9521 | 0.9526 | 8.3966 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9542 (0.0048) | 0.0295 | 0.0163 | 0.9542 | 0.9547 | 0.1659 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0211 | 0.0274 | 0.9516 | 0.9500 | 0.8088 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9468 (0.0051) | 0.0295 | 0.0237 | 0.9468 | 0.9416 | 0.1008 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9542 (0.0048) | 0.0247 | 0.0211 | 0.9542 | 0.9500 | 0.0881 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0316 | 0.0258 | 0.9426 | 0.9453 | 0.3214 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | G | fiducial GPQ | 0.8052 | 1.000 | 0.9505 (0.0050) | 0.0258 | 0.0237 | 0.9505 | 0.9358 | 0.3216 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6923 | 1.000 | 0.9537 (0.0048) | 0.0353 | 0.0111 | 0.9537 | 0.9568 | 0.5173 | errors 0 | fail |
| 20×5×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8052 | 1.000 | 0.9511 (0.0049) | 0.0242 | 0.0247 | 0.9511 | 0.9311 | 0.3209 | errors 0 | pass |
| 20×5×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6923 | 1.000 | 0.9542 (0.0048) | 0.0342 | 0.0116 | 0.9542 | 0.9579 | 0.5181 | errors 0 | fail |
| 20×5×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9437 (0.0053) | 0.0258 | 0.0305 | 0.9437 | 0.9437 | 6.5490 | errors 0 | pass |
| 20×5×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9632 (0.0043) | 0.0216 | 0.0153 | 0.9632 | 0.9637 | 4.9821 | errors 0 | fail |
| 20×5×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9421 (0.0054) | 0.0353 | 0.0226 | 0.9421 | 0.9416 | 4.8984 | errors 0 | fail |
| 20×5×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0258 | 0.0242 | 0.9500 | 0.9484 | 0.4012 | errors 0 | pass |
| 20×5×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0258 | 0.0226 | 0.9516 | 0.9505 | 0.4042 | errors 0 | pass |
| 20×5×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0247 | 0.0284 | 0.9468 | 0.9474 | 0.9269 | errors 0 | pass |
| 20×5×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0226 | 0.0247 | 0.9526 | 0.9526 | 0.1609 | errors 0 | pass |
| 20×5×5 | high-g | normal | G | fiducial GPQ | 0.9479 | 1.000 | 0.9479 (0.0051) | 0.0274 | 0.0247 | 0.9479 | 0.9316 | 0.0822 | errors 0 | pass |
| 20×5×5 | high-g | normal | Phi | fiducial GPQ | 0.9009 | 1.000 | 0.9489 (0.0050) | 0.0463 | 0.0047 | 0.9489 | 0.9637 | 0.3014 | errors 0 | fail |
| 20×5×5 | high-g | normal | G | fiducial GPQ, signed | 0.9479 | 1.000 | 0.9479 (0.0051) | 0.0274 | 0.0247 | 0.9479 | 0.9316 | 0.0822 | errors 0 | pass |
| 20×5×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9009 | 1.000 | 0.9495 (0.0050) | 0.0458 | 0.0047 | 0.9495 | 0.9653 | 0.3016 | errors 0 | fail |
| 20×8×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0363 | 0.0200 | 0.9437 | 0.9432 | 2.6551 | errors 0 | fail |
| 20×8×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0247 | 0.0195 | 0.9558 | 0.9558 | 5.8001 | errors 0 | pass |
| 20×8×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0263 | 0.0147 | 0.9589 | 0.9589 | 1164.7910 | errors 0 | fail |
| 20×8×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0332 | 0.0247 | 0.9421 | 0.9437 | 0.7842 | errors 0 | pass |
| 20×8×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0232 | 0.0279 | 0.9489 | 0.9489 | 1.7497 | errors 0 | pass |
| 20×8×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0205 | 0.0284 | 0.9511 | 0.9511 | 3.8879 | errors 0 | pass |
| 20×8×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0221 | 0.0295 | 0.9484 | 0.9474 | 0.4946 | errors 0 | pass |
| 20×8×2 | all-one | normal | G | fiducial GPQ | 0.5926 | 1.000 | 0.9453 (0.0052) | 0.0295 | 0.0253 | 0.9453 | 0.9347 | 0.6548 | errors 0 | pass |
| 20×8×2 | all-one | normal | Phi | fiducial GPQ | 0.4211 | 1.000 | 0.9458 (0.0052) | 0.0526 | 0.0016 | 0.9458 | 0.9495 | 0.6615 | errors 0 | fail |
| 20×8×2 | all-one | normal | G | fiducial GPQ, signed | 0.5926 | 1.000 | 0.9453 (0.0052) | 0.0295 | 0.0253 | 0.9453 | 0.9347 | 0.6548 | errors 0 | pass |
| 20×8×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.4211 | 1.000 | 0.9521 (0.0049) | 0.0463 | 0.0016 | 0.9521 | 0.9547 | 0.6790 | errors 0 | fail |
| 20×8×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0258 | 0.0189 | 0.9553 | 0.9542 | 1.8473 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0263 | 0.0195 | 0.9542 | 0.9542 | 3.9574 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9558 (0.0047) | 0.0221 | 0.0221 | 0.9558 | 0.9589 | 19.0073 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0316 | 0.0211 | 0.9474 | 0.9458 | 0.7824 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9553 (0.0047) | 0.0216 | 0.0232 | 0.9553 | 0.9537 | 0.1653 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9447 (0.0052) | 0.0268 | 0.0284 | 0.9447 | 0.9437 | 0.1980 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0205 | 0.0211 | 0.9584 | 0.9600 | 0.4950 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | G | fiducial GPQ | 0.8386 | 1.000 | 0.9537 (0.0048) | 0.0358 | 0.0105 | 0.9537 | 0.9442 | 0.2796 | errors 0 | fail |
| 20×8×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7558 | 1.000 | 0.9479 (0.0051) | 0.0521 | 0.0000 | 0.9479 | 0.9684 | 0.6550 | errors 0 | fail |
| 20×8×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8386 | 1.000 | 0.9516 (0.0049) | 0.0311 | 0.0174 | 0.9516 | 0.9379 | 0.2798 | errors 0 | pass |
| 20×8×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7558 | 1.000 | 0.9521 (0.0049) | 0.0474 | 0.0005 | 0.9521 | 0.9732 | 0.6593 | errors 0 | fail |
| 20×8×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9500 (0.0050) | 0.0284 | 0.0216 | 0.9500 | 0.9489 | 6.7064 | errors 0 | pass |
| 20×8×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0326 | 0.0221 | 0.9453 | 0.9463 | 2.8855 | errors 0 | pass |
| 20×8×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0289 | 0.0263 | 0.9447 | 0.9484 | 620.4747 | errors 0 | pass |
| 20×8×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9416 (0.0054) | 0.0358 | 0.0226 | 0.9416 | 0.9421 | 0.3914 | errors 0 | fail |
| 20×8×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0263 | 0.0205 | 0.9532 | 0.9526 | 0.8670 | errors 0 | pass |
| 20×8×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0258 | 0.0205 | 0.9537 | 0.9521 | 1.9075 | errors 0 | pass |
| 20×8×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0237 | 0.0237 | 0.9526 | 0.9526 | 0.2477 | errors 0 | pass |
| 20×8×2 | high-g | normal | G | fiducial GPQ | 0.9209 | 1.000 | 0.9505 (0.0050) | 0.0268 | 0.0226 | 0.9505 | 0.9432 | 0.1695 | errors 0 | pass |
| 20×8×2 | high-g | normal | Phi | fiducial GPQ | 0.8533 | 1.000 | 0.9463 (0.0052) | 0.0521 | 0.0016 | 0.9463 | 0.9568 | 0.8136 | errors 0 | fail |
| 20×8×2 | high-g | normal | G | fiducial GPQ, signed | 0.9209 | 1.000 | 0.9505 (0.0050) | 0.0268 | 0.0226 | 0.9505 | 0.9432 | 0.1695 | errors 0 | pass |
| 20×8×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8533 | 1.000 | 0.9437 (0.0053) | 0.0505 | 0.0058 | 0.9437 | 0.9574 | 0.8177 | errors 0 | fail |
| 20×8×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9400 (0.0054) | 0.0374 | 0.0226 | 0.9400 | 0.9389 | 2.3491 | errors 0 | fail |
| 20×8×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0279 | 0.0247 | 0.9474 | 0.9474 | 5.0891 | errors 0 | pass |
| 20×8×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0232 | 0.0237 | 0.9532 | 0.9558 | 47.6937 | errors 0 | pass |
| 20×8×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0200 | 0.0232 | 0.9568 | 0.9553 | 0.6708 | errors 0 | pass |
| 20×8×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0274 | 0.0295 | 0.9432 | 0.9442 | 1.1253 | errors 0 | pass |
| 20×8×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0305 | 0.0247 | 0.9447 | 0.9442 | 2.0455 | errors 0 | pass |
| 20×8×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0216 | 0.0242 | 0.9542 | 0.9542 | 0.3446 | errors 0 | pass |
| 20×8×3 | all-one | normal | G | fiducial GPQ | 0.6667 | 1.000 | 0.9442 (0.0053) | 0.0326 | 0.0232 | 0.9442 | 0.9274 | 0.5314 | errors 0 | pass |
| 20×8×3 | all-one | normal | Phi | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0421 | 0.0042 | 0.9537 | 0.9595 | 0.6352 | errors 0 | fail |
| 20×8×3 | all-one | normal | G | fiducial GPQ, signed | 0.6667 | 1.000 | 0.9442 (0.0053) | 0.0326 | 0.0232 | 0.9442 | 0.9274 | 0.5314 | errors 0 | pass |
| 20×8×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0405 | 0.0058 | 0.9537 | 0.9589 | 0.6382 | errors 0 | fail |
| 20×8×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0253 | 0.0258 | 0.9489 | 0.9484 | 1.8205 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0263 | 0.0189 | 0.9547 | 0.9537 | 3.8768 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9584 | 0.7109 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0268 | 0.0184 | 0.9547 | 0.9547 | 0.6705 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9432 (0.0053) | 0.0242 | 0.0326 | 0.9432 | 0.9416 | 0.1013 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9537 (0.0048) | 0.0232 | 0.0232 | 0.9537 | 0.9558 | 0.0987 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0279 | 0.0200 | 0.9521 | 0.9526 | 0.3432 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | G | fiducial GPQ | 0.8547 | 1.000 | 0.9558 (0.0047) | 0.0268 | 0.0174 | 0.9558 | 0.9421 | 0.2324 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7700 | 1.000 | 0.9616 (0.0044) | 0.0347 | 0.0037 | 0.9616 | 0.9711 | 0.4017 | errors 0 | fail |
| 20×8×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8547 | 1.000 | 0.9521 (0.0049) | 0.0253 | 0.0226 | 0.9521 | 0.9342 | 0.2313 | errors 0 | pass |
| 20×8×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7700 | 1.000 | 0.9637 (0.0043) | 0.0321 | 0.0042 | 0.9637 | 0.9700 | 0.4021 | errors 0 | fail |
| 20×8×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9568 (0.0047) | 0.0221 | 0.0211 | 0.9568 | 0.9568 | 6.5864 | errors 0 | pass |
| 20×8×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0258 | 0.0174 | 0.9568 | 0.9568 | 2.5346 | errors 0 | pass |
| 20×8×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0216 | 0.0226 | 0.9558 | 0.9584 | 23.4778 | errors 0 | pass |
| 20×8×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9432 (0.0053) | 0.0316 | 0.0253 | 0.9432 | 0.9432 | 0.3355 | errors 0 | pass |
| 20×8×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0279 | 0.0253 | 0.9468 | 0.9474 | 0.5578 | errors 0 | pass |
| 20×8×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0200 | 0.0268 | 0.9532 | 0.9547 | 1.0345 | errors 0 | pass |
| 20×8×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0226 | 0.0263 | 0.9511 | 0.9500 | 0.1727 | errors 0 | pass |
| 20×8×3 | high-g | normal | G | fiducial GPQ | 0.9412 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9463 | 0.1009 | errors 0 | pass |
| 20×8×3 | high-g | normal | Phi | fiducial GPQ | 0.8889 | 1.000 | 0.9563 (0.0047) | 0.0400 | 0.0037 | 0.9563 | 0.9711 | 0.5201 | errors 0 | fail |
| 20×8×3 | high-g | normal | G | fiducial GPQ, signed | 0.9412 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9463 | 0.1009 | errors 0 | pass |
| 20×8×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.8889 | 1.000 | 0.9563 (0.0047) | 0.0389 | 0.0047 | 0.9563 | 0.9705 | 0.5206 | errors 0 | fail |
| 20×8×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0237 | 0.0258 | 0.9505 | 0.9479 | 2.0909 | errors 0 | pass |
| 20×8×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0326 | 0.0211 | 0.9463 | 0.9479 | 4.6630 | errors 0 | pass |
| 20×8×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0195 | 0.0195 | 0.9611 | 0.9611 | 9.0853 | errors 0 | fail |
| 20×8×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0253 | 0.0305 | 0.9442 | 0.9437 | 0.5965 | errors 0 | pass |
| 20×8×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0253 | 0.0268 | 0.9479 | 0.9474 | 0.7492 | errors 0 | pass |
| 20×8×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9595 (0.0045) | 0.0189 | 0.0216 | 0.9595 | 0.9589 | 1.2566 | errors 0 | pass |
| 20×8×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0168 | 0.0263 | 0.9568 | 0.9568 | 0.2423 | errors 0 | pass |
| 20×8×5 | all-one | normal | G | fiducial GPQ | 0.7407 | 1.000 | 0.9500 (0.0050) | 0.0232 | 0.0268 | 0.9500 | 0.9284 | 0.3973 | errors 0 | pass |
| 20×8×5 | all-one | normal | Phi | fiducial GPQ | 0.5882 | 1.000 | 0.9526 (0.0049) | 0.0416 | 0.0058 | 0.9526 | 0.9632 | 0.5313 | errors 0 | fail |
| 20×8×5 | all-one | normal | G | fiducial GPQ, signed | 0.7407 | 1.000 | 0.9500 (0.0050) | 0.0232 | 0.0268 | 0.9500 | 0.9284 | 0.3973 | errors 0 | pass |
| 20×8×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.5882 | 1.000 | 0.9526 (0.0049) | 0.0416 | 0.0058 | 0.9526 | 0.9626 | 0.5317 | errors 0 | fail |
| 20×8×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9332 (0.0057) | 0.0374 | 0.0295 | 0.9332 | 0.9332 | 1.7798 | errors 0 | fail |
| 20×8×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0289 | 0.0268 | 0.9442 | 0.9447 | 3.9728 | errors 0 | pass |
| 20×8×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9500 (0.0050) | 0.0316 | 0.0184 | 0.9500 | 0.9495 | 0.1356 | errors 0 | pass |
| 20×8×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0237 | 0.0232 | 0.9532 | 0.9542 | 0.5955 | errors 0 | pass |
| 20×8×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9453 (0.0052) | 0.0268 | 0.0279 | 0.9453 | 0.9468 | 0.0672 | errors 0 | pass |
| 20×8×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9505 (0.0050) | 0.0289 | 0.0205 | 0.9505 | 0.9495 | 0.0590 | errors 0 | pass |
| 20×8×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0211 | 0.0205 | 0.9584 | 0.9589 | 0.2416 | errors 0 | pass |
| 20×8×5 | near-zero-rater | normal | G | fiducial GPQ | 0.8681 | 1.000 | 0.9363 (0.0056) | 0.0374 | 0.0263 | 0.9363 | 0.9274 | 0.2090 | errors 0 | fail |
| 20×8×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7817 | 1.000 | 0.9458 (0.0052) | 0.0400 | 0.0142 | 0.9458 | 0.9505 | 0.3629 | errors 0 | fail |
| 20×8×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8681 | 1.000 | 0.9353 (0.0056) | 0.0368 | 0.0279 | 0.9353 | 0.9247 | 0.2083 | errors 0 | fail |
| 20×8×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7817 | 1.000 | 0.9463 (0.0052) | 0.0384 | 0.0153 | 0.9463 | 0.9500 | 0.3628 | errors 0 | fail |
| 20×8×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9458 (0.0052) | 0.0289 | 0.0253 | 0.9458 | 0.9458 | 6.4987 | errors 0 | pass |
| 20×8×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0279 | 0.0226 | 0.9495 | 0.9489 | 2.3243 | errors 0 | pass |
| 20×8×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0253 | 0.0253 | 0.9495 | 0.9500 | 4.6618 | errors 0 | pass |
| 20×8×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0221 | 0.0247 | 0.9532 | 0.9537 | 0.2982 | errors 0 | pass |
| 20×8×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0195 | 0.0295 | 0.9511 | 0.9526 | 0.3765 | errors 0 | pass |
| 20×8×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0279 | 0.0195 | 0.9526 | 0.9505 | 0.6247 | errors 0 | pass |
| 20×8×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0284 | 0.0211 | 0.9505 | 0.9516 | 0.1208 | errors 0 | pass |
| 20×8×5 | high-g | normal | G | fiducial GPQ | 0.9581 | 1.000 | 0.9526 (0.0049) | 0.0247 | 0.0226 | 0.9526 | 0.9358 | 0.0648 | errors 0 | pass |
| 20×8×5 | high-g | normal | Phi | fiducial GPQ | 0.9195 | 1.000 | 0.9611 (0.0044) | 0.0358 | 0.0032 | 0.9611 | 0.9721 | 0.2270 | errors 0 | fail |
| 20×8×5 | high-g | normal | G | fiducial GPQ, signed | 0.9581 | 1.000 | 0.9526 (0.0049) | 0.0247 | 0.0226 | 0.9526 | 0.9358 | 0.0648 | errors 0 | pass |
| 20×8×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9195 | 1.000 | 0.9616 (0.0044) | 0.0353 | 0.0032 | 0.9616 | 0.9721 | 0.2269 | errors 0 | fail |
| 20×20×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0247 | 0.0179 | 0.9574 | 0.9579 | 2.5393 | errors 0 | pass |
| 20×20×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0347 | 0.0195 | 0.9458 | 0.9463 | 2.5170 | errors 0 | pass |
| 20×20×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0253 | 0.0232 | 0.9516 | 0.9532 | 1136.0901 | errors 0 | pass |
| 20×20×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0263 | 0.0263 | 0.9474 | 0.9468 | 0.4670 | errors 0 | pass |
| 20×20×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0216 | 0.0300 | 0.9484 | 0.9489 | 1.6368 | errors 0 | pass |
| 20×20×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9358 (0.0056) | 0.0363 | 0.0279 | 0.9358 | 0.9363 | 1.6275 | errors 0 | fail |
| 20×20×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0258 | 0.0221 | 0.9521 | 0.9521 | 0.2943 | errors 0 | pass |
| 20×20×2 | all-one | normal | G | fiducial GPQ | 0.6349 | 1.000 | 0.9574 (0.0046) | 0.0195 | 0.0232 | 0.9574 | 0.9542 | 0.6521 | errors 0 | pass |
| 20×20×2 | all-one | normal | Phi | fiducial GPQ | 0.4651 | 1.000 | 0.9668 (0.0041) | 0.0311 | 0.0021 | 0.9668 | 0.9658 | 0.7096 | errors 0 | fail |
| 20×20×2 | all-one | normal | G | fiducial GPQ, signed | 0.6349 | 1.000 | 0.9574 (0.0046) | 0.0195 | 0.0232 | 0.9574 | 0.9542 | 0.6521 | errors 0 | pass |
| 20×20×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.4651 | 1.000 | 0.9668 (0.0041) | 0.0311 | 0.0021 | 0.9668 | 0.9658 | 0.7120 | errors 0 | fail |
| 20×20×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9484 | 1.6860 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9595 (0.0045) | 0.0226 | 0.0179 | 0.9595 | 0.9595 | 1.6835 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9521 (0.0049) | 0.0274 | 0.0205 | 0.9521 | 0.9537 | 13.3431 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0232 | 0.0216 | 0.9553 | 0.9542 | 0.4673 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9442 (0.0053) | 0.0263 | 0.0295 | 0.9442 | 0.9468 | 0.0774 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9458 (0.0052) | 0.0274 | 0.0268 | 0.9458 | 0.9479 | 0.0788 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0284 | 0.0279 | 0.9437 | 0.9437 | 0.2948 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | G | fiducial GPQ | 0.9259 | 1.000 | 0.9474 (0.0051) | 0.0379 | 0.0147 | 0.9474 | 0.9442 | 0.1239 | errors 0 | fail |
| 20×20×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8809 | 1.000 | 0.9474 (0.0051) | 0.0526 | 0.0000 | 0.9474 | 0.9668 | 0.6159 | errors 0 | fail |
| 20×20×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9259 | 1.000 | 0.9479 (0.0051) | 0.0321 | 0.0200 | 0.9479 | 0.9358 | 0.1235 | errors 0 | pass |
| 20×20×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8809 | 1.000 | 0.9516 (0.0049) | 0.0484 | 0.0000 | 0.9516 | 0.9689 | 0.6167 | errors 0 | fail |
| 20×20×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9553 (0.0047) | 0.0274 | 0.0174 | 0.9553 | 0.9553 | 6.6491 | errors 0 | pass |
| 20×20×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0268 | 0.0284 | 0.9447 | 0.9442 | 1.2773 | errors 0 | pass |
| 20×20×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9389 (0.0055) | 0.0305 | 0.0305 | 0.9389 | 0.9405 | 558.1421 | errors 0 | fail |
| 20×20×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9626 (0.0044) | 0.0163 | 0.0211 | 0.9626 | 0.9626 | 0.2331 | errors 0 | fail |
| 20×20×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9526 | 0.8197 | errors 0 | pass |
| 20×20×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9432 (0.0053) | 0.0295 | 0.0274 | 0.9432 | 0.9421 | 0.8133 | errors 0 | pass |
| 20×20×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9547 | 0.1469 | errors 0 | pass |
| 20×20×2 | high-g | normal | G | fiducial GPQ | 0.9329 | 1.000 | 0.9495 (0.0050) | 0.0274 | 0.0232 | 0.9495 | 0.9432 | 0.1526 | errors 0 | pass |
| 20×20×2 | high-g | normal | Phi | fiducial GPQ | 0.8743 | 1.000 | 0.9526 (0.0049) | 0.0405 | 0.0068 | 0.9526 | 0.9589 | 0.8072 | errors 0 | fail |
| 20×20×2 | high-g | normal | G | fiducial GPQ, signed | 0.9329 | 1.000 | 0.9495 (0.0050) | 0.0274 | 0.0232 | 0.9495 | 0.9432 | 0.1526 | errors 0 | pass |
| 20×20×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8743 | 1.000 | 0.9521 (0.0049) | 0.0405 | 0.0074 | 0.9521 | 0.9584 | 0.8074 | errors 0 | fail |
| 20×20×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0279 | 0.0305 | 0.9416 | 0.9421 | 2.2033 | errors 0 | pass |
| 20×20×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9458 | 2.1916 | errors 0 | pass |
| 20×20×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0226 | 0.0226 | 0.9547 | 0.9547 | 42.8498 | errors 0 | pass |
| 20×20×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0247 | 0.0274 | 0.9479 | 0.9479 | 0.3996 | errors 0 | pass |
| 20×20×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0247 | 0.0232 | 0.9521 | 0.9505 | 1.0403 | errors 0 | pass |
| 20×20×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0321 | 0.0263 | 0.9416 | 0.9421 | 1.0411 | errors 0 | pass |
| 20×20×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0195 | 0.0242 | 0.9563 | 0.9542 | 0.2075 | errors 0 | pass |
| 20×20×3 | all-one | normal | G | fiducial GPQ | 0.7143 | 1.000 | 0.9421 (0.0054) | 0.0284 | 0.0295 | 0.9421 | 0.9279 | 0.4877 | errors 0 | pass |
| 20×20×3 | all-one | normal | Phi | fiducial GPQ | 0.5556 | 1.000 | 0.9495 (0.0050) | 0.0421 | 0.0084 | 0.9495 | 0.9547 | 0.6635 | errors 0 | fail |
| 20×20×3 | all-one | normal | G | fiducial GPQ, signed | 0.7143 | 1.000 | 0.9421 (0.0054) | 0.0284 | 0.0295 | 0.9421 | 0.9279 | 0.4877 | errors 0 | pass |
| 20×20×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.5556 | 1.000 | 0.9495 (0.0050) | 0.0421 | 0.0084 | 0.9495 | 0.9542 | 0.6638 | errors 0 | fail |
| 20×20×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0237 | 0.0289 | 0.9474 | 0.9484 | 1.6703 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0263 | 0.0216 | 0.9521 | 0.9547 | 1.6354 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9542 (0.0048) | 0.0195 | 0.0263 | 0.9542 | 0.9568 | 0.5370 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0237 | 0.0242 | 0.9521 | 0.9511 | 0.3998 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9426 (0.0053) | 0.0316 | 0.0258 | 0.9426 | 0.9463 | 0.0490 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9558 (0.0047) | 0.0200 | 0.0242 | 0.9558 | 0.9532 | 0.0489 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0242 | 0.0279 | 0.9479 | 0.9479 | 0.2073 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | G | fiducial GPQ | 0.9346 | 1.000 | 0.9484 (0.0051) | 0.0263 | 0.0253 | 0.9484 | 0.9305 | 0.0996 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8901 | 1.000 | 0.9505 (0.0050) | 0.0411 | 0.0084 | 0.9505 | 0.9632 | 0.2257 | errors 0 | fail |
| 20×20×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9346 | 1.000 | 0.9489 (0.0050) | 0.0242 | 0.0268 | 0.9489 | 0.9279 | 0.0991 | errors 0 | pass |
| 20×20×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8901 | 1.000 | 0.9516 (0.0049) | 0.0389 | 0.0095 | 0.9516 | 0.9621 | 0.2254 | errors 0 | fail |
| 20×20×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9437 (0.0053) | 0.0284 | 0.0279 | 0.9437 | 0.9437 | 6.5074 | errors 0 | pass |
| 20×20×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0295 | 0.0258 | 0.9447 | 0.9426 | 1.0975 | errors 0 | pass |
| 20×20×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0295 | 0.0258 | 0.9447 | 0.9426 | 20.7602 | errors 0 | pass |
| 20×20×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0232 | 0.0274 | 0.9495 | 0.9495 | 0.1995 | errors 0 | pass |
| 20×20×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0253 | 0.0268 | 0.9479 | 0.9474 | 0.5194 | errors 0 | pass |
| 20×20×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0274 | 0.0284 | 0.9442 | 0.9447 | 0.5239 | errors 0 | pass |
| 20×20×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9458 (0.0052) | 0.0258 | 0.0284 | 0.9458 | 0.9468 | 0.1036 | errors 0 | pass |
| 20×20×3 | high-g | normal | G | fiducial GPQ | 0.9524 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9279 | 0.0852 | errors 0 | pass |
| 20×20×3 | high-g | normal | Phi | fiducial GPQ | 0.9091 | 1.000 | 0.9521 (0.0049) | 0.0384 | 0.0095 | 0.9521 | 0.9584 | 0.5016 | errors 0 | fail |
| 20×20×3 | high-g | normal | G | fiducial GPQ, signed | 0.9524 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9279 | 0.0852 | errors 0 | pass |
| 20×20×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.9091 | 1.000 | 0.9511 (0.0049) | 0.0384 | 0.0105 | 0.9511 | 0.9568 | 0.5015 | errors 0 | fail |
| 20×20×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0237 | 0.0179 | 0.9584 | 0.9579 | 1.9738 | errors 0 | pass |
| 20×20×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9489 | 1.9648 | errors 0 | pass |
| 20×20×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0247 | 0.0253 | 0.9500 | 0.9505 | 8.7309 | errors 0 | pass |
| 20×20×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0258 | 0.0211 | 0.9532 | 0.9532 | 0.3548 | errors 0 | pass |
| 20×20×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0316 | 0.0253 | 0.9432 | 0.9416 | 0.6967 | errors 0 | pass |
| 20×20×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0211 | 0.0279 | 0.9511 | 0.9505 | 0.7028 | errors 0 | pass |
| 20×20×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9521 | 0.1460 | errors 0 | pass |
| 20×20×5 | all-one | normal | G | fiducial GPQ | 0.7937 | 1.000 | 0.9521 (0.0049) | 0.0263 | 0.0216 | 0.9521 | 0.9384 | 0.3193 | errors 0 | pass |
| 20×20×5 | all-one | normal | Phi | fiducial GPQ | 0.6579 | 1.000 | 0.9479 (0.0051) | 0.0379 | 0.0142 | 0.9479 | 0.9526 | 0.5091 | errors 0 | fail |
| 20×20×5 | all-one | normal | G | fiducial GPQ, signed | 0.7937 | 1.000 | 0.9521 (0.0049) | 0.0263 | 0.0216 | 0.9521 | 0.9384 | 0.3193 | errors 0 | pass |
| 20×20×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.6579 | 1.000 | 0.9479 (0.0051) | 0.0379 | 0.0142 | 0.9479 | 0.9526 | 0.5091 | errors 0 | fail |
| 20×20×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0247 | 0.0268 | 0.9484 | 0.9479 | 1.6553 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0211 | 0.0189 | 0.9600 | 0.9605 | 1.6605 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9484 (0.0051) | 0.0289 | 0.0226 | 0.9484 | 0.9484 | 0.1055 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9679 (0.0040) | 0.0163 | 0.0158 | 0.9679 | 0.9679 | 0.3548 | errors 0 | fail |
| 20×20×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9500 (0.0050) | 0.0253 | 0.0247 | 0.9500 | 0.9500 | 0.0338 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9468 (0.0051) | 0.0284 | 0.0247 | 0.9468 | 0.9463 | 0.0336 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0289 | 0.0263 | 0.9447 | 0.9437 | 0.1463 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | G | fiducial GPQ | 0.9416 | 1.000 | 0.9458 (0.0052) | 0.0263 | 0.0279 | 0.9458 | 0.9274 | 0.0859 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8976 | 1.000 | 0.9553 (0.0047) | 0.0305 | 0.0142 | 0.9553 | 0.9505 | 0.1599 | errors 0 | fail |
| 20×20×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9416 | 1.000 | 0.9447 (0.0052) | 0.0258 | 0.0295 | 0.9447 | 0.9268 | 0.0857 | errors 0 | pass |
| 20×20×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8976 | 1.000 | 0.9553 (0.0047) | 0.0305 | 0.0142 | 0.9553 | 0.9505 | 0.1597 | errors 0 | fail |
| 20×20×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9516 (0.0049) | 0.0226 | 0.0258 | 0.9516 | 0.9516 | 6.4213 | errors 0 | pass |
| 20×20×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9411 (0.0054) | 0.0311 | 0.0279 | 0.9411 | 0.9411 | 0.9822 | errors 0 | pass |
| 20×20×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0253 | 0.0247 | 0.9500 | 0.9500 | 4.3276 | errors 0 | pass |
| 20×20×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9421 (0.0054) | 0.0326 | 0.0253 | 0.9421 | 0.9426 | 0.1772 | errors 0 | pass |
| 20×20×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0268 | 0.0253 | 0.9479 | 0.9479 | 0.3507 | errors 0 | pass |
| 20×20×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9474 (0.0051) | 0.0247 | 0.0279 | 0.9474 | 0.9505 | 0.3511 | errors 0 | pass |
| 20×20×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0242 | 0.0289 | 0.9468 | 0.9484 | 0.0731 | errors 0 | pass |
| 20×20×5 | high-g | normal | G | fiducial GPQ | 0.9685 | 1.000 | 0.9479 (0.0051) | 0.0279 | 0.0242 | 0.9479 | 0.9353 | 0.0495 | errors 0 | pass |
| 20×20×5 | high-g | normal | Phi | fiducial GPQ | 0.9390 | 1.000 | 0.9542 (0.0048) | 0.0353 | 0.0105 | 0.9542 | 0.9626 | 0.1944 | errors 0 | fail |
| 20×20×5 | high-g | normal | G | fiducial GPQ, signed | 0.9685 | 1.000 | 0.9479 (0.0051) | 0.0279 | 0.0242 | 0.9479 | 0.9353 | 0.0495 | errors 0 | pass |
| 20×20×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9390 | 1.000 | 0.9542 (0.0048) | 0.0353 | 0.0105 | 0.9542 | 0.9616 | 0.1944 | errors 0 | fail |
| 50×5×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0300 | 0.0211 | 0.9489 | 0.9489 | 1.6205 | errors 0 | pass |
| 50×5×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0253 | 0.0147 | 0.9600 | 0.9589 | 12.1406 | errors 0 | fail |
| 50×5×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0274 | 0.0195 | 0.9532 | 0.9595 | 1344.7698 | errors 0 | pass |
| 50×5×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0321 | 0.0263 | 0.9416 | 0.9416 | 0.6382 | errors 0 | pass |
| 50×5×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0189 | 0.0242 | 0.9568 | 0.9553 | 1.0224 | errors 0 | pass |
| 50×5×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0284 | 0.0258 | 0.9458 | 0.9458 | 8.0684 | errors 0 | pass |
| 50×5×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0268 | 0.0258 | 0.9474 | 0.9484 | 0.4012 | errors 0 | pass |
| 50×5×2 | all-one | normal | G | fiducial GPQ | 0.5556 | 1.000 | 0.9584 (0.0046) | 0.0237 | 0.0179 | 0.9584 | 0.9584 | 0.4703 | errors 0 | pass |
| 50×5×2 | all-one | normal | Phi | fiducial GPQ | 0.3846 | 1.000 | 0.9395 (0.0055) | 0.0600 | 0.0005 | 0.9395 | 0.9368 | 0.5517 | errors 0 | fail |
| 50×5×2 | all-one | normal | G | fiducial GPQ, signed | 0.5556 | 1.000 | 0.9584 (0.0046) | 0.0237 | 0.0179 | 0.9584 | 0.9584 | 0.4703 | errors 0 | pass |
| 50×5×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.3846 | 1.000 | 0.9489 (0.0050) | 0.0500 | 0.0011 | 0.9489 | 0.9474 | 0.6285 | errors 0 | fail |
| 50×5×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0268 | 0.0247 | 0.9484 | 0.9495 | 1.1249 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9489 | 8.2816 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9574 (0.0046) | 0.0289 | 0.0137 | 0.9574 | 0.9621 | 16.1115 | errors 0 | fail |
| 50×5×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0247 | 0.0263 | 0.9489 | 0.9489 | 0.6400 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9563 (0.0047) | 0.0216 | 0.0221 | 0.9563 | 0.9489 | 0.1295 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9516 (0.0049) | 0.0268 | 0.0216 | 0.9516 | 0.9558 | 0.2215 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0274 | 0.0237 | 0.9489 | 0.9489 | 0.4032 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | G | fiducial GPQ | 0.7663 | 1.000 | 0.9526 (0.0049) | 0.0284 | 0.0189 | 0.9526 | 0.9516 | 0.2320 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6618 | 1.000 | 0.9505 (0.0050) | 0.0484 | 0.0011 | 0.9505 | 0.9616 | 0.6020 | errors 0 | fail |
| 50×5×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.7663 | 1.000 | 0.9511 (0.0049) | 0.0237 | 0.0253 | 0.9511 | 0.9442 | 0.2345 | errors 0 | pass |
| 50×5×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6618 | 1.000 | 0.9584 (0.0046) | 0.0405 | 0.0011 | 0.9584 | 0.9668 | 0.6085 | errors 0 | fail |
| 50×5×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9526 (0.0049) | 0.0247 | 0.0226 | 0.9526 | 0.9521 | 3.7589 | errors 0 | pass |
| 50×5×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0284 | 0.0153 | 0.9563 | 0.9574 | 5.8681 | errors 0 | pass |
| 50×5×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0268 | 0.0205 | 0.9526 | 0.9558 | 618.8915 | errors 0 | pass |
| 50×5×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0279 | 0.0274 | 0.9447 | 0.9447 | 0.3206 | errors 0 | pass |
| 50×5×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9600 (0.0045) | 0.0221 | 0.0179 | 0.9600 | 0.9600 | 0.5150 | errors 0 | pass |
| 50×5×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0226 | 0.0268 | 0.9505 | 0.9489 | 3.9406 | errors 0 | pass |
| 50×5×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0226 | 0.0237 | 0.9537 | 0.9542 | 0.2019 | errors 0 | pass |
| 50×5×2 | high-g | normal | G | fiducial GPQ | 0.9091 | 1.000 | 0.9495 (0.0050) | 0.0279 | 0.0226 | 0.9495 | 0.9468 | 0.1006 | errors 0 | pass |
| 50×5×2 | high-g | normal | Phi | fiducial GPQ | 0.8333 | 1.000 | 0.9447 (0.0052) | 0.0553 | 0.0000 | 0.9447 | 0.9516 | 0.7883 | errors 0 | fail |
| 50×5×2 | high-g | normal | G | fiducial GPQ, signed | 0.9091 | 1.000 | 0.9495 (0.0050) | 0.0279 | 0.0226 | 0.9495 | 0.9468 | 0.1006 | errors 0 | pass |
| 50×5×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8333 | 1.000 | 0.9663 (0.0041) | 0.0326 | 0.0011 | 0.9663 | 0.9705 | 0.8131 | errors 0 | fail |
| 50×5×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0326 | 0.0242 | 0.9432 | 0.9432 | 1.3944 | errors 0 | pass |
| 50×5×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0258 | 0.0158 | 0.9584 | 0.9579 | 10.4507 | errors 0 | pass |
| 50×5×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0226 | 0.0237 | 0.9537 | 0.9553 | 49.5956 | errors 0 | pass |
| 50×5×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0247 | 0.0263 | 0.9489 | 0.9489 | 0.5466 | errors 0 | pass |
| 50×5×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0274 | 0.0300 | 0.9426 | 0.9437 | 0.7020 | errors 0 | pass |
| 50×5×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0195 | 0.0295 | 0.9511 | 0.9516 | 3.3307 | errors 0 | pass |
| 50×5×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0237 | 0.0311 | 0.9453 | 0.9468 | 0.2827 | errors 0 | pass |
| 50×5×3 | all-one | normal | G | fiducial GPQ | 0.6250 | 1.000 | 0.9474 (0.0051) | 0.0311 | 0.0216 | 0.9474 | 0.9426 | 0.3562 | errors 0 | pass |
| 50×5×3 | all-one | normal | Phi | fiducial GPQ | 0.4545 | 1.000 | 0.9432 (0.0053) | 0.0558 | 0.0011 | 0.9432 | 0.9442 | 0.5380 | errors 0 | fail |
| 50×5×3 | all-one | normal | G | fiducial GPQ, signed | 0.6250 | 1.000 | 0.9474 (0.0051) | 0.0311 | 0.0216 | 0.9474 | 0.9426 | 0.3562 | errors 0 | pass |
| 50×5×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.4545 | 1.000 | 0.9463 (0.0052) | 0.0526 | 0.0011 | 0.9463 | 0.9474 | 0.5532 | errors 0 | fail |
| 50×5×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0279 | 0.0268 | 0.9453 | 0.9442 | 1.0949 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0195 | 0.0316 | 0.9489 | 0.9484 | 8.2588 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9516 (0.0049) | 0.0289 | 0.0195 | 0.9516 | 0.9542 | 0.6204 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0300 | 0.0189 | 0.9511 | 0.9526 | 0.5447 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9395 (0.0055) | 0.0300 | 0.0305 | 0.9395 | 0.9395 | 0.0875 | errors 0 | fail |
| 50×5×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9489 (0.0050) | 0.0295 | 0.0216 | 0.9489 | 0.9479 | 0.0866 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0247 | 0.0295 | 0.9458 | 0.9458 | 0.2824 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | G | fiducial GPQ | 0.7874 | 1.000 | 0.9426 (0.0053) | 0.0284 | 0.0289 | 0.9426 | 0.9321 | 0.2018 | errors 0 | pass |
| 50×5×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6784 | 1.000 | 0.9521 (0.0049) | 0.0432 | 0.0047 | 0.9521 | 0.9547 | 0.4519 | errors 0 | fail |
| 50×5×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.7874 | 1.000 | 0.9395 (0.0055) | 0.0258 | 0.0347 | 0.9395 | 0.9311 | 0.2019 | errors 0 | fail |
| 50×5×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6784 | 1.000 | 0.9537 (0.0048) | 0.0416 | 0.0047 | 0.9537 | 0.9574 | 0.4537 | errors 0 | fail |
| 50×5×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9584 (0.0046) | 0.0184 | 0.0232 | 0.9584 | 0.9605 | 3.6974 | errors 0 | pass |
| 50×5×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0316 | 0.0174 | 0.9511 | 0.9511 | 5.2302 | errors 0 | pass |
| 50×5×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9611 (0.0044) | 0.0216 | 0.0174 | 0.9611 | 0.9616 | 24.2840 | errors 0 | fail |
| 50×5×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0268 | 0.0279 | 0.9453 | 0.9447 | 0.2727 | errors 0 | pass |
| 50×5×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9574 (0.0046) | 0.0232 | 0.0195 | 0.9574 | 0.9568 | 0.3511 | errors 0 | pass |
| 50×5×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9526 | 1.6324 | errors 0 | pass |
| 50×5×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0305 | 0.0195 | 0.9500 | 0.9500 | 0.1411 | errors 0 | pass |
| 50×5×3 | high-g | normal | G | fiducial GPQ | 0.9302 | 1.000 | 0.9621 (0.0044) | 0.0163 | 0.0216 | 0.9621 | 0.9511 | 0.0660 | errors 0 | fail |
| 50×5×3 | high-g | normal | Phi | fiducial GPQ | 0.8696 | 1.000 | 0.9574 (0.0046) | 0.0405 | 0.0021 | 0.9574 | 0.9621 | 0.5271 | errors 0 | fail |
| 50×5×3 | high-g | normal | G | fiducial GPQ, signed | 0.9302 | 1.000 | 0.9621 (0.0044) | 0.0163 | 0.0216 | 0.9621 | 0.9511 | 0.0660 | errors 0 | fail |
| 50×5×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.8696 | 1.000 | 0.9589 (0.0046) | 0.0379 | 0.0032 | 0.9589 | 0.9626 | 0.5311 | errors 0 | fail |
| 50×5×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0237 | 0.0237 | 0.9526 | 0.9532 | 1.2437 | errors 0 | pass |
| 50×5×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0311 | 0.0232 | 0.9458 | 0.9458 | 9.6224 | errors 0 | pass |
| 50×5×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0279 | 0.0221 | 0.9500 | 0.9521 | 9.6162 | errors 0 | pass |
| 50×5×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0221 | 0.0305 | 0.9474 | 0.9474 | 0.4854 | errors 0 | pass |
| 50×5×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0226 | 0.0237 | 0.9537 | 0.9532 | 0.4855 | errors 0 | pass |
| 50×5×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0311 | 0.0189 | 0.9500 | 0.9495 | 1.7826 | errors 0 | pass |
| 50×5×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0279 | 0.0247 | 0.9474 | 0.9453 | 0.1990 | errors 0 | pass |
| 50×5×5 | all-one | normal | G | fiducial GPQ | 0.6944 | 1.000 | 0.9479 (0.0051) | 0.0279 | 0.0242 | 0.9479 | 0.9447 | 0.2722 | errors 0 | pass |
| 50×5×5 | all-one | normal | Phi | fiducial GPQ | 0.5319 | 1.000 | 0.9474 (0.0051) | 0.0489 | 0.0037 | 0.9474 | 0.9505 | 0.4678 | errors 0 | fail |
| 50×5×5 | all-one | normal | G | fiducial GPQ, signed | 0.6944 | 1.000 | 0.9479 (0.0051) | 0.0279 | 0.0242 | 0.9479 | 0.9447 | 0.2722 | errors 0 | pass |
| 50×5×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.5319 | 1.000 | 0.9474 (0.0051) | 0.0479 | 0.0047 | 0.9474 | 0.9511 | 0.4700 | errors 0 | fail |
| 50×5×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0268 | 0.0168 | 0.9563 | 0.9563 | 1.0617 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0268 | 0.0237 | 0.9495 | 0.9484 | 8.0189 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9474 (0.0051) | 0.0289 | 0.0237 | 0.9474 | 0.9521 | 0.1257 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0274 | 0.0226 | 0.9500 | 0.9500 | 0.4853 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9489 (0.0050) | 0.0237 | 0.0274 | 0.9489 | 0.9400 | 0.0622 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9532 (0.0048) | 0.0189 | 0.0279 | 0.9532 | 0.9568 | 0.0486 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0284 | 0.0232 | 0.9484 | 0.9489 | 0.1984 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | G | fiducial GPQ | 0.8052 | 1.000 | 0.9484 (0.0051) | 0.0279 | 0.0237 | 0.9484 | 0.9474 | 0.1838 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6923 | 1.000 | 0.9558 (0.0047) | 0.0316 | 0.0126 | 0.9558 | 0.9547 | 0.4373 | errors 0 | fail |
| 50×5×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8052 | 1.000 | 0.9500 (0.0050) | 0.0263 | 0.0237 | 0.9500 | 0.9453 | 0.1836 | errors 0 | pass |
| 50×5×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6923 | 1.000 | 0.9558 (0.0047) | 0.0311 | 0.0132 | 0.9558 | 0.9542 | 0.4378 | errors 0 | fail |
| 50×5×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9516 (0.0049) | 0.0258 | 0.0226 | 0.9516 | 0.9511 | 3.6300 | errors 0 | pass |
| 50×5×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0242 | 0.0247 | 0.9511 | 0.9532 | 4.9081 | errors 0 | pass |
| 50×5×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9595 (0.0045) | 0.0237 | 0.0168 | 0.9595 | 0.9589 | 4.7426 | errors 0 | pass |
| 50×5×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0274 | 0.0258 | 0.9468 | 0.9479 | 0.2428 | errors 0 | pass |
| 50×5×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0258 | 0.0258 | 0.9484 | 0.9489 | 0.2425 | errors 0 | pass |
| 50×5×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9400 (0.0054) | 0.0353 | 0.0247 | 0.9400 | 0.9400 | 0.8984 | errors 0 | fail |
| 50×5×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0179 | 0.0258 | 0.9563 | 0.9568 | 0.0996 | errors 0 | pass |
| 50×5×5 | high-g | normal | G | fiducial GPQ | 0.9479 | 1.000 | 0.9468 (0.0051) | 0.0274 | 0.0258 | 0.9468 | 0.9442 | 0.0464 | errors 0 | pass |
| 50×5×5 | high-g | normal | Phi | fiducial GPQ | 0.9009 | 1.000 | 0.9516 (0.0049) | 0.0463 | 0.0021 | 0.9516 | 0.9589 | 0.2646 | errors 0 | fail |
| 50×5×5 | high-g | normal | G | fiducial GPQ, signed | 0.9479 | 1.000 | 0.9468 (0.0051) | 0.0274 | 0.0258 | 0.9468 | 0.9442 | 0.0464 | errors 0 | pass |
| 50×5×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9009 | 1.000 | 0.9526 (0.0049) | 0.0442 | 0.0032 | 0.9526 | 0.9589 | 0.2650 | errors 0 | fail |
| 50×8×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0247 | 0.0247 | 0.9505 | 0.9516 | 1.5214 | errors 0 | pass |
| 50×8×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0295 | 0.0126 | 0.9579 | 0.9589 | 5.5731 | errors 0 | fail |
| 50×8×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0253 | 0.0237 | 0.9511 | 0.9511 | 1169.9091 | errors 0 | pass |
| 50×8×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0363 | 0.0247 | 0.9389 | 0.9353 | 0.4771 | errors 0 | fail |
| 50×8×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0242 | 0.0226 | 0.9532 | 0.9532 | 0.9643 | errors 0 | pass |
| 50×8×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0247 | 0.0200 | 0.9553 | 0.9537 | 3.7343 | errors 0 | pass |
| 50×8×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0247 | 0.0216 | 0.9537 | 0.9537 | 0.3022 | errors 0 | pass |
| 50×8×2 | all-one | normal | G | fiducial GPQ | 0.5926 | 1.000 | 0.9447 (0.0052) | 0.0279 | 0.0274 | 0.9447 | 0.9421 | 0.4515 | errors 0 | pass |
| 50×8×2 | all-one | normal | Phi | fiducial GPQ | 0.4211 | 1.000 | 0.9474 (0.0051) | 0.0505 | 0.0021 | 0.9474 | 0.9384 | 0.5946 | errors 0 | fail |
| 50×8×2 | all-one | normal | G | fiducial GPQ, signed | 0.5926 | 1.000 | 0.9447 (0.0052) | 0.0279 | 0.0274 | 0.9447 | 0.9421 | 0.4515 | errors 0 | pass |
| 50×8×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.4211 | 1.000 | 0.9500 (0.0050) | 0.0474 | 0.0026 | 0.9500 | 0.9400 | 0.6126 | errors 0 | fail |
| 50×8×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0232 | 0.0295 | 0.9474 | 0.9495 | 1.0269 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0247 | 0.0200 | 0.9553 | 0.9542 | 3.7947 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9526 (0.0049) | 0.0242 | 0.0232 | 0.9526 | 0.9621 | 15.1364 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0284 | 0.0242 | 0.9474 | 0.9474 | 0.4792 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9468 | 0.0855 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9468 (0.0051) | 0.0258 | 0.0274 | 0.9468 | 0.9505 | 0.1036 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0284 | 0.0189 | 0.9526 | 0.9537 | 0.3018 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | G | fiducial GPQ | 0.8386 | 1.000 | 0.9484 (0.0051) | 0.0305 | 0.0211 | 0.9484 | 0.9416 | 0.1531 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7558 | 1.000 | 0.9400 (0.0054) | 0.0584 | 0.0016 | 0.9400 | 0.9532 | 0.5821 | errors 0 | fail |
| 50×8×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8386 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9363 | 0.1541 | errors 0 | pass |
| 50×8×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7558 | 1.000 | 0.9437 (0.0053) | 0.0547 | 0.0016 | 0.9437 | 0.9574 | 0.5849 | errors 0 | fail |
| 50×8×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9500 (0.0050) | 0.0263 | 0.0237 | 0.9500 | 0.9479 | 3.6919 | errors 0 | pass |
| 50×8×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0274 | 0.0216 | 0.9511 | 0.9516 | 2.7595 | errors 0 | pass |
| 50×8×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0300 | 0.0253 | 0.9447 | 0.9489 | 592.8390 | errors 0 | pass |
| 50×8×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0289 | 0.0189 | 0.9521 | 0.9532 | 0.2391 | errors 0 | pass |
| 50×8×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0242 | 0.0226 | 0.9532 | 0.9521 | 0.4794 | errors 0 | pass |
| 50×8×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0211 | 0.0242 | 0.9547 | 0.9542 | 1.8780 | errors 0 | pass |
| 50×8×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0247 | 0.0274 | 0.9479 | 0.9489 | 0.1508 | errors 0 | pass |
| 50×8×2 | high-g | normal | G | fiducial GPQ | 0.9209 | 1.000 | 0.9468 (0.0051) | 0.0311 | 0.0221 | 0.9468 | 0.9479 | 0.0901 | errors 0 | pass |
| 50×8×2 | high-g | normal | Phi | fiducial GPQ | 0.8533 | 1.000 | 0.9521 (0.0049) | 0.0437 | 0.0042 | 0.9521 | 0.9574 | 0.7889 | errors 0 | fail |
| 50×8×2 | high-g | normal | G | fiducial GPQ, signed | 0.9209 | 1.000 | 0.9468 (0.0051) | 0.0311 | 0.0221 | 0.9468 | 0.9479 | 0.0901 | errors 0 | pass |
| 50×8×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8533 | 1.000 | 0.9526 (0.0049) | 0.0421 | 0.0053 | 0.9526 | 0.9574 | 0.7942 | errors 0 | fail |
| 50×8×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0174 | 0.0268 | 0.9558 | 0.9547 | 1.3236 | errors 0 | pass |
| 50×8×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9484 | 4.9558 | errors 0 | pass |
| 50×8×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0242 | 0.0295 | 0.9463 | 0.9447 | 45.1293 | errors 0 | pass |
| 50×8×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9353 (0.0056) | 0.0358 | 0.0289 | 0.9353 | 0.9347 | 0.4099 | errors 0 | fail |
| 50×8×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0279 | 0.0200 | 0.9521 | 0.9521 | 0.6548 | errors 0 | pass |
| 50×8×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0300 | 0.0232 | 0.9468 | 0.9474 | 1.9605 | errors 0 | pass |
| 50×8×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9463 | 0.2127 | errors 0 | pass |
| 50×8×3 | all-one | normal | G | fiducial GPQ | 0.6667 | 1.000 | 0.9526 (0.0049) | 0.0211 | 0.0263 | 0.9526 | 0.9500 | 0.3150 | errors 0 | pass |
| 50×8×3 | all-one | normal | Phi | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0400 | 0.0068 | 0.9532 | 0.9484 | 0.5543 | errors 0 | fail |
| 50×8×3 | all-one | normal | G | fiducial GPQ, signed | 0.6667 | 1.000 | 0.9526 (0.0049) | 0.0211 | 0.0263 | 0.9526 | 0.9500 | 0.3150 | errors 0 | pass |
| 50×8×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0384 | 0.0074 | 0.9542 | 0.9474 | 0.5576 | errors 0 | fail |
| 50×8×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0205 | 0.0195 | 0.9600 | 0.9579 | 1.0002 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0263 | 0.0184 | 0.9553 | 0.9558 | 3.8404 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9458 (0.0052) | 0.0263 | 0.0279 | 0.9458 | 0.9516 | 0.5667 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0232 | 0.0247 | 0.9521 | 0.9521 | 0.4103 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9484 (0.0051) | 0.0268 | 0.0247 | 0.9484 | 0.9479 | 0.0589 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9495 (0.0050) | 0.0253 | 0.0253 | 0.9495 | 0.9532 | 0.0530 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0237 | 0.0242 | 0.9521 | 0.9526 | 0.2129 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | G | fiducial GPQ | 0.8547 | 1.000 | 0.9589 (0.0046) | 0.0242 | 0.0168 | 0.9589 | 0.9500 | 0.1318 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7700 | 1.000 | 0.9632 (0.0043) | 0.0321 | 0.0047 | 0.9632 | 0.9679 | 0.3179 | errors 0 | fail |
| 50×8×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8547 | 1.000 | 0.9574 (0.0046) | 0.0221 | 0.0205 | 0.9574 | 0.9495 | 0.1317 | errors 0 | pass |
| 50×8×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7700 | 1.000 | 0.9647 (0.0042) | 0.0305 | 0.0047 | 0.9647 | 0.9705 | 0.3185 | errors 0 | fail |
| 50×8×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9474 (0.0051) | 0.0263 | 0.0263 | 0.9474 | 0.9474 | 3.6491 | errors 0 | pass |
| 50×8×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9574 (0.0046) | 0.0205 | 0.0221 | 0.9574 | 0.9579 | 2.5250 | errors 0 | pass |
| 50×8×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0195 | 0.0268 | 0.9537 | 0.9537 | 22.3913 | errors 0 | pass |
| 50×8×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0247 | 0.0284 | 0.9468 | 0.9474 | 0.2053 | errors 0 | pass |
| 50×8×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9416 (0.0054) | 0.0321 | 0.0263 | 0.9416 | 0.9421 | 0.3269 | errors 0 | pass |
| 50×8×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9495 | 0.9982 | errors 0 | pass |
| 50×8×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0221 | 0.0216 | 0.9563 | 0.9558 | 0.1062 | errors 0 | pass |
| 50×8×3 | high-g | normal | G | fiducial GPQ | 0.9412 | 1.000 | 0.9489 (0.0050) | 0.0232 | 0.0279 | 0.9489 | 0.9411 | 0.0564 | errors 0 | pass |
| 50×8×3 | high-g | normal | Phi | fiducial GPQ | 0.8889 | 1.000 | 0.9526 (0.0049) | 0.0442 | 0.0032 | 0.9526 | 0.9589 | 0.4883 | errors 0 | fail |
| 50×8×3 | high-g | normal | G | fiducial GPQ, signed | 0.9412 | 1.000 | 0.9489 (0.0050) | 0.0232 | 0.0279 | 0.9489 | 0.9411 | 0.0564 | errors 0 | pass |
| 50×8×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.8889 | 1.000 | 0.9516 (0.0049) | 0.0437 | 0.0047 | 0.9516 | 0.9595 | 0.4891 | errors 0 | fail |
| 50×8×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0221 | 0.0211 | 0.9568 | 0.9563 | 1.1580 | errors 0 | pass |
| 50×8×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0305 | 0.0221 | 0.9474 | 0.9468 | 4.4781 | errors 0 | pass |
| 50×8×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0237 | 0.0247 | 0.9516 | 0.9511 | 9.2188 | errors 0 | pass |
| 50×8×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0237 | 0.0284 | 0.9479 | 0.9489 | 0.3650 | errors 0 | pass |
| 50×8×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0237 | 0.0300 | 0.9463 | 0.9500 | 0.4569 | errors 0 | pass |
| 50×8×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0247 | 0.0242 | 0.9511 | 0.9511 | 1.2155 | errors 0 | pass |
| 50×8×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0284 | 0.0258 | 0.9458 | 0.9468 | 0.1499 | errors 0 | pass |
| 50×8×5 | all-one | normal | G | fiducial GPQ | 0.7407 | 1.000 | 0.9553 (0.0047) | 0.0232 | 0.0216 | 0.9553 | 0.9542 | 0.2303 | errors 0 | pass |
| 50×8×5 | all-one | normal | Phi | fiducial GPQ | 0.5882 | 1.000 | 0.9505 (0.0050) | 0.0437 | 0.0058 | 0.9505 | 0.9526 | 0.4316 | errors 0 | fail |
| 50×8×5 | all-one | normal | G | fiducial GPQ, signed | 0.7407 | 1.000 | 0.9553 (0.0047) | 0.0232 | 0.0216 | 0.9553 | 0.9542 | 0.2303 | errors 0 | pass |
| 50×8×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.5882 | 1.000 | 0.9495 (0.0050) | 0.0437 | 0.0068 | 0.9495 | 0.9516 | 0.4320 | errors 0 | fail |
| 50×8×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9495 | 0.9922 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0226 | 0.0226 | 0.9547 | 0.9553 | 3.8356 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9505 (0.0050) | 0.0316 | 0.0179 | 0.9505 | 0.9500 | 0.1070 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0268 | 0.0237 | 0.9495 | 0.9495 | 0.3642 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9511 (0.0049) | 0.0253 | 0.0237 | 0.9511 | 0.9489 | 0.0418 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9484 (0.0051) | 0.0205 | 0.0311 | 0.9484 | 0.9505 | 0.0342 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0258 | 0.0195 | 0.9547 | 0.9547 | 0.1500 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | G | fiducial GPQ | 0.8681 | 1.000 | 0.9468 (0.0051) | 0.0295 | 0.0237 | 0.9468 | 0.9389 | 0.1169 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7817 | 1.000 | 0.9516 (0.0049) | 0.0353 | 0.0132 | 0.9516 | 0.9558 | 0.2819 | errors 0 | fail |
| 50×8×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8681 | 1.000 | 0.9474 (0.0051) | 0.0279 | 0.0247 | 0.9474 | 0.9389 | 0.1167 | errors 0 | pass |
| 50×8×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7817 | 1.000 | 0.9526 (0.0049) | 0.0342 | 0.0132 | 0.9526 | 0.9553 | 0.2820 | errors 0 | fail |
| 50×8×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9479 (0.0051) | 0.0253 | 0.0268 | 0.9479 | 0.9474 | 3.5606 | errors 0 | pass |
| 50×8×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0295 | 0.0179 | 0.9526 | 0.9526 | 2.2616 | errors 0 | pass |
| 50×8×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9463 (0.0052) | 0.0263 | 0.0274 | 0.9463 | 0.9463 | 4.5622 | errors 0 | pass |
| 50×8×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9368 (0.0056) | 0.0321 | 0.0311 | 0.9368 | 0.9368 | 0.1823 | errors 0 | fail |
| 50×8×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0232 | 0.0274 | 0.9495 | 0.9495 | 0.2270 | errors 0 | pass |
| 50×8×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0253 | 0.0268 | 0.9479 | 0.9484 | 0.6131 | errors 0 | pass |
| 50×8×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9500 | 0.0749 | errors 0 | pass |
| 50×8×5 | high-g | normal | G | fiducial GPQ | 0.9581 | 1.000 | 0.9500 (0.0050) | 0.0237 | 0.0263 | 0.9500 | 0.9442 | 0.0372 | errors 0 | pass |
| 50×8×5 | high-g | normal | Phi | fiducial GPQ | 0.9195 | 1.000 | 0.9542 (0.0048) | 0.0432 | 0.0026 | 0.9542 | 0.9589 | 0.1940 | errors 0 | fail |
| 50×8×5 | high-g | normal | G | fiducial GPQ, signed | 0.9581 | 1.000 | 0.9500 (0.0050) | 0.0237 | 0.0263 | 0.9500 | 0.9442 | 0.0372 | errors 0 | pass |
| 50×8×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9195 | 1.000 | 0.9537 (0.0048) | 0.0432 | 0.0032 | 0.9537 | 0.9589 | 0.1941 | errors 0 | fail |
| 50×20×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0274 | 0.0179 | 0.9547 | 0.9558 | 1.4164 | errors 0 | pass |
| 50×20×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0316 | 0.0195 | 0.9489 | 0.9489 | 2.4643 | errors 0 | pass |
| 50×20×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0311 | 0.0221 | 0.9468 | 0.9489 | 1126.3336 | errors 0 | pass |
| 50×20×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9379 (0.0055) | 0.0332 | 0.0289 | 0.9379 | 0.9374 | 0.2886 | errors 0 | fail |
| 50×20×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0232 | 0.0226 | 0.9542 | 0.9553 | 0.9023 | errors 0 | pass |
| 50×20×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0247 | 0.0195 | 0.9558 | 0.9537 | 1.5655 | errors 0 | pass |
| 50×20×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0316 | 0.0211 | 0.9474 | 0.9463 | 0.1822 | errors 0 | pass |
| 50×20×2 | all-one | normal | G | fiducial GPQ | 0.6349 | 1.000 | 0.9547 (0.0048) | 0.0232 | 0.0221 | 0.9547 | 0.9468 | 0.4344 | errors 0 | pass |
| 50×20×2 | all-one | normal | Phi | fiducial GPQ | 0.4651 | 1.000 | 0.9468 (0.0051) | 0.0453 | 0.0079 | 0.9468 | 0.9368 | 0.6372 | errors 0 | fail |
| 50×20×2 | all-one | normal | G | fiducial GPQ, signed | 0.6349 | 1.000 | 0.9547 (0.0048) | 0.0232 | 0.0221 | 0.9547 | 0.9468 | 0.4344 | errors 0 | pass |
| 50×20×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.4651 | 1.000 | 0.9468 (0.0051) | 0.0453 | 0.0079 | 0.9468 | 0.9363 | 0.6392 | errors 0 | fail |
| 50×20×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0232 | 0.0258 | 0.9511 | 0.9505 | 0.9225 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0253 | 0.0279 | 0.9468 | 0.9468 | 1.6157 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9547 (0.0048) | 0.0189 | 0.0263 | 0.9547 | 0.9584 | 12.5725 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0247 | 0.0242 | 0.9511 | 0.9511 | 0.2884 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9495 (0.0050) | 0.0284 | 0.0221 | 0.9495 | 0.9526 | 0.0419 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9516 (0.0049) | 0.0253 | 0.0232 | 0.9516 | 0.9495 | 0.0433 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0195 | 0.0258 | 0.9547 | 0.9558 | 0.1824 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | G | fiducial GPQ | 0.9259 | 1.000 | 0.9553 (0.0047) | 0.0274 | 0.0174 | 0.9553 | 0.9468 | 0.0682 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8809 | 1.000 | 0.9400 (0.0054) | 0.0589 | 0.0011 | 0.9400 | 0.9563 | 0.5805 | errors 0 | fail |
| 50×20×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9259 | 1.000 | 0.9489 (0.0050) | 0.0263 | 0.0247 | 0.9489 | 0.9405 | 0.0684 | errors 0 | pass |
| 50×20×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8809 | 1.000 | 0.9447 (0.0052) | 0.0542 | 0.0011 | 0.9447 | 0.9595 | 0.5812 | errors 0 | fail |
| 50×20×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9474 (0.0051) | 0.0253 | 0.0274 | 0.9474 | 0.9468 | 3.6922 | errors 0 | pass |
| 50×20×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0268 | 0.0174 | 0.9558 | 0.9558 | 1.2274 | errors 0 | pass |
| 50×20×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0242 | 0.0211 | 0.9547 | 0.9489 | 510.7586 | errors 0 | pass |
| 50×20×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0268 | 0.0221 | 0.9511 | 0.9511 | 0.1442 | errors 0 | pass |
| 50×20×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0211 | 0.0305 | 0.9484 | 0.9505 | 0.4513 | errors 0 | pass |
| 50×20×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0200 | 0.0258 | 0.9542 | 0.9547 | 0.7991 | errors 0 | pass |
| 50×20×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0200 | 0.0279 | 0.9521 | 0.9521 | 0.0912 | errors 0 | pass |
| 50×20×2 | high-g | normal | G | fiducial GPQ | 0.9329 | 1.000 | 0.9421 (0.0054) | 0.0326 | 0.0253 | 0.9421 | 0.9453 | 0.0798 | errors 0 | pass |
| 50×20×2 | high-g | normal | Phi | fiducial GPQ | 0.8743 | 1.000 | 0.9579 (0.0046) | 0.0326 | 0.0095 | 0.9579 | 0.9547 | 0.7985 | errors 0 | fail |
| 50×20×2 | high-g | normal | G | fiducial GPQ, signed | 0.9329 | 1.000 | 0.9421 (0.0054) | 0.0326 | 0.0253 | 0.9421 | 0.9453 | 0.0798 | errors 0 | pass |
| 50×20×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8743 | 1.000 | 0.9563 (0.0047) | 0.0326 | 0.0111 | 0.9563 | 0.9521 | 0.7990 | errors 0 | fail |
| 50×20×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0226 | 0.0258 | 0.9516 | 0.9521 | 1.2243 | errors 0 | pass |
| 50×20×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0284 | 0.0221 | 0.9495 | 0.9495 | 2.1477 | errors 0 | pass |
| 50×20×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0316 | 0.0221 | 0.9463 | 0.9447 | 41.1948 | errors 0 | pass |
| 50×20×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0226 | 0.0337 | 0.9437 | 0.9447 | 0.2471 | errors 0 | pass |
| 50×20×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0253 | 0.0226 | 0.9521 | 0.9484 | 0.6056 | errors 0 | pass |
| 50×20×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0221 | 0.0268 | 0.9511 | 0.9511 | 1.0147 | errors 0 | pass |
| 50×20×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0226 | 0.0289 | 0.9484 | 0.9489 | 0.1286 | errors 0 | pass |
| 50×20×3 | all-one | normal | G | fiducial GPQ | 0.7143 | 1.000 | 0.9505 (0.0050) | 0.0247 | 0.0247 | 0.9505 | 0.9484 | 0.2806 | errors 0 | pass |
| 50×20×3 | all-one | normal | Phi | fiducial GPQ | 0.5556 | 1.000 | 0.9526 (0.0049) | 0.0374 | 0.0100 | 0.9526 | 0.9458 | 0.5894 | errors 0 | fail |
| 50×20×3 | all-one | normal | G | fiducial GPQ, signed | 0.7143 | 1.000 | 0.9505 (0.0050) | 0.0247 | 0.0247 | 0.9505 | 0.9484 | 0.2806 | errors 0 | pass |
| 50×20×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.5556 | 1.000 | 0.9505 (0.0050) | 0.0374 | 0.0121 | 0.9505 | 0.9437 | 0.5896 | errors 0 | fail |
| 50×20×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9642 (0.0043) | 0.0158 | 0.0200 | 0.9642 | 0.9647 | 0.9175 | errors 0 | fail |
| 50×20×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0263 | 0.0279 | 0.9458 | 0.9474 | 1.6172 | errors 0 | pass |
| 50×20×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9421 (0.0054) | 0.0311 | 0.0268 | 0.9421 | 0.9447 | 0.4719 | errors 0 | pass |
| 50×20×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0211 | 0.0205 | 0.9584 | 0.9589 | 0.2466 | errors 0 | pass |
| 50×20×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9505 (0.0050) | 0.0253 | 0.0242 | 0.9505 | 0.9521 | 0.0300 | errors 0 | pass |
| 50×20×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9568 (0.0047) | 0.0205 | 0.0226 | 0.9568 | 0.9563 | 0.0281 | errors 0 | pass |
| 50×20×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0311 | 0.0274 | 0.9416 | 0.9421 | 0.1286 | errors 0 | pass |
| 50×20×3 | near-zero-rater | normal | G | fiducial GPQ | 0.9346 | 1.000 | 0.9621 (0.0044) | 0.0189 | 0.0189 | 0.9621 | 0.9574 | 0.0564 | errors 0 | fail |
| 50×20×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8901 | 1.000 | 0.9616 (0.0044) | 0.0347 | 0.0037 | 0.9616 | 0.9716 | 0.1698 | errors 0 | fail |
| 50×20×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9346 | 1.000 | 0.9616 (0.0044) | 0.0184 | 0.0200 | 0.9616 | 0.9558 | 0.0563 | errors 0 | fail |
| 50×20×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8901 | 1.000 | 0.9621 (0.0044) | 0.0342 | 0.0037 | 0.9621 | 0.9732 | 0.1698 | errors 0 | fail |
| 50×20×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9484 (0.0051) | 0.0258 | 0.0258 | 0.9484 | 0.9474 | 3.5854 | errors 0 | pass |
| 50×20×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0268 | 0.0237 | 0.9495 | 0.9495 | 1.0739 | errors 0 | pass |
| 50×20×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0221 | 0.0253 | 0.9526 | 0.9558 | 21.7003 | errors 0 | pass |
| 50×20×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0247 | 0.0216 | 0.9537 | 0.9547 | 0.1235 | errors 0 | pass |
| 50×20×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0263 | 0.0237 | 0.9500 | 0.9505 | 0.3065 | errors 0 | pass |
| 50×20×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0274 | 0.0200 | 0.9526 | 0.9532 | 0.5070 | errors 0 | pass |
| 50×20×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0200 | 0.0295 | 0.9505 | 0.9516 | 0.0643 | errors 0 | pass |
| 50×20×3 | high-g | normal | G | fiducial GPQ | 0.9524 | 1.000 | 0.9505 (0.0050) | 0.0221 | 0.0274 | 0.9505 | 0.9426 | 0.0477 | errors 0 | pass |
| 50×20×3 | high-g | normal | Phi | fiducial GPQ | 0.9091 | 1.000 | 0.9447 (0.0052) | 0.0384 | 0.0168 | 0.9447 | 0.9505 | 0.4919 | errors 0 | fail |
| 50×20×3 | high-g | normal | G | fiducial GPQ, signed | 0.9524 | 1.000 | 0.9505 (0.0050) | 0.0221 | 0.0274 | 0.9505 | 0.9426 | 0.0477 | errors 0 | pass |
| 50×20×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.9091 | 1.000 | 0.9437 (0.0053) | 0.0384 | 0.0179 | 0.9437 | 0.9500 | 0.4919 | errors 0 | fail |
| 50×20×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0226 | 0.0295 | 0.9479 | 0.9495 | 1.0879 | errors 0 | pass |
| 50×20×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0247 | 0.0195 | 0.9558 | 0.9558 | 1.9085 | errors 0 | pass |
| 50×20×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0237 | 0.0347 | 0.9416 | 0.9432 | 8.6949 | errors 0 | pass |
| 50×20×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0258 | 0.0253 | 0.9489 | 0.9505 | 0.2192 | errors 0 | pass |
| 50×20×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0274 | 0.0211 | 0.9516 | 0.9521 | 0.4220 | errors 0 | pass |
| 50×20×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0247 | 0.0195 | 0.9558 | 0.9558 | 0.6807 | errors 0 | pass |
| 50×20×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0289 | 0.0284 | 0.9426 | 0.9416 | 0.0909 | errors 0 | pass |
| 50×20×5 | all-one | normal | G | fiducial GPQ | 0.7937 | 1.000 | 0.9411 (0.0054) | 0.0284 | 0.0305 | 0.9411 | 0.9358 | 0.1840 | errors 0 | pass |
| 50×20×5 | all-one | normal | Phi | fiducial GPQ | 0.6579 | 1.000 | 0.9542 (0.0048) | 0.0321 | 0.0137 | 0.9542 | 0.9563 | 0.4291 | errors 0 | fail |
| 50×20×5 | all-one | normal | G | fiducial GPQ, signed | 0.7937 | 1.000 | 0.9411 (0.0054) | 0.0284 | 0.0305 | 0.9411 | 0.9358 | 0.1840 | errors 0 | pass |
| 50×20×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.6579 | 1.000 | 0.9542 (0.0048) | 0.0321 | 0.0137 | 0.9542 | 0.9563 | 0.4291 | errors 0 | fail |
| 50×20×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0242 | 0.0226 | 0.9532 | 0.9532 | 0.9088 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9500 | 1.5934 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9463 | 0.0916 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0221 | 0.0337 | 0.9442 | 0.9437 | 0.2193 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9447 (0.0052) | 0.0289 | 0.0263 | 0.9447 | 0.9447 | 0.0222 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9632 (0.0043) | 0.0168 | 0.0200 | 0.9632 | 0.9616 | 0.0198 | errors 0 | fail |
| 50×20×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0205 | 0.0232 | 0.9563 | 0.9568 | 0.0909 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | G | fiducial GPQ | 0.9416 | 1.000 | 0.9542 (0.0048) | 0.0258 | 0.0200 | 0.9542 | 0.9437 | 0.0495 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8976 | 1.000 | 0.9558 (0.0047) | 0.0263 | 0.0179 | 0.9558 | 0.9542 | 0.1081 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9416 | 1.000 | 0.9537 (0.0048) | 0.0258 | 0.0205 | 0.9537 | 0.9437 | 0.0495 | errors 0 | pass |
| 50×20×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8976 | 1.000 | 0.9563 (0.0047) | 0.0258 | 0.0179 | 0.9563 | 0.9542 | 0.1081 | errors 0 | pass |
| 50×20×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9463 (0.0052) | 0.0237 | 0.0300 | 0.9463 | 0.9500 | 3.5557 | errors 0 | pass |
| 50×20×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0284 | 0.0221 | 0.9495 | 0.9489 | 0.9467 | errors 0 | pass |
| 50×20×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0232 | 0.0284 | 0.9484 | 0.9516 | 4.2921 | errors 0 | pass |
| 50×20×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0242 | 0.0253 | 0.9505 | 0.9521 | 0.1100 | errors 0 | pass |
| 50×20×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9542 | 0.2120 | errors 0 | pass |
| 50×20×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0237 | 0.0268 | 0.9495 | 0.9495 | 0.3416 | errors 0 | pass |
| 50×20×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0242 | 0.0216 | 0.9542 | 0.9532 | 0.0455 | errors 0 | pass |
| 50×20×5 | high-g | normal | G | fiducial GPQ | 0.9685 | 1.000 | 0.9537 (0.0048) | 0.0211 | 0.0253 | 0.9537 | 0.9458 | 0.0282 | errors 0 | pass |
| 50×20×5 | high-g | normal | Phi | fiducial GPQ | 0.9390 | 1.000 | 0.9500 (0.0050) | 0.0384 | 0.0116 | 0.9500 | 0.9547 | 0.1700 | errors 0 | fail |
| 50×20×5 | high-g | normal | G | fiducial GPQ, signed | 0.9685 | 1.000 | 0.9537 (0.0048) | 0.0211 | 0.0253 | 0.9537 | 0.9458 | 0.0282 | errors 0 | pass |
| 50×20×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9390 | 1.000 | 0.9500 (0.0050) | 0.0384 | 0.0116 | 0.9500 | 0.9547 | 0.1700 | errors 0 | fail |
| 200×5×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0216 | 0.0279 | 0.9505 | 0.9542 | 0.7651 | errors 0 | pass |
| 200×5×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0247 | 0.0153 | 0.9600 | 0.9616 | 11.7512 | errors 0 | pass |
| 200×5×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9653 (0.0042) | 0.0226 | 0.0121 | 0.9653 | 0.9658 | 1180.9518 | errors 0 | fail |
| 200×5×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0253 | 0.0226 | 0.9521 | 0.9521 | 0.3123 | errors 0 | pass |
| 200×5×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0295 | 0.0232 | 0.9474 | 0.9458 | 0.4807 | errors 0 | pass |
| 200×5×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0258 | 0.0284 | 0.9458 | 0.9458 | 7.8054 | errors 0 | pass |
| 200×5×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0200 | 0.0253 | 0.9547 | 0.9542 | 0.1975 | errors 0 | pass |
| 200×5×2 | all-one | normal | G | fiducial GPQ | 0.5556 | 1.000 | 0.9505 (0.0050) | 0.0205 | 0.0289 | 0.9505 | 0.9516 | 0.2253 | errors 0 | pass |
| 200×5×2 | all-one | normal | Phi | fiducial GPQ | 0.3846 | 1.000 | 0.9574 (0.0046) | 0.0426 | 0.0000 | 0.9574 | 0.9321 | 0.5003 | errors 0 | fail |
| 200×5×2 | all-one | normal | G | fiducial GPQ, signed | 0.5556 | 1.000 | 0.9505 (0.0050) | 0.0205 | 0.0289 | 0.9505 | 0.9516 | 0.2253 | errors 0 | pass |
| 200×5×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.3846 | 1.000 | 0.9742 (0.0036) | 0.0258 | 0.0000 | 0.9742 | 0.9553 | 0.5881 | errors 0 | fail |
| 200×5×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0258 | 0.0268 | 0.9474 | 0.9484 | 0.5292 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0242 | 0.0237 | 0.9521 | 0.9505 | 7.9056 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9505 (0.0050) | 0.0337 | 0.0158 | 0.9505 | 0.9516 | 13.4337 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0205 | 0.0253 | 0.9542 | 0.9547 | 0.3126 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9437 (0.0053) | 0.0263 | 0.0300 | 0.9437 | 0.9432 | 0.0594 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9574 (0.0046) | 0.0237 | 0.0189 | 0.9574 | 0.9579 | 0.1163 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0242 | 0.0300 | 0.9458 | 0.9468 | 0.1974 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | G | fiducial GPQ | 0.7663 | 1.000 | 0.9526 (0.0049) | 0.0279 | 0.0195 | 0.9526 | 0.9537 | 0.1080 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6618 | 1.000 | 0.9474 (0.0051) | 0.0511 | 0.0016 | 0.9474 | 0.9416 | 0.5541 | errors 0 | fail |
| 200×5×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.7663 | 1.000 | 0.9516 (0.0049) | 0.0237 | 0.0247 | 0.9516 | 0.9516 | 0.1096 | errors 0 | pass |
| 200×5×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6618 | 1.000 | 0.9526 (0.0049) | 0.0458 | 0.0016 | 0.9526 | 0.9447 | 0.5573 | errors 0 | fail |
| 200×5×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9537 (0.0048) | 0.0253 | 0.0211 | 0.9537 | 0.9547 | 1.7607 | errors 0 | pass |
| 200×5×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9611 (0.0044) | 0.0305 | 0.0084 | 0.9611 | 0.9595 | 5.9301 | errors 0 | fail |
| 200×5×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0263 | 0.0168 | 0.9568 | 0.9579 | 605.7971 | errors 0 | pass |
| 200×5×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0253 | 0.0205 | 0.9542 | 0.9553 | 0.1560 | errors 0 | pass |
| 200×5×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0289 | 0.0274 | 0.9437 | 0.9432 | 0.2405 | errors 0 | pass |
| 200×5×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0221 | 0.0263 | 0.9516 | 0.9516 | 3.9743 | errors 0 | pass |
| 200×5×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0268 | 0.0247 | 0.9484 | 0.9489 | 0.0987 | errors 0 | pass |
| 200×5×2 | high-g | normal | G | fiducial GPQ | 0.9091 | 1.000 | 0.9458 (0.0052) | 0.0237 | 0.0305 | 0.9458 | 0.9437 | 0.0463 | errors 0 | pass |
| 200×5×2 | high-g | normal | Phi | fiducial GPQ | 0.8333 | 1.000 | 0.9589 (0.0046) | 0.0405 | 0.0005 | 0.9589 | 0.9574 | 0.7771 | errors 0 | fail |
| 200×5×2 | high-g | normal | G | fiducial GPQ, signed | 0.9091 | 1.000 | 0.9458 (0.0052) | 0.0237 | 0.0305 | 0.9458 | 0.9437 | 0.0463 | errors 0 | pass |
| 200×5×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8333 | 1.000 | 0.9695 (0.0039) | 0.0284 | 0.0021 | 0.9695 | 0.9689 | 0.8060 | errors 0 | fail |
| 200×5×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0311 | 0.0221 | 0.9468 | 0.9495 | 0.6543 | errors 0 | pass |
| 200×5×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0337 | 0.0174 | 0.9489 | 0.9489 | 10.4037 | errors 0 | pass |
| 200×5×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0274 | 0.0237 | 0.9489 | 0.9521 | 48.2210 | errors 0 | pass |
| 200×5×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0279 | 0.0274 | 0.9447 | 0.9468 | 0.2670 | errors 0 | pass |
| 200×5×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0263 | 0.0316 | 0.9421 | 0.9437 | 0.3381 | errors 0 | pass |
| 200×5×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0247 | 0.0226 | 0.9526 | 0.9516 | 3.2562 | errors 0 | pass |
| 200×5×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0216 | 0.0279 | 0.9505 | 0.9532 | 0.1392 | errors 0 | pass |
| 200×5×3 | all-one | normal | G | fiducial GPQ | 0.6250 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9447 | 0.1686 | errors 0 | pass |
| 200×5×3 | all-one | normal | Phi | fiducial GPQ | 0.4545 | 1.000 | 0.9500 (0.0050) | 0.0495 | 0.0005 | 0.9500 | 0.9368 | 0.4804 | errors 0 | fail |
| 200×5×3 | all-one | normal | G | fiducial GPQ, signed | 0.6250 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9447 | 0.1686 | errors 0 | pass |
| 200×5×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.4545 | 1.000 | 0.9521 (0.0049) | 0.0468 | 0.0011 | 0.9521 | 0.9426 | 0.4986 | errors 0 | fail |
| 200×5×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0232 | 0.0279 | 0.9489 | 0.9479 | 0.5117 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0258 | 0.0232 | 0.9511 | 0.9516 | 7.9928 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9653 (0.0042) | 0.0189 | 0.0158 | 0.9653 | 0.9658 | 0.5086 | errors 0 | fail |
| 200×5×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0232 | 0.0284 | 0.9484 | 0.9474 | 0.2679 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9458 (0.0052) | 0.0284 | 0.0258 | 0.9458 | 0.9474 | 0.0446 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9479 (0.0051) | 0.0247 | 0.0274 | 0.9479 | 0.9489 | 0.0479 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0237 | 0.0237 | 0.9526 | 0.9516 | 0.1393 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | G | fiducial GPQ | 0.7874 | 1.000 | 0.9521 (0.0049) | 0.0242 | 0.0237 | 0.9521 | 0.9489 | 0.0960 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6784 | 1.000 | 0.9579 (0.0046) | 0.0321 | 0.0100 | 0.9579 | 0.9537 | 0.3979 | errors 0 | fail |
| 200×5×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.7874 | 1.000 | 0.9526 (0.0049) | 0.0226 | 0.0247 | 0.9526 | 0.9474 | 0.0963 | errors 0 | pass |
| 200×5×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6784 | 1.000 | 0.9568 (0.0047) | 0.0321 | 0.0111 | 0.9568 | 0.9547 | 0.3987 | errors 0 | fail |
| 200×5×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9558 (0.0047) | 0.0179 | 0.0263 | 0.9558 | 0.9547 | 1.7261 | errors 0 | pass |
| 200×5×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9574 (0.0046) | 0.0242 | 0.0184 | 0.9574 | 0.9616 | 5.3305 | errors 0 | pass |
| 200×5×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0295 | 0.0189 | 0.9516 | 0.9521 | 23.0614 | errors 0 | pass |
| 200×5×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0226 | 0.0258 | 0.9516 | 0.9558 | 0.1340 | errors 0 | pass |
| 200×5×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0268 | 0.0253 | 0.9479 | 0.9474 | 0.1689 | errors 0 | pass |
| 200×5×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0247 | 0.0221 | 0.9532 | 0.9511 | 1.6035 | errors 0 | pass |
| 200×5×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9458 (0.0052) | 0.0305 | 0.0237 | 0.9458 | 0.9463 | 0.0695 | errors 0 | pass |
| 200×5×3 | high-g | normal | G | fiducial GPQ | 0.9302 | 1.000 | 0.9537 (0.0048) | 0.0184 | 0.0279 | 0.9537 | 0.9505 | 0.0314 | errors 0 | pass |
| 200×5×3 | high-g | normal | Phi | fiducial GPQ | 0.8696 | 1.000 | 0.9600 (0.0045) | 0.0379 | 0.0021 | 0.9600 | 0.9600 | 0.5017 | errors 0 | fail |
| 200×5×3 | high-g | normal | G | fiducial GPQ, signed | 0.9302 | 1.000 | 0.9537 (0.0048) | 0.0184 | 0.0279 | 0.9537 | 0.9505 | 0.0314 | errors 0 | pass |
| 200×5×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.8696 | 1.000 | 0.9626 (0.0044) | 0.0353 | 0.0021 | 0.9626 | 0.9621 | 0.5067 | errors 0 | fail |
| 200×5×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0274 | 0.0237 | 0.9489 | 0.9484 | 0.5829 | errors 0 | pass |
| 200×5×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9484 | 9.3495 | errors 0 | pass |
| 200×5×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0258 | 0.0221 | 0.9521 | 0.9537 | 9.7510 | errors 0 | pass |
| 200×5×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0247 | 0.0253 | 0.9500 | 0.9500 | 0.2377 | errors 0 | pass |
| 200×5×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0253 | 0.0232 | 0.9516 | 0.9526 | 0.2373 | errors 0 | pass |
| 200×5×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0216 | 0.0321 | 0.9463 | 0.9495 | 1.7908 | errors 0 | pass |
| 200×5×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0242 | 0.0221 | 0.9537 | 0.9553 | 0.0984 | errors 0 | pass |
| 200×5×5 | all-one | normal | G | fiducial GPQ | 0.6944 | 1.000 | 0.9495 (0.0050) | 0.0258 | 0.0247 | 0.9495 | 0.9474 | 0.1297 | errors 0 | pass |
| 200×5×5 | all-one | normal | Phi | fiducial GPQ | 0.5319 | 1.000 | 0.9532 (0.0048) | 0.0437 | 0.0032 | 0.9532 | 0.9442 | 0.4077 | errors 0 | fail |
| 200×5×5 | all-one | normal | G | fiducial GPQ, signed | 0.6944 | 1.000 | 0.9495 (0.0050) | 0.0258 | 0.0247 | 0.9495 | 0.9474 | 0.1297 | errors 0 | pass |
| 200×5×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.5319 | 1.000 | 0.9532 (0.0048) | 0.0437 | 0.0032 | 0.9532 | 0.9453 | 0.4107 | errors 0 | fail |
| 200×5×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0311 | 0.0232 | 0.9458 | 0.9453 | 0.4992 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0247 | 0.0274 | 0.9479 | 0.9484 | 7.9364 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9542 (0.0048) | 0.0253 | 0.0205 | 0.9542 | 0.9553 | 0.1030 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0237 | 0.0226 | 0.9537 | 0.9532 | 0.2375 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9532 (0.0048) | 0.0258 | 0.0211 | 0.9532 | 0.9458 | 0.0334 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9463 (0.0052) | 0.0263 | 0.0274 | 0.9463 | 0.9458 | 0.0264 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9495 | 0.0983 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | G | fiducial GPQ | 0.8052 | 1.000 | 0.9426 (0.0053) | 0.0305 | 0.0268 | 0.9426 | 0.9395 | 0.0870 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.6923 | 1.000 | 0.9511 (0.0049) | 0.0284 | 0.0205 | 0.9511 | 0.9495 | 0.3892 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8052 | 1.000 | 0.9426 (0.0053) | 0.0300 | 0.0274 | 0.9426 | 0.9395 | 0.0870 | errors 0 | pass |
| 200×5×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.6923 | 1.000 | 0.9516 (0.0049) | 0.0279 | 0.0205 | 0.9516 | 0.9495 | 0.3894 | errors 0 | pass |
| 200×5×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9511 (0.0049) | 0.0289 | 0.0200 | 0.9511 | 0.9511 | 1.6892 | errors 0 | pass |
| 200×5×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0242 | 0.0226 | 0.9532 | 0.9532 | 4.7369 | errors 0 | pass |
| 200×5×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9374 (0.0056) | 0.0358 | 0.0268 | 0.9374 | 0.9368 | 4.8574 | errors 0 | fail |
| 200×5×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0232 | 0.0258 | 0.9511 | 0.9500 | 0.1186 | errors 0 | pass |
| 200×5×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0211 | 0.0258 | 0.9532 | 0.9532 | 0.1191 | errors 0 | pass |
| 200×5×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0274 | 0.0289 | 0.9437 | 0.9453 | 0.8899 | errors 0 | pass |
| 200×5×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0237 | 0.0242 | 0.9521 | 0.9537 | 0.0491 | errors 0 | pass |
| 200×5×5 | high-g | normal | G | fiducial GPQ | 0.9479 | 1.000 | 0.9542 (0.0048) | 0.0258 | 0.0200 | 0.9542 | 0.9542 | 0.0222 | errors 0 | pass |
| 200×5×5 | high-g | normal | Phi | fiducial GPQ | 0.9009 | 1.000 | 0.9537 (0.0048) | 0.0421 | 0.0042 | 0.9537 | 0.9568 | 0.2463 | errors 0 | fail |
| 200×5×5 | high-g | normal | G | fiducial GPQ, signed | 0.9479 | 1.000 | 0.9542 (0.0048) | 0.0258 | 0.0200 | 0.9542 | 0.9542 | 0.0222 | errors 0 | pass |
| 200×5×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9009 | 1.000 | 0.9547 (0.0048) | 0.0411 | 0.0042 | 0.9547 | 0.9574 | 0.2469 | errors 0 | fail |
| 200×8×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0279 | 0.0200 | 0.9521 | 0.9521 | 0.7145 | errors 0 | pass |
| 200×8×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0358 | 0.0158 | 0.9484 | 0.9489 | 5.4640 | errors 0 | fail |
| 200×8×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0221 | 0.0253 | 0.9526 | 0.9542 | 1161.2588 | errors 0 | pass |
| 200×8×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0300 | 0.0268 | 0.9432 | 0.9447 | 0.2354 | errors 0 | pass |
| 200×8×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9521 | 0.4519 | errors 0 | pass |
| 200×8×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0221 | 0.0211 | 0.9568 | 0.9574 | 3.7140 | errors 0 | pass |
| 200×8×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9395 (0.0055) | 0.0289 | 0.0316 | 0.9395 | 0.9400 | 0.1489 | errors 0 | fail |
| 200×8×2 | all-one | normal | G | fiducial GPQ | 0.5926 | 1.000 | 0.9568 (0.0047) | 0.0258 | 0.0174 | 0.9568 | 0.9558 | 0.2129 | errors 0 | pass |
| 200×8×2 | all-one | normal | Phi | fiducial GPQ | 0.4211 | 1.000 | 0.9547 (0.0048) | 0.0416 | 0.0037 | 0.9547 | 0.9353 | 0.5356 | errors 0 | fail |
| 200×8×2 | all-one | normal | G | fiducial GPQ, signed | 0.5926 | 1.000 | 0.9568 (0.0047) | 0.0258 | 0.0174 | 0.9568 | 0.9558 | 0.2129 | errors 0 | pass |
| 200×8×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.4211 | 1.000 | 0.9553 (0.0047) | 0.0400 | 0.0047 | 0.9553 | 0.9395 | 0.5558 | errors 0 | fail |
| 200×8×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0242 | 0.0247 | 0.9511 | 0.9511 | 0.4794 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0232 | 0.0195 | 0.9574 | 0.9579 | 3.6765 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9532 (0.0048) | 0.0226 | 0.0242 | 0.9532 | 0.9563 | 12.8707 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0242 | 0.0258 | 0.9500 | 0.9505 | 0.2356 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9605 (0.0045) | 0.0179 | 0.0216 | 0.9605 | 0.9632 | 0.0415 | errors 0 | fail |
| 200×8×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9458 (0.0052) | 0.0242 | 0.0300 | 0.9458 | 0.9484 | 0.0560 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0226 | 0.0284 | 0.9489 | 0.9474 | 0.1488 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | G | fiducial GPQ | 0.8386 | 1.000 | 0.9584 (0.0046) | 0.0226 | 0.0189 | 0.9584 | 0.9563 | 0.0719 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7558 | 1.000 | 0.9584 (0.0046) | 0.0400 | 0.0016 | 0.9584 | 0.9584 | 0.5411 | errors 0 | fail |
| 200×8×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8386 | 1.000 | 0.9574 (0.0046) | 0.0211 | 0.0216 | 0.9574 | 0.9532 | 0.0728 | errors 0 | pass |
| 200×8×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7558 | 1.000 | 0.9605 (0.0045) | 0.0379 | 0.0016 | 0.9605 | 0.9600 | 0.5423 | errors 0 | fail |
| 200×8×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9516 (0.0049) | 0.0232 | 0.0253 | 0.9516 | 0.9505 | 1.7476 | errors 0 | pass |
| 200×8×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9553 (0.0047) | 0.0279 | 0.0168 | 0.9553 | 0.9542 | 2.7838 | errors 0 | pass |
| 200×8×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9584 (0.0046) | 0.0247 | 0.0168 | 0.9584 | 0.9568 | 543.0020 | errors 0 | pass |
| 200×8×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0279 | 0.0221 | 0.9500 | 0.9516 | 0.1176 | errors 0 | pass |
| 200×8×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0295 | 0.0263 | 0.9442 | 0.9442 | 0.2255 | errors 0 | pass |
| 200×8×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9405 (0.0054) | 0.0284 | 0.0311 | 0.9405 | 0.9405 | 1.8616 | errors 0 | pass |
| 200×8×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0211 | 0.0258 | 0.9532 | 0.9532 | 0.0744 | errors 0 | pass |
| 200×8×2 | high-g | normal | G | fiducial GPQ | 0.9209 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9484 | 0.0412 | errors 0 | pass |
| 200×8×2 | high-g | normal | Phi | fiducial GPQ | 0.8533 | 1.000 | 0.9637 (0.0043) | 0.0316 | 0.0047 | 0.9637 | 0.9558 | 0.7745 | errors 0 | fail |
| 200×8×2 | high-g | normal | G | fiducial GPQ, signed | 0.9209 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9484 | 0.0412 | errors 0 | pass |
| 200×8×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8533 | 1.000 | 0.9653 (0.0042) | 0.0279 | 0.0068 | 0.9653 | 0.9568 | 0.7812 | errors 0 | fail |
| 200×8×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0279 | 0.0232 | 0.9489 | 0.9479 | 0.6099 | errors 0 | pass |
| 200×8×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0221 | 0.0179 | 0.9600 | 0.9605 | 4.9974 | errors 0 | pass |
| 200×8×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0184 | 0.0232 | 0.9584 | 0.9584 | 43.8146 | errors 0 | pass |
| 200×8×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0263 | 0.0232 | 0.9505 | 0.9495 | 0.2015 | errors 0 | pass |
| 200×8×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0300 | 0.0216 | 0.9484 | 0.9489 | 0.3159 | errors 0 | pass |
| 200×8×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0242 | 0.0216 | 0.9542 | 0.9542 | 1.9570 | errors 0 | pass |
| 200×8×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0226 | 0.0274 | 0.9500 | 0.9505 | 0.1051 | errors 0 | pass |
| 200×8×3 | all-one | normal | G | fiducial GPQ | 0.6667 | 1.000 | 0.9447 (0.0052) | 0.0295 | 0.0258 | 0.9447 | 0.9442 | 0.1516 | errors 0 | pass |
| 200×8×3 | all-one | normal | Phi | fiducial GPQ | 0.5000 | 1.000 | 0.9616 (0.0044) | 0.0363 | 0.0021 | 0.9616 | 0.9500 | 0.4991 | errors 0 | fail |
| 200×8×3 | all-one | normal | G | fiducial GPQ, signed | 0.6667 | 1.000 | 0.9447 (0.0052) | 0.0295 | 0.0258 | 0.9447 | 0.9442 | 0.1516 | errors 0 | pass |
| 200×8×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.5000 | 1.000 | 0.9621 (0.0044) | 0.0358 | 0.0021 | 0.9621 | 0.9511 | 0.5031 | errors 0 | fail |
| 200×8×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0316 | 0.0263 | 0.9421 | 0.9432 | 0.4694 | errors 0 | pass |
| 200×8×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0289 | 0.0258 | 0.9453 | 0.9458 | 3.7734 | errors 0 | pass |
| 200×8×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9484 (0.0051) | 0.0237 | 0.0279 | 0.9484 | 0.9495 | 0.4724 | errors 0 | pass |
| 200×8×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0247 | 0.0300 | 0.9453 | 0.9447 | 0.2018 | errors 0 | pass |
| 200×8×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9553 (0.0047) | 0.0263 | 0.0184 | 0.9553 | 0.9542 | 0.0310 | errors 0 | pass |
| 200×8×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9363 (0.0056) | 0.0268 | 0.0368 | 0.9363 | 0.9379 | 0.0293 | errors 0 | fail |
| 200×8×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9526 | 0.1051 | errors 0 | pass |
| 200×8×3 | near-zero-rater | normal | G | fiducial GPQ | 0.8547 | 1.000 | 0.9416 (0.0054) | 0.0353 | 0.0232 | 0.9416 | 0.9384 | 0.0626 | errors 0 | fail |
| 200×8×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7700 | 1.000 | 0.9479 (0.0051) | 0.0426 | 0.0095 | 0.9479 | 0.9489 | 0.2632 | errors 0 | fail |
| 200×8×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8547 | 1.000 | 0.9389 (0.0055) | 0.0353 | 0.0258 | 0.9389 | 0.9368 | 0.0627 | errors 0 | fail |
| 200×8×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7700 | 1.000 | 0.9484 (0.0051) | 0.0421 | 0.0095 | 0.9484 | 0.9489 | 0.2634 | errors 0 | fail |
| 200×8×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9474 (0.0051) | 0.0216 | 0.0311 | 0.9474 | 0.9468 | 1.7077 | errors 0 | pass |
| 200×8×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0253 | 0.0179 | 0.9568 | 0.9568 | 2.4463 | errors 0 | pass |
| 200×8×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0247 | 0.0268 | 0.9484 | 0.9489 | 22.2902 | errors 0 | pass |
| 200×8×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0247 | 0.0211 | 0.9542 | 0.9537 | 0.1007 | errors 0 | pass |
| 200×8×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0268 | 0.0226 | 0.9505 | 0.9500 | 0.1578 | errors 0 | pass |
| 200×8×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0232 | 0.0274 | 0.9495 | 0.9463 | 0.9677 | errors 0 | pass |
| 200×8×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9463 (0.0052) | 0.0253 | 0.0284 | 0.9463 | 0.9463 | 0.0526 | errors 0 | pass |
| 200×8×3 | high-g | normal | G | fiducial GPQ | 0.9412 | 1.000 | 0.9526 (0.0049) | 0.0242 | 0.0232 | 0.9526 | 0.9521 | 0.0266 | errors 0 | pass |
| 200×8×3 | high-g | normal | Phi | fiducial GPQ | 0.8889 | 1.000 | 0.9579 (0.0046) | 0.0353 | 0.0068 | 0.9579 | 0.9558 | 0.4772 | errors 0 | fail |
| 200×8×3 | high-g | normal | G | fiducial GPQ, signed | 0.9412 | 1.000 | 0.9526 (0.0049) | 0.0242 | 0.0232 | 0.9526 | 0.9521 | 0.0266 | errors 0 | pass |
| 200×8×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.8889 | 1.000 | 0.9574 (0.0046) | 0.0353 | 0.0074 | 0.9574 | 0.9553 | 0.4782 | errors 0 | fail |
| 200×8×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0221 | 0.0268 | 0.9511 | 0.9516 | 0.5453 | errors 0 | pass |
| 200×8×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0232 | 0.0247 | 0.9521 | 0.9521 | 4.6021 | errors 0 | pass |
| 200×8×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0221 | 0.0289 | 0.9489 | 0.9500 | 9.1416 | errors 0 | pass |
| 200×8×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0279 | 0.0226 | 0.9495 | 0.9484 | 0.1791 | errors 0 | pass |
| 200×8×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0211 | 0.0242 | 0.9547 | 0.9553 | 0.2221 | errors 0 | pass |
| 200×8×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0205 | 0.0216 | 0.9579 | 0.9574 | 1.2125 | errors 0 | pass |
| 200×8×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9526 | 0.0743 | errors 0 | pass |
| 200×8×5 | all-one | normal | G | fiducial GPQ | 0.7407 | 1.000 | 0.9500 (0.0050) | 0.0216 | 0.0284 | 0.9500 | 0.9495 | 0.1090 | errors 0 | pass |
| 200×8×5 | all-one | normal | Phi | fiducial GPQ | 0.5882 | 1.000 | 0.9479 (0.0051) | 0.0479 | 0.0042 | 0.9479 | 0.9453 | 0.3695 | errors 0 | fail |
| 200×8×5 | all-one | normal | G | fiducial GPQ, signed | 0.7407 | 1.000 | 0.9500 (0.0050) | 0.0216 | 0.0284 | 0.9500 | 0.9495 | 0.1090 | errors 0 | pass |
| 200×8×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.5882 | 1.000 | 0.9479 (0.0051) | 0.0479 | 0.0042 | 0.9479 | 0.9453 | 0.3699 | errors 0 | fail |
| 200×8×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0247 | 0.0311 | 0.9442 | 0.9442 | 0.4620 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0205 | 0.0216 | 0.9579 | 0.9579 | 3.7442 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9521 (0.0049) | 0.0216 | 0.0263 | 0.9521 | 0.9542 | 0.0954 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0226 | 0.0253 | 0.9521 | 0.9516 | 0.1793 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9558 (0.0047) | 0.0205 | 0.0237 | 0.9558 | 0.9558 | 0.0238 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9479 (0.0051) | 0.0274 | 0.0247 | 0.9479 | 0.9463 | 0.0179 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0242 | 0.0337 | 0.9421 | 0.9426 | 0.0743 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | G | fiducial GPQ | 0.8681 | 1.000 | 0.9489 (0.0050) | 0.0242 | 0.0268 | 0.9489 | 0.9447 | 0.0562 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.7817 | 1.000 | 0.9605 (0.0045) | 0.0258 | 0.0137 | 0.9605 | 0.9605 | 0.2365 | errors 0 | fail |
| 200×8×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.8681 | 1.000 | 0.9489 (0.0050) | 0.0237 | 0.0274 | 0.9489 | 0.9453 | 0.0562 | errors 0 | pass |
| 200×8×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.7817 | 1.000 | 0.9611 (0.0044) | 0.0253 | 0.0137 | 0.9611 | 0.9611 | 0.2366 | errors 0 | fail |
| 200×8×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9453 (0.0052) | 0.0274 | 0.0274 | 0.9453 | 0.9447 | 1.6691 | errors 0 | pass |
| 200×8×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0268 | 0.0237 | 0.9495 | 0.9500 | 2.2225 | errors 0 | pass |
| 200×8×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9463 (0.0052) | 0.0232 | 0.0305 | 0.9463 | 0.9532 | 4.6580 | errors 0 | pass |
| 200×8×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0274 | 0.0289 | 0.9437 | 0.9442 | 0.0896 | errors 0 | pass |
| 200×8×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9458 (0.0052) | 0.0284 | 0.0258 | 0.9458 | 0.9458 | 0.1110 | errors 0 | pass |
| 200×8×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0226 | 0.0274 | 0.9500 | 0.9511 | 0.6086 | errors 0 | pass |
| 200×8×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0258 | 0.0237 | 0.9505 | 0.9500 | 0.0371 | errors 0 | pass |
| 200×8×5 | high-g | normal | G | fiducial GPQ | 0.9581 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9479 | 0.0177 | errors 0 | pass |
| 200×8×5 | high-g | normal | Phi | fiducial GPQ | 0.9195 | 1.000 | 0.9479 (0.0051) | 0.0479 | 0.0042 | 0.9479 | 0.9537 | 0.1792 | errors 0 | fail |
| 200×8×5 | high-g | normal | G | fiducial GPQ, signed | 0.9581 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9479 | 0.0177 | errors 0 | pass |
| 200×8×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9195 | 1.000 | 0.9474 (0.0051) | 0.0479 | 0.0047 | 0.9474 | 0.9532 | 0.1793 | errors 0 | fail |
| 200×20×2 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0274 | 0.0200 | 0.9526 | 0.9532 | 0.6646 | errors 0 | pass |
| 200×20×2 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0305 | 0.0174 | 0.9521 | 0.9537 | 2.3908 | errors 0 | pass |
| 200×20×2 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0247 | 0.0216 | 0.9537 | 0.9542 | 1062.4410 | errors 0 | pass |
| 200×20×2 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0247 | 0.0232 | 0.9521 | 0.9521 | 0.1427 | errors 0 | pass |
| 200×20×2 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0211 | 0.0311 | 0.9479 | 0.9479 | 0.4203 | errors 0 | pass |
| 200×20×2 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0295 | 0.0279 | 0.9426 | 0.9426 | 1.5547 | errors 0 | pass |
| 200×20×2 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0158 | 0.0305 | 0.9537 | 0.9553 | 0.0903 | errors 0 | pass |
| 200×20×2 | all-one | normal | G | fiducial GPQ | 0.6349 | 1.000 | 0.9616 (0.0044) | 0.0226 | 0.0158 | 0.9616 | 0.9589 | 0.1995 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | fiducial GPQ | 0.4651 | 1.000 | 0.9532 (0.0048) | 0.0347 | 0.0121 | 0.9532 | 0.9284 | 0.5858 | errors 0 | fail |
| 200×20×2 | all-one | normal | G | fiducial GPQ, signed | 0.6349 | 1.000 | 0.9616 (0.0044) | 0.0226 | 0.0158 | 0.9616 | 0.9589 | 0.1995 | errors 0 | fail |
| 200×20×2 | all-one | normal | Phi | fiducial GPQ, signed | 0.4651 | 1.000 | 0.9526 (0.0049) | 0.0347 | 0.0126 | 0.9526 | 0.9284 | 0.5878 | errors 0 | fail |
| 200×20×2 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0216 | 0.0237 | 0.9547 | 0.9558 | 0.4334 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0284 | 0.0247 | 0.9468 | 0.9468 | 1.5689 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9521 (0.0049) | 0.0274 | 0.0205 | 0.9521 | 0.9532 | 11.2575 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0263 | 0.0263 | 0.9474 | 0.9474 | 0.1426 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9405 (0.0054) | 0.0300 | 0.0295 | 0.9405 | 0.9411 | 0.0220 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9442 (0.0053) | 0.0289 | 0.0268 | 0.9442 | 0.9442 | 0.0235 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0226 | 0.0268 | 0.9505 | 0.9516 | 0.0902 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | G | fiducial GPQ | 0.9259 | 1.000 | 0.9568 (0.0047) | 0.0211 | 0.0221 | 0.9568 | 0.9563 | 0.0320 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8809 | 1.000 | 0.9505 (0.0050) | 0.0479 | 0.0016 | 0.9505 | 0.9532 | 0.5469 | errors 0 | fail |
| 200×20×2 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9259 | 1.000 | 0.9558 (0.0047) | 0.0200 | 0.0242 | 0.9558 | 0.9547 | 0.0321 | errors 0 | pass |
| 200×20×2 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8809 | 1.000 | 0.9511 (0.0049) | 0.0474 | 0.0016 | 0.9511 | 0.9542 | 0.5470 | errors 0 | fail |
| 200×20×2 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9484 (0.0051) | 0.0258 | 0.0258 | 0.9484 | 0.9495 | 1.7212 | errors 0 | pass |
| 200×20×2 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9579 (0.0046) | 0.0268 | 0.0153 | 0.9579 | 0.9584 | 1.2213 | errors 0 | pass |
| 200×20×2 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9458 (0.0052) | 0.0263 | 0.0279 | 0.9458 | 0.9505 | 556.1709 | errors 0 | pass |
| 200×20×2 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0258 | 0.0205 | 0.9537 | 0.9542 | 0.0713 | errors 0 | pass |
| 200×20×2 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0221 | 0.0232 | 0.9547 | 0.9542 | 0.2098 | errors 0 | pass |
| 200×20×2 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0205 | 0.0263 | 0.9532 | 0.9542 | 0.7797 | errors 0 | pass |
| 200×20×2 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0226 | 0.0205 | 0.9568 | 0.9589 | 0.0451 | errors 0 | pass |
| 200×20×2 | high-g | normal | G | fiducial GPQ | 0.9329 | 1.000 | 0.9484 (0.0051) | 0.0279 | 0.0237 | 0.9484 | 0.9479 | 0.0365 | errors 0 | pass |
| 200×20×2 | high-g | normal | Phi | fiducial GPQ | 0.8743 | 1.000 | 0.9479 (0.0051) | 0.0295 | 0.0226 | 0.9479 | 0.9421 | 0.7898 | errors 0 | pass |
| 200×20×2 | high-g | normal | G | fiducial GPQ, signed | 0.9329 | 1.000 | 0.9484 (0.0051) | 0.0279 | 0.0237 | 0.9484 | 0.9479 | 0.0365 | errors 0 | pass |
| 200×20×2 | high-g | normal | Phi | fiducial GPQ, signed | 0.8743 | 1.000 | 0.9474 (0.0051) | 0.0295 | 0.0232 | 0.9474 | 0.9421 | 0.7903 | errors 0 | pass |
| 200×20×3 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0226 | 0.0300 | 0.9474 | 0.9479 | 0.5707 | errors 0 | pass |
| 200×20×3 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0268 | 0.0174 | 0.9558 | 0.9547 | 2.1028 | errors 0 | pass |
| 200×20×3 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0205 | 0.0211 | 0.9584 | 0.9589 | 40.7815 | errors 0 | pass |
| 200×20×3 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0242 | 0.0253 | 0.9505 | 0.9505 | 0.1221 | errors 0 | pass |
| 200×20×3 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0284 | 0.0295 | 0.9421 | 0.9400 | 0.2941 | errors 0 | pass |
| 200×20×3 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9405 (0.0054) | 0.0321 | 0.0274 | 0.9405 | 0.9416 | 1.0023 | errors 0 | pass |
| 200×20×3 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0226 | 0.0258 | 0.9516 | 0.9516 | 0.0638 | errors 0 | pass |
| 200×20×3 | all-one | normal | G | fiducial GPQ | 0.7143 | 1.000 | 0.9489 (0.0050) | 0.0216 | 0.0295 | 0.9489 | 0.9437 | 0.1338 | errors 0 | pass |
| 200×20×3 | all-one | normal | Phi | fiducial GPQ | 0.5556 | 1.000 | 0.9584 (0.0046) | 0.0242 | 0.0174 | 0.9584 | 0.9479 | 0.5378 | errors 0 | pass |
| 200×20×3 | all-one | normal | G | fiducial GPQ, signed | 0.7143 | 1.000 | 0.9489 (0.0050) | 0.0216 | 0.0295 | 0.9489 | 0.9437 | 0.1338 | errors 0 | pass |
| 200×20×3 | all-one | normal | Phi | fiducial GPQ, signed | 0.5556 | 1.000 | 0.9584 (0.0046) | 0.0242 | 0.0174 | 0.9584 | 0.9474 | 0.5382 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0253 | 0.0237 | 0.9511 | 0.9511 | 0.4281 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0300 | 0.0226 | 0.9474 | 0.9474 | 1.5666 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9558 (0.0047) | 0.0211 | 0.0232 | 0.9558 | 0.9526 | 0.4138 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0284 | 0.0189 | 0.9526 | 0.9521 | 0.1222 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9447 (0.0052) | 0.0321 | 0.0232 | 0.9447 | 0.9458 | 0.0164 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9532 (0.0048) | 0.0247 | 0.0221 | 0.9532 | 0.9521 | 0.0148 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0216 | 0.0300 | 0.9484 | 0.9458 | 0.0638 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | G | fiducial GPQ | 0.9346 | 1.000 | 0.9568 (0.0047) | 0.0216 | 0.0216 | 0.9568 | 0.9547 | 0.0271 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8901 | 1.000 | 0.9584 (0.0046) | 0.0389 | 0.0026 | 0.9584 | 0.9626 | 0.1330 | errors 0 | fail |
| 200×20×3 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9346 | 1.000 | 0.9568 (0.0047) | 0.0216 | 0.0216 | 0.9568 | 0.9547 | 0.0272 | errors 0 | pass |
| 200×20×3 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8901 | 1.000 | 0.9584 (0.0046) | 0.0389 | 0.0026 | 0.9584 | 0.9626 | 0.1330 | errors 0 | fail |
| 200×20×3 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9421 (0.0054) | 0.0274 | 0.0305 | 0.9421 | 0.9416 | 1.6853 | errors 0 | pass |
| 200×20×3 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0300 | 0.0221 | 0.9479 | 0.9495 | 1.0692 | errors 0 | pass |
| 200×20×3 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9405 (0.0054) | 0.0321 | 0.0274 | 0.9405 | 0.9411 | 20.7686 | errors 0 | pass |
| 200×20×3 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0237 | 0.0205 | 0.9558 | 0.9584 | 0.0610 | errors 0 | pass |
| 200×20×3 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9421 (0.0054) | 0.0289 | 0.0289 | 0.9421 | 0.9421 | 0.1472 | errors 0 | pass |
| 200×20×3 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0258 | 0.0247 | 0.9495 | 0.9489 | 0.4977 | errors 0 | pass |
| 200×20×3 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0268 | 0.0295 | 0.9437 | 0.9447 | 0.0319 | errors 0 | pass |
| 200×20×3 | high-g | normal | G | fiducial GPQ | 0.9524 | 1.000 | 0.9389 (0.0055) | 0.0295 | 0.0316 | 0.9389 | 0.9389 | 0.0223 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | fiducial GPQ | 0.9091 | 1.000 | 0.9426 (0.0053) | 0.0295 | 0.0279 | 0.9426 | 0.9416 | 0.4690 | errors 0 | pass |
| 200×20×3 | high-g | normal | G | fiducial GPQ, signed | 0.9524 | 1.000 | 0.9389 (0.0055) | 0.0295 | 0.0316 | 0.9389 | 0.9389 | 0.0223 | errors 0 | fail |
| 200×20×3 | high-g | normal | Phi | fiducial GPQ, signed | 0.9091 | 1.000 | 0.9421 (0.0054) | 0.0295 | 0.0284 | 0.9421 | 0.9416 | 0.4691 | errors 0 | pass |
| 200×20×5 | all-one | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9495 | 0.5050 | errors 0 | pass |
| 200×20×5 | all-one | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0253 | 0.0253 | 0.9495 | 0.9495 | 1.8767 | errors 0 | pass |
| 200×20×5 | all-one | normal | r | fiducial GPQ | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0232 | 0.0268 | 0.9500 | 0.9516 | 8.4196 | errors 0 | pass |
| 200×20×5 | all-one | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0226 | 0.0284 | 0.9489 | 0.9500 | 0.1087 | errors 0 | pass |
| 200×20×5 | all-one | normal | pr | fiducial GPQ | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0305 | 0.0232 | 0.9463 | 0.9484 | 0.2068 | errors 0 | pass |
| 200×20×5 | all-one | normal | ir | fiducial GPQ | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0263 | 0.0179 | 0.9558 | 0.9542 | 0.6691 | errors 0 | pass |
| 200×20×5 | all-one | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0216 | 0.0232 | 0.9553 | 0.9553 | 0.0451 | errors 0 | pass |
| 200×20×5 | all-one | normal | G | fiducial GPQ | 0.7937 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9463 | 0.0887 | errors 0 | pass |
| 200×20×5 | all-one | normal | Phi | fiducial GPQ | 0.6579 | 1.000 | 0.9384 (0.0055) | 0.0447 | 0.0168 | 0.9384 | 0.9368 | 0.3750 | errors 0 | fail |
| 200×20×5 | all-one | normal | G | fiducial GPQ, signed | 0.7937 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9463 | 0.0887 | errors 0 | pass |
| 200×20×5 | all-one | normal | Phi | fiducial GPQ, signed | 0.6579 | 1.000 | 0.9384 (0.0055) | 0.0447 | 0.0168 | 0.9384 | 0.9368 | 0.3750 | errors 0 | fail |
| 200×20×5 | near-zero-rater | normal | p | fiducial GPQ | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0279 | 0.0263 | 0.9458 | 0.9458 | 0.4250 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | i | fiducial GPQ | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0263 | 0.0268 | 0.9468 | 0.9458 | 1.5577 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | r | fiducial GPQ | 0.0100 | 1.000 | 0.9489 (0.0050) | 0.0247 | 0.0263 | 0.9489 | 0.9484 | 0.0856 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | pi | fiducial GPQ | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0263 | 0.0258 | 0.9479 | 0.9479 | 0.1086 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | pr | fiducial GPQ | 0.0100 | 1.000 | 0.9589 (0.0046) | 0.0158 | 0.0253 | 0.9589 | 0.9589 | 0.0120 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | ir | fiducial GPQ | 0.0100 | 1.000 | 0.9495 (0.0050) | 0.0195 | 0.0311 | 0.9495 | 0.9495 | 0.0100 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | pir | fiducial GPQ | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0247 | 0.0232 | 0.9521 | 0.9516 | 0.0451 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | G | fiducial GPQ | 0.9416 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9463 | 0.0238 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | Phi | fiducial GPQ | 0.8976 | 1.000 | 0.9521 (0.0049) | 0.0316 | 0.0163 | 0.9521 | 0.9542 | 0.0768 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | G | fiducial GPQ, signed | 0.9416 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9463 | 0.0238 | errors 0 | pass |
| 200×20×5 | near-zero-rater | normal | Phi | fiducial GPQ, signed | 0.8976 | 1.000 | 0.9521 (0.0049) | 0.0316 | 0.0163 | 0.9521 | 0.9542 | 0.0768 | errors 0 | pass |
| 200×20×5 | high-g | normal | p | fiducial GPQ | 4.0000 | 1.000 | 0.9532 (0.0048) | 0.0221 | 0.0247 | 0.9532 | 0.9526 | 1.6498 | errors 0 | pass |
| 200×20×5 | high-g | normal | i | fiducial GPQ | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0268 | 0.0279 | 0.9453 | 0.9458 | 0.9336 | errors 0 | pass |
| 200×20×5 | high-g | normal | r | fiducial GPQ | 0.5000 | 1.000 | 0.9474 (0.0051) | 0.0237 | 0.0289 | 0.9474 | 0.9474 | 4.2175 | errors 0 | pass |
| 200×20×5 | high-g | normal | pi | fiducial GPQ | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0221 | 0.0295 | 0.9484 | 0.9479 | 0.0543 | errors 0 | pass |
| 200×20×5 | high-g | normal | pr | fiducial GPQ | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0253 | 0.0242 | 0.9505 | 0.9516 | 0.1035 | errors 0 | pass |
| 200×20×5 | high-g | normal | ir | fiducial GPQ | 0.5000 | 1.000 | 0.9458 (0.0052) | 0.0295 | 0.0247 | 0.9458 | 0.9442 | 0.3341 | errors 0 | pass |
| 200×20×5 | high-g | normal | pir | fiducial GPQ | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0242 | 0.0237 | 0.9521 | 0.9521 | 0.0225 | errors 0 | pass |
| 200×20×5 | high-g | normal | G | fiducial GPQ | 0.9685 | 1.000 | 0.9474 (0.0051) | 0.0247 | 0.0279 | 0.9474 | 0.9484 | 0.0135 | errors 0 | pass |
| 200×20×5 | high-g | normal | Phi | fiducial GPQ | 0.9390 | 1.000 | 0.9432 (0.0053) | 0.0395 | 0.0174 | 0.9432 | 0.9474 | 0.1552 | errors 0 | fail |
| 200×20×5 | high-g | normal | G | fiducial GPQ, signed | 0.9685 | 1.000 | 0.9474 (0.0051) | 0.0247 | 0.0279 | 0.9474 | 0.9484 | 0.0135 | errors 0 | pass |
| 200×20×5 | high-g | normal | Phi | fiducial GPQ, signed | 0.9390 | 1.000 | 0.9432 (0.0053) | 0.0395 | 0.0174 | 0.9432 | 0.9474 | 0.1552 | errors 0 | fail |
| 50×5×2 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9163 (0.0064) | 0.0611 | 0.0226 | 0.9163 | 0.9153 | 0.0459 | errors 0 | fail |
| 50×5×2 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9663 (0.0041) | 0.0258 | 0.0079 | 0.9663 | 0.9653 | 0.2715 | errors 0 | fail |
| 50×5×2 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9663 (0.0041) | 0.0205 | 0.0132 | 0.9663 | 0.9684 | 28.9508 | errors 0 | fail |
| 50×5×2 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.8784 (0.0075) | 0.0726 | 0.0489 | 0.8784 | 0.8763 | 0.0368 | errors 0 | fail |
| 50×5×2 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.9068 (0.0067) | 0.0495 | 0.0437 | 0.9068 | 0.9089 | 0.0407 | errors 0 | fail |
| 50×5×2 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9537 (0.0048) | 0.0247 | 0.0216 | 0.9537 | 0.9537 | 0.2122 | errors 0 | pass |
| 50×5×2 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.8495 (0.0082) | 0.0932 | 0.0574 | 0.8495 | 0.8495 | 0.0430 | errors 0 | fail |
| 50×5×2 | all-one | binary | G | fiducial GPQ | 0.4481 | 1.000 | 0.9495 (0.0050) | 0.0326 | 0.0179 | 0.9495 | 0.9411 | 0.5487 | errors 0 | pass |
| 50×5×2 | all-one | binary | Phi | fiducial GPQ | 0.3289 | 1.000 | 0.9184 (0.0063) | 0.0805 | 0.0011 | 0.9184 | 0.9289 | 0.5091 | errors 0 | fail |
| 50×5×2 | all-one | binary | G | fiducial GPQ, signed | 0.4481 | 1.000 | 0.9484 (0.0051) | 0.0326 | 0.0189 | 0.9484 | 0.9411 | 0.5507 | errors 0 | pass |
| 50×5×2 | all-one | binary | Phi | fiducial GPQ, signed | 0.3289 | 1.000 | 0.9384 (0.0055) | 0.0605 | 0.0011 | 0.9384 | 0.9463 | 0.5722 | errors 0 | fail |
| 50×5×3 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9337 (0.0057) | 0.0416 | 0.0247 | 0.9337 | 0.9347 | 0.0385 | errors 0 | fail |
| 50×5×3 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9589 (0.0046) | 0.0226 | 0.0184 | 0.9589 | 0.9600 | 0.2579 | errors 0 | pass |
| 50×5×3 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9621 (0.0044) | 0.0205 | 0.0174 | 0.9621 | 0.9621 | 1.1129 | errors 0 | fail |
| 50×5×3 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.8537 (0.0081) | 0.0874 | 0.0589 | 0.8537 | 0.8558 | 0.0264 | errors 0 | fail |
| 50×5×3 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.8811 (0.0074) | 0.0663 | 0.0526 | 0.8811 | 0.8821 | 0.0277 | errors 0 | fail |
| 50×5×3 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9589 (0.0046) | 0.0200 | 0.0211 | 0.9589 | 0.9616 | 0.0879 | errors 0 | pass |
| 50×5×3 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.8505 (0.0082) | 0.0900 | 0.0595 | 0.8505 | 0.8484 | 0.0302 | errors 0 | fail |
| 50×5×3 | all-one | binary | G | fiducial GPQ | 0.5280 | 1.000 | 0.9711 (0.0038) | 0.0174 | 0.0116 | 0.9711 | 0.9637 | 0.4570 | errors 0 | fail |
| 50×5×3 | all-one | binary | Phi | fiducial GPQ | 0.4000 | 1.000 | 0.9400 (0.0054) | 0.0595 | 0.0005 | 0.9400 | 0.9489 | 0.5098 | errors 0 | fail |
| 50×5×3 | all-one | binary | G | fiducial GPQ, signed | 0.5280 | 1.000 | 0.9711 (0.0038) | 0.0174 | 0.0116 | 0.9711 | 0.9637 | 0.4571 | errors 0 | fail |
| 50×5×3 | all-one | binary | Phi | fiducial GPQ, signed | 0.4000 | 1.000 | 0.9463 (0.0052) | 0.0532 | 0.0005 | 0.9463 | 0.9516 | 0.5224 | errors 0 | fail |
| 50×5×5 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9316 (0.0058) | 0.0421 | 0.0263 | 0.9316 | 0.9321 | 0.0321 | errors 0 | fail |
| 50×5×5 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9589 (0.0046) | 0.0263 | 0.0147 | 0.9589 | 0.9589 | 0.2221 | errors 0 | fail |
| 50×5×5 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9674 (0.0041) | 0.0163 | 0.0163 | 0.9674 | 0.9700 | 0.2320 | errors 0 | fail |
| 50×5×5 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.8537 (0.0081) | 0.0853 | 0.0611 | 0.8537 | 0.8547 | 0.0191 | errors 0 | fail |
| 50×5×5 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.8642 (0.0079) | 0.0800 | 0.0558 | 0.8642 | 0.8642 | 0.0191 | errors 0 | fail |
| 50×5×5 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9484 (0.0051) | 0.0279 | 0.0237 | 0.9484 | 0.9489 | 0.0474 | errors 0 | pass |
| 50×5×5 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.8021 (0.0091) | 0.1137 | 0.0842 | 0.8021 | 0.8032 | 0.0212 | errors 0 | fail |
| 50×5×5 | all-one | binary | G | fiducial GPQ | 0.6160 | 1.000 | 0.9616 (0.0044) | 0.0216 | 0.0168 | 0.9616 | 0.9637 | 0.3537 | errors 0 | fail |
| 50×5×5 | all-one | binary | Phi | fiducial GPQ | 0.4838 | 1.000 | 0.9389 (0.0055) | 0.0595 | 0.0016 | 0.9389 | 0.9458 | 0.4569 | errors 0 | fail |
| 50×5×5 | all-one | binary | G | fiducial GPQ, signed | 0.6160 | 1.000 | 0.9616 (0.0044) | 0.0216 | 0.0168 | 0.9616 | 0.9637 | 0.3537 | errors 0 | fail |
| 50×5×5 | all-one | binary | Phi | fiducial GPQ, signed | 0.4838 | 1.000 | 0.9384 (0.0055) | 0.0595 | 0.0021 | 0.9384 | 0.9463 | 0.4589 | errors 0 | fail |
| 50×8×2 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9274 (0.0060) | 0.0484 | 0.0242 | 0.9274 | 0.9284 | 0.0415 | errors 0 | fail |
| 50×8×2 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9563 (0.0047) | 0.0268 | 0.0168 | 0.9563 | 0.9553 | 0.1307 | errors 0 | pass |
| 50×8×2 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9532 (0.0048) | 0.0253 | 0.0216 | 0.9532 | 0.9537 | 26.7005 | errors 0 | pass |
| 50×8×2 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.8526 (0.0081) | 0.0937 | 0.0537 | 0.8526 | 0.8532 | 0.0284 | errors 0 | fail |
| 50×8×2 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.9095 (0.0066) | 0.0542 | 0.0363 | 0.9095 | 0.9084 | 0.0330 | errors 0 | fail |
| 50×8×2 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9437 (0.0053) | 0.0268 | 0.0295 | 0.9437 | 0.9484 | 0.1021 | errors 0 | pass |
| 50×8×2 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.8211 (0.0088) | 0.0995 | 0.0795 | 0.8211 | 0.8232 | 0.0325 | errors 0 | fail |
| 50×8×2 | all-one | binary | G | fiducial GPQ | 0.5065 | 1.000 | 0.9558 (0.0047) | 0.0205 | 0.0237 | 0.9558 | 0.9558 | 0.5256 | errors 0 | pass |
| 50×8×2 | all-one | binary | Phi | fiducial GPQ | 0.3749 | 1.000 | 0.9337 (0.0057) | 0.0647 | 0.0016 | 0.9337 | 0.9316 | 0.5595 | errors 0 | fail |
| 50×8×2 | all-one | binary | G | fiducial GPQ, signed | 0.5065 | 1.000 | 0.9563 (0.0047) | 0.0200 | 0.0237 | 0.9563 | 0.9558 | 0.5258 | errors 0 | pass |
| 50×8×2 | all-one | binary | Phi | fiducial GPQ, signed | 0.3749 | 1.000 | 0.9363 (0.0056) | 0.0616 | 0.0021 | 0.9363 | 0.9353 | 0.5773 | errors 0 | fail |
| 50×8×3 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9221 (0.0061) | 0.0411 | 0.0368 | 0.9221 | 0.9221 | 0.0341 | errors 0 | fail |
| 50×8×3 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9679 (0.0040) | 0.0174 | 0.0147 | 0.9679 | 0.9674 | 0.1201 | errors 0 | fail |
| 50×8×3 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9589 (0.0046) | 0.0253 | 0.0158 | 0.9589 | 0.9584 | 1.0234 | errors 0 | pass |
| 50×8×3 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.8332 (0.0086) | 0.0926 | 0.0742 | 0.8332 | 0.8332 | 0.0199 | errors 0 | fail |
| 50×8×3 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.8826 (0.0074) | 0.0611 | 0.0563 | 0.8826 | 0.8837 | 0.0226 | errors 0 | fail |
| 50×8×3 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9553 (0.0047) | 0.0232 | 0.0216 | 0.9553 | 0.9553 | 0.0533 | errors 0 | pass |
| 50×8×3 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.8074 (0.0090) | 0.1011 | 0.0916 | 0.8074 | 0.8121 | 0.0228 | errors 0 | fail |
| 50×8×3 | all-one | binary | G | fiducial GPQ | 0.5900 | 1.000 | 0.9542 (0.0048) | 0.0242 | 0.0216 | 0.9542 | 0.9368 | 0.4032 | errors 0 | pass |
| 50×8×3 | all-one | binary | Phi | fiducial GPQ | 0.4548 | 1.000 | 0.9389 (0.0055) | 0.0563 | 0.0047 | 0.9389 | 0.9453 | 0.5258 | errors 0 | fail |
| 50×8×3 | all-one | binary | G | fiducial GPQ, signed | 0.5900 | 1.000 | 0.9542 (0.0048) | 0.0242 | 0.0216 | 0.9542 | 0.9368 | 0.4032 | errors 0 | pass |
| 50×8×3 | all-one | binary | Phi | fiducial GPQ, signed | 0.4548 | 1.000 | 0.9389 (0.0055) | 0.0558 | 0.0053 | 0.9389 | 0.9458 | 0.5291 | errors 0 | fail |
| 50×8×5 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9411 (0.0054) | 0.0332 | 0.0258 | 0.9411 | 0.9368 | 0.0290 | errors 0 | pass |
| 50×8×5 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9647 (0.0042) | 0.0211 | 0.0142 | 0.9647 | 0.9647 | 0.1068 | errors 0 | fail |
| 50×8×5 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9632 (0.0043) | 0.0242 | 0.0126 | 0.9632 | 0.9632 | 0.2071 | errors 0 | fail |
| 50×8×5 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.8232 (0.0088) | 0.0995 | 0.0774 | 0.8232 | 0.8226 | 0.0144 | errors 0 | fail |
| 50×8×5 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.8574 (0.0080) | 0.0858 | 0.0568 | 0.8574 | 0.8574 | 0.0155 | errors 0 | fail |
| 50×8×5 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9484 (0.0051) | 0.0232 | 0.0284 | 0.9484 | 0.9484 | 0.0324 | errors 0 | pass |
| 50×8×5 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.7621 (0.0098) | 0.1311 | 0.1068 | 0.7621 | 0.7605 | 0.0160 | errors 0 | fail |
| 50×8×5 | all-one | binary | G | fiducial GPQ | 0.6796 | 1.000 | 0.9558 (0.0047) | 0.0195 | 0.0247 | 0.9558 | 0.9511 | 0.2856 | errors 0 | pass |
| 50×8×5 | all-one | binary | Phi | fiducial GPQ | 0.5484 | 1.000 | 0.9474 (0.0051) | 0.0468 | 0.0058 | 0.9474 | 0.9542 | 0.4256 | errors 0 | fail |
| 50×8×5 | all-one | binary | G | fiducial GPQ, signed | 0.6796 | 1.000 | 0.9558 (0.0047) | 0.0195 | 0.0247 | 0.9558 | 0.9511 | 0.2856 | errors 0 | pass |
| 50×8×5 | all-one | binary | Phi | fiducial GPQ, signed | 0.5484 | 1.000 | 0.9479 (0.0051) | 0.0463 | 0.0058 | 0.9479 | 0.9542 | 0.4260 | errors 0 | fail |
| 50×20×2 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9284 (0.0059) | 0.0453 | 0.0263 | 0.9284 | 0.9284 | 0.0360 | errors 0 | fail |
| 50×20×2 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9616 (0.0044) | 0.0226 | 0.0158 | 0.9616 | 0.9621 | 0.0595 | errors 0 | fail |
| 50×20×2 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9621 (0.0044) | 0.0216 | 0.0163 | 0.9621 | 0.9637 | 25.0862 | errors 0 | fail |
| 50×20×2 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.7763 (0.0096) | 0.1268 | 0.0968 | 0.7763 | 0.7768 | 0.0173 | errors 0 | fail |
| 50×20×2 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.9116 (0.0065) | 0.0442 | 0.0442 | 0.9116 | 0.9116 | 0.0258 | errors 0 | fail |
| 50×20×2 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9463 (0.0052) | 0.0305 | 0.0232 | 0.9463 | 0.9458 | 0.0418 | errors 0 | pass |
| 50×20×2 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.7858 (0.0094) | 0.1268 | 0.0874 | 0.7858 | 0.7837 | 0.0195 | errors 0 | fail |
| 50×20×2 | all-one | binary | G | fiducial GPQ | 0.5826 | 1.000 | 0.9532 (0.0048) | 0.0226 | 0.0242 | 0.9532 | 0.9521 | 0.4802 | errors 0 | pass |
| 50×20×2 | all-one | binary | Phi | fiducial GPQ | 0.4360 | 1.000 | 0.9337 (0.0057) | 0.0632 | 0.0032 | 0.9337 | 0.9253 | 0.6159 | errors 0 | fail |
| 50×20×2 | all-one | binary | G | fiducial GPQ, signed | 0.5826 | 1.000 | 0.9532 (0.0048) | 0.0226 | 0.0242 | 0.9532 | 0.9521 | 0.4802 | errors 0 | pass |
| 50×20×2 | all-one | binary | Phi | fiducial GPQ, signed | 0.4360 | 1.000 | 0.9342 (0.0057) | 0.0626 | 0.0032 | 0.9342 | 0.9247 | 0.6180 | errors 0 | fail |
| 50×20×3 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9374 (0.0056) | 0.0368 | 0.0258 | 0.9374 | 0.9379 | 0.0297 | errors 0 | fail |
| 50×20×3 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9526 (0.0049) | 0.0226 | 0.0247 | 0.9526 | 0.9537 | 0.0516 | errors 0 | pass |
| 50×20×3 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9589 (0.0046) | 0.0189 | 0.0221 | 0.9589 | 0.9595 | 0.9521 | errors 0 | pass |
| 50×20×3 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.7542 (0.0099) | 0.1379 | 0.1079 | 0.7542 | 0.7547 | 0.0120 | errors 0 | fail |
| 50×20×3 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.9074 (0.0067) | 0.0489 | 0.0437 | 0.9074 | 0.9063 | 0.0176 | errors 0 | fail |
| 50×20×3 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9489 (0.0050) | 0.0216 | 0.0295 | 0.9489 | 0.9463 | 0.0268 | errors 0 | pass |
| 50×20×3 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.7321 (0.0102) | 0.1516 | 0.1163 | 0.7321 | 0.7326 | 0.0138 | errors 0 | fail |
| 50×20×3 | all-one | binary | G | fiducial GPQ | 0.6685 | 1.000 | 0.9442 (0.0053) | 0.0274 | 0.0284 | 0.9442 | 0.9426 | 0.3332 | errors 0 | pass |
| 50×20×3 | all-one | binary | Phi | fiducial GPQ | 0.5270 | 1.000 | 0.9421 (0.0054) | 0.0516 | 0.0063 | 0.9421 | 0.9432 | 0.5695 | errors 0 | fail |
| 50×20×3 | all-one | binary | G | fiducial GPQ, signed | 0.6685 | 1.000 | 0.9442 (0.0053) | 0.0274 | 0.0284 | 0.9442 | 0.9426 | 0.3332 | errors 0 | pass |
| 50×20×3 | all-one | binary | Phi | fiducial GPQ, signed | 0.5270 | 1.000 | 0.9416 (0.0054) | 0.0516 | 0.0068 | 0.9416 | 0.9421 | 0.5697 | errors 0 | fail |
| 50×20×5 | all-one | binary | p | fiducial GPQ | 0.0228 | 1.000 | 0.9500 (0.0050) | 0.0300 | 0.0200 | 0.9500 | 0.9511 | 0.0257 | errors 0 | pass |
| 50×20×5 | all-one | binary | i | fiducial GPQ | 0.0228 | 1.000 | 0.9500 (0.0050) | 0.0316 | 0.0184 | 0.9500 | 0.9500 | 0.0447 | errors 0 | pass |
| 50×20×5 | all-one | binary | r | fiducial GPQ | 0.0228 | 1.000 | 0.9732 (0.0037) | 0.0126 | 0.0142 | 0.9732 | 0.9747 | 0.1976 | errors 0 | fail |
| 50×20×5 | all-one | binary | pi | fiducial GPQ | 0.0249 | 1.000 | 0.7232 (0.0103) | 0.1500 | 0.1268 | 0.7232 | 0.7226 | 0.0087 | errors 0 | fail |
| 50×20×5 | all-one | binary | pr | fiducial GPQ | 0.0249 | 1.000 | 0.8858 (0.0073) | 0.0516 | 0.0626 | 0.8858 | 0.8853 | 0.0122 | errors 0 | fail |
| 50×20×5 | all-one | binary | ir | fiducial GPQ | 0.0249 | 1.000 | 0.9332 (0.0057) | 0.0368 | 0.0300 | 0.9332 | 0.9337 | 0.0181 | errors 0 | fail |
| 50×20×5 | all-one | binary | pir | fiducial GPQ | 0.1070 | 1.000 | 0.6853 (0.0107) | 0.1489 | 0.1658 | 0.6853 | 0.6868 | 0.0097 | errors 0 | fail |
| 50×20×5 | all-one | binary | G | fiducial GPQ | 0.7580 | 1.000 | 0.9574 (0.0046) | 0.0226 | 0.0200 | 0.9574 | 0.9553 | 0.2199 | errors 0 | pass |
| 50×20×5 | all-one | binary | Phi | fiducial GPQ | 0.6328 | 1.000 | 0.9411 (0.0054) | 0.0516 | 0.0074 | 0.9411 | 0.9495 | 0.4263 | errors 0 | fail |
| 50×20×5 | all-one | binary | G | fiducial GPQ, signed | 0.7580 | 1.000 | 0.9574 (0.0046) | 0.0226 | 0.0200 | 0.9574 | 0.9553 | 0.2199 | errors 0 | pass |
| 50×20×5 | all-one | binary | Phi | fiducial GPQ, signed | 0.6328 | 1.000 | 0.9405 (0.0054) | 0.0516 | 0.0079 | 0.9405 | 0.9489 | 0.4263 | errors 0 | fail |
