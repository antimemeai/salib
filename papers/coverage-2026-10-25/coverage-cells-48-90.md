# G-theory coverage study (full), cells 48..90 of 90

n_sim = 1900, B = 999, nominal 0.95. Pass: coverage in [.94, .96] and each tail miss in [.015, .035]; a None interval counts as a miss. Lower miss: the interval lies entirely below the truth; upper miss: entirely above. Conditional coverage counts available intervals only; bias-eliminated coverage targets the mean point estimate.

| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 50×20×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0463 | 0.0042 | 0.9495 | 0.9484 | 1.1213 | facet-SE ratio 1.009 | fail |
| 50×20×3 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9663 (0.0041) | 0.0000 | 0.0337 | 0.9663 | 0.9668 | 1.3340 |  | fail |
| 50×20×3 | all-one | normal | p | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9632 (0.0043) | 0.0000 | 0.0368 | 0.9632 | 0.9663 | 1.8924 |  | fail |
| 50×20×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9205 (0.0062) | 0.0742 | 0.0053 | 0.9205 | 0.9200 | 1.7412 | facet-SE ratio 0.954 | fail |
| 50×20×3 | all-one | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0000 | 0.0442 | 0.9558 | 0.9563 | 1740559661.7042 |  | fail |
| 50×20×3 | all-one | normal | i | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9837 (0.0029) | 0.0000 | 0.0163 | 0.9837 | 0.9837 | 2.3610 |  | fail |
| 50×20×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7500 (0.0099) | 0.2500 | 0.0000 | 0.7500 | 0.7595 | 3.8012 | facet-SE ratio 0.978 | fail |
| 50×20×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.931 | 0.9000 (0.0069) | 0.0000 | 0.0311 | 0.9666 | 0.8958 | 36078674846.0305 |  | fail |
| 50×20×3 | all-one | normal | r | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.6611 (0.0109) | 0.3389 | 0.0000 | 0.6611 | 0.6711 | 1.9298 |  | fail |
| 50×20×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0295 | 0.0179 | 0.9526 | 0.9526 | 0.2447 | facet-SE ratio 1.002 | pass |
| 50×20×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0200 | 0.0226 | 0.9574 | 0.9558 | 0.2475 |  | pass |
| 50×20×3 | all-one | normal | pi | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9989 (0.0007) | 0.0000 | 0.0011 | 0.9989 | 0.9989 | 1.1646 |  | fail |
| 50×20×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9379 (0.0055) | 0.0479 | 0.0142 | 0.9379 | 0.9379 | 0.5843 | facet-SE ratio 0.989 | fail |
| 50×20×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0232 | 0.0242 | 0.9526 | 0.9526 | 0.6134 |  | pass |
| 50×20×3 | all-one | normal | pr | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9126 (0.0065) | 0.0874 | 0.0000 | 0.9126 | 0.9126 | 1.2340 |  | fail |
| 50×20×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9305 (0.0058) | 0.0632 | 0.0063 | 0.9305 | 0.9305 | 0.9046 | facet-SE ratio 0.974 | fail |
| 50×20×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0200 | 0.0221 | 0.9579 | 0.9553 | 1.0134 |  | pass |
| 50×20×3 | all-one | normal | ir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.8395 (0.0084) | 0.1605 | 0.0000 | 0.8395 | 0.8432 | 1.2918 |  | fail |
| 50×20×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0258 | 0.0263 | 0.9479 | 0.9484 | 0.1278 | facet-SE ratio 1.011 | pass |
| 50×20×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0221 | 0.0279 | 0.9500 | 0.9495 | 0.1287 |  | pass |
| 50×20×3 | all-one | normal | pir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.4332 (0.0114) | 0.5668 | 0.0000 | 0.4332 | 0.4347 | 0.9946 |  | fail |
| 50×20×3 | all-one | normal | G | F interval | 0.7143 | 1.000 | 0.9495 (0.0050) | 0.0300 | 0.0205 | 0.9495 | 0.9479 | 0.2818 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×3 | all-one | normal | Phi | parametric | 0.5556 | 1.000 | 0.9358 (0.0056) | 0.0026 | 0.0616 | 0.9358 | 0.9468 | 0.4096 |  | fail |
| 50×20×3 | all-one | normal | Phi | parametric, attached | 0.5556 | 0.931 | 0.8847 (0.0073) | 0.0026 | 0.0437 | 0.9503 | 0.8921 | 0.4173 |  | fail |
| 50×20×3 | all-one | normal | G | 0.3.0 boot-p,i,r | 0.7143 | 1.000 | 0.9316 (0.0058) | 0.0000 | 0.0684 | 0.9316 | 0.9089 | 0.3709 |  | fail |
| 50×20×3 | all-one | normal | Phi | 0.3.0 boot-p,i,r | 0.5556 | 1.000 | 0.8147 (0.0089) | 0.0000 | 0.1853 | 0.8147 | 0.8374 | 0.4915 |  | fail |
| 50×20×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9311 (0.0058) | 0.0542 | 0.0147 | 0.9311 | 0.9321 | 0.8408 | facet-SE ratio 0.997 | fail |
| 50×20×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0158 | 0.0263 | 0.9579 | 0.9574 | 0.9261 |  | pass |
| 50×20×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9363 (0.0056) | 0.0605 | 0.0032 | 0.9363 | 0.9321 | 1.3127 | facet-SE ratio 1.010 | fail |
| 50×20×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9595 (0.0045) | 0.0126 | 0.0279 | 0.9595 | 0.9611 | 1.6483 |  | fail |
| 50×20×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.7737 (0.0096) | 0.2263 | 0.0000 | 0.7737 | 0.7632 | 0.0445 | facet-SE ratio 0.967 | fail |
| 50×20×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.868 | 0.8232 (0.0088) | 0.0000 | 0.0453 | 0.9479 | 0.8289 | 2013503459.8096 |  | fail |
| 50×20×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0263 | 0.0163 | 0.9574 | 0.9553 | 0.2448 | facet-SE ratio 1.025 | pass |
| 50×20×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9637 (0.0043) | 0.0142 | 0.0221 | 0.9637 | 0.9626 | 0.2480 |  | fail |
| 50×20×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9884 (0.0025) | 0.0000 | 0.0116 | 0.9884 | 0.9884 | 0.0343 | facet-SE ratio 1.015 | fail |
| 50×20×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.887 | 0.7826 (0.0095) | 0.0000 | 0.1042 | 0.8825 | 0.7858 | 6095003145.1291 |  | fail |
| 50×20×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9621 (0.0044) | 0.0305 | 0.0074 | 0.9621 | 0.9526 | 0.0272 | facet-SE ratio 0.959 | fail |
| 50×20×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.945 | 0.8516 (0.0082) | 0.0000 | 0.0937 | 0.9009 | 0.8605 | 2835191648.7410 |  | fail |
| 50×20×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0300 | 0.0232 | 0.9468 | 0.9489 | 0.1277 | facet-SE ratio 1.005 | pass |
| 50×20×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0247 | 0.0247 | 0.9505 | 0.9511 | 0.1286 |  | pass |
| 50×20×3 | near-zero-rater | normal | G | F interval | 0.9346 | 1.000 | 0.9563 (0.0047) | 0.0200 | 0.0237 | 0.9563 | 0.9537 | 0.0564 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×3 | near-zero-rater | normal | Phi | parametric | 0.8901 | 1.000 | 0.9574 (0.0046) | 0.0295 | 0.0132 | 0.9574 | 0.9568 | 0.1087 |  | fail |
| 50×20×3 | near-zero-rater | normal | Phi | parametric, attached | 0.8901 | 0.728 | 0.6968 (0.0105) | 0.0205 | 0.0111 | 0.9566 | 0.6937 | 0.1091 |  | fail |
| 50×20×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9332 (0.0057) | 0.0584 | 0.0084 | 0.9332 | 0.9363 | 3.2754 | facet-SE ratio 0.985 | fail |
| 50×20×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9558 (0.0047) | 0.0211 | 0.0232 | 0.9558 | 0.9547 | 3.5928 |  | pass |
| 50×20×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9174 (0.0063) | 0.0789 | 0.0037 | 0.9174 | 0.9189 | 0.8678 | facet-SE ratio 0.961 | fail |
| 50×20×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.999 | 0.9605 (0.0045) | 0.0000 | 0.0384 | 0.9615 | 0.9589 | 3853927354.1467 |  | fail |
| 50×20×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7605 (0.0098) | 0.2395 | 0.0000 | 0.7605 | 0.7626 | 1.9279 | facet-SE ratio 0.983 | fail |
| 50×20×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.927 | 0.8958 (0.0070) | 0.0000 | 0.0316 | 0.9659 | 0.8937 | 17458425898.2973 |  | fail |
| 50×20×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9379 (0.0055) | 0.0311 | 0.0311 | 0.9379 | 0.9358 | 0.1227 | facet-SE ratio 0.970 | fail |
| 50×20×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9416 (0.0054) | 0.0216 | 0.0368 | 0.9416 | 0.9416 | 0.1241 |  | fail |
| 50×20×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9416 (0.0054) | 0.0447 | 0.0137 | 0.9416 | 0.9416 | 0.2921 | facet-SE ratio 0.995 | fail |
| 50×20×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0195 | 0.0247 | 0.9558 | 0.9558 | 0.3063 |  | pass |
| 50×20×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9400 (0.0054) | 0.0532 | 0.0068 | 0.9400 | 0.9400 | 0.4539 | facet-SE ratio 0.986 | fail |
| 50×20×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0242 | 0.0247 | 0.9511 | 0.9511 | 0.5085 |  | pass |
| 50×20×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0337 | 0.0221 | 0.9442 | 0.9453 | 0.0640 | facet-SE ratio 1.006 | pass |
| 50×20×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0284 | 0.0232 | 0.9484 | 0.9495 | 0.0644 |  | pass |
| 50×20×3 | high-g | normal | G | F interval | 0.9524 | 1.000 | 0.9532 (0.0048) | 0.0247 | 0.0221 | 0.9532 | 0.9468 | 0.0476 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×3 | high-g | normal | Phi | parametric | 0.9091 | 1.000 | 0.9037 (0.0068) | 0.0000 | 0.0963 | 0.9037 | 0.8911 | 0.1487 |  | fail |
| 50×20×3 | high-g | normal | Phi | parametric, attached | 0.9091 | 0.926 | 0.8611 (0.0079) | 0.0000 | 0.0653 | 0.9295 | 0.8516 | 0.1557 |  | fail |
| 50×20×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0432 | 0.0089 | 0.9479 | 0.9468 | 0.9949 | facet-SE ratio 1.018 | fail |
| 50×20×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0079 | 0.0337 | 0.9584 | 0.9611 | 1.1379 |  | fail |
| 50×20×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9200 (0.0062) | 0.0763 | 0.0037 | 0.9200 | 0.9168 | 1.5541 | facet-SE ratio 0.953 | fail |
| 50×20×5 | all-one | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0000 | 0.0421 | 0.9579 | 0.9600 | 2.1752 |  | fail |
| 50×20×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8295 (0.0086) | 0.1705 | 0.0000 | 0.8295 | 0.8279 | 2.8658 | facet-SE ratio 0.930 | fail |
| 50×20×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.991 | 0.9616 (0.0044) | 0.0000 | 0.0295 | 0.9703 | 0.9632 | 4108099413.8466 |  | fail |
| 50×20×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0326 | 0.0226 | 0.9447 | 0.9447 | 0.2182 | facet-SE ratio 0.989 | pass |
| 50×20×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0237 | 0.0311 | 0.9453 | 0.9442 | 0.2203 |  | pass |
| 50×20×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0363 | 0.0179 | 0.9458 | 0.9453 | 0.4137 | facet-SE ratio 1.004 | fail |
| 50×20×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0232 | 0.0258 | 0.9511 | 0.9505 | 0.4249 |  | pass |
| 50×20×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9400 (0.0054) | 0.0532 | 0.0068 | 0.9400 | 0.9416 | 0.6423 | facet-SE ratio 1.003 | fail |
| 50×20×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0237 | 0.0184 | 0.9579 | 0.9584 | 0.6800 |  | pass |
| 50×20×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0274 | 0.0268 | 0.9458 | 0.9447 | 0.0904 | facet-SE ratio 1.008 | pass |
| 50×20×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0253 | 0.0295 | 0.9453 | 0.9474 | 0.0910 |  | pass |
| 50×20×5 | all-one | normal | G | F interval | 0.7937 | 1.000 | 0.9558 (0.0047) | 0.0184 | 0.0258 | 0.9558 | 0.9416 | 0.1843 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×5 | all-one | normal | Phi | parametric | 0.6579 | 1.000 | 0.9474 (0.0051) | 0.0105 | 0.0421 | 0.9474 | 0.9453 | 0.3194 |  | fail |
| 50×20×5 | all-one | normal | Phi | parametric, attached | 0.6579 | 0.991 | 0.9421 (0.0054) | 0.0105 | 0.0384 | 0.9506 | 0.9400 | 0.3203 |  | fail |
| 50×20×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0495 | 0.0079 | 0.9426 | 0.9432 | 0.8336 | facet-SE ratio 1.018 | fail |
| 50×20×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0189 | 0.0200 | 0.9611 | 0.9605 | 0.9169 |  | fail |
| 50×20×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9221 (0.0061) | 0.0758 | 0.0021 | 0.9221 | 0.9205 | 1.2936 | facet-SE ratio 0.993 | fail |
| 50×20×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0189 | 0.0237 | 0.9574 | 0.9584 | 1.6212 |  | pass |
| 50×20×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8358 (0.0085) | 0.1642 | 0.0000 | 0.8358 | 0.8342 | 0.0314 | facet-SE ratio 0.936 | fail |
| 50×20×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.962 | 0.9268 (0.0060) | 0.0000 | 0.0353 | 0.9633 | 0.9289 | 357988069.7026 |  | fail |
| 50×20×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9395 (0.0055) | 0.0421 | 0.0184 | 0.9395 | 0.9411 | 0.2173 | facet-SE ratio 0.968 | fail |
| 50×20×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0300 | 0.0237 | 0.9463 | 0.9479 | 0.2196 |  | pass |
| 50×20×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9816 (0.0031) | 0.0042 | 0.0142 | 0.9816 | 0.9795 | 0.0242 | facet-SE ratio 1.003 | fail |
| 50×20×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.955 | 0.8647 (0.0078) | 0.0000 | 0.0905 | 0.9052 | 0.8700 | 1852955804.3385 |  | fail |
| 50×20×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9505 (0.0050) | 0.0400 | 0.0095 | 0.9505 | 0.9516 | 0.0190 | facet-SE ratio 0.996 | fail |
| 50×20×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.992 | 0.9263 (0.0060) | 0.0000 | 0.0653 | 0.9342 | 0.9247 | 435445977.1038 |  | fail |
| 50×20×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0316 | 0.0184 | 0.9500 | 0.9511 | 0.0906 | facet-SE ratio 0.990 | pass |
| 50×20×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0268 | 0.0200 | 0.9532 | 0.9526 | 0.0910 |  | pass |
| 50×20×5 | near-zero-rater | normal | G | F interval | 0.9416 | 1.000 | 0.9589 (0.0046) | 0.0195 | 0.0216 | 0.9589 | 0.9532 | 0.0495 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×5 | near-zero-rater | normal | Phi | parametric | 0.8976 | 1.000 | 0.9542 (0.0048) | 0.0332 | 0.0126 | 0.9542 | 0.9542 | 0.1016 |  | fail |
| 50×20×5 | near-zero-rater | normal | Phi | parametric, attached | 0.8976 | 0.912 | 0.8721 (0.0077) | 0.0279 | 0.0116 | 0.9567 | 0.8726 | 0.1015 |  | fail |
| 50×20×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9368 (0.0056) | 0.0589 | 0.0042 | 0.9368 | 0.9368 | 3.2512 | facet-SE ratio 1.020 | fail |
| 50×20×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9558 (0.0047) | 0.0247 | 0.0195 | 0.9558 | 0.9574 | 3.5557 |  | pass |
| 50×20×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9232 (0.0061) | 0.0737 | 0.0032 | 0.9232 | 0.9216 | 0.7724 | facet-SE ratio 0.994 | fail |
| 50×20×5 | high-g | normal | i | Satterthwaite | 0.5000 | 1.000 | 0.9684 (0.0040) | 0.0000 | 0.0316 | 0.9684 | 0.9684 | 1.0794 |  | fail |
| 50×20×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8389 (0.0084) | 0.1611 | 0.0000 | 0.8389 | 0.8368 | 1.4332 | facet-SE ratio 0.949 | fail |
| 50×20×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.988 | 0.9637 (0.0043) | 0.0000 | 0.0247 | 0.9750 | 0.9653 | 1579202351.9707 |  | fail |
| 50×20×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0242 | 0.0247 | 0.9511 | 0.9474 | 0.1091 | facet-SE ratio 1.006 | pass |
| 50×20×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0195 | 0.0263 | 0.9542 | 0.9532 | 0.1103 |  | pass |
| 50×20×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9416 (0.0054) | 0.0400 | 0.0184 | 0.9416 | 0.9442 | 0.2058 | facet-SE ratio 0.970 | fail |
| 50×20×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9463 | 0.2114 |  | pass |
| 50×20×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0474 | 0.0058 | 0.9468 | 0.9505 | 0.3188 | facet-SE ratio 1.006 | fail |
| 50×20×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0226 | 0.0216 | 0.9558 | 0.9558 | 0.3378 |  | pass |
| 50×20×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0337 | 0.0226 | 0.9437 | 0.9442 | 0.0452 | facet-SE ratio 1.008 | pass |
| 50×20×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0279 | 0.0253 | 0.9468 | 0.9474 | 0.0455 |  | pass |
| 50×20×5 | high-g | normal | G | F interval | 0.9685 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9484 | 0.0281 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×5 | high-g | normal | Phi | parametric | 0.9390 | 1.000 | 0.9374 (0.0056) | 0.0084 | 0.0542 | 0.9374 | 0.9295 | 0.0856 |  | fail |
| 50×20×5 | high-g | normal | Phi | parametric, attached | 0.9390 | 0.988 | 0.9300 (0.0059) | 0.0084 | 0.0500 | 0.9409 | 0.9226 | 0.0861 |  | fail |
| 200×5×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0342 | 0.0174 | 0.9484 | 0.9484 | 0.7443 | facet-SE ratio 0.986 | pass |
| 200×5×2 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0026 | 0.0384 | 0.9589 | 0.9579 | 0.8034 |  | fail |
| 200×5×2 | all-one | normal | p | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.8832 (0.0074) | 0.0000 | 0.1168 | 0.8832 | 0.8789 | 2.3244 |  | fail |
| 200×5×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9321 (0.0058) | 0.0679 | 0.0000 | 0.9321 | 0.9284 | 4.6487 | facet-SE ratio 0.949 | fail |
| 200×5×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.856 | 0.7932 (0.0093) | 0.0000 | 0.0632 | 0.9262 | 0.7963 | 817955899869.6361 |  | fail |
| 200×5×2 | all-one | normal | i | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9637 (0.0043) | 0.0358 | 0.0005 | 0.9637 | 0.9626 | 4.0448 |  | fail |
| 200×5×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7774 (0.0095) | 0.2226 | 0.0000 | 0.7774 | 0.7747 | 6.5936 | facet-SE ratio 1.165 | fail |
| 200×5×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.714 | 0.6663 (0.0108) | 0.0000 | 0.0479 | 0.9329 | 0.6674 | 968003565213.9547 |  | fail |
| 200×5×2 | all-one | normal | r | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.7184 (0.0103) | 0.2816 | 0.0000 | 0.7184 | 0.7079 | 2.8435 |  | fail |
| 200×5×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0305 | 0.0189 | 0.9505 | 0.9505 | 0.3089 | facet-SE ratio 1.015 | pass |
| 200×5×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0168 | 0.0300 | 0.9532 | 0.9547 | 0.3142 |  | pass |
| 200×5×2 | all-one | normal | pi | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 1.4829 |  | fail |
| 200×5×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0395 | 0.0221 | 0.9384 | 0.9389 | 0.4705 | facet-SE ratio 0.996 | fail |
| 200×5×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0189 | 0.0274 | 0.9537 | 0.9537 | 0.4857 |  | pass |
| 200×5×2 | all-one | normal | pr | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 1.5399 |  | fail |
| 200×5×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8453 (0.0083) | 0.1547 | 0.0000 | 0.8453 | 0.8442 | 2.6788 | facet-SE ratio 0.972 | fail |
| 200×5×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0184 | 0.0226 | 0.9589 | 0.9584 | 8.0835 |  | pass |
| 200×5×2 | all-one | normal | ir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.6358 (0.0110) | 0.3642 | 0.0000 | 0.6358 | 0.6332 | 1.5238 |  | fail |
| 200×5×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0289 | 0.0200 | 0.9511 | 0.9511 | 0.1955 | facet-SE ratio 1.000 | pass |
| 200×5×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0211 | 0.0226 | 0.9563 | 0.9574 | 0.1973 |  | pass |
| 200×5×2 | all-one | normal | pir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.3758 (0.0111) | 0.6242 | 0.0000 | 0.3758 | 0.3768 | 0.9847 |  | fail |
| 200×5×2 | all-one | normal | G | F interval | 0.5556 | 1.000 | 0.9505 (0.0050) | 0.0216 | 0.0279 | 0.9505 | 0.9500 | 0.2255 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×2 | all-one | normal | Phi | parametric | 0.3846 | 1.000 | 0.9305 (0.0058) | 0.0021 | 0.0674 | 0.9305 | 0.9547 | 0.3467 |  | fail |
| 200×5×2 | all-one | normal | Phi | parametric, attached | 0.3846 | 0.624 | 0.5989 (0.0112) | 0.0021 | 0.0232 | 0.9595 | 0.6100 | 0.3715 |  | fail |
| 200×5×2 | all-one | normal | G | 0.3.0 boot-p,i,r | 0.5556 | 1.000 | 0.9216 (0.0062) | 0.0000 | 0.0784 | 0.9216 | 0.9058 | 0.4609 |  | fail |
| 200×5×2 | all-one | normal | Phi | 0.3.0 boot-p,i,r | 0.3846 | 1.000 | 0.7447 (0.0100) | 0.0000 | 0.2553 | 0.7447 | 0.8084 | 0.5881 |  | fail |
| 200×5×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0342 | 0.0163 | 0.9495 | 0.9489 | 0.5173 | facet-SE ratio 0.994 | pass |
| 200×5×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0153 | 0.0347 | 0.9500 | 0.9521 | 0.5370 |  | pass |
| 200×5×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8379 (0.0085) | 0.1621 | 0.0000 | 0.8379 | 0.8353 | 2.7181 | facet-SE ratio 0.971 | fail |
| 200×5×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9626 (0.0044) | 0.0100 | 0.0274 | 0.9626 | 0.9647 | 8.3566 |  | fail |
| 200×5×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8526 (0.0081) | 0.1474 | 0.0000 | 0.8526 | 0.8421 | 0.0755 | facet-SE ratio 1.142 | fail |
| 200×5×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.655 | 0.5989 (0.0112) | 0.0000 | 0.0558 | 0.9148 | 0.6037 | 16055023924.6575 |  | fail |
| 200×5×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0326 | 0.0200 | 0.9474 | 0.9468 | 0.3094 | facet-SE ratio 0.988 | pass |
| 200×5×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0195 | 0.0284 | 0.9521 | 0.9516 | 0.3142 |  | pass |
| 200×5×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9821 (0.0030) | 0.0000 | 0.0179 | 0.9821 | 0.9821 | 0.0929 | facet-SE ratio 0.974 | fail |
| 200×5×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.660 | 0.4484 (0.0114) | 0.0000 | 0.2116 | 0.6794 | 0.4479 | 36698774301.2439 |  | fail |
| 200×5×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.8447 (0.0083) | 0.1553 | 0.0000 | 0.8447 | 0.8416 | 0.0405 | facet-SE ratio 0.930 | fail |
| 200×5×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.852 | 0.7963 (0.0092) | 0.0000 | 0.0553 | 0.9351 | 0.7968 | 3443064922.8996 |  | fail |
| 200×5×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0332 | 0.0147 | 0.9521 | 0.9521 | 0.1955 | facet-SE ratio 1.006 | fail |
| 200×5×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0279 | 0.0195 | 0.9526 | 0.9521 | 0.1973 |  | pass |
| 200×5×2 | near-zero-rater | normal | G | F interval | 0.7663 | 1.000 | 0.9516 (0.0049) | 0.0200 | 0.0284 | 0.9516 | 0.9495 | 0.1095 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×2 | near-zero-rater | normal | Phi | parametric | 0.6618 | 1.000 | 0.9174 (0.0063) | 0.0016 | 0.0811 | 0.9174 | 0.9200 | 0.2390 |  | fail |
| 200×5×2 | near-zero-rater | normal | Phi | parametric, attached | 0.6618 | 0.365 | 0.3416 (0.0109) | 0.0005 | 0.0226 | 0.9365 | 0.3416 | 0.2429 |  | fail |
| 200×5×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9442 (0.0053) | 0.0411 | 0.0147 | 0.9442 | 0.9474 | 1.7188 | facet-SE ratio 0.993 | fail |
| 200×5×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9516 (0.0049) | 0.0237 | 0.0247 | 0.9516 | 0.9511 | 1.7680 |  | pass |
| 200×5×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9384 (0.0055) | 0.0616 | 0.0000 | 0.9384 | 0.9384 | 2.3007 | facet-SE ratio 0.941 | fail |
| 200×5×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.846 | 0.7837 (0.0094) | 0.0000 | 0.0621 | 0.9266 | 0.7837 | 367941052966.4169 |  | fail |
| 200×5×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7653 (0.0097) | 0.2347 | 0.0000 | 0.7653 | 0.7653 | 3.2442 | facet-SE ratio 1.152 | fail |
| 200×5×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.689 | 0.6474 (0.0110) | 0.0000 | 0.0421 | 0.9389 | 0.6474 | 512469143916.1990 |  | fail |
| 200×5×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0253 | 0.0200 | 0.9547 | 0.9542 | 0.1546 | facet-SE ratio 1.012 | pass |
| 200×5×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0137 | 0.0300 | 0.9563 | 0.9547 | 0.1571 |  | fail |
| 200×5×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0358 | 0.0137 | 0.9505 | 0.9489 | 0.2358 | facet-SE ratio 0.997 | fail |
| 200×5×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0163 | 0.0295 | 0.9542 | 0.9526 | 0.2436 |  | pass |
| 200×5×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8432 (0.0083) | 0.1568 | 0.0000 | 0.8432 | 0.8468 | 1.3130 | facet-SE ratio 0.993 | fail |
| 200×5×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9647 (0.0042) | 0.0205 | 0.0147 | 0.9647 | 0.9647 | 3.9623 |  | fail |
| 200×5×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0263 | 0.0221 | 0.9516 | 0.9495 | 0.0978 | facet-SE ratio 0.992 | pass |
| 200×5×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0211 | 0.0289 | 0.9500 | 0.9511 | 0.0988 |  | pass |
| 200×5×2 | high-g | normal | G | F interval | 0.9091 | 1.000 | 0.9437 (0.0053) | 0.0284 | 0.0279 | 0.9437 | 0.9447 | 0.0463 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×2 | high-g | normal | Phi | parametric | 0.8333 | 1.000 | 0.8879 (0.0072) | 0.0000 | 0.1121 | 0.8879 | 0.8900 | 0.2208 |  | fail |
| 200×5×2 | high-g | normal | Phi | parametric, attached | 0.8333 | 0.593 | 0.5695 (0.0114) | 0.0000 | 0.0232 | 0.9609 | 0.5705 | 0.2669 |  | fail |
| 200×5×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0374 | 0.0142 | 0.9484 | 0.9474 | 0.6365 | facet-SE ratio 1.001 | fail |
| 200×5×3 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0116 | 0.0368 | 0.9516 | 0.9495 | 0.6740 |  | fail |
| 200×5×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8621 (0.0079) | 0.1379 | 0.0000 | 0.8621 | 0.8611 | 3.7419 | facet-SE ratio 0.949 | fail |
| 200×5×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.907 | 0.8553 (0.0081) | 0.0000 | 0.0521 | 0.9426 | 0.8553 | 333704178957.1995 |  | fail |
| 200×5×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7805 (0.0095) | 0.2195 | 0.0000 | 0.7805 | 0.7821 | 4.5349 | facet-SE ratio 0.961 | fail |
| 200×5×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.837 | 0.7884 (0.0094) | 0.0000 | 0.0484 | 0.9421 | 0.7884 | 374168896525.0202 |  | fail |
| 200×5×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0347 | 0.0226 | 0.9426 | 0.9421 | 0.2645 | facet-SE ratio 0.975 | pass |
| 200×5×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0242 | 0.0321 | 0.9437 | 0.9437 | 0.2684 |  | pass |
| 200×5×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0358 | 0.0211 | 0.9432 | 0.9432 | 0.3328 | facet-SE ratio 0.985 | fail |
| 200×5×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0247 | 0.0300 | 0.9453 | 0.9458 | 0.3394 |  | pass |
| 200×5×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8847 (0.0073) | 0.1153 | 0.0000 | 0.8847 | 0.8868 | 1.9098 | facet-SE ratio 1.001 | fail |
| 200×5×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0216 | 0.0200 | 0.9584 | 0.9574 | 3.2294 |  | pass |
| 200×5×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0284 | 0.0237 | 0.9479 | 0.9474 | 0.1382 | facet-SE ratio 1.007 | pass |
| 200×5×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0200 | 0.0268 | 0.9532 | 0.9526 | 0.1393 |  | pass |
| 200×5×3 | all-one | normal | G | F interval | 0.6250 | 1.000 | 0.9489 (0.0050) | 0.0258 | 0.0253 | 0.9489 | 0.9416 | 0.1688 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×3 | all-one | normal | Phi | parametric | 0.4545 | 1.000 | 0.9305 (0.0058) | 0.0021 | 0.0674 | 0.9305 | 0.9442 | 0.3169 |  | fail |
| 200×5×3 | all-one | normal | Phi | parametric, attached | 0.4545 | 0.766 | 0.7384 (0.0101) | 0.0021 | 0.0253 | 0.9643 | 0.7432 | 0.3307 |  | fail |
| 200×5×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0321 | 0.0195 | 0.9484 | 0.9484 | 0.5000 | facet-SE ratio 1.004 | pass |
| 200×5×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0200 | 0.0305 | 0.9495 | 0.9495 | 0.5178 |  | pass |
| 200×5×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8363 (0.0085) | 0.1637 | 0.0000 | 0.8363 | 0.8374 | 2.6570 | facet-SE ratio 0.951 | fail |
| 200×5×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.999 | 0.9674 (0.0041) | 0.0068 | 0.0253 | 0.9679 | 0.9679 | 8.1232 |  | fail |
| 200×5×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8453 (0.0083) | 0.1547 | 0.0000 | 0.8453 | 0.8363 | 0.0521 | facet-SE ratio 0.961 | fail |
| 200×5×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.789 | 0.7432 (0.0100) | 0.0000 | 0.0458 | 0.9420 | 0.7447 | 6025933557.1106 |  | fail |
| 200×5×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0316 | 0.0189 | 0.9495 | 0.9484 | 0.2644 | facet-SE ratio 1.001 | pass |
| 200×5×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0189 | 0.0258 | 0.9553 | 0.9532 | 0.2679 |  | pass |
| 200×5×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9837 (0.0029) | 0.0000 | 0.0163 | 0.9837 | 0.9837 | 0.0649 | facet-SE ratio 0.998 | fail |
| 200×5×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.720 | 0.5363 (0.0114) | 0.0000 | 0.1837 | 0.7449 | 0.5342 | 23516500300.2495 |  | fail |
| 200×5×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.8884 (0.0072) | 0.1116 | 0.0000 | 0.8884 | 0.8863 | 0.0289 | facet-SE ratio 0.937 | fail |
| 200×5×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.956 | 0.9063 (0.0067) | 0.0000 | 0.0495 | 0.9482 | 0.9063 | 1142041603.4833 |  | fail |
| 200×5×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0363 | 0.0211 | 0.9426 | 0.9432 | 0.1382 | facet-SE ratio 0.981 | fail |
| 200×5×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0289 | 0.0263 | 0.9447 | 0.9437 | 0.1392 |  | pass |
| 200×5×3 | near-zero-rater | normal | G | F interval | 0.7874 | 1.000 | 0.9521 (0.0049) | 0.0232 | 0.0247 | 0.9521 | 0.9500 | 0.0959 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×3 | near-zero-rater | normal | Phi | parametric | 0.6784 | 1.000 | 0.8937 (0.0071) | 0.0016 | 0.1047 | 0.8937 | 0.8995 | 0.2381 |  | fail |
| 200×5×3 | near-zero-rater | normal | Phi | parametric, attached | 0.6784 | 0.538 | 0.4842 (0.0115) | 0.0011 | 0.0526 | 0.9002 | 0.4863 | 0.2381 |  | fail |
| 200×5×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9421 (0.0054) | 0.0426 | 0.0153 | 0.9421 | 0.9421 | 1.6838 | facet-SE ratio 0.994 | fail |
| 200×5×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9432 (0.0053) | 0.0284 | 0.0284 | 0.9432 | 0.9437 | 1.7281 |  | pass |
| 200×5×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8705 (0.0077) | 0.1295 | 0.0000 | 0.8705 | 0.8716 | 1.8741 | facet-SE ratio 0.934 | fail |
| 200×5×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.892 | 0.8416 (0.0084) | 0.0000 | 0.0505 | 0.9434 | 0.8416 | 151295971523.5078 |  | fail |
| 200×5×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7632 (0.0098) | 0.2368 | 0.0000 | 0.7632 | 0.7774 | 2.1822 | facet-SE ratio 0.962 | fail |
| 200×5×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.831 | 0.7863 (0.0094) | 0.0000 | 0.0442 | 0.9468 | 0.7800 | 184272641843.4423 |  | fail |
| 200×5×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0321 | 0.0189 | 0.9489 | 0.9489 | 0.1323 | facet-SE ratio 0.997 | pass |
| 200×5×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0211 | 0.0284 | 0.9505 | 0.9505 | 0.1342 |  | pass |
| 200×5×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0363 | 0.0195 | 0.9442 | 0.9474 | 0.1662 | facet-SE ratio 1.011 | fail |
| 200×5×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0221 | 0.0289 | 0.9489 | 0.9479 | 0.1693 |  | pass |
| 200×5×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8737 (0.0076) | 0.1263 | 0.0000 | 0.8737 | 0.8726 | 0.9667 | facet-SE ratio 0.979 | fail |
| 200×5×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0253 | 0.0242 | 0.9505 | 0.9516 | 1.6352 |  | pass |
| 200×5×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0274 | 0.0179 | 0.9547 | 0.9542 | 0.0691 | facet-SE ratio 1.014 | pass |
| 200×5×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0221 | 0.0216 | 0.9563 | 0.9568 | 0.0696 |  | pass |
| 200×5×3 | high-g | normal | G | F interval | 0.9302 | 1.000 | 0.9495 (0.0050) | 0.0232 | 0.0274 | 0.9495 | 0.9495 | 0.0313 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×3 | high-g | normal | Phi | parametric | 0.8696 | 1.000 | 0.9142 (0.0064) | 0.0000 | 0.0858 | 0.9142 | 0.9221 | 0.1539 |  | fail |
| 200×5×3 | high-g | normal | Phi | parametric, attached | 0.8696 | 0.741 | 0.7084 (0.0104) | 0.0000 | 0.0321 | 0.9566 | 0.7111 | 0.1684 |  | fail |
| 200×5×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0374 | 0.0184 | 0.9442 | 0.9442 | 0.5667 | facet-SE ratio 0.977 | fail |
| 200×5×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0095 | 0.0374 | 0.9532 | 0.9537 | 0.5931 |  | fail |
| 200×5×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8426 (0.0084) | 0.1574 | 0.0000 | 0.8426 | 0.8421 | 3.2444 | facet-SE ratio 0.951 | fail |
| 200×5×5 | all-one | normal | i | Satterthwaite | 1.0000 | 0.950 | 0.9147 (0.0064) | 0.0000 | 0.0353 | 0.9629 | 0.9153 | 57627543638.3736 |  | fail |
| 200×5×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8395 (0.0084) | 0.1605 | 0.0000 | 0.8395 | 0.8342 | 3.2944 | facet-SE ratio 0.938 | fail |
| 200×5×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.954 | 0.9111 (0.0065) | 0.0000 | 0.0432 | 0.9548 | 0.9132 | 71571985535.3689 |  | fail |
| 200×5×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0342 | 0.0168 | 0.9489 | 0.9500 | 0.2356 | facet-SE ratio 0.975 | pass |
| 200×5×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0263 | 0.0242 | 0.9495 | 0.9500 | 0.2379 |  | pass |
| 200×5×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0279 | 0.0158 | 0.9563 | 0.9563 | 0.2357 | facet-SE ratio 1.027 | pass |
| 200×5×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0189 | 0.0226 | 0.9584 | 0.9605 | 0.2384 |  | pass |
| 200×5×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9232 (0.0061) | 0.0763 | 0.0005 | 0.9232 | 0.9295 | 1.3544 | facet-SE ratio 1.043 | fail |
| 200×5×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9626 (0.0044) | 0.0189 | 0.0184 | 0.9626 | 0.9626 | 1.7546 |  | fail |
| 200×5×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0253 | 0.0184 | 0.9563 | 0.9558 | 0.0976 | facet-SE ratio 1.039 | pass |
| 200×5×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0216 | 0.0195 | 0.9589 | 0.9579 | 0.0983 |  | pass |
| 200×5×5 | all-one | normal | G | F interval | 0.6944 | 1.000 | 0.9442 (0.0053) | 0.0232 | 0.0326 | 0.9442 | 0.9442 | 0.1302 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×5 | all-one | normal | Phi | parametric | 0.5319 | 1.000 | 0.9421 (0.0054) | 0.0032 | 0.0547 | 0.9421 | 0.9447 | 0.2806 |  | fail |
| 200×5×5 | all-one | normal | Phi | parametric, attached | 0.5319 | 0.907 | 0.8647 (0.0078) | 0.0032 | 0.0395 | 0.9530 | 0.8668 | 0.2839 |  | fail |
| 200×5×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0447 | 0.0163 | 0.9389 | 0.9395 | 0.4880 | facet-SE ratio 0.989 | fail |
| 200×5×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0216 | 0.0295 | 0.9489 | 0.9489 | 0.5052 |  | pass |
| 200×5×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8268 (0.0087) | 0.1732 | 0.0000 | 0.8268 | 0.8189 | 2.7532 | facet-SE ratio 0.946 | fail |
| 200×5×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0153 | 0.0316 | 0.9532 | 0.9542 | 8.3637 |  | pass |
| 200×5×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8453 (0.0083) | 0.1547 | 0.0000 | 0.8453 | 0.8468 | 0.0353 | facet-SE ratio 0.954 | fail |
| 200×5×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.903 | 0.8584 (0.0080) | 0.0000 | 0.0442 | 0.9510 | 0.8563 | 1754558909.5418 |  | fail |
| 200×5×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9411 (0.0054) | 0.0347 | 0.0242 | 0.9411 | 0.9416 | 0.2356 | facet-SE ratio 0.971 | pass |
| 200×5×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0237 | 0.0311 | 0.9453 | 0.9453 | 0.2383 |  | pass |
| 200×5×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9805 (0.0032) | 0.0000 | 0.0195 | 0.9805 | 0.9737 | 0.0457 | facet-SE ratio 0.966 | fail |
| 200×5×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.791 | 0.6332 (0.0111) | 0.0000 | 0.1579 | 0.8004 | 0.6179 | 14789181547.3457 |  | fail |
| 200×5×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9163 (0.0064) | 0.0826 | 0.0011 | 0.9163 | 0.9153 | 0.0206 | facet-SE ratio 0.982 | fail |
| 200×5×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.993 | 0.9526 (0.0049) | 0.0000 | 0.0405 | 0.9592 | 0.9537 | 106507337.9002 |  | fail |
| 200×5×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0268 | 0.0216 | 0.9516 | 0.9516 | 0.0978 | facet-SE ratio 0.990 | pass |
| 200×5×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0242 | 0.0226 | 0.9532 | 0.9532 | 0.0984 |  | pass |
| 200×5×5 | near-zero-rater | normal | G | F interval | 0.8052 | 1.000 | 0.9479 (0.0051) | 0.0263 | 0.0258 | 0.9479 | 0.9495 | 0.0869 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×5 | near-zero-rater | normal | Phi | parametric | 0.6923 | 1.000 | 0.8800 (0.0075) | 0.0000 | 0.1200 | 0.8800 | 0.8832 | 0.2446 |  | fail |
| 200×5×5 | near-zero-rater | normal | Phi | parametric, attached | 0.6923 | 0.712 | 0.6274 (0.0111) | 0.0000 | 0.0842 | 0.8817 | 0.6305 | 0.2463 |  | fail |
| 200×5×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9416 (0.0054) | 0.0421 | 0.0163 | 0.9416 | 0.9411 | 1.6477 | facet-SE ratio 1.001 | fail |
| 200×5×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9511 (0.0049) | 0.0242 | 0.0247 | 0.9511 | 0.9505 | 1.6913 |  | pass |
| 200×5×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8337 (0.0085) | 0.1663 | 0.0000 | 0.8337 | 0.8374 | 1.6121 | facet-SE ratio 0.916 | fail |
| 200×5×5 | high-g | normal | i | Satterthwaite | 0.5000 | 0.949 | 0.9089 (0.0066) | 0.0000 | 0.0405 | 0.9573 | 0.9074 | 44756688212.5460 |  | fail |
| 200×5×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8342 (0.0085) | 0.1658 | 0.0000 | 0.8342 | 0.8384 | 1.5921 | facet-SE ratio 0.915 | fail |
| 200×5×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.950 | 0.9074 (0.0067) | 0.0000 | 0.0426 | 0.9551 | 0.9042 | 45974834160.5212 |  | fail |
| 200×5×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0316 | 0.0232 | 0.9453 | 0.9453 | 0.1177 | facet-SE ratio 0.994 | pass |
| 200×5×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0211 | 0.0268 | 0.9521 | 0.9521 | 0.1191 |  | pass |
| 200×5×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0311 | 0.0168 | 0.9521 | 0.9521 | 0.1178 | facet-SE ratio 1.020 | pass |
| 200×5×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9632 (0.0043) | 0.0174 | 0.0195 | 0.9632 | 0.9632 | 0.1191 |  | fail |
| 200×5×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9142 (0.0064) | 0.0837 | 0.0021 | 0.9142 | 0.9137 | 0.6821 | facet-SE ratio 1.017 | fail |
| 200×5×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0295 | 0.0237 | 0.9468 | 0.9463 | 0.8834 |  | pass |
| 200×5×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0289 | 0.0211 | 0.9500 | 0.9500 | 0.0489 | facet-SE ratio 1.024 | pass |
| 200×5×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0253 | 0.0221 | 0.9526 | 0.9537 | 0.0491 |  | pass |
| 200×5×5 | high-g | normal | G | F interval | 0.9479 | 1.000 | 0.9532 (0.0048) | 0.0247 | 0.0221 | 0.9532 | 0.9537 | 0.0222 | unfit 0, errors 0, failed fits 0 | pass |
| 200×5×5 | high-g | normal | Phi | parametric | 0.9009 | 1.000 | 0.9179 (0.0063) | 0.0011 | 0.0811 | 0.9179 | 0.9184 | 0.1032 |  | fail |
| 200×5×5 | high-g | normal | Phi | parametric, attached | 0.9009 | 0.906 | 0.8511 (0.0082) | 0.0011 | 0.0542 | 0.9390 | 0.8516 | 0.1060 |  | fail |
| 200×8×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0368 | 0.0142 | 0.9489 | 0.9484 | 0.6985 | facet-SE ratio 0.986 | fail |
| 200×8×2 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0084 | 0.0479 | 0.9437 | 0.9468 | 0.7469 |  | fail |
| 200×8×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9353 (0.0056) | 0.0647 | 0.0000 | 0.9353 | 0.9295 | 3.4297 | facet-SE ratio 0.958 | fail |
| 200×8×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.924 | 0.8568 (0.0080) | 0.0000 | 0.0668 | 0.9276 | 0.8632 | 356715774734.4214 |  | fail |
| 200×8×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.6768 (0.0107) | 0.3232 | 0.0000 | 0.6768 | 0.6826 | 5.7030 | facet-SE ratio 1.187 | fail |
| 200×8×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.755 | 0.7142 (0.0104) | 0.0000 | 0.0411 | 0.9456 | 0.7121 | 491961958130.8621 |  | fail |
| 200×8×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9342 (0.0057) | 0.0395 | 0.0263 | 0.9342 | 0.9337 | 0.2334 | facet-SE ratio 0.963 | fail |
| 200×8×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9411 (0.0054) | 0.0268 | 0.0321 | 0.9411 | 0.9411 | 0.2364 |  | pass |
| 200×8×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0400 | 0.0137 | 0.9463 | 0.9479 | 0.4392 | facet-SE ratio 1.014 | fail |
| 200×8×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0247 | 0.0237 | 0.9516 | 0.9500 | 0.4521 |  | pass |
| 200×8×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8763 (0.0076) | 0.1237 | 0.0000 | 0.8763 | 0.8826 | 2.0171 | facet-SE ratio 0.952 | fail |
| 200×8×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0226 | 0.0232 | 0.9542 | 0.9521 | 3.6892 |  | pass |
| 200×8×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0332 | 0.0189 | 0.9479 | 0.9489 | 0.1478 | facet-SE ratio 0.991 | pass |
| 200×8×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0258 | 0.0216 | 0.9526 | 0.9526 | 0.1490 |  | pass |
| 200×8×2 | all-one | normal | G | F interval | 0.5926 | 1.000 | 0.9463 (0.0052) | 0.0268 | 0.0268 | 0.9463 | 0.9468 | 0.2108 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×2 | all-one | normal | Phi | parametric | 0.4211 | 1.000 | 0.8889 (0.0072) | 0.0000 | 0.1111 | 0.8889 | 0.9326 | 0.3445 |  | fail |
| 200×8×2 | all-one | normal | Phi | parametric, attached | 0.4211 | 0.699 | 0.6626 (0.0108) | 0.0000 | 0.0368 | 0.9473 | 0.6784 | 0.3761 |  | fail |
| 200×8×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0400 | 0.0158 | 0.9442 | 0.9442 | 0.4683 | facet-SE ratio 0.972 | fail |
| 200×8×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0253 | 0.0289 | 0.9458 | 0.9458 | 0.4831 |  | pass |
| 200×8×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8716 (0.0077) | 0.1284 | 0.0000 | 0.8716 | 0.8737 | 2.0328 | facet-SE ratio 0.951 | fail |
| 200×8×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0200 | 0.0247 | 0.9553 | 0.9532 | 3.7591 |  | pass |
| 200×8×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.7637 (0.0097) | 0.2363 | 0.0000 | 0.7637 | 0.7411 | 0.0664 | facet-SE ratio 1.165 | fail |
| 200×8×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.713 | 0.6632 (0.0108) | 0.0000 | 0.0495 | 0.9306 | 0.6668 | 7487378756.3507 |  | fail |
| 200×8×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0347 | 0.0221 | 0.9432 | 0.9432 | 0.2339 | facet-SE ratio 1.002 | pass |
| 200×8×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0221 | 0.0284 | 0.9495 | 0.9500 | 0.2366 |  | pass |
| 200×8×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9832 (0.0030) | 0.0000 | 0.0168 | 0.9832 | 0.9821 | 0.0565 | facet-SE ratio 0.985 | fail |
| 200×8×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.745 | 0.5937 (0.0113) | 0.0000 | 0.1516 | 0.7966 | 0.5805 | 16952544120.1525 |  | fail |
| 200×8×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.8753 (0.0076) | 0.1247 | 0.0000 | 0.8753 | 0.8753 | 0.0307 | facet-SE ratio 0.933 | fail |
| 200×8×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.936 | 0.8811 (0.0074) | 0.0000 | 0.0553 | 0.9410 | 0.8811 | 1329941929.3083 |  | fail |
| 200×8×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9411 (0.0054) | 0.0311 | 0.0279 | 0.9411 | 0.9411 | 0.1479 | facet-SE ratio 0.992 | pass |
| 200×8×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0268 | 0.0311 | 0.9421 | 0.9421 | 0.1489 |  | pass |
| 200×8×2 | near-zero-rater | normal | G | F interval | 0.8386 | 1.000 | 0.9453 (0.0052) | 0.0284 | 0.0263 | 0.9453 | 0.9416 | 0.0726 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×2 | near-zero-rater | normal | Phi | parametric | 0.7558 | 1.000 | 0.9300 (0.0059) | 0.0047 | 0.0653 | 0.9300 | 0.9300 | 0.1675 |  | fail |
| 200×8×2 | near-zero-rater | normal | Phi | parametric, attached | 0.7558 | 0.502 | 0.4753 (0.0115) | 0.0032 | 0.0237 | 0.9465 | 0.4753 | 0.1696 |  | fail |
| 200×8×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9389 (0.0055) | 0.0400 | 0.0211 | 0.9389 | 0.9389 | 1.6996 | facet-SE ratio 0.975 | fail |
| 200×8×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9479 (0.0051) | 0.0211 | 0.0311 | 0.9479 | 0.9474 | 1.7475 |  | pass |
| 200×8×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9374 (0.0056) | 0.0626 | 0.0000 | 0.9374 | 0.9284 | 1.7365 | facet-SE ratio 0.952 | fail |
| 200×8×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.918 | 0.8505 (0.0082) | 0.0000 | 0.0674 | 0.9266 | 0.8574 | 197195696020.9809 |  | fail |
| 200×8×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.6758 (0.0107) | 0.3242 | 0.0000 | 0.6758 | 0.6779 | 2.9034 | facet-SE ratio 1.180 | fail |
| 200×8×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.732 | 0.6916 (0.0106) | 0.0000 | 0.0405 | 0.9446 | 0.6900 | 218166763446.2679 |  | fail |
| 200×8×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0321 | 0.0226 | 0.9453 | 0.9453 | 0.1168 | facet-SE ratio 0.991 | pass |
| 200×8×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0232 | 0.0284 | 0.9484 | 0.9474 | 0.1182 |  | pass |
| 200×8×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9368 (0.0056) | 0.0421 | 0.0211 | 0.9368 | 0.9384 | 0.2208 | facet-SE ratio 0.975 | fail |
| 200×8×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0242 | 0.0321 | 0.9437 | 0.9447 | 0.2271 |  | pass |
| 200×8×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8763 (0.0076) | 0.1237 | 0.0000 | 0.8763 | 0.8742 | 1.0295 | facet-SE ratio 0.966 | fail |
| 200×8×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0263 | 0.0226 | 0.9511 | 0.9511 | 1.8849 |  | pass |
| 200×8×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0300 | 0.0247 | 0.9453 | 0.9458 | 0.0739 | facet-SE ratio 1.002 | pass |
| 200×8×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0253 | 0.0268 | 0.9479 | 0.9484 | 0.0745 |  | pass |
| 200×8×2 | high-g | normal | G | F interval | 0.9209 | 1.000 | 0.9384 (0.0055) | 0.0311 | 0.0305 | 0.9384 | 0.9384 | 0.0413 | unfit 0, errors 0, failed fits 0 | fail |
| 200×8×2 | high-g | normal | Phi | parametric | 0.8533 | 1.000 | 0.8195 (0.0088) | 0.0000 | 0.1805 | 0.8195 | 0.8289 | 0.1985 |  | fail |
| 200×8×2 | high-g | normal | Phi | parametric, attached | 0.8533 | 0.678 | 0.6337 (0.0111) | 0.0000 | 0.0442 | 0.9348 | 0.6384 | 0.2451 |  | fail |
| 200×8×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0353 | 0.0116 | 0.9532 | 0.9532 | 0.5968 | facet-SE ratio 1.032 | fail |
| 200×8×3 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9616 (0.0044) | 0.0126 | 0.0258 | 0.9616 | 0.9621 | 0.6271 |  | fail |
| 200×8×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8989 (0.0069) | 0.1005 | 0.0005 | 0.8989 | 0.8958 | 2.8334 | facet-SE ratio 0.968 | fail |
| 200×8×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.963 | 0.9179 (0.0063) | 0.0000 | 0.0453 | 0.9530 | 0.9205 | 109829969678.1367 |  | fail |
| 200×8×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7621 (0.0098) | 0.2379 | 0.0000 | 0.7621 | 0.7632 | 4.1389 | facet-SE ratio 0.992 | fail |
| 200×8×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.892 | 0.8584 (0.0080) | 0.0000 | 0.0337 | 0.9622 | 0.8579 | 123281321125.1935 |  | fail |
| 200×8×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0326 | 0.0179 | 0.9495 | 0.9495 | 0.2003 | facet-SE ratio 1.001 | pass |
| 200×8×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0247 | 0.0237 | 0.9516 | 0.9521 | 0.2021 |  | pass |
| 200×8×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0247 | 0.0211 | 0.9542 | 0.9532 | 0.3120 | facet-SE ratio 1.012 | pass |
| 200×8×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0179 | 0.0295 | 0.9526 | 0.9526 | 0.3171 |  | pass |
| 200×8×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9016 (0.0068) | 0.0953 | 0.0032 | 0.9016 | 0.9053 | 1.4490 | facet-SE ratio 0.958 | fail |
| 200×8×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0263 | 0.0200 | 0.9537 | 0.9547 | 1.9454 |  | pass |
| 200×8×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0274 | 0.0200 | 0.9526 | 0.9526 | 0.1044 | facet-SE ratio 1.030 | pass |
| 200×8×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0232 | 0.0211 | 0.9558 | 0.9558 | 0.1052 |  | pass |
| 200×8×3 | all-one | normal | G | F interval | 0.6667 | 1.000 | 0.9637 (0.0043) | 0.0195 | 0.0168 | 0.9637 | 0.9637 | 0.1512 | unfit 0, errors 0, failed fits 0 | fail |
| 200×8×3 | all-one | normal | Phi | parametric | 0.5000 | 1.000 | 0.9116 (0.0065) | 0.0005 | 0.0879 | 0.9116 | 0.9300 | 0.3094 |  | fail |
| 200×8×3 | all-one | normal | Phi | parametric, attached | 0.5000 | 0.858 | 0.8179 (0.0089) | 0.0005 | 0.0400 | 0.9528 | 0.8279 | 0.3222 |  | fail |
| 200×8×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0421 | 0.0147 | 0.9432 | 0.9437 | 0.4580 | facet-SE ratio 0.999 | fail |
| 200×8×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0232 | 0.0284 | 0.9484 | 0.9484 | 0.4726 |  | pass |
| 200×8×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8863 (0.0073) | 0.1137 | 0.0000 | 0.8863 | 0.8811 | 2.0915 | facet-SE ratio 0.978 | fail |
| 200×8×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0174 | 0.0274 | 0.9553 | 0.9568 | 3.8553 |  | pass |
| 200×8×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.7711 (0.0096) | 0.2289 | 0.0000 | 0.7711 | 0.7784 | 0.0432 | facet-SE ratio 0.977 | fail |
| 200×8×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.857 | 0.8205 (0.0088) | 0.0000 | 0.0363 | 0.9576 | 0.8184 | 2845254931.6120 |  | fail |
| 200×8×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0289 | 0.0221 | 0.9489 | 0.9505 | 0.2000 | facet-SE ratio 1.003 | pass |
| 200×8×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0184 | 0.0258 | 0.9558 | 0.9563 | 0.2022 |  | pass |
| 200×8×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9842 (0.0029) | 0.0000 | 0.0158 | 0.9842 | 0.9842 | 0.0398 | facet-SE ratio 1.022 | fail |
| 200×8×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.844 | 0.7184 (0.0103) | 0.0000 | 0.1253 | 0.8515 | 0.7189 | 11602224889.0673 |  | fail |
| 200×8×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9095 (0.0066) | 0.0895 | 0.0011 | 0.9095 | 0.9100 | 0.0218 | facet-SE ratio 0.962 | fail |
| 200×8×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.991 | 0.9474 (0.0051) | 0.0000 | 0.0432 | 0.9564 | 0.9463 | 269078170.4897 |  | fail |
| 200×8×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0279 | 0.0200 | 0.9521 | 0.9526 | 0.1046 | facet-SE ratio 1.020 | pass |
| 200×8×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0226 | 0.0221 | 0.9553 | 0.9542 | 0.1051 |  | pass |
| 200×8×3 | near-zero-rater | normal | G | F interval | 0.8547 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9511 | 0.0627 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×3 | near-zero-rater | normal | Phi | parametric | 0.7700 | 1.000 | 0.9305 (0.0058) | 0.0042 | 0.0653 | 0.9305 | 0.9263 | 0.1676 |  | fail |
| 200×8×3 | near-zero-rater | normal | Phi | parametric, attached | 0.7700 | 0.716 | 0.6642 (0.0108) | 0.0037 | 0.0479 | 0.9279 | 0.6632 | 0.1684 |  | fail |
| 200×8×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9458 (0.0052) | 0.0400 | 0.0142 | 0.9458 | 0.9458 | 1.6621 | facet-SE ratio 1.001 | fail |
| 200×8×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9505 (0.0050) | 0.0247 | 0.0247 | 0.9505 | 0.9505 | 1.7071 |  | pass |
| 200×8×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8821 (0.0074) | 0.1174 | 0.0005 | 0.8821 | 0.8842 | 1.3992 | facet-SE ratio 0.937 | fail |
| 200×8×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.963 | 0.9163 (0.0064) | 0.0000 | 0.0463 | 0.9519 | 0.9147 | 39736404123.0158 |  | fail |
| 200×8×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7679 (0.0097) | 0.2321 | 0.0000 | 0.7679 | 0.7658 | 2.1120 | facet-SE ratio 0.970 | fail |
| 200×8×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.899 | 0.8626 (0.0079) | 0.0000 | 0.0368 | 0.9590 | 0.8632 | 70718200860.8384 |  | fail |
| 200×8×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0274 | 0.0242 | 0.9484 | 0.9453 | 0.1003 | facet-SE ratio 0.989 | pass |
| 200×8×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9474 (0.0051) | 0.0221 | 0.0305 | 0.9474 | 0.9479 | 0.1012 |  | pass |
| 200×8×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0289 | 0.0216 | 0.9495 | 0.9489 | 0.1560 | facet-SE ratio 1.009 | pass |
| 200×8×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0179 | 0.0289 | 0.9532 | 0.9558 | 0.1587 |  | pass |
| 200×8×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9068 (0.0067) | 0.0900 | 0.0032 | 0.9068 | 0.9047 | 0.7354 | facet-SE ratio 0.967 | fail |
| 200×8×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0332 | 0.0232 | 0.9437 | 0.9453 | 0.9884 |  | pass |
| 200×8×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9474 (0.0051) | 0.0311 | 0.0216 | 0.9474 | 0.9479 | 0.0523 | facet-SE ratio 1.002 | pass |
| 200×8×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9495 | 0.0526 |  | pass |
| 200×8×3 | high-g | normal | G | F interval | 0.9412 | 1.000 | 0.9547 (0.0048) | 0.0226 | 0.0226 | 0.9547 | 0.9521 | 0.0268 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×3 | high-g | normal | Phi | parametric | 0.8889 | 1.000 | 0.8547 (0.0081) | 0.0005 | 0.1447 | 0.8547 | 0.8537 | 0.1369 |  | fail |
| 200×8×3 | high-g | normal | Phi | parametric, attached | 0.8889 | 0.868 | 0.7884 (0.0094) | 0.0005 | 0.0795 | 0.9079 | 0.7884 | 0.1461 |  | fail |
| 200×8×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0384 | 0.0158 | 0.9458 | 0.9458 | 0.5296 | facet-SE ratio 0.981 | fail |
| 200×8×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0189 | 0.0311 | 0.9500 | 0.9489 | 0.5510 |  | pass |
| 200×8×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8774 (0.0075) | 0.1211 | 0.0016 | 0.8774 | 0.8742 | 2.4982 | facet-SE ratio 0.958 | fail |
| 200×8×5 | all-one | normal | i | Satterthwaite | 1.0000 | 0.991 | 0.9595 (0.0045) | 0.0000 | 0.0316 | 0.9681 | 0.9611 | 10696483219.6436 |  | fail |
| 200×8×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8395 (0.0084) | 0.1605 | 0.0000 | 0.8395 | 0.8379 | 3.0521 | facet-SE ratio 0.920 | fail |
| 200×8×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.978 | 0.9395 (0.0055) | 0.0000 | 0.0389 | 0.9602 | 0.9416 | 29148737407.5994 |  | fail |
| 200×8×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0311 | 0.0247 | 0.9442 | 0.9442 | 0.1777 | facet-SE ratio 0.989 | pass |
| 200×8×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0232 | 0.0279 | 0.9489 | 0.9474 | 0.1795 |  | pass |
| 200×8×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0253 | 0.0158 | 0.9589 | 0.9589 | 0.2202 | facet-SE ratio 1.031 | pass |
| 200×8×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0205 | 0.0216 | 0.9579 | 0.9574 | 0.2225 |  | pass |
| 200×8×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9363 (0.0056) | 0.0600 | 0.0037 | 0.9363 | 0.9358 | 1.0438 | facet-SE ratio 0.985 | fail |
| 200×8×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0237 | 0.0237 | 0.9526 | 0.9537 | 1.2089 |  | pass |
| 200×8×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0263 | 0.0205 | 0.9532 | 0.9526 | 0.0738 | facet-SE ratio 1.010 | pass |
| 200×8×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0211 | 0.0242 | 0.9547 | 0.9547 | 0.0743 |  | pass |
| 200×8×5 | all-one | normal | G | F interval | 0.7407 | 1.000 | 0.9495 (0.0050) | 0.0258 | 0.0247 | 0.9495 | 0.9500 | 0.1097 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×5 | all-one | normal | Phi | parametric | 0.5882 | 1.000 | 0.9332 (0.0057) | 0.0042 | 0.0626 | 0.9332 | 0.9347 | 0.2532 |  | fail |
| 200×8×5 | all-one | normal | Phi | parametric, attached | 0.5882 | 0.969 | 0.9111 (0.0065) | 0.0042 | 0.0542 | 0.9397 | 0.9126 | 0.2547 |  | fail |
| 200×8×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0374 | 0.0163 | 0.9463 | 0.9474 | 0.4503 | facet-SE ratio 0.996 | fail |
| 200×8×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0284 | 0.0263 | 0.9453 | 0.9437 | 0.4640 |  | pass |
| 200×8×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8647 (0.0078) | 0.1342 | 0.0011 | 0.8647 | 0.8679 | 2.0309 | facet-SE ratio 0.962 | fail |
| 200×8×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0226 | 0.0221 | 0.9553 | 0.9537 | 3.7338 |  | pass |
| 200×8×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8432 (0.0083) | 0.1568 | 0.0000 | 0.8432 | 0.8432 | 0.0319 | facet-SE ratio 0.954 | fail |
| 200×8×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.958 | 0.9189 (0.0063) | 0.0000 | 0.0395 | 0.9588 | 0.9200 | 570193104.7970 |  | fail |
| 200×8×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0353 | 0.0189 | 0.9458 | 0.9479 | 0.1780 | facet-SE ratio 1.003 | fail |
| 200×8×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0268 | 0.0253 | 0.9479 | 0.9495 | 0.1794 |  | pass |
| 200×8×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9805 (0.0032) | 0.0000 | 0.0195 | 0.9805 | 0.9805 | 0.0280 | facet-SE ratio 0.995 | fail |
| 200×8×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.917 | 0.8005 (0.0092) | 0.0000 | 0.1163 | 0.8731 | 0.8026 | 4732447387.5687 |  | fail |
| 200×8×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9332 (0.0057) | 0.0616 | 0.0053 | 0.9332 | 0.9332 | 0.0156 | facet-SE ratio 0.954 | fail |
| 200×8×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 1.000 | 0.9568 (0.0047) | 0.0000 | 0.0432 | 0.9568 | 0.9574 | 10243321.0762 |  | fail |
| 200×8×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0221 | 0.0300 | 0.9479 | 0.9463 | 0.0739 | facet-SE ratio 0.998 | pass |
| 200×8×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0189 | 0.0316 | 0.9495 | 0.9489 | 0.0744 |  | pass |
| 200×8×5 | near-zero-rater | normal | G | F interval | 0.8681 | 1.000 | 0.9495 (0.0050) | 0.0242 | 0.0263 | 0.9495 | 0.9500 | 0.0562 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×5 | near-zero-rater | normal | Phi | parametric | 0.7817 | 1.000 | 0.9058 (0.0067) | 0.0026 | 0.0916 | 0.9058 | 0.9100 | 0.1650 |  | fail |
| 200×8×5 | near-zero-rater | normal | Phi | parametric, attached | 0.7817 | 0.880 | 0.7968 (0.0092) | 0.0026 | 0.0805 | 0.9055 | 0.8011 | 0.1649 |  | fail |
| 200×8×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9479 (0.0051) | 0.0353 | 0.0168 | 0.9479 | 0.9479 | 1.6326 | facet-SE ratio 1.018 | fail |
| 200×8×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9511 (0.0049) | 0.0221 | 0.0268 | 0.9511 | 0.9511 | 1.6764 |  | pass |
| 200×8×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8768 (0.0075) | 0.1232 | 0.0000 | 0.8768 | 0.8795 | 1.2325 | facet-SE ratio 0.948 | fail |
| 200×8×5 | high-g | normal | i | Satterthwaite | 0.5000 | 0.985 | 0.9553 (0.0047) | 0.0000 | 0.0300 | 0.9696 | 0.9553 | 7828951903.6938 |  | fail |
| 200×8×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8316 (0.0086) | 0.1684 | 0.0000 | 0.8316 | 0.8284 | 1.5366 | facet-SE ratio 0.942 | fail |
| 200×8×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.977 | 0.9421 (0.0054) | 0.0000 | 0.0347 | 0.9644 | 0.9463 | 11497589546.5817 |  | fail |
| 200×8×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9474 (0.0051) | 0.0353 | 0.0174 | 0.9474 | 0.9479 | 0.0889 | facet-SE ratio 1.000 | fail |
| 200×8×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9463 (0.0052) | 0.0305 | 0.0232 | 0.9463 | 0.9453 | 0.0896 |  | pass |
| 200×8×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0289 | 0.0142 | 0.9568 | 0.9568 | 0.1101 | facet-SE ratio 1.028 | fail |
| 200×8×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9600 (0.0045) | 0.0205 | 0.0195 | 0.9600 | 0.9600 | 0.1113 |  | pass |
| 200×8×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9347 (0.0057) | 0.0595 | 0.0058 | 0.9347 | 0.9295 | 0.5262 | facet-SE ratio 0.989 | fail |
| 200×8×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0174 | 0.0316 | 0.9511 | 0.9537 | 0.6092 |  | pass |
| 200×8×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0289 | 0.0211 | 0.9500 | 0.9495 | 0.0369 | facet-SE ratio 1.008 | pass |
| 200×8×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0263 | 0.0221 | 0.9516 | 0.9511 | 0.0372 |  | pass |
| 200×8×5 | high-g | normal | G | F interval | 0.9581 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9505 | 0.0177 | unfit 0, errors 0, failed fits 0 | pass |
| 200×8×5 | high-g | normal | Phi | parametric | 0.9195 | 1.000 | 0.9195 (0.0062) | 0.0000 | 0.0805 | 0.9195 | 0.9147 | 0.0815 |  | fail |
| 200×8×5 | high-g | normal | Phi | parametric, attached | 0.9195 | 0.963 | 0.8974 (0.0070) | 0.0000 | 0.0653 | 0.9322 | 0.8932 | 0.0826 |  | fail |
| 200×20×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0389 | 0.0158 | 0.9453 | 0.9447 | 0.6480 | facet-SE ratio 0.999 | fail |
| 200×20×2 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0053 | 0.0358 | 0.9589 | 0.9584 | 0.6875 |  | fail |
| 200×20×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9284 (0.0059) | 0.0668 | 0.0047 | 0.9284 | 0.9284 | 2.0180 | facet-SE ratio 0.996 | fail |
| 200×20×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.991 | 0.9437 (0.0053) | 0.0000 | 0.0474 | 0.9522 | 0.9437 | 29400181085.1720 |  | fail |
| 200×20×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.6411 (0.0110) | 0.3589 | 0.0000 | 0.6411 | 0.6442 | 5.1492 | facet-SE ratio 1.223 | fail |
| 200×20×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.832 | 0.8068 (0.0091) | 0.0000 | 0.0253 | 0.9696 | 0.8042 | 180171634511.0591 |  | fail |
| 200×20×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0279 | 0.0221 | 0.9500 | 0.9505 | 0.1418 | facet-SE ratio 1.016 | pass |
| 200×20×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0179 | 0.0263 | 0.9558 | 0.9563 | 0.1429 |  | pass |
| 200×20×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0358 | 0.0158 | 0.9484 | 0.9489 | 0.4097 | facet-SE ratio 1.020 | fail |
| 200×20×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0237 | 0.0216 | 0.9547 | 0.9553 | 0.4209 |  | pass |
| 200×20×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9184 (0.0063) | 0.0779 | 0.0037 | 0.9184 | 0.9195 | 1.2486 | facet-SE ratio 0.968 | fail |
| 200×20×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0237 | 0.0237 | 0.9526 | 0.9516 | 1.5525 |  | pass |
| 200×20×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0289 | 0.0184 | 0.9526 | 0.9532 | 0.0898 | facet-SE ratio 1.042 | pass |
| 200×20×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0232 | 0.0200 | 0.9568 | 0.9574 | 0.0902 |  | pass |
| 200×20×2 | all-one | normal | G | F interval | 0.6349 | 1.000 | 0.9516 (0.0049) | 0.0226 | 0.0258 | 0.9516 | 0.9479 | 0.1991 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×2 | all-one | normal | Phi | parametric | 0.4651 | 1.000 | 0.7816 (0.0095) | 0.0005 | 0.2179 | 0.7816 | 0.8642 | 0.3538 |  | fail |
| 200×20×2 | all-one | normal | Phi | parametric, attached | 0.4651 | 0.825 | 0.7205 (0.0103) | 0.0005 | 0.1042 | 0.8731 | 0.7632 | 0.3834 |  | fail |
| 200×20×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0447 | 0.0126 | 0.9426 | 0.9426 | 0.4212 | facet-SE ratio 1.006 | fail |
| 200×20×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9511 | 0.4331 |  | pass |
| 200×20×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9205 (0.0062) | 0.0726 | 0.0068 | 0.9205 | 0.9189 | 1.2847 | facet-SE ratio 0.955 | fail |
| 200×20×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0242 | 0.0279 | 0.9479 | 0.9484 | 1.5994 |  | pass |
| 200×20×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.6274 (0.0111) | 0.3726 | 0.0000 | 0.6274 | 0.6274 | 0.0548 | facet-SE ratio 1.171 | fail |
| 200×20×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.780 | 0.7389 (0.0101) | 0.0000 | 0.0411 | 0.9474 | 0.7389 | 2370925742.7311 |  | fail |
| 200×20×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9521 | 0.1418 | facet-SE ratio 1.021 | pass |
| 200×20×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0200 | 0.0253 | 0.9547 | 0.9553 | 0.1428 |  | pass |
| 200×20×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9795 (0.0033) | 0.0068 | 0.0137 | 0.9795 | 0.9821 | 0.0239 | facet-SE ratio 1.010 | fail |
| 200×20×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.958 | 0.8689 (0.0077) | 0.0000 | 0.0895 | 0.9066 | 0.8653 | 1650634742.6288 |  | fail |
| 200×20×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9263 (0.0060) | 0.0716 | 0.0021 | 0.9263 | 0.9268 | 0.0186 | facet-SE ratio 0.992 | fail |
| 200×20×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.997 | 0.9605 (0.0045) | 0.0000 | 0.0363 | 0.9636 | 0.9563 | 58420989.4930 |  | fail |
| 200×20×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0289 | 0.0174 | 0.9537 | 0.9521 | 0.0897 | facet-SE ratio 0.991 | pass |
| 200×20×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0268 | 0.0226 | 0.9505 | 0.9521 | 0.0902 |  | pass |
| 200×20×2 | near-zero-rater | normal | G | F interval | 0.9259 | 1.000 | 0.9463 (0.0052) | 0.0274 | 0.0263 | 0.9463 | 0.9447 | 0.0322 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×2 | near-zero-rater | normal | Phi | parametric | 0.8809 | 1.000 | 0.9489 (0.0050) | 0.0153 | 0.0358 | 0.9489 | 0.9489 | 0.0765 |  | fail |
| 200×20×2 | near-zero-rater | normal | Phi | parametric, attached | 0.8809 | 0.745 | 0.7111 (0.0104) | 0.0132 | 0.0205 | 0.9548 | 0.7116 | 0.0790 |  | fail |
| 200×20×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9553 (0.0047) | 0.0342 | 0.0105 | 0.9553 | 0.9568 | 1.6731 | facet-SE ratio 1.020 | fail |
| 200×20×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9574 (0.0046) | 0.0242 | 0.0184 | 0.9574 | 0.9558 | 1.7197 |  | pass |
| 200×20×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9321 (0.0058) | 0.0663 | 0.0016 | 0.9321 | 0.9300 | 1.0141 | facet-SE ratio 0.985 | fail |
| 200×20×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.987 | 0.9347 (0.0057) | 0.0000 | 0.0526 | 0.9467 | 0.9363 | 16566879413.6818 |  | fail |
| 200×20×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.6526 (0.0109) | 0.3474 | 0.0000 | 0.6526 | 0.6474 | 2.7576 | facet-SE ratio 1.196 | fail |
| 200×20×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.809 | 0.7747 (0.0096) | 0.0000 | 0.0347 | 0.9571 | 0.7768 | 54166753745.4557 |  | fail |
| 200×20×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9405 (0.0054) | 0.0268 | 0.0326 | 0.9405 | 0.9421 | 0.0710 | facet-SE ratio 0.977 | pass |
| 200×20×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9405 (0.0054) | 0.0195 | 0.0400 | 0.9405 | 0.9432 | 0.0715 |  | fail |
| 200×20×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0332 | 0.0158 | 0.9511 | 0.9511 | 0.2056 | facet-SE ratio 1.017 | pass |
| 200×20×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0226 | 0.0226 | 0.9547 | 0.9558 | 0.2110 |  | pass |
| 200×20×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9247 (0.0061) | 0.0726 | 0.0026 | 0.9247 | 0.9274 | 0.6248 | facet-SE ratio 1.004 | fail |
| 200×20×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0263 | 0.0200 | 0.9537 | 0.9558 | 0.7756 |  | pass |
| 200×20×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0263 | 0.0237 | 0.9500 | 0.9516 | 0.0448 | facet-SE ratio 0.982 | pass |
| 200×20×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0216 | 0.0237 | 0.9547 | 0.9553 | 0.0451 |  | pass |
| 200×20×2 | high-g | normal | G | F interval | 0.9329 | 1.000 | 0.9553 (0.0047) | 0.0247 | 0.0200 | 0.9553 | 0.9526 | 0.0367 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×2 | high-g | normal | Phi | parametric | 0.8743 | 1.000 | 0.6937 (0.0106) | 0.0000 | 0.3063 | 0.6937 | 0.7021 | 0.1908 |  | fail |
| 200×20×2 | high-g | normal | Phi | parametric, attached | 0.8743 | 0.800 | 0.6737 (0.0108) | 0.0000 | 0.1263 | 0.8421 | 0.6784 | 0.2248 |  | fail |
| 200×20×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0411 | 0.0147 | 0.9442 | 0.9468 | 0.5542 | facet-SE ratio 1.016 | fail |
| 200×20×3 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0179 | 0.0279 | 0.9542 | 0.9542 | 0.5788 |  | pass |
| 200×20×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9316 (0.0058) | 0.0674 | 0.0011 | 0.9316 | 0.9279 | 1.7316 | facet-SE ratio 0.992 | fail |
| 200×20×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.999 | 0.9589 (0.0046) | 0.0000 | 0.0405 | 0.9595 | 0.9642 | 1574553434.7759 |  | fail |
| 200×20×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7374 (0.0101) | 0.2626 | 0.0000 | 0.7374 | 0.7400 | 3.7637 | facet-SE ratio 0.976 | fail |
| 200×20×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.944 | 0.9116 (0.0065) | 0.0000 | 0.0326 | 0.9654 | 0.9100 | 32126926446.1212 |  | fail |
| 200×20×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0221 | 0.0205 | 0.9574 | 0.9584 | 0.1215 | facet-SE ratio 1.014 | pass |
| 200×20×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0189 | 0.0242 | 0.9568 | 0.9568 | 0.1223 |  | pass |
| 200×20×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0274 | 0.0147 | 0.9579 | 0.9574 | 0.2904 | facet-SE ratio 1.026 | fail |
| 200×20×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0184 | 0.0247 | 0.9568 | 0.9568 | 0.2951 |  | pass |
| 200×20×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9321 (0.0058) | 0.0611 | 0.0068 | 0.9321 | 0.9316 | 0.8959 | facet-SE ratio 0.982 | fail |
| 200×20×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0247 | 0.0268 | 0.9484 | 0.9489 | 1.0008 |  | pass |
| 200×20×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0263 | 0.0216 | 0.9521 | 0.9521 | 0.0634 | facet-SE ratio 1.006 | pass |
| 200×20×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0232 | 0.0263 | 0.9505 | 0.9505 | 0.0638 |  | pass |
| 200×20×3 | all-one | normal | G | F interval | 0.7143 | 1.000 | 0.9516 (0.0049) | 0.0247 | 0.0237 | 0.9516 | 0.9521 | 0.1347 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×3 | all-one | normal | Phi | parametric | 0.5556 | 1.000 | 0.8247 (0.0087) | 0.0000 | 0.1753 | 0.8247 | 0.8647 | 0.3082 |  | fail |
| 200×20×3 | all-one | normal | Phi | parametric, attached | 0.5556 | 0.944 | 0.8121 (0.0090) | 0.0000 | 0.1316 | 0.8606 | 0.8479 | 0.3174 |  | fail |
| 200×20×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0342 | 0.0153 | 0.9505 | 0.9505 | 0.4188 | facet-SE ratio 1.019 | pass |
| 200×20×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9516 | 0.4304 |  | pass |
| 200×20×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9300 (0.0059) | 0.0663 | 0.0037 | 0.9300 | 0.9284 | 1.2698 | facet-SE ratio 0.994 | fail |
| 200×20×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0226 | 0.0263 | 0.9511 | 0.9505 | 1.5804 |  | pass |
| 200×20×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.7789 (0.0095) | 0.2211 | 0.0000 | 0.7789 | 0.7779 | 0.0399 | facet-SE ratio 1.010 | fail |
| 200×20×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.921 | 0.8911 (0.0071) | 0.0000 | 0.0300 | 0.9674 | 0.8921 | 422588520.9965 |  | fail |
| 200×20×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0316 | 0.0226 | 0.9458 | 0.9453 | 0.1214 | facet-SE ratio 0.984 | pass |
| 200×20×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0232 | 0.0258 | 0.9511 | 0.9505 | 0.1222 |  | pass |
| 200×20×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9574 (0.0046) | 0.0258 | 0.0168 | 0.9574 | 0.9542 | 0.0169 | facet-SE ratio 1.018 | pass |
| 200×20×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.993 | 0.9226 (0.0061) | 0.0000 | 0.0705 | 0.9290 | 0.9289 | 369280878.0359 |  | fail |
| 200×20×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9374 (0.0056) | 0.0553 | 0.0074 | 0.9374 | 0.9368 | 0.0134 | facet-SE ratio 0.997 | fail |
| 200×20×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 1.000 | 0.9679 (0.0040) | 0.0000 | 0.0321 | 0.9679 | 0.9679 | 0.0173 |  | fail |
| 200×20×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0289 | 0.0295 | 0.9416 | 0.9411 | 0.0634 | facet-SE ratio 0.980 | pass |
| 200×20×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0263 | 0.0300 | 0.9437 | 0.9437 | 0.0638 |  | pass |
| 200×20×3 | near-zero-rater | normal | G | F interval | 0.9346 | 1.000 | 0.9563 (0.0047) | 0.0205 | 0.0232 | 0.9563 | 0.9532 | 0.0271 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×3 | near-zero-rater | normal | Phi | parametric | 0.8901 | 1.000 | 0.9547 (0.0048) | 0.0147 | 0.0305 | 0.9547 | 0.9489 | 0.0677 |  | fail |
| 200×20×3 | near-zero-rater | normal | Phi | parametric, attached | 0.8901 | 0.915 | 0.8768 (0.0075) | 0.0121 | 0.0258 | 0.9586 | 0.8716 | 0.0678 |  | fail |
| 200×20×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9400 (0.0054) | 0.0379 | 0.0221 | 0.9400 | 0.9400 | 1.6399 | facet-SE ratio 0.995 | fail |
| 200×20×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9484 (0.0051) | 0.0247 | 0.0268 | 0.9484 | 0.9484 | 1.6848 |  | pass |
| 200×20×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9247 (0.0061) | 0.0726 | 0.0026 | 0.9247 | 0.9258 | 0.8550 | facet-SE ratio 0.982 | fail |
| 200×20×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.999 | 0.9621 (0.0044) | 0.0000 | 0.0368 | 0.9631 | 0.9595 | 754147246.2567 |  | fail |
| 200×20×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7600 (0.0098) | 0.2400 | 0.0000 | 0.7600 | 0.7653 | 1.8944 | facet-SE ratio 1.012 | fail |
| 200×20×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.954 | 0.9316 (0.0058) | 0.0000 | 0.0221 | 0.9768 | 0.9295 | 10412973941.7098 |  | fail |
| 200×20×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0263 | 0.0242 | 0.9495 | 0.9495 | 0.0608 | facet-SE ratio 1.004 | pass |
| 200×20×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0211 | 0.0268 | 0.9521 | 0.9521 | 0.0611 |  | pass |
| 200×20×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9426 (0.0053) | 0.0368 | 0.0205 | 0.9426 | 0.9437 | 0.1449 | facet-SE ratio 0.998 | fail |
| 200×20×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0247 | 0.0237 | 0.9516 | 0.9484 | 0.1472 |  | pass |
| 200×20×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9274 (0.0060) | 0.0658 | 0.0068 | 0.9274 | 0.9274 | 0.4469 | facet-SE ratio 0.961 | fail |
| 200×20×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9384 (0.0055) | 0.0295 | 0.0321 | 0.9384 | 0.9389 | 0.4984 |  | fail |
| 200×20×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0258 | 0.0226 | 0.9516 | 0.9505 | 0.0317 | facet-SE ratio 0.995 | pass |
| 200×20×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0216 | 0.0247 | 0.9537 | 0.9532 | 0.0319 |  | pass |
| 200×20×3 | high-g | normal | G | F interval | 0.9524 | 1.000 | 0.9474 (0.0051) | 0.0263 | 0.0263 | 0.9474 | 0.9458 | 0.0223 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×3 | high-g | normal | Phi | parametric | 0.9091 | 1.000 | 0.8005 (0.0092) | 0.0000 | 0.1995 | 0.8005 | 0.8116 | 0.1230 |  | fail |
| 200×20×3 | high-g | normal | Phi | parametric, attached | 0.9091 | 0.953 | 0.7974 (0.0092) | 0.0000 | 0.1553 | 0.8370 | 0.8079 | 0.1275 |  | fail |
| 200×20×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9405 (0.0054) | 0.0395 | 0.0200 | 0.9405 | 0.9411 | 0.4950 | facet-SE ratio 0.993 | fail |
| 200×20×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0184 | 0.0332 | 0.9484 | 0.9505 | 0.5130 |  | pass |
| 200×20×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9100 (0.0066) | 0.0868 | 0.0032 | 0.9100 | 0.9137 | 1.5051 | facet-SE ratio 0.961 | fail |
| 200×20×5 | all-one | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9726 (0.0037) | 0.0000 | 0.0274 | 0.9726 | 0.9711 | 2.0848 |  | fail |
| 200×20×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8358 (0.0085) | 0.1642 | 0.0000 | 0.8358 | 0.8311 | 2.8304 | facet-SE ratio 0.959 | fail |
| 200×20×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.997 | 0.9668 (0.0041) | 0.0000 | 0.0305 | 0.9694 | 0.9684 | 1878929939.9570 |  | fail |
| 200×20×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0242 | 0.0268 | 0.9489 | 0.9495 | 0.1083 | facet-SE ratio 1.020 | pass |
| 200×20×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0216 | 0.0279 | 0.9505 | 0.9526 | 0.1088 |  | pass |
| 200×20×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0247 | 0.0216 | 0.9537 | 0.9542 | 0.2056 | facet-SE ratio 1.006 | pass |
| 200×20×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0195 | 0.0258 | 0.9547 | 0.9547 | 0.2074 |  | pass |
| 200×20×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9353 (0.0056) | 0.0542 | 0.0105 | 0.9353 | 0.9374 | 0.6336 | facet-SE ratio 0.949 | fail |
| 200×20×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0263 | 0.0253 | 0.9484 | 0.9479 | 0.6711 |  | pass |
| 200×20×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0274 | 0.0289 | 0.9437 | 0.9463 | 0.0449 | facet-SE ratio 0.989 | pass |
| 200×20×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0237 | 0.0279 | 0.9484 | 0.9489 | 0.0451 |  | pass |
| 200×20×5 | all-one | normal | G | F interval | 0.7937 | 1.000 | 0.9453 (0.0052) | 0.0295 | 0.0253 | 0.9453 | 0.9453 | 0.0885 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×5 | all-one | normal | Phi | parametric | 0.6579 | 1.000 | 0.8984 (0.0069) | 0.0021 | 0.0995 | 0.8984 | 0.9032 | 0.2398 |  | fail |
| 200×20×5 | all-one | normal | Phi | parametric, attached | 0.6579 | 0.997 | 0.8984 (0.0069) | 0.0021 | 0.0968 | 0.9008 | 0.9032 | 0.2402 |  | fail |
| 200×20×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0405 | 0.0205 | 0.9389 | 0.9384 | 0.4153 | facet-SE ratio 0.984 | fail |
| 200×20×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0247 | 0.0305 | 0.9447 | 0.9453 | 0.4268 |  | pass |
| 200×20×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9221 (0.0061) | 0.0753 | 0.0026 | 0.9221 | 0.9253 | 1.2500 | facet-SE ratio 1.000 | fail |
| 200×20×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0226 | 0.0163 | 0.9611 | 0.9605 | 1.5554 |  | fail |
| 200×20×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8268 (0.0087) | 0.1732 | 0.0000 | 0.8268 | 0.8279 | 0.0285 | facet-SE ratio 0.940 | fail |
| 200×20×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.989 | 0.9611 (0.0044) | 0.0000 | 0.0279 | 0.9718 | 0.9611 | 56967539.5072 |  | fail |
| 200×20×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9563 (0.0047) | 0.0263 | 0.0174 | 0.9563 | 0.9568 | 0.1081 | facet-SE ratio 1.035 | pass |
| 200×20×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0221 | 0.0195 | 0.9584 | 0.9579 | 0.1087 |  | pass |
| 200×20×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9526 (0.0049) | 0.0263 | 0.0211 | 0.9526 | 0.9526 | 0.0119 | facet-SE ratio 1.009 | pass |
| 200×20×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 1.000 | 0.9374 (0.0056) | 0.0000 | 0.0626 | 0.9374 | 0.9374 | 120862.0706 |  | fail |
| 200×20×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9479 (0.0051) | 0.0447 | 0.0074 | 0.9479 | 0.9489 | 0.0094 | facet-SE ratio 0.996 | fail |
| 200×20×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 1.000 | 0.9626 (0.0044) | 0.0032 | 0.0342 | 0.9626 | 0.9626 | 0.0107 |  | fail |
| 200×20×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0221 | 0.0232 | 0.9547 | 0.9542 | 0.0449 | facet-SE ratio 1.006 | pass |
| 200×20×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0216 | 0.0216 | 0.9568 | 0.9568 | 0.0451 |  | pass |
| 200×20×5 | near-zero-rater | normal | G | F interval | 0.9416 | 1.000 | 0.9458 (0.0052) | 0.0242 | 0.0300 | 0.9458 | 0.9437 | 0.0238 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×5 | near-zero-rater | normal | Phi | parametric | 0.8976 | 1.000 | 0.9411 (0.0054) | 0.0126 | 0.0463 | 0.9411 | 0.9416 | 0.0643 |  | fail |
| 200×20×5 | near-zero-rater | normal | Phi | parametric, attached | 0.8976 | 0.989 | 0.9311 (0.0058) | 0.0126 | 0.0453 | 0.9415 | 0.9316 | 0.0643 |  | fail |
| 200×20×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9442 (0.0053) | 0.0416 | 0.0142 | 0.9442 | 0.9421 | 1.6162 | facet-SE ratio 0.996 | fail |
| 200×20×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9521 (0.0049) | 0.0274 | 0.0205 | 0.9521 | 0.9532 | 1.6578 |  | pass |
| 200×20×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9358 (0.0056) | 0.0626 | 0.0016 | 0.9358 | 0.9363 | 0.7594 | facet-SE ratio 0.999 | fail |
| 200×20×5 | high-g | normal | i | Satterthwaite | 0.5000 | 1.000 | 0.9705 (0.0039) | 0.0000 | 0.0295 | 0.9705 | 0.9705 | 1.0534 |  | fail |
| 200×20×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8311 (0.0086) | 0.1689 | 0.0000 | 0.8311 | 0.8347 | 1.3765 | facet-SE ratio 0.928 | fail |
| 200×20×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.996 | 0.9679 (0.0040) | 0.0000 | 0.0279 | 0.9720 | 0.9653 | 1426885045.5260 |  | fail |
| 200×20×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9426 (0.0053) | 0.0321 | 0.0253 | 0.9426 | 0.9442 | 0.0540 | facet-SE ratio 1.001 | pass |
| 200×20×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0268 | 0.0295 | 0.9437 | 0.9432 | 0.0544 |  | pass |
| 200×20×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0316 | 0.0216 | 0.9468 | 0.9468 | 0.1027 | facet-SE ratio 0.997 | pass |
| 200×20×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0253 | 0.0247 | 0.9500 | 0.9511 | 0.1037 |  | pass |
| 200×20×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0453 | 0.0111 | 0.9437 | 0.9400 | 0.3197 | facet-SE ratio 0.972 | fail |
| 200×20×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0232 | 0.0253 | 0.9516 | 0.9516 | 0.3379 |  | pass |
| 200×20×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0247 | 0.0258 | 0.9495 | 0.9500 | 0.0224 | facet-SE ratio 1.017 | pass |
| 200×20×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0211 | 0.0247 | 0.9542 | 0.9547 | 0.0225 |  | pass |
| 200×20×5 | high-g | normal | G | F interval | 0.9685 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9489 | 0.0135 | unfit 0, errors 0, failed fits 0 | pass |
| 200×20×5 | high-g | normal | Phi | parametric | 0.9390 | 1.000 | 0.8695 (0.0077) | 0.0005 | 0.1300 | 0.8695 | 0.8716 | 0.0654 |  | fail |
| 200×20×5 | high-g | normal | Phi | parametric, attached | 0.9390 | 0.996 | 0.8695 (0.0077) | 0.0005 | 0.1258 | 0.8732 | 0.8716 | 0.0656 |  | fail |
| 50×5×2 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9158 (0.0064) | 0.0621 | 0.0221 | 0.9158 | 0.9163 | 0.0451 | facet-SE ratio 0.861 | fail |
| 50×5×2 | all-one | binary | p | Satterthwaite | 0.0228 | 0.983 | 0.8989 (0.0069) | 0.0000 | 0.0837 | 0.9148 | 0.8989 | 1931839263.4234 |  | fail |
| 50×5×2 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.9353 (0.0056) | 0.0647 | 0.0000 | 0.9353 | 0.9389 | 0.1152 | facet-SE ratio 0.943 | fail |
| 50×5×2 | all-one | binary | i | Satterthwaite | 0.0228 | 0.812 | 0.7379 (0.0101) | 0.0000 | 0.0737 | 0.9092 | 0.7347 | 26847460688.3672 |  | fail |
| 50×5×2 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.8263 (0.0087) | 0.1737 | 0.0000 | 0.8263 | 0.8268 | 0.1579 | facet-SE ratio 1.163 | fail |
| 50×5×2 | all-one | binary | r | Satterthwaite | 0.0228 | 0.680 | 0.6321 (0.0111) | 0.0000 | 0.0479 | 0.9296 | 0.6316 | 28561443057.9139 |  | fail |
| 50×5×2 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.8647 (0.0078) | 0.0879 | 0.0474 | 0.8647 | 0.8653 | 0.0376 | facet-SE ratio 0.804 | fail |
| 50×5×2 | all-one | binary | pi | Satterthwaite | 0.0249 | 0.989 | 0.8711 (0.0077) | 0.0000 | 0.1184 | 0.8803 | 0.8758 | 585971423.5197 |  | fail |
| 50×5×2 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8884 (0.0072) | 0.0937 | 0.0179 | 0.8884 | 0.8884 | 0.0374 | facet-SE ratio 0.839 | fail |
| 50×5×2 | all-one | binary | pr | Satterthwaite | 0.0249 | 0.999 | 0.9232 (0.0061) | 0.0000 | 0.0758 | 0.9241 | 0.9253 | 86039903.9682 |  | fail |
| 50×5×2 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.8537 (0.0081) | 0.1463 | 0.0000 | 0.8537 | 0.8489 | 0.0733 | facet-SE ratio 0.952 | fail |
| 50×5×2 | all-one | binary | ir | Satterthwaite | 0.0249 | 0.989 | 0.9595 (0.0045) | 0.0000 | 0.0300 | 0.9697 | 0.9611 | 150880036.7580 |  | fail |
| 50×5×2 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.8605 (0.0079) | 0.1005 | 0.0389 | 0.8605 | 0.8621 | 0.0421 | facet-SE ratio 0.736 | fail |
| 50×5×2 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.8684 (0.0078) | 0.0821 | 0.0495 | 0.8684 | 0.8684 | 0.0431 |  | fail |
| 50×5×2 | all-one | binary | G | F interval | 0.4481 | 1.000 | 0.9421 (0.0054) | 0.0284 | 0.0295 | 0.9421 | 0.9389 | 0.5477 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×2 | all-one | binary | Phi | parametric | 0.3289 | 1.000 | 0.9458 (0.0052) | 0.0232 | 0.0311 | 0.9458 | 0.9474 | 0.5018 |  | pass |
| 50×5×2 | all-one | binary | Phi | parametric, attached | 0.3289 | 0.541 | 0.5221 (0.0115) | 0.0100 | 0.0089 | 0.9650 | 0.5221 | 0.5033 |  | fail |
| 50×5×3 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9047 (0.0067) | 0.0832 | 0.0121 | 0.9047 | 0.9047 | 0.0353 | facet-SE ratio 0.854 | fail |
| 50×5×3 | all-one | binary | p | Satterthwaite | 0.0228 | 0.998 | 0.9395 (0.0055) | 0.0000 | 0.0589 | 0.9410 | 0.9389 | 274993811.3449 |  | fail |
| 50×5×3 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.9142 (0.0064) | 0.0858 | 0.0000 | 0.9142 | 0.9032 | 0.0931 | facet-SE ratio 0.943 | fail |
| 50×5×3 | all-one | binary | i | Satterthwaite | 0.0228 | 0.883 | 0.8332 (0.0086) | 0.0000 | 0.0495 | 0.9439 | 0.8379 | 9837859106.3208 |  | fail |
| 50×5×3 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.8353 (0.0085) | 0.1647 | 0.0000 | 0.8353 | 0.8279 | 0.1112 | facet-SE ratio 1.013 | fail |
| 50×5×3 | all-one | binary | r | Satterthwaite | 0.0228 | 0.832 | 0.7911 (0.0093) | 0.0000 | 0.0405 | 0.9513 | 0.7926 | 8614860930.8064 |  | fail |
| 50×5×3 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.8558 (0.0081) | 0.0963 | 0.0479 | 0.8558 | 0.8574 | 0.0258 | facet-SE ratio 0.784 | fail |
| 50×5×3 | all-one | binary | pi | Satterthwaite | 0.0249 | 1.000 | 0.9042 (0.0068) | 0.0000 | 0.0958 | 0.9042 | 0.9037 | 21.1133 |  | fail |
| 50×5×3 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8763 (0.0076) | 0.0905 | 0.0332 | 0.8763 | 0.8779 | 0.0262 | facet-SE ratio 0.814 | fail |
| 50×5×3 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.9268 (0.0060) | 0.0026 | 0.0705 | 0.9268 | 0.9221 | 1125625.0516 |  | fail |
| 50×5×3 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.8874 (0.0073) | 0.1126 | 0.0000 | 0.8874 | 0.8874 | 0.0517 | facet-SE ratio 0.953 | fail |
| 50×5×3 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9700 (0.0039) | 0.0000 | 0.0300 | 0.9700 | 0.9700 | 26136803.0451 |  | fail |
| 50×5×3 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.8479 (0.0082) | 0.1068 | 0.0453 | 0.8479 | 0.8458 | 0.0297 | facet-SE ratio 0.686 | fail |
| 50×5×3 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.8453 (0.0083) | 0.0958 | 0.0589 | 0.8453 | 0.8463 | 0.0302 |  | fail |
| 50×5×3 | all-one | binary | G | F interval | 0.5280 | 1.000 | 0.9563 (0.0047) | 0.0268 | 0.0168 | 0.9563 | 0.9537 | 0.4561 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×3 | all-one | binary | Phi | parametric | 0.4000 | 1.000 | 0.9458 (0.0052) | 0.0326 | 0.0216 | 0.9458 | 0.9484 | 0.4568 |  | pass |
| 50×5×3 | all-one | binary | Phi | parametric, attached | 0.4000 | 0.738 | 0.7037 (0.0105) | 0.0247 | 0.0100 | 0.9530 | 0.7063 | 0.4577 |  | fail |
| 50×5×5 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9174 (0.0063) | 0.0732 | 0.0095 | 0.9174 | 0.9158 | 0.0297 | facet-SE ratio 0.873 | fail |
| 50×5×5 | all-one | binary | p | Satterthwaite | 0.0228 | 1.000 | 0.9558 (0.0047) | 0.0005 | 0.0437 | 0.9558 | 0.9579 | 0.4475 |  | fail |
| 50×5×5 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.8705 (0.0077) | 0.1295 | 0.0000 | 0.8705 | 0.8684 | 0.0781 | facet-SE ratio 0.981 | fail |
| 50×5×5 | all-one | binary | i | Satterthwaite | 0.0228 | 0.939 | 0.9047 (0.0067) | 0.0000 | 0.0342 | 0.9636 | 0.9068 | 2651216020.3470 |  | fail |
| 50×5×5 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.8453 (0.0083) | 0.1547 | 0.0000 | 0.8453 | 0.8495 | 0.0760 | facet-SE ratio 0.972 | fail |
| 50×5×5 | all-one | binary | r | Satterthwaite | 0.0228 | 0.931 | 0.9000 (0.0069) | 0.0000 | 0.0311 | 0.9666 | 0.8968 | 3038219675.8025 |  | fail |
| 50×5×5 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.8521 (0.0081) | 0.1005 | 0.0474 | 0.8521 | 0.8542 | 0.0188 | facet-SE ratio 0.762 | fail |
| 50×5×5 | all-one | binary | pi | Satterthwaite | 0.0249 | 1.000 | 0.8800 (0.0075) | 0.0342 | 0.0858 | 0.8800 | 0.8816 | 0.0203 |  | fail |
| 50×5×5 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8632 (0.0079) | 0.0889 | 0.0479 | 0.8632 | 0.8616 | 0.0188 | facet-SE ratio 0.770 | fail |
| 50×5×5 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.8847 (0.0073) | 0.0326 | 0.0826 | 0.8847 | 0.8868 | 0.0203 |  | fail |
| 50×5×5 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.9179 (0.0063) | 0.0795 | 0.0026 | 0.9179 | 0.9147 | 0.0371 | facet-SE ratio 1.004 | fail |
| 50×5×5 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9579 (0.0046) | 0.0089 | 0.0332 | 0.9579 | 0.9595 | 0.0502 |  | fail |
| 50×5×5 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.8147 (0.0089) | 0.1042 | 0.0811 | 0.8147 | 0.8168 | 0.0212 | facet-SE ratio 0.648 | fail |
| 50×5×5 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.8153 (0.0089) | 0.0937 | 0.0911 | 0.8153 | 0.8189 | 0.0213 |  | fail |
| 50×5×5 | all-one | binary | G | F interval | 0.6160 | 1.000 | 0.9547 (0.0048) | 0.0226 | 0.0226 | 0.9547 | 0.9458 | 0.3526 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×5 | all-one | binary | Phi | parametric | 0.4838 | 1.000 | 0.9484 (0.0051) | 0.0326 | 0.0189 | 0.9484 | 0.9511 | 0.4034 |  | pass |
| 50×5×5 | all-one | binary | Phi | parametric, attached | 0.4838 | 0.876 | 0.8347 (0.0085) | 0.0305 | 0.0105 | 0.9531 | 0.8379 | 0.4045 |  | fail |
| 50×8×2 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9111 (0.0065) | 0.0705 | 0.0184 | 0.9111 | 0.9068 | 0.0392 | facet-SE ratio 0.853 | fail |
| 50×8×2 | all-one | binary | p | Satterthwaite | 0.0228 | 0.993 | 0.9147 (0.0064) | 0.0000 | 0.0779 | 0.9215 | 0.9200 | 552977384.2603 |  | fail |
| 50×8×2 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.9532 (0.0048) | 0.0458 | 0.0011 | 0.9532 | 0.9563 | 0.0843 | facet-SE ratio 0.972 | fail |
| 50×8×2 | all-one | binary | i | Satterthwaite | 0.0228 | 0.884 | 0.8321 (0.0086) | 0.0000 | 0.0516 | 0.9416 | 0.8237 | 12081816859.1499 |  | fail |
| 50×8×2 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.7337 (0.0101) | 0.2663 | 0.0000 | 0.7337 | 0.7453 | 0.1378 | facet-SE ratio 1.181 | fail |
| 50×8×2 | all-one | binary | r | Satterthwaite | 0.0228 | 0.713 | 0.6758 (0.0107) | 0.0000 | 0.0374 | 0.9476 | 0.6737 | 14573358908.5037 |  | fail |
| 50×8×2 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.8574 (0.0080) | 0.0900 | 0.0526 | 0.8574 | 0.8558 | 0.0284 | facet-SE ratio 0.780 | fail |
| 50×8×2 | all-one | binary | pi | Satterthwaite | 0.0249 | 0.998 | 0.8879 (0.0072) | 0.0000 | 0.1105 | 0.8893 | 0.8884 | 44274491.1049 |  | fail |
| 50×8×2 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8763 (0.0076) | 0.1032 | 0.0205 | 0.8763 | 0.8763 | 0.0304 | facet-SE ratio 0.829 | fail |
| 50×8×2 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.9347 (0.0057) | 0.0016 | 0.0637 | 0.9347 | 0.9347 | 0.0387 |  | fail |
| 50×8×2 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.8853 (0.0073) | 0.1147 | 0.0000 | 0.8853 | 0.8789 | 0.0561 | facet-SE ratio 0.941 | fail |
| 50×8×2 | all-one | binary | ir | Satterthwaite | 0.0249 | 0.999 | 0.9711 (0.0038) | 0.0000 | 0.0284 | 0.9716 | 0.9742 | 43303534.3007 |  | fail |
| 50×8×2 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.8332 (0.0086) | 0.1074 | 0.0595 | 0.8332 | 0.8342 | 0.0319 | facet-SE ratio 0.680 | fail |
| 50×8×2 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.8337 (0.0085) | 0.0926 | 0.0737 | 0.8337 | 0.8368 | 0.0325 |  | fail |
| 50×8×2 | all-one | binary | G | F interval | 0.5065 | 1.000 | 0.9468 (0.0051) | 0.0237 | 0.0295 | 0.9468 | 0.9432 | 0.5193 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×2 | all-one | binary | Phi | parametric | 0.3749 | 1.000 | 0.9474 (0.0051) | 0.0132 | 0.0395 | 0.9474 | 0.9489 | 0.5059 |  | fail |
| 50×8×2 | all-one | binary | Phi | parametric, attached | 0.3749 | 0.631 | 0.6163 (0.0112) | 0.0058 | 0.0089 | 0.9766 | 0.6163 | 0.5140 |  | fail |
| 50×8×3 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9253 (0.0060) | 0.0621 | 0.0126 | 0.9253 | 0.9263 | 0.0313 | facet-SE ratio 0.880 | fail |
| 50×8×3 | all-one | binary | p | Satterthwaite | 0.0228 | 1.000 | 0.9489 (0.0050) | 0.0005 | 0.0505 | 0.9489 | 0.9484 | 14783683.1446 |  | fail |
| 50×8×3 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.8932 (0.0071) | 0.1068 | 0.0000 | 0.8932 | 0.9016 | 0.0671 | facet-SE ratio 0.974 | fail |
| 50×8×3 | all-one | binary | i | Satterthwaite | 0.0228 | 0.952 | 0.9174 (0.0063) | 0.0000 | 0.0342 | 0.9640 | 0.9116 | 2963375749.5544 |  | fail |
| 50×8×3 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.7926 (0.0093) | 0.2074 | 0.0000 | 0.7926 | 0.7895 | 0.1011 | facet-SE ratio 1.030 | fail |
| 50×8×3 | all-one | binary | r | Satterthwaite | 0.0228 | 0.873 | 0.8405 (0.0084) | 0.0000 | 0.0321 | 0.9632 | 0.8421 | 4116687680.3578 |  | fail |
| 50×8×3 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.8300 (0.0086) | 0.1168 | 0.0532 | 0.8300 | 0.8305 | 0.0194 | facet-SE ratio 0.719 | fail |
| 50×8×3 | all-one | binary | pi | Satterthwaite | 0.0249 | 1.000 | 0.8863 (0.0073) | 0.0184 | 0.0953 | 0.8863 | 0.8811 | 9.1110 |  | fail |
| 50×8×3 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8800 (0.0075) | 0.0884 | 0.0316 | 0.8800 | 0.8800 | 0.0214 | facet-SE ratio 0.802 | fail |
| 50×8×3 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.9021 (0.0068) | 0.0274 | 0.0705 | 0.9021 | 0.9021 | 0.0237 |  | fail |
| 50×8×3 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.9053 (0.0067) | 0.0926 | 0.0021 | 0.9053 | 0.9047 | 0.0394 | facet-SE ratio 0.921 | fail |
| 50×8×3 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9453 (0.0052) | 0.0174 | 0.0374 | 0.9453 | 0.9458 | 0.0557 |  | fail |
| 50×8×3 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.8053 (0.0091) | 0.1216 | 0.0732 | 0.8053 | 0.8016 | 0.0224 | facet-SE ratio 0.629 | fail |
| 50×8×3 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.8063 (0.0091) | 0.1111 | 0.0826 | 0.8063 | 0.8016 | 0.0227 |  | fail |
| 50×8×3 | all-one | binary | G | F interval | 0.5900 | 1.000 | 0.9537 (0.0048) | 0.0216 | 0.0247 | 0.9537 | 0.9474 | 0.3993 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×3 | all-one | binary | Phi | parametric | 0.4548 | 1.000 | 0.9553 (0.0047) | 0.0184 | 0.0263 | 0.9553 | 0.9537 | 0.4426 |  | pass |
| 50×8×3 | all-one | binary | Phi | parametric, attached | 0.4548 | 0.832 | 0.8016 (0.0091) | 0.0174 | 0.0132 | 0.9633 | 0.8016 | 0.4471 |  | fail |
| 50×8×5 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9216 (0.0062) | 0.0689 | 0.0095 | 0.9216 | 0.9216 | 0.0266 | facet-SE ratio 0.880 | fail |
| 50×8×5 | all-one | binary | p | Satterthwaite | 0.0228 | 1.000 | 0.9584 (0.0046) | 0.0011 | 0.0405 | 0.9584 | 0.9584 | 0.0321 |  | fail |
| 50×8×5 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.8853 (0.0073) | 0.1147 | 0.0000 | 0.8853 | 0.8868 | 0.0585 | facet-SE ratio 0.961 | fail |
| 50×8×5 | all-one | binary | i | Satterthwaite | 0.0228 | 0.982 | 0.9595 (0.0045) | 0.0000 | 0.0221 | 0.9775 | 0.9579 | 817410746.0750 |  | fail |
| 50×8×5 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.8542 (0.0081) | 0.1458 | 0.0000 | 0.8542 | 0.8500 | 0.0719 | facet-SE ratio 0.976 | fail |
| 50×8×5 | all-one | binary | r | Satterthwaite | 0.0228 | 0.966 | 0.9395 (0.0055) | 0.0000 | 0.0263 | 0.9728 | 0.9426 | 1233450652.9967 |  | fail |
| 50×8×5 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.8300 (0.0086) | 0.1126 | 0.0574 | 0.8300 | 0.8326 | 0.0141 | facet-SE ratio 0.715 | fail |
| 50×8×5 | all-one | binary | pi | Satterthwaite | 0.0249 | 1.000 | 0.8384 (0.0084) | 0.0695 | 0.0921 | 0.8384 | 0.8389 | 0.0148 |  | fail |
| 50×8×5 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8421 (0.0084) | 0.1068 | 0.0511 | 0.8421 | 0.8416 | 0.0152 | facet-SE ratio 0.741 | fail |
| 50×8×5 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.8547 (0.0081) | 0.0600 | 0.0853 | 0.8547 | 0.8542 | 0.0160 |  | fail |
| 50×8×5 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.9226 (0.0061) | 0.0711 | 0.0063 | 0.9226 | 0.9221 | 0.0280 | facet-SE ratio 0.949 | fail |
| 50×8×5 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9495 (0.0050) | 0.0174 | 0.0332 | 0.9495 | 0.9495 | 0.0332 |  | pass |
| 50×8×5 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.7874 (0.0094) | 0.1237 | 0.0889 | 0.7874 | 0.7874 | 0.0159 | facet-SE ratio 0.618 | fail |
| 50×8×5 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.7853 (0.0094) | 0.1163 | 0.0984 | 0.7853 | 0.7847 | 0.0160 |  | fail |
| 50×8×5 | all-one | binary | G | F interval | 0.6796 | 1.000 | 0.9558 (0.0047) | 0.0211 | 0.0232 | 0.9558 | 0.9505 | 0.2871 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×5 | all-one | binary | Phi | parametric | 0.5484 | 1.000 | 0.9468 (0.0051) | 0.0321 | 0.0211 | 0.9468 | 0.9489 | 0.3661 |  | pass |
| 50×8×5 | all-one | binary | Phi | parametric, attached | 0.5484 | 0.948 | 0.8989 (0.0069) | 0.0321 | 0.0174 | 0.9478 | 0.9026 | 0.3671 |  | fail |
| 50×20×2 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9147 (0.0064) | 0.0716 | 0.0137 | 0.9147 | 0.9121 | 0.0333 | facet-SE ratio 0.875 | fail |
| 50×20×2 | all-one | binary | p | Satterthwaite | 0.0228 | 1.000 | 0.9411 (0.0054) | 0.0005 | 0.0584 | 0.9411 | 0.9437 | 210273448.3952 |  | fail |
| 50×20×2 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.9337 (0.0057) | 0.0611 | 0.0053 | 0.9337 | 0.9326 | 0.0502 | facet-SE ratio 0.928 | fail |
| 50×20×2 | all-one | binary | i | Satterthwaite | 0.0228 | 0.983 | 0.9263 (0.0060) | 0.0000 | 0.0563 | 0.9427 | 0.9263 | 2344459737.1910 |  | fail |
| 50×20×2 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.6716 (0.0108) | 0.3284 | 0.0000 | 0.6716 | 0.6653 | 0.1310 | facet-SE ratio 1.218 | fail |
| 50×20×2 | all-one | binary | r | Satterthwaite | 0.0228 | 0.791 | 0.7542 (0.0099) | 0.0000 | 0.0363 | 0.9541 | 0.7584 | 4876137713.8093 |  | fail |
| 50×20×2 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.7658 (0.0097) | 0.1416 | 0.0926 | 0.7658 | 0.7684 | 0.0171 | facet-SE ratio 0.644 | fail |
| 50×20×2 | all-one | binary | pi | Satterthwaite | 0.0249 | 0.999 | 0.8300 (0.0086) | 0.0311 | 0.1384 | 0.8304 | 0.8274 | 41098709.1901 |  | fail |
| 50×20×2 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.9074 (0.0067) | 0.0784 | 0.0142 | 0.9074 | 0.9105 | 0.0235 | facet-SE ratio 0.891 | fail |
| 50×20×2 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.9326 (0.0058) | 0.0258 | 0.0416 | 0.9326 | 0.9311 | 0.0266 |  | fail |
| 50×20×2 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.9111 (0.0065) | 0.0847 | 0.0042 | 0.9111 | 0.9105 | 0.0339 | facet-SE ratio 0.934 | fail |
| 50×20×2 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9484 (0.0051) | 0.0179 | 0.0337 | 0.9484 | 0.9495 | 0.0437 |  | pass |
| 50×20×2 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.7747 (0.0096) | 0.1289 | 0.0963 | 0.7747 | 0.7763 | 0.0193 | facet-SE ratio 0.618 | fail |
| 50×20×2 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.7732 (0.0096) | 0.1247 | 0.1021 | 0.7732 | 0.7737 | 0.0195 |  | fail |
| 50×20×2 | all-one | binary | G | F interval | 0.5826 | 1.000 | 0.9516 (0.0049) | 0.0221 | 0.0263 | 0.9516 | 0.9489 | 0.4742 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×2 | all-one | binary | Phi | parametric | 0.4360 | 1.000 | 0.9305 (0.0058) | 0.0074 | 0.0621 | 0.9305 | 0.9384 | 0.5082 |  | fail |
| 50×20×2 | all-one | binary | Phi | parametric, attached | 0.4360 | 0.779 | 0.7463 (0.0100) | 0.0068 | 0.0258 | 0.9581 | 0.7500 | 0.5244 |  | fail |
| 50×20×3 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9179 (0.0063) | 0.0747 | 0.0074 | 0.9179 | 0.9158 | 0.0274 | facet-SE ratio 0.867 | fail |
| 50×20×3 | all-one | binary | p | Satterthwaite | 0.0228 | 1.000 | 0.9526 (0.0049) | 0.0037 | 0.0437 | 0.9526 | 0.9537 | 0.0338 |  | fail |
| 50×20×3 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.9200 (0.0062) | 0.0789 | 0.0011 | 0.9200 | 0.9268 | 0.0405 | facet-SE ratio 0.938 | fail |
| 50×20×3 | all-one | binary | i | Satterthwaite | 0.0228 | 0.997 | 0.9689 (0.0040) | 0.0000 | 0.0284 | 0.9715 | 0.9621 | 324272188.5096 |  | fail |
| 50×20×3 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.7805 (0.0095) | 0.2195 | 0.0000 | 0.7805 | 0.7737 | 0.0935 | facet-SE ratio 1.031 | fail |
| 50×20×3 | all-one | binary | r | Satterthwaite | 0.0228 | 0.937 | 0.9132 (0.0065) | 0.0000 | 0.0242 | 0.9742 | 0.9158 | 1025316375.3059 |  | fail |
| 50×20×3 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.7505 (0.0099) | 0.1574 | 0.0921 | 0.7505 | 0.7453 | 0.0118 | facet-SE ratio 0.616 | fail |
| 50×20×3 | all-one | binary | pi | Satterthwaite | 0.0249 | 1.000 | 0.7574 (0.0098) | 0.1168 | 0.1258 | 0.7574 | 0.7542 | 0.0122 |  | fail |
| 50×20×3 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8932 (0.0071) | 0.0863 | 0.0205 | 0.8932 | 0.8947 | 0.0168 | facet-SE ratio 0.830 | fail |
| 50×20×3 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.9147 (0.0064) | 0.0437 | 0.0416 | 0.9147 | 0.9126 | 0.0179 |  | fail |
| 50×20×3 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.9221 (0.0061) | 0.0705 | 0.0074 | 0.9221 | 0.9174 | 0.0242 | facet-SE ratio 0.930 | fail |
| 50×20×3 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9432 (0.0053) | 0.0242 | 0.0326 | 0.9432 | 0.9437 | 0.0275 |  | pass |
| 50×20×3 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.7342 (0.0101) | 0.1463 | 0.1195 | 0.7342 | 0.7353 | 0.0137 | facet-SE ratio 0.575 | fail |
| 50×20×3 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.7337 (0.0101) | 0.1411 | 0.1253 | 0.7337 | 0.7321 | 0.0138 |  | fail |
| 50×20×3 | all-one | binary | G | F interval | 0.6685 | 1.000 | 0.9547 (0.0048) | 0.0221 | 0.0232 | 0.9547 | 0.9495 | 0.3293 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×3 | all-one | binary | Phi | parametric | 0.5270 | 1.000 | 0.9274 (0.0060) | 0.0137 | 0.0589 | 0.9274 | 0.9247 | 0.4249 |  | fail |
| 50×20×3 | all-one | binary | Phi | parametric, attached | 0.5270 | 0.935 | 0.8774 (0.0075) | 0.0137 | 0.0437 | 0.9386 | 0.8753 | 0.4310 |  | fail |
| 50×20×5 | all-one | binary | p | parametric | 0.0228 | 1.000 | 0.9305 (0.0058) | 0.0595 | 0.0100 | 0.9305 | 0.9311 | 0.0238 | facet-SE ratio 0.886 | fail |
| 50×20×5 | all-one | binary | p | Satterthwaite | 0.0228 | 1.000 | 0.9453 (0.0052) | 0.0126 | 0.0421 | 0.9453 | 0.9453 | 0.0275 |  | fail |
| 50×20×5 | all-one | binary | i | parametric | 0.0228 | 1.000 | 0.9353 (0.0056) | 0.0611 | 0.0037 | 0.9353 | 0.9353 | 0.0362 | facet-SE ratio 0.932 | fail |
| 50×20×5 | all-one | binary | i | Satterthwaite | 0.0228 | 0.999 | 0.9705 (0.0039) | 0.0000 | 0.0289 | 0.9710 | 0.9689 | 0.0667 |  | fail |
| 50×20×5 | all-one | binary | r | parametric | 0.0228 | 1.000 | 0.8542 (0.0081) | 0.1458 | 0.0000 | 0.8542 | 0.8511 | 0.0659 | facet-SE ratio 0.989 | fail |
| 50×20×5 | all-one | binary | r | Satterthwaite | 0.0228 | 0.989 | 0.9711 (0.0038) | 0.0000 | 0.0179 | 0.9819 | 0.9711 | 125090566.1428 |  | fail |
| 50×20×5 | all-one | binary | pi | parametric | 0.0249 | 1.000 | 0.7211 (0.0103) | 0.1600 | 0.1189 | 0.7211 | 0.7211 | 0.0086 | facet-SE ratio 0.600 | fail |
| 50×20×5 | all-one | binary | pi | Satterthwaite | 0.0249 | 1.000 | 0.7258 (0.0102) | 0.1332 | 0.1411 | 0.7258 | 0.7268 | 0.0088 |  | fail |
| 50×20×5 | all-one | binary | pr | parametric | 0.0249 | 1.000 | 0.8668 (0.0078) | 0.0905 | 0.0426 | 0.8668 | 0.8668 | 0.0119 | facet-SE ratio 0.783 | fail |
| 50×20×5 | all-one | binary | pr | Satterthwaite | 0.0249 | 1.000 | 0.8763 (0.0076) | 0.0600 | 0.0637 | 0.8763 | 0.8758 | 0.0123 |  | fail |
| 50×20×5 | all-one | binary | ir | parametric | 0.0249 | 1.000 | 0.9358 (0.0056) | 0.0516 | 0.0126 | 0.9358 | 0.9358 | 0.0171 | facet-SE ratio 0.947 | fail |
| 50×20×5 | all-one | binary | ir | Satterthwaite | 0.0249 | 1.000 | 0.9447 (0.0052) | 0.0258 | 0.0295 | 0.9447 | 0.9442 | 0.0182 |  | pass |
| 50×20×5 | all-one | binary | pir | parametric | 0.1070 | 1.000 | 0.6889 (0.0106) | 0.1621 | 0.1489 | 0.6889 | 0.6889 | 0.0097 | facet-SE ratio 0.572 | fail |
| 50×20×5 | all-one | binary | pir | Satterthwaite | 0.1070 | 1.000 | 0.6926 (0.0106) | 0.1542 | 0.1532 | 0.6926 | 0.6926 | 0.0097 |  | fail |
| 50×20×5 | all-one | binary | G | F interval | 0.7580 | 1.000 | 0.9474 (0.0051) | 0.0253 | 0.0274 | 0.9474 | 0.9432 | 0.2184 | unfit 0, errors 0, failed fits 0 | pass |
| 50×20×5 | all-one | binary | Phi | parametric | 0.6328 | 1.000 | 0.9384 (0.0055) | 0.0258 | 0.0358 | 0.9384 | 0.9326 | 0.3311 |  | fail |
| 50×20×5 | all-one | binary | Phi | parametric, attached | 0.6328 | 0.988 | 0.9295 (0.0059) | 0.0253 | 0.0337 | 0.9404 | 0.9237 | 0.3319 |  | fail |
