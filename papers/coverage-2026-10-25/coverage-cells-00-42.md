# G-theory coverage study (full), cells 0..90 of 90

n_sim = 1900, B = 999, nominal 0.95. Pass: coverage in [.94, .96] and each tail miss in [.015, .035]; a None interval counts as a miss. Lower miss: the interval lies entirely below the truth; upper miss: entirely above. Conditional coverage counts available intervals only; bias-eliminated coverage targets the mean point estimate.

| cell | pattern | data | estimand | method | truth | avail | coverage (MCSE) | lower miss | upper miss | conditional | bias-elim | mean width | notes | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 20×5×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9347 (0.0057) | 0.0574 | 0.0079 | 0.9347 | 0.9332 | 2.4407 | facet-SE ratio 0.963 | fail |
| 20×5×2 | all-one | normal | p | Satterthwaite | 1.0000 | 0.968 | 0.9121 (0.0065) | 0.0000 | 0.0563 | 0.9418 | 0.9142 | 170704393663.9220 |  | fail |
| 20×5×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9400 (0.0054) | 0.0595 | 0.0005 | 0.9400 | 0.9400 | 4.9318 | facet-SE ratio 0.924 | fail |
| 20×5×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.816 | 0.7400 (0.0101) | 0.0000 | 0.0758 | 0.9071 | 0.7405 | 799500756594.8252 |  | fail |
| 20×5×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8226 (0.0088) | 0.1774 | 0.0000 | 0.8226 | 0.8247 | 6.8758 | facet-SE ratio 1.135 | fail |
| 20×5×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.661 | 0.6147 (0.0112) | 0.0000 | 0.0458 | 0.9307 | 0.6147 | 1532155053067.1016 |  | fail |
| 20×5×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9347 (0.0057) | 0.0537 | 0.0116 | 0.9347 | 0.9358 | 1.0035 | facet-SE ratio 0.939 | fail |
| 20×5×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0005 | 0.0484 | 0.9511 | 0.9516 | 1.1661 |  | fail |
| 20×5×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9121 (0.0065) | 0.0842 | 0.0037 | 0.9121 | 0.9142 | 1.5007 | facet-SE ratio 0.961 | fail |
| 20×5×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9668 (0.0041) | 0.0000 | 0.0332 | 0.9668 | 0.9663 | 2.0710 |  | fail |
| 20×5×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8500 (0.0082) | 0.1500 | 0.0000 | 0.8500 | 0.8437 | 2.8493 | facet-SE ratio 0.952 | fail |
| 20×5×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 0.993 | 0.9621 (0.0044) | 0.0000 | 0.0311 | 0.9687 | 0.9647 | 883676376.8359 |  | fail |
| 20×5×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0432 | 0.0137 | 0.9432 | 0.9416 | 0.6310 | facet-SE ratio 0.985 | fail |
| 20×5×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0184 | 0.0258 | 0.9558 | 0.9558 | 0.6663 |  | pass |
| 20×5×2 | all-one | normal | G | F interval | 0.5556 | 1.000 | 0.9463 (0.0052) | 0.0237 | 0.0300 | 0.9463 | 0.9374 | 0.6513 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×2 | all-one | normal | Phi | parametric | 0.3846 | 1.000 | 0.9711 (0.0038) | 0.0121 | 0.0168 | 0.9711 | 0.9721 | 0.6196 |  | fail |
| 20×5×2 | all-one | normal | Phi | parametric, attached | 0.3846 | 0.531 | 0.5216 (0.0115) | 0.0037 | 0.0058 | 0.9822 | 0.5216 | 0.6241 |  | fail |
| 20×5×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9216 (0.0062) | 0.0747 | 0.0037 | 0.9216 | 0.9195 | 1.6877 | facet-SE ratio 0.976 | fail |
| 20×5×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 0.998 | 0.9653 (0.0042) | 0.0000 | 0.0332 | 0.9668 | 0.9668 | 667295.1491 |  | fail |
| 20×5×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8484 (0.0082) | 0.1511 | 0.0005 | 0.8484 | 0.8484 | 2.8944 | facet-SE ratio 0.913 | fail |
| 20×5×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.988 | 0.9563 (0.0047) | 0.0000 | 0.0321 | 0.9675 | 0.9563 | 5835364482.5129 |  | fail |
| 20×5×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.1745 | facet-SE ratio 1.038 | fail |
| 20×5×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.496 | 0.3958 (0.0112) | 0.0000 | 0.1000 | 0.7983 | 0.3853 | 83613145362.9140 |  | fail |
| 20×5×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0395 | 0.0105 | 0.9500 | 0.9489 | 1.0047 | facet-SE ratio 0.998 | fail |
| 20×5×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0005 | 0.0384 | 0.9611 | 0.9621 | 1.1558 |  | fail |
| 20×5×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9947 (0.0017) | 0.0000 | 0.0053 | 0.9947 | 0.9947 | 0.3242 | facet-SE ratio 0.982 | fail |
| 20×5×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.529 | 0.2347 (0.0097) | 0.0000 | 0.2942 | 0.4438 | 0.2458 | 205756783242.1884 |  | fail |
| 20×5×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.1957 | facet-SE ratio 0.927 | fail |
| 20×5×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.519 | 0.3226 (0.0107) | 0.0000 | 0.1968 | 0.6211 | 0.3405 | 96173107651.6616 |  | fail |
| 20×5×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0332 | 0.0084 | 0.9584 | 0.9584 | 0.6320 | facet-SE ratio 1.034 | fail |
| 20×5×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9595 (0.0045) | 0.0174 | 0.0232 | 0.9595 | 0.9595 | 0.6680 |  | pass |
| 20×5×2 | near-zero-rater | normal | G | F interval | 0.7663 | 1.000 | 0.9547 (0.0048) | 0.0268 | 0.0184 | 0.9547 | 0.9384 | 0.4076 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×2 | near-zero-rater | normal | Phi | parametric | 0.6618 | 1.000 | 0.9442 (0.0053) | 0.0468 | 0.0089 | 0.9442 | 0.9516 | 0.5061 |  | fail |
| 20×5×2 | near-zero-rater | normal | Phi | parametric, attached | 0.6618 | 0.098 | 0.0953 (0.0067) | 0.0032 | 0.0000 | 0.9679 | 0.0963 | 0.5204 |  | fail |
| 20×5×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9221 (0.0061) | 0.0763 | 0.0016 | 0.9221 | 0.9216 | 5.5402 | facet-SE ratio 1.007 | fail |
| 20×5×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9721 (0.0038) | 0.0063 | 0.0216 | 0.9721 | 0.9716 | 7.2002 |  | fail |
| 20×5×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0547 | 0.0000 | 0.9453 | 0.9484 | 2.4148 | facet-SE ratio 0.928 | fail |
| 20×5×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.822 | 0.7484 (0.0100) | 0.0000 | 0.0732 | 0.9110 | 0.7458 | 432225934384.8378 |  | fail |
| 20×5×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8100 (0.0090) | 0.1900 | 0.0000 | 0.8100 | 0.8284 | 3.3147 | facet-SE ratio 1.143 | fail |
| 20×5×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.652 | 0.6011 (0.0112) | 0.0000 | 0.0505 | 0.9225 | 0.5947 | 629703980920.5248 |  | fail |
| 20×5×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9395 (0.0055) | 0.0484 | 0.0121 | 0.9395 | 0.9416 | 0.4991 | facet-SE ratio 0.962 | fail |
| 20×5×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9663 (0.0041) | 0.0000 | 0.0337 | 0.9663 | 0.9611 | 0.5784 |  | fail |
| 20×5×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9111 (0.0065) | 0.0853 | 0.0037 | 0.9111 | 0.9147 | 0.7522 | facet-SE ratio 0.965 | fail |
| 20×5×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9679 (0.0040) | 0.0005 | 0.0316 | 0.9679 | 0.9674 | 1.0552 |  | fail |
| 20×5×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8516 (0.0082) | 0.1484 | 0.0000 | 0.8516 | 0.8468 | 1.4156 | facet-SE ratio 0.962 | fail |
| 20×5×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 0.997 | 0.9689 (0.0040) | 0.0000 | 0.0279 | 0.9720 | 0.9705 | 1085900135.7384 |  | fail |
| 20×5×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0437 | 0.0111 | 0.9453 | 0.9432 | 0.3174 | facet-SE ratio 0.964 | fail |
| 20×5×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9463 | 0.3358 |  | pass |
| 20×5×2 | high-g | normal | G | F interval | 0.9091 | 1.000 | 0.9426 (0.0053) | 0.0247 | 0.0326 | 0.9426 | 0.9363 | 0.1787 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×2 | high-g | normal | Phi | parametric | 0.8333 | 1.000 | 0.9605 (0.0045) | 0.0105 | 0.0289 | 0.9605 | 0.9568 | 0.3411 |  | fail |
| 20×5×2 | high-g | normal | Phi | parametric, attached | 0.8333 | 0.545 | 0.5326 (0.0114) | 0.0063 | 0.0058 | 0.9778 | 0.5321 | 0.3847 |  | fail |
| 20×5×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9174 (0.0063) | 0.0795 | 0.0032 | 0.9174 | 0.9184 | 2.0427 | facet-SE ratio 0.970 | fail |
| 20×5×3 | all-one | normal | p | Satterthwaite | 1.0000 | 0.988 | 0.9400 (0.0054) | 0.0000 | 0.0484 | 0.9510 | 0.9395 | 22611926474.8633 |  | fail |
| 20×5×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8916 (0.0071) | 0.1084 | 0.0000 | 0.8916 | 0.8811 | 4.0345 | facet-SE ratio 0.928 | fail |
| 20×5×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.892 | 0.8284 (0.0086) | 0.0000 | 0.0637 | 0.9286 | 0.8358 | 371473520809.0274 |  | fail |
| 20×5×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8042 (0.0091) | 0.1958 | 0.0000 | 0.8042 | 0.8158 | 4.6706 | facet-SE ratio 0.963 | fail |
| 20×5×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.812 | 0.7674 (0.0097) | 0.0000 | 0.0442 | 0.9455 | 0.7642 | 449544376738.9615 |  | fail |
| 20×5×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9368 (0.0056) | 0.0516 | 0.0116 | 0.9368 | 0.9374 | 0.8537 | facet-SE ratio 0.981 | fail |
| 20×5×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9626 (0.0044) | 0.0084 | 0.0289 | 0.9626 | 0.9595 | 0.9424 |  | fail |
| 20×5×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9300 (0.0059) | 0.0647 | 0.0053 | 0.9300 | 0.9300 | 1.0731 | facet-SE ratio 0.978 | fail |
| 20×5×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9595 (0.0045) | 0.0116 | 0.0289 | 0.9595 | 0.9595 | 1.2546 |  | fail |
| 20×5×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8874 (0.0073) | 0.1126 | 0.0000 | 0.8874 | 0.8874 | 2.0003 | facet-SE ratio 0.989 | fail |
| 20×5×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9753 (0.0036) | 0.0026 | 0.0221 | 0.9753 | 0.9753 | 3.5674 |  | fail |
| 20×5×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0374 | 0.0126 | 0.9500 | 0.9500 | 0.4464 | facet-SE ratio 1.006 | fail |
| 20×5×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0242 | 0.0263 | 0.9495 | 0.9489 | 0.4603 |  | pass |
| 20×5×3 | all-one | normal | G | F interval | 0.6250 | 1.000 | 0.9521 (0.0049) | 0.0247 | 0.0232 | 0.9521 | 0.9379 | 0.5605 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×3 | all-one | normal | Phi | parametric | 0.4545 | 1.000 | 0.9516 (0.0049) | 0.0389 | 0.0095 | 0.9516 | 0.9547 | 0.5829 |  | fail |
| 20×5×3 | all-one | normal | Phi | parametric, attached | 0.4545 | 0.723 | 0.6953 (0.0106) | 0.0237 | 0.0037 | 0.9621 | 0.7000 | 0.5837 |  | fail |
| 20×5×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9037 (0.0068) | 0.0911 | 0.0053 | 0.9037 | 0.9074 | 1.6008 | facet-SE ratio 0.958 | fail |
| 20×5×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9689 (0.0040) | 0.0000 | 0.0311 | 0.9689 | 0.9679 | 76.3941 |  | fail |
| 20×5×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8537 (0.0081) | 0.1463 | 0.0000 | 0.8537 | 0.8532 | 2.8444 | facet-SE ratio 0.963 | fail |
| 20×5×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.994 | 0.9700 (0.0039) | 0.0000 | 0.0237 | 0.9762 | 0.9700 | 2009314367.1734 |  | fail |
| 20×5×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.1133 | facet-SE ratio 0.932 | fail |
| 20×5×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.602 | 0.5058 (0.0115) | 0.0000 | 0.0963 | 0.8400 | 0.5063 | 51323275229.2566 |  | fail |
| 20×5×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0542 | 0.0074 | 0.9384 | 0.9416 | 0.8459 | facet-SE ratio 0.959 | fail |
| 20×5×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0095 | 0.0295 | 0.9611 | 0.9568 | 0.9348 |  | fail |
| 20×5×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9837 (0.0029) | 0.0000 | 0.0163 | 0.9837 | 0.9842 | 0.2219 | facet-SE ratio 0.946 | fail |
| 20×5×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.556 | 0.2789 (0.0103) | 0.0000 | 0.2768 | 0.5019 | 0.3011 | 104036580693.5766 |  | fail |
| 20×5×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9984 (0.0009) | 0.0000 | 0.0016 | 0.9984 | 0.9984 | 0.1318 | facet-SE ratio 0.917 | fail |
| 20×5×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.576 | 0.3805 (0.0111) | 0.0000 | 0.1958 | 0.6603 | 0.4084 | 60487406361.9286 |  | fail |
| 20×5×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9405 (0.0054) | 0.0516 | 0.0079 | 0.9405 | 0.9426 | 0.4450 | facet-SE ratio 0.967 | fail |
| 20×5×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0321 | 0.0200 | 0.9479 | 0.9479 | 0.4584 |  | pass |
| 20×5×3 | near-zero-rater | normal | G | F interval | 0.7874 | 1.000 | 0.9474 (0.0051) | 0.0295 | 0.0232 | 0.9474 | 0.9284 | 0.3545 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×3 | near-zero-rater | normal | Phi | parametric | 0.6784 | 1.000 | 0.9453 (0.0052) | 0.0405 | 0.0142 | 0.9453 | 0.9484 | 0.4810 |  | fail |
| 20×5×3 | near-zero-rater | normal | Phi | parametric, attached | 0.6784 | 0.161 | 0.1542 (0.0083) | 0.0058 | 0.0011 | 0.9575 | 0.1547 | 0.4744 |  | fail |
| 20×5×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9221 (0.0061) | 0.0742 | 0.0037 | 0.9221 | 0.9242 | 5.3893 | facet-SE ratio 1.000 | fail |
| 20×5×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9626 (0.0044) | 0.0147 | 0.0226 | 0.9626 | 0.9621 | 6.9089 |  | fail |
| 20×5×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8737 (0.0076) | 0.1263 | 0.0000 | 0.8737 | 0.8742 | 1.9487 | facet-SE ratio 0.928 | fail |
| 20×5×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.873 | 0.8174 (0.0089) | 0.0000 | 0.0558 | 0.9361 | 0.8174 | 171013070201.1822 |  | fail |
| 20×5×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8084 (0.0090) | 0.1916 | 0.0000 | 0.8084 | 0.7932 | 2.4760 | facet-SE ratio 0.971 | fail |
| 20×5×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.829 | 0.7816 (0.0095) | 0.0000 | 0.0479 | 0.9423 | 0.7905 | 246999656330.7861 |  | fail |
| 20×5×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9321 (0.0058) | 0.0553 | 0.0126 | 0.9321 | 0.9342 | 0.4273 | facet-SE ratio 0.933 | fail |
| 20×5×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0095 | 0.0363 | 0.9542 | 0.9532 | 0.4721 |  | fail |
| 20×5×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9400 (0.0054) | 0.0542 | 0.0058 | 0.9400 | 0.9405 | 0.5362 | facet-SE ratio 0.984 | fail |
| 20×5×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9616 (0.0044) | 0.0095 | 0.0289 | 0.9616 | 0.9621 | 0.6264 |  | fail |
| 20×5×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8842 (0.0073) | 0.1158 | 0.0000 | 0.8842 | 0.8884 | 0.9811 | facet-SE ratio 0.979 | fail |
| 20×5×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 0.999 | 0.9732 (0.0037) | 0.0021 | 0.0242 | 0.9737 | 0.9689 | 64.4656 |  | fail |
| 20×5×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0337 | 0.0142 | 0.9521 | 0.9521 | 0.2241 | facet-SE ratio 1.008 | fail |
| 20×5×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0200 | 0.0263 | 0.9537 | 0.9547 | 0.2310 |  | pass |
| 20×5×3 | high-g | normal | G | F interval | 0.9302 | 1.000 | 0.9521 (0.0049) | 0.0247 | 0.0232 | 0.9521 | 0.9363 | 0.1161 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×3 | high-g | normal | Phi | parametric | 0.8696 | 1.000 | 0.9558 (0.0047) | 0.0247 | 0.0195 | 0.9558 | 0.9516 | 0.2638 |  | pass |
| 20×5×3 | high-g | normal | Phi | parametric, attached | 0.8696 | 0.728 | 0.6974 (0.0105) | 0.0226 | 0.0079 | 0.9581 | 0.7032 | 0.2813 |  | fail |
| 20×5×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9153 (0.0064) | 0.0821 | 0.0026 | 0.9153 | 0.9184 | 1.8107 | facet-SE ratio 0.976 | fail |
| 20×5×5 | all-one | normal | p | Satterthwaite | 1.0000 | 0.997 | 0.9568 (0.0047) | 0.0000 | 0.0405 | 0.9594 | 0.9568 | 2677274626.6622 |  | fail |
| 20×5×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8395 (0.0084) | 0.1605 | 0.0000 | 0.8395 | 0.8374 | 3.4043 | facet-SE ratio 0.932 | fail |
| 20×5×5 | all-one | normal | i | Satterthwaite | 1.0000 | 0.936 | 0.8958 (0.0070) | 0.0000 | 0.0405 | 0.9567 | 0.8968 | 76858748293.2715 |  | fail |
| 20×5×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8374 (0.0085) | 0.1626 | 0.0000 | 0.8374 | 0.8379 | 3.3858 | facet-SE ratio 0.950 | fail |
| 20×5×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.925 | 0.8837 (0.0074) | 0.0000 | 0.0411 | 0.9556 | 0.8832 | 111861304403.2623 |  | fail |
| 20×5×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0426 | 0.0100 | 0.9474 | 0.9468 | 0.7579 | facet-SE ratio 0.982 | fail |
| 20×5×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0137 | 0.0263 | 0.9600 | 0.9579 | 0.8204 |  | fail |
| 20×5×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9311 (0.0058) | 0.0558 | 0.0132 | 0.9311 | 0.9311 | 0.7619 | facet-SE ratio 0.971 | fail |
| 20×5×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0205 | 0.0295 | 0.9500 | 0.9500 | 0.8239 |  | pass |
| 20×5×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9100 (0.0066) | 0.0884 | 0.0016 | 0.9100 | 0.9111 | 1.4277 | facet-SE ratio 1.011 | fail |
| 20×5×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0242 | 0.0274 | 0.9484 | 0.9479 | 1.8942 |  | pass |
| 20×5×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0316 | 0.0158 | 0.9526 | 0.9511 | 0.3170 | facet-SE ratio 1.030 | pass |
| 20×5×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0216 | 0.0211 | 0.9574 | 0.9579 | 0.3226 |  | pass |
| 20×5×5 | all-one | normal | G | F interval | 0.6944 | 1.000 | 0.9489 (0.0050) | 0.0289 | 0.0221 | 0.9489 | 0.9295 | 0.4586 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×5 | all-one | normal | Phi | parametric | 0.5319 | 1.000 | 0.9489 (0.0050) | 0.0432 | 0.0079 | 0.9489 | 0.9521 | 0.5410 |  | fail |
| 20×5×5 | all-one | normal | Phi | parametric, attached | 0.5319 | 0.863 | 0.8205 (0.0088) | 0.0374 | 0.0047 | 0.9512 | 0.8232 | 0.5423 |  | fail |
| 20×5×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9232 (0.0061) | 0.0747 | 0.0021 | 0.9232 | 0.9205 | 1.5805 | facet-SE ratio 0.972 | fail |
| 20×5×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9626 (0.0044) | 0.0000 | 0.0374 | 0.9626 | 0.9642 | 2.2614 |  | fail |
| 20×5×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8405 (0.0084) | 0.1595 | 0.0000 | 0.8405 | 0.8421 | 2.7955 | facet-SE ratio 0.959 | fail |
| 20×5×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.995 | 0.9695 (0.0039) | 0.0000 | 0.0258 | 0.9741 | 0.9684 | 6529574515.4587 |  | fail |
| 20×5×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.0712 | facet-SE ratio 0.936 | fail |
| 20×5×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.673 | 0.5832 (0.0113) | 0.0000 | 0.0895 | 0.8670 | 0.5732 | 23022747998.1983 |  | fail |
| 20×5×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0416 | 0.0111 | 0.9474 | 0.9479 | 0.7633 | facet-SE ratio 1.008 | fail |
| 20×5×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9626 (0.0044) | 0.0089 | 0.0284 | 0.9626 | 0.9637 | 0.8259 |  | fail |
| 20×5×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9905 (0.0022) | 0.0000 | 0.0095 | 0.9905 | 0.9911 | 0.1530 | facet-SE ratio 1.007 | fail |
| 20×5×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.603 | 0.3542 (0.0110) | 0.0000 | 0.2484 | 0.5878 | 0.3689 | 58309370453.7094 |  | fail |
| 20×5×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9963 (0.0014) | 0.0000 | 0.0037 | 0.9963 | 0.9963 | 0.0887 | facet-SE ratio 0.920 | fail |
| 20×5×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.638 | 0.4768 (0.0115) | 0.0000 | 0.1616 | 0.7469 | 0.4758 | 37193623864.6502 |  | fail |
| 20×5×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0289 | 0.0111 | 0.9600 | 0.9611 | 0.3155 | facet-SE ratio 1.024 | fail |
| 20×5×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0237 | 0.0184 | 0.9579 | 0.9568 | 0.3209 |  | pass |
| 20×5×5 | near-zero-rater | normal | G | F interval | 0.8052 | 1.000 | 0.9516 (0.0049) | 0.0242 | 0.0242 | 0.9516 | 0.9442 | 0.3189 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×5 | near-zero-rater | normal | Phi | parametric | 0.6923 | 1.000 | 0.9511 (0.0049) | 0.0300 | 0.0189 | 0.9511 | 0.9547 | 0.4592 |  | pass |
| 20×5×5 | near-zero-rater | normal | Phi | parametric, attached | 0.6923 | 0.228 | 0.2189 (0.0095) | 0.0074 | 0.0021 | 0.9585 | 0.2205 | 0.4592 |  | fail |
| 20×5×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9268 (0.0060) | 0.0668 | 0.0063 | 0.9268 | 0.9258 | 5.3556 | facet-SE ratio 0.998 | fail |
| 20×5×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9553 (0.0047) | 0.0200 | 0.0247 | 0.9553 | 0.9563 | 6.8029 |  | pass |
| 20×5×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8368 (0.0085) | 0.1632 | 0.0000 | 0.8368 | 0.8389 | 1.6755 | facet-SE ratio 0.951 | fail |
| 20×5×5 | high-g | normal | i | Satterthwaite | 0.5000 | 0.931 | 0.8995 (0.0069) | 0.0000 | 0.0316 | 0.9661 | 0.8968 | 55807151023.5891 |  | fail |
| 20×5×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8416 (0.0084) | 0.1584 | 0.0000 | 0.8416 | 0.8442 | 1.6651 | facet-SE ratio 0.956 | fail |
| 20×5×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.938 | 0.9058 (0.0067) | 0.0000 | 0.0321 | 0.9658 | 0.9021 | 56552216410.5048 |  | fail |
| 20×5×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0353 | 0.0111 | 0.9537 | 0.9505 | 0.3822 | facet-SE ratio 0.996 | fail |
| 20×5×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9626 (0.0044) | 0.0111 | 0.0263 | 0.9626 | 0.9626 | 0.4133 |  | fail |
| 20×5×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0421 | 0.0137 | 0.9442 | 0.9458 | 0.3793 | facet-SE ratio 0.987 | fail |
| 20×5×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9595 (0.0045) | 0.0132 | 0.0274 | 0.9595 | 0.9589 | 0.4108 |  | fail |
| 20×5×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9158 (0.0064) | 0.0816 | 0.0026 | 0.9158 | 0.9174 | 0.7106 | facet-SE ratio 1.023 | fail |
| 20×5×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9553 (0.0047) | 0.0211 | 0.0237 | 0.9553 | 0.9547 | 0.9421 |  | pass |
| 20×5×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9495 (0.0050) | 0.0347 | 0.0158 | 0.9495 | 0.9500 | 0.1579 | facet-SE ratio 0.996 | pass |
| 20×5×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0258 | 0.0226 | 0.9516 | 0.9521 | 0.1608 |  | pass |
| 20×5×5 | high-g | normal | G | F interval | 0.9479 | 1.000 | 0.9500 (0.0050) | 0.0226 | 0.0274 | 0.9500 | 0.9411 | 0.0795 | unfit 0, errors 0, failed fits 0 | pass |
| 20×5×5 | high-g | normal | Phi | parametric | 0.9009 | 1.000 | 0.9568 (0.0047) | 0.0289 | 0.0142 | 0.9568 | 0.9568 | 0.1875 |  | fail |
| 20×5×5 | high-g | normal | Phi | parametric, attached | 0.9009 | 0.874 | 0.8379 (0.0085) | 0.0284 | 0.0074 | 0.9590 | 0.8389 | 0.1914 |  | fail |
| 20×8×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9279 (0.0059) | 0.0668 | 0.0053 | 0.9279 | 0.9295 | 2.2753 | facet-SE ratio 0.971 | fail |
| 20×8×2 | all-one | normal | p | Satterthwaite | 1.0000 | 0.971 | 0.9105 (0.0065) | 0.0000 | 0.0605 | 0.9377 | 0.9089 | 94379798844.5122 |  | fail |
| 20×8×2 | all-one | normal | p | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9853 (0.0028) | 0.0000 | 0.0147 | 0.9853 | 0.9847 | 3.6261 |  | fail |
| 20×8×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9411 (0.0054) | 0.0584 | 0.0005 | 0.9411 | 0.9453 | 3.5767 | facet-SE ratio 0.958 | fail |
| 20×8×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.903 | 0.8405 (0.0084) | 0.0000 | 0.0626 | 0.9307 | 0.8389 | 512322086448.1279 |  | fail |
| 20×8×2 | all-one | normal | i | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9953 (0.0016) | 0.0042 | 0.0005 | 0.9953 | 0.9953 | 4.1486 |  | fail |
| 20×8×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7221 (0.0103) | 0.2779 | 0.0000 | 0.7221 | 0.7305 | 6.0837 | facet-SE ratio 1.157 | fail |
| 20×8×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.697 | 0.6526 (0.0109) | 0.0000 | 0.0447 | 0.9358 | 0.6516 | 795193171355.5231 |  | fail |
| 20×8×2 | all-one | normal | r | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.7163 (0.0103) | 0.2837 | 0.0000 | 0.7163 | 0.7268 | 2.7723 |  | fail |
| 20×8×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0395 | 0.0147 | 0.9458 | 0.9463 | 0.7551 | facet-SE ratio 0.989 | fail |
| 20×8×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9637 (0.0043) | 0.0021 | 0.0342 | 0.9637 | 0.9626 | 0.8183 |  | fail |
| 20×8×2 | all-one | normal | pi | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 1.8923 |  | fail |
| 20×8×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9142 (0.0064) | 0.0811 | 0.0047 | 0.9142 | 0.9137 | 1.4177 | facet-SE ratio 0.959 | fail |
| 20×8×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0089 | 0.0321 | 0.9589 | 0.9600 | 1.8629 |  | fail |
| 20×8×2 | all-one | normal | pr | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0532 | 0.0000 | 0.9468 | 0.9463 | 1.7698 |  | fail |
| 20×8×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8895 (0.0072) | 0.1105 | 0.0000 | 0.8895 | 0.8842 | 2.1945 | facet-SE ratio 0.955 | fail |
| 20×8×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 0.999 | 0.9679 (0.0040) | 0.0011 | 0.0305 | 0.9684 | 0.9711 | 4.2634 |  | fail |
| 20×8×2 | all-one | normal | ir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.7932 (0.0093) | 0.2068 | 0.0000 | 0.7932 | 0.7816 | 1.7490 |  | fail |
| 20×8×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9395 (0.0055) | 0.0432 | 0.0174 | 0.9395 | 0.9384 | 0.4793 | facet-SE ratio 0.990 | fail |
| 20×8×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0284 | 0.0300 | 0.9416 | 0.9400 | 0.4962 |  | pass |
| 20×8×2 | all-one | normal | pir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.8126 (0.0090) | 0.1874 | 0.0000 | 0.8126 | 0.8079 | 1.1307 |  | fail |
| 20×8×2 | all-one | normal | G | F interval | 0.5926 | 1.000 | 0.9384 (0.0055) | 0.0295 | 0.0321 | 0.9384 | 0.9289 | 0.6430 | unfit 0, errors 0, failed fits 0 | fail |
| 20×8×2 | all-one | normal | Phi | parametric | 0.4211 | 1.000 | 0.9663 (0.0041) | 0.0116 | 0.0221 | 0.9663 | 0.9668 | 0.6331 |  | fail |
| 20×8×2 | all-one | normal | Phi | parametric, attached | 0.4211 | 0.615 | 0.6042 (0.0112) | 0.0016 | 0.0095 | 0.9820 | 0.6037 | 0.6389 |  | fail |
| 20×8×2 | all-one | normal | G | 0.3.0 boot-p,i,r | 0.5926 | 1.000 | 0.9642 (0.0043) | 0.0000 | 0.0358 | 0.9642 | 0.9384 | 0.8996 |  | fail |
| 20×8×2 | all-one | normal | Phi | 0.3.0 boot-p,i,r | 0.4211 | 1.000 | 0.9516 (0.0049) | 0.0000 | 0.0484 | 0.9516 | 0.9489 | 0.8740 |  | fail |
| 20×8×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9268 (0.0060) | 0.0695 | 0.0037 | 0.9268 | 0.9295 | 1.5031 | facet-SE ratio 0.973 | fail |
| 20×8×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9689 (0.0040) | 0.0005 | 0.0305 | 0.9689 | 0.9684 | 2.0481 |  | fail |
| 20×8×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8647 (0.0078) | 0.1353 | 0.0000 | 0.8647 | 0.8668 | 2.1885 | facet-SE ratio 0.947 | fail |
| 20×8×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.999 | 0.9732 (0.0037) | 0.0000 | 0.0263 | 0.9737 | 0.9695 | 801134482.5845 |  | fail |
| 20×8×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.1226 | facet-SE ratio 1.076 | fail |
| 20×8×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.549 | 0.4721 (0.0115) | 0.0000 | 0.0774 | 0.8592 | 0.4742 | 65821018936.1611 |  | fail |
| 20×8×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0400 | 0.0158 | 0.9442 | 0.9442 | 0.7569 | facet-SE ratio 0.993 | fail |
| 20×8×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9600 (0.0045) | 0.0032 | 0.0368 | 0.9600 | 0.9600 | 0.8184 |  | fail |
| 20×8×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9932 (0.0019) | 0.0000 | 0.0068 | 0.9932 | 0.9932 | 0.1953 | facet-SE ratio 0.990 | fail |
| 20×8×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.547 | 0.3342 (0.0108) | 0.0000 | 0.2132 | 0.6106 | 0.3216 | 95747621773.4764 |  | fail |
| 20×8×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9995 (0.0005) | 0.0000 | 0.0005 | 0.9995 | 0.9995 | 0.1402 | facet-SE ratio 0.928 | fail |
| 20×8×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.554 | 0.3774 (0.0111) | 0.0000 | 0.1763 | 0.6816 | 0.3779 | 83925891728.7216 |  | fail |
| 20×8×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0416 | 0.0095 | 0.9489 | 0.9495 | 0.4772 | facet-SE ratio 1.023 | fail |
| 20×8×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0263 | 0.0226 | 0.9511 | 0.9511 | 0.4940 |  | pass |
| 20×8×2 | near-zero-rater | normal | G | F interval | 0.8386 | 1.000 | 0.9521 (0.0049) | 0.0232 | 0.0247 | 0.9521 | 0.9295 | 0.2700 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×2 | near-zero-rater | normal | Phi | parametric | 0.7558 | 1.000 | 0.9358 (0.0056) | 0.0568 | 0.0074 | 0.9358 | 0.9521 | 0.3896 |  | fail |
| 20×8×2 | near-zero-rater | normal | Phi | parametric, attached | 0.7558 | 0.136 | 0.1258 (0.0076) | 0.0095 | 0.0005 | 0.9264 | 0.1311 | 0.4137 |  | fail |
| 20×8×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9195 (0.0062) | 0.0795 | 0.0011 | 0.9195 | 0.9200 | 5.4434 | facet-SE ratio 0.966 | fail |
| 20×8×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9611 (0.0044) | 0.0126 | 0.0263 | 0.9611 | 0.9616 | 7.0371 |  | fail |
| 20×8×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9353 (0.0056) | 0.0637 | 0.0011 | 0.9353 | 0.9368 | 1.7804 | facet-SE ratio 0.935 | fail |
| 20×8×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.892 | 0.8289 (0.0086) | 0.0000 | 0.0632 | 0.9292 | 0.8274 | 204475426127.8296 |  | fail |
| 20×8×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7100 (0.0104) | 0.2900 | 0.0000 | 0.7100 | 0.7311 | 2.9599 | facet-SE ratio 1.166 | fail |
| 20×8×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.696 | 0.6600 (0.0109) | 0.0000 | 0.0363 | 0.9478 | 0.6537 | 322375018226.1332 |  | fail |
| 20×8×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0347 | 0.0116 | 0.9537 | 0.9532 | 0.3788 | facet-SE ratio 0.995 | fail |
| 20×8×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9637 (0.0043) | 0.0042 | 0.0321 | 0.9637 | 0.9647 | 0.4102 |  | fail |
| 20×8×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9147 (0.0064) | 0.0837 | 0.0016 | 0.9147 | 0.9147 | 0.7058 | facet-SE ratio 0.986 | fail |
| 20×8×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9637 (0.0043) | 0.0105 | 0.0258 | 0.9637 | 0.9642 | 0.9279 |  | fail |
| 20×8×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8716 (0.0077) | 0.1284 | 0.0000 | 0.8716 | 0.8716 | 1.0703 | facet-SE ratio 0.941 | fail |
| 20×8×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9668 (0.0041) | 0.0005 | 0.0326 | 0.9668 | 0.9674 | 5432.2143 |  | fail |
| 20×8×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0389 | 0.0100 | 0.9511 | 0.9511 | 0.2390 | facet-SE ratio 1.012 | fail |
| 20×8×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0226 | 0.0211 | 0.9563 | 0.9563 | 0.2472 |  | pass |
| 20×8×2 | high-g | normal | G | F interval | 0.9209 | 1.000 | 0.9458 (0.0052) | 0.0268 | 0.0274 | 0.9458 | 0.9332 | 0.1643 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×2 | high-g | normal | Phi | parametric | 0.8533 | 1.000 | 0.9579 (0.0046) | 0.0095 | 0.0326 | 0.9579 | 0.9447 | 0.3136 |  | fail |
| 20×8×2 | high-g | normal | Phi | parametric, attached | 0.8533 | 0.629 | 0.6153 (0.0112) | 0.0074 | 0.0068 | 0.9774 | 0.6132 | 0.3572 |  | fail |
| 20×8×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9274 (0.0060) | 0.0689 | 0.0037 | 0.9274 | 0.9237 | 1.9273 | facet-SE ratio 0.986 | fail |
| 20×8×3 | all-one | normal | p | Satterthwaite | 1.0000 | 0.994 | 0.9495 (0.0050) | 0.0000 | 0.0447 | 0.9550 | 0.9516 | 19196187950.9328 |  | fail |
| 20×8×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8858 (0.0073) | 0.1137 | 0.0005 | 0.8858 | 0.8874 | 2.9182 | facet-SE ratio 0.960 | fail |
| 20×8×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.951 | 0.9058 (0.0067) | 0.0000 | 0.0453 | 0.9524 | 0.9042 | 131783845608.3324 |  | fail |
| 20×8×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7663 (0.0097) | 0.2337 | 0.0000 | 0.7663 | 0.7653 | 4.3886 | facet-SE ratio 0.972 | fail |
| 20×8×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.858 | 0.8221 (0.0088) | 0.0000 | 0.0358 | 0.9583 | 0.8226 | 192707864552.8138 |  | fail |
| 20×8×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0400 | 0.0121 | 0.9479 | 0.9489 | 0.6458 | facet-SE ratio 0.978 | fail |
| 20×8×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0158 | 0.0353 | 0.9489 | 0.9484 | 0.6840 |  | fail |
| 20×8×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9316 (0.0058) | 0.0600 | 0.0084 | 0.9316 | 0.9316 | 1.0040 | facet-SE ratio 0.956 | fail |
| 20×8×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0195 | 0.0337 | 0.9468 | 0.9468 | 1.1489 |  | pass |
| 20×8×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9126 (0.0065) | 0.0863 | 0.0011 | 0.9126 | 0.9126 | 1.5243 | facet-SE ratio 0.975 | fail |
| 20×8×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0158 | 0.0258 | 0.9584 | 0.9574 | 2.1058 |  | pass |
| 20×8×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9374 (0.0056) | 0.0442 | 0.0184 | 0.9374 | 0.9379 | 0.3380 | facet-SE ratio 0.974 | fail |
| 20×8×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0274 | 0.0274 | 0.9453 | 0.9447 | 0.3445 |  | pass |
| 20×8×3 | all-one | normal | G | F interval | 0.6667 | 1.000 | 0.9474 (0.0051) | 0.0253 | 0.0274 | 0.9474 | 0.9379 | 0.5189 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×3 | all-one | normal | Phi | parametric | 0.5000 | 1.000 | 0.9595 (0.0045) | 0.0311 | 0.0095 | 0.9595 | 0.9568 | 0.5799 |  | fail |
| 20×8×3 | all-one | normal | Phi | parametric, attached | 0.5000 | 0.814 | 0.7847 (0.0094) | 0.0242 | 0.0047 | 0.9644 | 0.7842 | 0.5802 |  | fail |
| 20×8×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9158 (0.0064) | 0.0821 | 0.0021 | 0.9158 | 0.9163 | 1.4719 | facet-SE ratio 0.971 | fail |
| 20×8×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9621 (0.0044) | 0.0021 | 0.0358 | 0.9621 | 0.9621 | 1.9806 |  | fail |
| 20×8×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8874 (0.0073) | 0.1121 | 0.0005 | 0.8874 | 0.8863 | 2.2008 | facet-SE ratio 0.937 | fail |
| 20×8×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9711 (0.0038) | 0.0000 | 0.0289 | 0.9711 | 0.9721 | 180955245.9144 |  | fail |
| 20×8×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.0779 | facet-SE ratio 0.944 | fail |
| 20×8×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.644 | 0.5689 (0.0114) | 0.0000 | 0.0747 | 0.8839 | 0.5637 | 27967909162.5078 |  | fail |
| 20×8×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0384 | 0.0163 | 0.9453 | 0.9426 | 0.6505 | facet-SE ratio 0.984 | fail |
| 20×8×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0105 | 0.0311 | 0.9584 | 0.9647 | 0.6900 |  | fail |
| 20×8×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9926 (0.0020) | 0.0000 | 0.0074 | 0.9926 | 0.9921 | 0.1347 | facet-SE ratio 1.000 | fail |
| 20×8×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.591 | 0.3795 (0.0111) | 0.0000 | 0.2116 | 0.6420 | 0.3732 | 67434977768.5396 |  | fail |
| 20×8×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9984 (0.0009) | 0.0000 | 0.0016 | 0.9984 | 0.9984 | 0.0970 | facet-SE ratio 0.947 | fail |
| 20×8×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.648 | 0.4684 (0.0114) | 0.0000 | 0.1795 | 0.7230 | 0.4826 | 40635715744.0391 |  | fail |
| 20×8×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0342 | 0.0189 | 0.9468 | 0.9474 | 0.3386 | facet-SE ratio 1.009 | pass |
| 20×8×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0216 | 0.0279 | 0.9505 | 0.9511 | 0.3452 |  | pass |
| 20×8×3 | near-zero-rater | normal | G | F interval | 0.8547 | 1.000 | 0.9521 (0.0049) | 0.0253 | 0.0226 | 0.9521 | 0.9321 | 0.2308 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×3 | near-zero-rater | normal | Phi | parametric | 0.7700 | 1.000 | 0.9411 (0.0054) | 0.0516 | 0.0074 | 0.9411 | 0.9547 | 0.3638 |  | fail |
| 20×8×3 | near-zero-rater | normal | Phi | parametric, attached | 0.7700 | 0.234 | 0.2179 (0.0095) | 0.0147 | 0.0016 | 0.9303 | 0.2247 | 0.3686 |  | fail |
| 20×8×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9232 (0.0061) | 0.0747 | 0.0021 | 0.9232 | 0.9205 | 5.3975 | facet-SE ratio 0.976 | fail |
| 20×8×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9553 (0.0047) | 0.0153 | 0.0295 | 0.9553 | 0.9584 | 6.8612 |  | pass |
| 20×8×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9058 (0.0067) | 0.0937 | 0.0005 | 0.9058 | 0.9074 | 1.4653 | facet-SE ratio 0.973 | fail |
| 20×8×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.954 | 0.9063 (0.0067) | 0.0000 | 0.0479 | 0.9498 | 0.9047 | 61716903943.2682 |  | fail |
| 20×8×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7605 (0.0098) | 0.2395 | 0.0000 | 0.7605 | 0.7516 | 2.2612 | facet-SE ratio 0.984 | fail |
| 20×8×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.868 | 0.8237 (0.0087) | 0.0000 | 0.0447 | 0.9485 | 0.8284 | 96858676692.6297 |  | fail |
| 20×8×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9389 (0.0055) | 0.0447 | 0.0163 | 0.9389 | 0.9411 | 0.3230 | facet-SE ratio 0.995 | fail |
| 20×8×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9579 (0.0046) | 0.0142 | 0.0279 | 0.9579 | 0.9568 | 0.3423 |  | fail |
| 20×8×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9311 (0.0058) | 0.0605 | 0.0084 | 0.9311 | 0.9347 | 0.4988 | facet-SE ratio 0.970 | fail |
| 20×8×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9537 (0.0048) | 0.0153 | 0.0311 | 0.9537 | 0.9542 | 0.5718 |  | pass |
| 20×8×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9179 (0.0063) | 0.0795 | 0.0026 | 0.9179 | 0.9163 | 0.7664 | facet-SE ratio 0.937 | fail |
| 20×8×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0189 | 0.0326 | 0.9484 | 0.9479 | 1.0579 |  | pass |
| 20×8×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9405 (0.0054) | 0.0395 | 0.0200 | 0.9405 | 0.9416 | 0.1692 | facet-SE ratio 0.948 | fail |
| 20×8×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9432 (0.0053) | 0.0268 | 0.0300 | 0.9432 | 0.9432 | 0.1725 |  | pass |
| 20×8×3 | high-g | normal | G | F interval | 0.9412 | 1.000 | 0.9442 (0.0053) | 0.0232 | 0.0326 | 0.9442 | 0.9189 | 0.0982 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×3 | high-g | normal | Phi | parametric | 0.8889 | 1.000 | 0.9632 (0.0043) | 0.0147 | 0.0221 | 0.9632 | 0.9563 | 0.2301 |  | fail |
| 20×8×3 | high-g | normal | Phi | parametric, attached | 0.8889 | 0.832 | 0.8084 (0.0090) | 0.0142 | 0.0089 | 0.9722 | 0.8063 | 0.2434 |  | fail |
| 20×8×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9279 (0.0059) | 0.0695 | 0.0026 | 0.9279 | 0.9295 | 1.6931 | facet-SE ratio 0.991 | fail |
| 20×8×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9653 (0.0042) | 0.0000 | 0.0347 | 0.9653 | 0.9626 | 3529470487.9618 |  | fail |
| 20×8×5 | all-one | normal | p | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9905 (0.0022) | 0.0011 | 0.0084 | 0.9905 | 0.9905 | 2.3109 |  | fail |
| 20×8×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8726 (0.0076) | 0.1268 | 0.0005 | 0.8726 | 0.8732 | 2.5566 | facet-SE ratio 0.937 | fail |
| 20×8×5 | all-one | normal | i | Satterthwaite | 1.0000 | 0.983 | 0.9453 (0.0052) | 0.0000 | 0.0379 | 0.9615 | 0.9416 | 27164526125.1487 |  | fail |
| 20×8×5 | all-one | normal | i | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9811 (0.0031) | 0.0189 | 0.0000 | 0.9811 | 0.9821 | 2.7309 |  | fail |
| 20×8×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8447 (0.0083) | 0.1553 | 0.0000 | 0.8447 | 0.8374 | 3.2141 | facet-SE ratio 0.933 | fail |
| 20×8×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.961 | 0.9232 (0.0061) | 0.0000 | 0.0374 | 0.9611 | 0.9274 | 36104530670.2045 |  | fail |
| 20×8×5 | all-one | normal | r | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9111 (0.0065) | 0.0889 | 0.0000 | 0.9111 | 0.9063 | 2.7058 |  | fail |
| 20×8×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0363 | 0.0163 | 0.9474 | 0.9453 | 0.5765 | facet-SE ratio 0.998 | fail |
| 20×8×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0142 | 0.0311 | 0.9547 | 0.9558 | 0.6044 |  | fail |
| 20×8×5 | all-one | normal | pi | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 1.0526 |  | fail |
| 20×8×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9400 (0.0054) | 0.0511 | 0.0089 | 0.9400 | 0.9400 | 0.7118 | facet-SE ratio 0.969 | fail |
| 20×8×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0232 | 0.0289 | 0.9479 | 0.9489 | 0.7633 |  | pass |
| 20×8×5 | all-one | normal | pr | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.9816 (0.0031) | 0.0184 | 0.0000 | 0.9816 | 0.9816 | 1.1294 |  | fail |
| 20×8×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9147 (0.0064) | 0.0763 | 0.0089 | 0.9147 | 0.9158 | 1.0861 | facet-SE ratio 0.937 | fail |
| 20×8×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0221 | 0.0342 | 0.9437 | 0.9432 | 1.2742 |  | pass |
| 20×8×5 | all-one | normal | ir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.8774 (0.0075) | 0.1226 | 0.0000 | 0.8774 | 0.8779 | 1.3066 |  | fail |
| 20×8×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9453 (0.0052) | 0.0379 | 0.0168 | 0.9453 | 0.9458 | 0.2390 | facet-SE ratio 0.994 | fail |
| 20×8×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0268 | 0.0242 | 0.9489 | 0.9505 | 0.2417 |  | pass |
| 20×8×5 | all-one | normal | pir | 0.3.0 boot-p,i,r | 1.0000 | 1.000 | 0.1342 (0.0078) | 0.8658 | 0.0000 | 0.1342 | 0.1400 | 0.5982 |  | fail |
| 20×8×5 | all-one | normal | G | F interval | 0.7407 | 1.000 | 0.9484 (0.0051) | 0.0253 | 0.0263 | 0.9484 | 0.9358 | 0.3926 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×5 | all-one | normal | Phi | parametric | 0.5882 | 1.000 | 0.9432 (0.0053) | 0.0447 | 0.0121 | 0.9432 | 0.9489 | 0.5089 |  | fail |
| 20×8×5 | all-one | normal | Phi | parametric, attached | 0.5882 | 0.944 | 0.8916 (0.0071) | 0.0437 | 0.0089 | 0.9443 | 0.8984 | 0.5093 |  | fail |
| 20×8×5 | all-one | normal | G | 0.3.0 boot-p,i,r | 0.7407 | 1.000 | 0.9837 (0.0029) | 0.0000 | 0.0163 | 0.9837 | 0.9668 | 0.4576 |  | fail |
| 20×8×5 | all-one | normal | Phi | 0.3.0 boot-p,i,r | 0.5882 | 1.000 | 0.9811 (0.0031) | 0.0000 | 0.0189 | 0.9811 | 0.9695 | 0.5392 |  | fail |
| 20×8×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9337 (0.0057) | 0.0637 | 0.0026 | 0.9337 | 0.9300 | 1.4567 | facet-SE ratio 1.001 | fail |
| 20×8×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9668 (0.0041) | 0.0037 | 0.0295 | 0.9668 | 0.9689 | 2.5588 |  | fail |
| 20×8×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8674 (0.0078) | 0.1326 | 0.0000 | 0.8674 | 0.8695 | 2.1478 | facet-SE ratio 0.956 | fail |
| 20×8×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.999 | 0.9726 (0.0037) | 0.0000 | 0.0263 | 0.9737 | 0.9726 | 625478524.2089 |  | fail |
| 20×8×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.9995 (0.0005) | 0.0005 | 0.0000 | 0.9995 | 1.0000 | 0.0525 | facet-SE ratio 0.928 | fail |
| 20×8×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.771 | 0.6942 (0.0106) | 0.0000 | 0.0768 | 0.9003 | 0.6905 | 11778031660.1686 |  | fail |
| 20×8×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0363 | 0.0163 | 0.9474 | 0.9479 | 0.5757 | facet-SE ratio 0.999 | fail |
| 20×8×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0179 | 0.0289 | 0.9532 | 0.9537 | 0.6032 |  | pass |
| 20×8×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9879 (0.0025) | 0.0000 | 0.0121 | 0.9879 | 0.9879 | 0.0929 | facet-SE ratio 0.999 | fail |
| 20×8×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.661 | 0.4537 (0.0114) | 0.0000 | 0.2074 | 0.6863 | 0.4489 | 43277339004.5354 |  | fail |
| 20×8×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9926 (0.0020) | 0.0000 | 0.0074 | 0.9926 | 0.9926 | 0.0658 | facet-SE ratio 0.916 | fail |
| 20×8×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.695 | 0.5416 (0.0114) | 0.0000 | 0.1532 | 0.7795 | 0.5326 | 25047974534.2905 |  | fail |
| 20×8×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0337 | 0.0247 | 0.9416 | 0.9421 | 0.2385 | facet-SE ratio 1.016 | pass |
| 20×8×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0232 | 0.0305 | 0.9463 | 0.9474 | 0.2416 |  | pass |
| 20×8×5 | near-zero-rater | normal | G | F interval | 0.8681 | 1.000 | 0.9547 (0.0048) | 0.0221 | 0.0232 | 0.9547 | 0.9374 | 0.2016 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×5 | near-zero-rater | normal | Phi | parametric | 0.7817 | 1.000 | 0.9526 (0.0049) | 0.0368 | 0.0105 | 0.9526 | 0.9547 | 0.3407 |  | fail |
| 20×8×5 | near-zero-rater | normal | Phi | parametric, attached | 0.7817 | 0.333 | 0.3174 (0.0107) | 0.0137 | 0.0016 | 0.9541 | 0.3189 | 0.3413 |  | fail |
| 20×8×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9247 (0.0061) | 0.0732 | 0.0021 | 0.9247 | 0.9263 | 5.1991 | facet-SE ratio 0.984 | fail |
| 20×8×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9584 (0.0046) | 0.0179 | 0.0237 | 0.9584 | 0.9579 | 6.5766 |  | pass |
| 20×8×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8763 (0.0076) | 0.1237 | 0.0000 | 0.8763 | 0.8811 | 1.2800 | facet-SE ratio 0.951 | fail |
| 20×8×5 | high-g | normal | i | Satterthwaite | 0.5000 | 0.977 | 0.9326 (0.0058) | 0.0000 | 0.0442 | 0.9547 | 0.9300 | 22195738967.8656 |  | fail |
| 20×8×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8568 (0.0080) | 0.1432 | 0.0000 | 0.8568 | 0.8526 | 1.5992 | facet-SE ratio 0.967 | fail |
| 20×8×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.969 | 0.9321 (0.0058) | 0.0000 | 0.0368 | 0.9620 | 0.9353 | 24790113598.9933 |  | fail |
| 20×8×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9437 (0.0053) | 0.0416 | 0.0147 | 0.9437 | 0.9437 | 0.2875 | facet-SE ratio 0.973 | fail |
| 20×8×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0221 | 0.0289 | 0.9489 | 0.9489 | 0.3014 |  | pass |
| 20×8×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0421 | 0.0100 | 0.9479 | 0.9479 | 0.3542 | facet-SE ratio 1.020 | fail |
| 20×8×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9626 (0.0044) | 0.0121 | 0.0253 | 0.9626 | 0.9626 | 0.3800 |  | fail |
| 20×8×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9337 (0.0057) | 0.0647 | 0.0016 | 0.9337 | 0.9289 | 0.5483 | facet-SE ratio 0.991 | fail |
| 20×8×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9589 (0.0046) | 0.0174 | 0.0237 | 0.9589 | 0.9600 | 0.6440 |  | pass |
| 20×8×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9426 (0.0053) | 0.0368 | 0.0205 | 0.9426 | 0.9421 | 0.1195 | facet-SE ratio 1.012 | fail |
| 20×8×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9479 (0.0051) | 0.0237 | 0.0284 | 0.9479 | 0.9500 | 0.1208 |  | pass |
| 20×8×5 | high-g | normal | G | F interval | 0.9581 | 1.000 | 0.9542 (0.0048) | 0.0242 | 0.0216 | 0.9542 | 0.9337 | 0.0640 | unfit 0, errors 0, failed fits 0 | pass |
| 20×8×5 | high-g | normal | Phi | parametric | 0.9195 | 1.000 | 0.9526 (0.0049) | 0.0332 | 0.0142 | 0.9526 | 0.9616 | 0.1561 |  | fail |
| 20×8×5 | high-g | normal | Phi | parametric, attached | 0.9195 | 0.947 | 0.9047 (0.0067) | 0.0321 | 0.0100 | 0.9555 | 0.9142 | 0.1579 |  | fail |
| 20×20×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9274 (0.0060) | 0.0674 | 0.0053 | 0.9274 | 0.9253 | 2.1465 | facet-SE ratio 0.958 | fail |
| 20×20×2 | all-one | normal | p | Satterthwaite | 1.0000 | 0.987 | 0.9253 (0.0060) | 0.0000 | 0.0616 | 0.9376 | 0.9326 | 62345846768.1039 |  | fail |
| 20×20×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9432 (0.0053) | 0.0505 | 0.0063 | 0.9432 | 0.9421 | 2.1229 | facet-SE ratio 1.031 | fail |
| 20×20×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.989 | 0.9432 (0.0053) | 0.0000 | 0.0463 | 0.9532 | 0.9458 | 46398290306.6124 |  | fail |
| 20×20×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.6521 (0.0109) | 0.3479 | 0.0000 | 0.6521 | 0.6521 | 5.6091 | facet-SE ratio 1.170 | fail |
| 20×20×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.759 | 0.7205 (0.0103) | 0.0000 | 0.0384 | 0.9494 | 0.7205 | 273203165566.5549 |  | fail |
| 20×20×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0311 | 0.0105 | 0.9584 | 0.9595 | 0.4579 | facet-SE ratio 1.035 | fail |
| 20×20×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9679 (0.0040) | 0.0132 | 0.0189 | 0.9679 | 0.9663 | 0.4719 |  | fail |
| 20×20×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9274 (0.0060) | 0.0705 | 0.0021 | 0.9274 | 0.9242 | 1.3219 | facet-SE ratio 0.991 | fail |
| 20×20×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0179 | 0.0268 | 0.9553 | 0.9547 | 1.6744 |  | pass |
| 20×20×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9211 (0.0062) | 0.0779 | 0.0011 | 0.9211 | 0.9216 | 1.3158 | facet-SE ratio 0.995 | fail |
| 20×20×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0179 | 0.0268 | 0.9553 | 0.9558 | 1.6664 |  | pass |
| 20×20×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9363 (0.0056) | 0.0447 | 0.0189 | 0.9363 | 0.9400 | 0.2888 | facet-SE ratio 0.982 | fail |
| 20×20×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9411 (0.0054) | 0.0326 | 0.0263 | 0.9411 | 0.9421 | 0.2934 |  | pass |
| 20×20×2 | all-one | normal | G | F interval | 0.6349 | 1.000 | 0.9489 (0.0050) | 0.0211 | 0.0300 | 0.9489 | 0.9411 | 0.6402 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×2 | all-one | normal | Phi | parametric | 0.4651 | 1.000 | 0.9558 (0.0047) | 0.0032 | 0.0411 | 0.9558 | 0.9574 | 0.6542 |  | fail |
| 20×20×2 | all-one | normal | Phi | parametric, attached | 0.4651 | 0.739 | 0.7184 (0.0103) | 0.0000 | 0.0205 | 0.9722 | 0.7211 | 0.6617 |  | fail |
| 20×20×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9168 (0.0063) | 0.0795 | 0.0037 | 0.9168 | 0.9211 | 1.3498 | facet-SE ratio 0.975 | fail |
| 20×20×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0111 | 0.0279 | 0.9611 | 0.9611 | 1.7326 |  | fail |
| 20×20×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9121 (0.0065) | 0.0868 | 0.0011 | 0.9121 | 0.9132 | 1.3484 | facet-SE ratio 0.970 | fail |
| 20×20×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9663 (0.0041) | 0.0116 | 0.0221 | 0.9663 | 0.9642 | 1.7315 |  | fail |
| 20×20×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.9789 (0.0033) | 0.0211 | 0.0000 | 0.9789 | 0.9800 | 0.0759 | facet-SE ratio 1.127 | fail |
| 20×20×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.616 | 0.5584 (0.0114) | 0.0000 | 0.0579 | 0.9061 | 0.5574 | 19472383885.8480 |  | fail |
| 20×20×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0300 | 0.0189 | 0.9511 | 0.9505 | 0.4596 | facet-SE ratio 1.000 | pass |
| 20×20×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0111 | 0.0342 | 0.9547 | 0.9547 | 0.4744 |  | fail |
| 20×20×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9968 (0.0013) | 0.0000 | 0.0032 | 0.9968 | 0.9968 | 0.0817 | facet-SE ratio 0.986 | fail |
| 20×20×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.673 | 0.5053 (0.0115) | 0.0000 | 0.1674 | 0.7512 | 0.5147 | 28597072549.2785 |  | fail |
| 20×20×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9974 (0.0012) | 0.0000 | 0.0026 | 0.9974 | 0.9974 | 0.0812 | facet-SE ratio 0.993 | fail |
| 20×20×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.673 | 0.5126 (0.0115) | 0.0000 | 0.1605 | 0.7615 | 0.5158 | 27725953083.1712 |  | fail |
| 20×20×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0289 | 0.0184 | 0.9526 | 0.9526 | 0.2909 | facet-SE ratio 1.006 | pass |
| 20×20×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9526 | 0.2955 |  | pass |
| 20×20×2 | near-zero-rater | normal | G | F interval | 0.9259 | 1.000 | 0.9495 (0.0050) | 0.0253 | 0.0253 | 0.9495 | 0.9279 | 0.1191 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×2 | near-zero-rater | normal | Phi | parametric | 0.8809 | 1.000 | 0.9342 (0.0057) | 0.0626 | 0.0032 | 0.9342 | 0.9463 | 0.2072 |  | fail |
| 20×20×2 | near-zero-rater | normal | Phi | parametric, attached | 0.8809 | 0.262 | 0.2426 (0.0098) | 0.0184 | 0.0005 | 0.9276 | 0.2516 | 0.2155 |  | fail |
| 20×20×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9274 (0.0060) | 0.0695 | 0.0032 | 0.9274 | 0.9279 | 5.4059 | facet-SE ratio 0.995 | fail |
| 20×20×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9563 (0.0047) | 0.0153 | 0.0284 | 0.9563 | 0.9563 | 6.9321 |  | pass |
| 20×20×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9337 (0.0057) | 0.0616 | 0.0047 | 0.9337 | 0.9332 | 1.0631 | facet-SE ratio 0.994 | fail |
| 20×20×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.991 | 0.9426 (0.0053) | 0.0000 | 0.0484 | 0.9511 | 0.9432 | 21423707693.7260 |  | fail |
| 20×20×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.6574 (0.0109) | 0.3426 | 0.0000 | 0.6574 | 0.6626 | 2.7422 | facet-SE ratio 1.205 | fail |
| 20×20×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.758 | 0.7247 (0.0102) | 0.0000 | 0.0332 | 0.9563 | 0.7211 | 145371757659.0547 |  | fail |
| 20×20×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9463 (0.0052) | 0.0379 | 0.0158 | 0.9463 | 0.9463 | 0.2294 | facet-SE ratio 0.972 | fail |
| 20×20×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0174 | 0.0326 | 0.9500 | 0.9479 | 0.2368 |  | pass |
| 20×20×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9163 (0.0064) | 0.0795 | 0.0042 | 0.9163 | 0.9126 | 0.6653 | facet-SE ratio 0.961 | fail |
| 20×20×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0158 | 0.0326 | 0.9516 | 0.9505 | 0.8420 |  | pass |
| 20×20×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9153 (0.0064) | 0.0826 | 0.0021 | 0.9153 | 0.9142 | 0.6609 | facet-SE ratio 0.958 | fail |
| 20×20×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0221 | 0.0326 | 0.9453 | 0.9458 | 0.8367 |  | pass |
| 20×20×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0374 | 0.0142 | 0.9484 | 0.9489 | 0.1451 | facet-SE ratio 1.008 | fail |
| 20×20×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9489 | 0.1474 |  | pass |
| 20×20×2 | high-g | normal | G | F interval | 0.9329 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9411 | 0.1489 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×2 | high-g | normal | Phi | parametric | 0.8743 | 1.000 | 0.9316 (0.0058) | 0.0058 | 0.0626 | 0.9316 | 0.9200 | 0.2903 |  | fail |
| 20×20×2 | high-g | normal | Phi | parametric, attached | 0.8743 | 0.751 | 0.7216 (0.0103) | 0.0047 | 0.0247 | 0.9608 | 0.7158 | 0.3286 |  | fail |
| 20×20×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9195 (0.0062) | 0.0753 | 0.0053 | 0.9195 | 0.9211 | 1.7885 | facet-SE ratio 0.958 | fail |
| 20×20×3 | all-one | normal | p | Satterthwaite | 1.0000 | 0.998 | 0.9616 (0.0044) | 0.0000 | 0.0363 | 0.9636 | 0.9595 | 5096601556.8644 |  | fail |
| 20×20×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9242 (0.0061) | 0.0732 | 0.0026 | 0.9242 | 0.9268 | 1.7788 | facet-SE ratio 0.992 | fail |
| 20×20×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.999 | 0.9632 (0.0043) | 0.0000 | 0.0358 | 0.9642 | 0.9605 | 2964091031.3842 |  | fail |
| 20×20×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7547 (0.0099) | 0.2453 | 0.0000 | 0.7547 | 0.7542 | 4.0668 | facet-SE ratio 0.989 | fail |
| 20×20×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.903 | 0.8689 (0.0077) | 0.0000 | 0.0337 | 0.9627 | 0.8695 | 71850976024.6107 |  | fail |
| 20×20×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0368 | 0.0195 | 0.9437 | 0.9453 | 0.3925 | facet-SE ratio 1.000 | fail |
| 20×20×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0163 | 0.0316 | 0.9521 | 0.9532 | 0.4021 |  | pass |
| 20×20×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9184 (0.0063) | 0.0700 | 0.0116 | 0.9184 | 0.9195 | 0.9379 | facet-SE ratio 0.945 | fail |
| 20×20×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0258 | 0.0321 | 0.9421 | 0.9421 | 1.0565 |  | pass |
| 20×20×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9305 (0.0058) | 0.0642 | 0.0053 | 0.9305 | 0.9311 | 0.9338 | facet-SE ratio 0.959 | fail |
| 20×20×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0263 | 0.0226 | 0.9511 | 0.9505 | 1.0508 |  | pass |
| 20×20×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0326 | 0.0237 | 0.9437 | 0.9447 | 0.2056 | facet-SE ratio 1.006 | pass |
| 20×20×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0247 | 0.0289 | 0.9463 | 0.9468 | 0.2076 |  | pass |
| 20×20×3 | all-one | normal | G | F interval | 0.7143 | 1.000 | 0.9495 (0.0050) | 0.0247 | 0.0258 | 0.9495 | 0.9342 | 0.4831 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×3 | all-one | normal | Phi | parametric | 0.5556 | 1.000 | 0.9516 (0.0049) | 0.0221 | 0.0263 | 0.9516 | 0.9495 | 0.5735 |  | pass |
| 20×20×3 | all-one | normal | Phi | parametric, attached | 0.5556 | 0.899 | 0.8637 (0.0079) | 0.0195 | 0.0163 | 0.9602 | 0.8626 | 0.5780 |  | fail |
| 20×20×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9289 (0.0059) | 0.0663 | 0.0047 | 0.9289 | 0.9253 | 1.3666 | facet-SE ratio 0.984 | fail |
| 20×20×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0137 | 0.0311 | 0.9553 | 0.9584 | 1.7430 |  | fail |
| 20×20×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9132 (0.0065) | 0.0832 | 0.0037 | 0.9132 | 0.9142 | 1.3401 | facet-SE ratio 0.970 | fail |
| 20×20×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9584 (0.0046) | 0.0163 | 0.0253 | 0.9584 | 0.9579 | 1.7135 |  | pass |
| 20×20×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8711 (0.0077) | 0.1289 | 0.0000 | 0.8711 | 0.8947 | 0.0501 | facet-SE ratio 0.963 | fail |
| 20×20×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.767 | 0.7258 (0.0102) | 0.0000 | 0.0416 | 0.9458 | 0.7189 | 6706873396.6576 |  | fail |
| 20×20×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9537 (0.0048) | 0.0347 | 0.0116 | 0.9537 | 0.9542 | 0.3924 | facet-SE ratio 1.008 | fail |
| 20×20×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9611 (0.0044) | 0.0195 | 0.0195 | 0.9611 | 0.9616 | 0.4023 |  | fail |
| 20×20×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9953 (0.0016) | 0.0000 | 0.0047 | 0.9953 | 0.9953 | 0.0559 | facet-SE ratio 0.979 | fail |
| 20×20×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.749 | 0.6042 (0.0112) | 0.0000 | 0.1453 | 0.8062 | 0.6021 | 18797340443.0922 |  | fail |
| 20×20×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9942 (0.0017) | 0.0000 | 0.0058 | 0.9942 | 0.9942 | 0.0560 | facet-SE ratio 0.999 | fail |
| 20×20×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.752 | 0.6116 (0.0112) | 0.0000 | 0.1400 | 0.8137 | 0.6121 | 17970014719.9415 |  | fail |
| 20×20×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0321 | 0.0174 | 0.9505 | 0.9500 | 0.2056 | facet-SE ratio 1.027 | pass |
| 20×20×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0237 | 0.0232 | 0.9532 | 0.9526 | 0.2075 |  | pass |
| 20×20×3 | near-zero-rater | normal | G | F interval | 0.9346 | 1.000 | 0.9537 (0.0048) | 0.0195 | 0.0268 | 0.9537 | 0.9379 | 0.0954 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×3 | near-zero-rater | normal | Phi | parametric | 0.8901 | 1.000 | 0.9405 (0.0054) | 0.0547 | 0.0047 | 0.9405 | 0.9489 | 0.1809 |  | fail |
| 20×20×3 | near-zero-rater | normal | Phi | parametric, attached | 0.8901 | 0.422 | 0.3921 (0.0112) | 0.0274 | 0.0026 | 0.9289 | 0.3974 | 0.1844 |  | fail |
| 20×20×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9200 (0.0062) | 0.0784 | 0.0016 | 0.9200 | 0.9200 | 5.2780 | facet-SE ratio 0.986 | fail |
| 20×20×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9500 (0.0050) | 0.0221 | 0.0279 | 0.9500 | 0.9505 | 6.6871 |  | pass |
| 20×20×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9263 (0.0060) | 0.0716 | 0.0021 | 0.9263 | 0.9253 | 0.8962 | facet-SE ratio 0.974 | fail |
| 20×20×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.998 | 0.9547 (0.0048) | 0.0000 | 0.0432 | 0.9568 | 0.9558 | 2268296924.8135 |  | fail |
| 20×20×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7800 (0.0095) | 0.2200 | 0.0000 | 0.7800 | 0.7779 | 2.0347 | facet-SE ratio 1.000 | fail |
| 20×20×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.922 | 0.8879 (0.0072) | 0.0000 | 0.0342 | 0.9629 | 0.8900 | 27003384096.1453 |  | fail |
| 20×20×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0405 | 0.0153 | 0.9442 | 0.9442 | 0.1959 | facet-SE ratio 0.975 | fail |
| 20×20×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0258 | 0.0274 | 0.9468 | 0.9463 | 0.2010 |  | pass |
| 20×20×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9400 (0.0054) | 0.0516 | 0.0084 | 0.9400 | 0.9384 | 0.4704 | facet-SE ratio 0.966 | fail |
| 20×20×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9468 (0.0051) | 0.0184 | 0.0347 | 0.9468 | 0.9489 | 0.5295 |  | pass |
| 20×20×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9421 (0.0054) | 0.0537 | 0.0042 | 0.9421 | 0.9426 | 0.4645 | facet-SE ratio 0.989 | fail |
| 20×20×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0200 | 0.0258 | 0.9542 | 0.9521 | 0.5227 |  | pass |
| 20×20×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9489 (0.0050) | 0.0284 | 0.0226 | 0.9489 | 0.9489 | 0.1024 | facet-SE ratio 1.043 | pass |
| 20×20×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9568 (0.0047) | 0.0189 | 0.0242 | 0.9568 | 0.9542 | 0.1035 |  | pass |
| 20×20×3 | high-g | normal | G | F interval | 0.9524 | 1.000 | 0.9453 (0.0052) | 0.0311 | 0.0237 | 0.9453 | 0.9337 | 0.0844 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×3 | high-g | normal | Phi | parametric | 0.9091 | 1.000 | 0.9589 (0.0046) | 0.0116 | 0.0295 | 0.9589 | 0.9447 | 0.2021 |  | fail |
| 20×20×3 | high-g | normal | Phi | parametric, attached | 0.9091 | 0.920 | 0.8895 (0.0072) | 0.0105 | 0.0200 | 0.9668 | 0.8832 | 0.2097 |  | fail |
| 20×20×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9163 (0.0064) | 0.0795 | 0.0042 | 0.9163 | 0.9179 | 1.5805 | facet-SE ratio 0.973 | fail |
| 20×20×5 | all-one | normal | p | Satterthwaite | 1.0000 | 0.999 | 0.9668 (0.0041) | 0.0000 | 0.0326 | 0.9674 | 0.9637 | 2.2967 |  | fail |
| 20×20×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9289 (0.0059) | 0.0700 | 0.0011 | 0.9289 | 0.9289 | 1.5886 | facet-SE ratio 1.014 | fail |
| 20×20×5 | all-one | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9753 (0.0036) | 0.0000 | 0.0247 | 0.9753 | 0.9753 | 1384843997.2534 |  | fail |
| 20×20×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8295 (0.0086) | 0.1705 | 0.0000 | 0.8295 | 0.8284 | 2.9488 | facet-SE ratio 0.914 | fail |
| 20×20×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.984 | 0.9511 (0.0049) | 0.0000 | 0.0332 | 0.9663 | 0.9537 | 9526591588.7937 |  | fail |
| 20×20×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0305 | 0.0189 | 0.9505 | 0.9495 | 0.3499 | facet-SE ratio 0.986 | pass |
| 20×20×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0153 | 0.0289 | 0.9558 | 0.9563 | 0.3571 |  | pass |
| 20×20×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0479 | 0.0095 | 0.9426 | 0.9426 | 0.6632 | facet-SE ratio 0.982 | fail |
| 20×20×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0258 | 0.0242 | 0.9500 | 0.9500 | 0.7052 |  | pass |
| 20×20×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0505 | 0.0105 | 0.9389 | 0.9416 | 0.6574 | facet-SE ratio 0.985 | fail |
| 20×20×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0289 | 0.0247 | 0.9463 | 0.9489 | 0.6994 |  | pass |
| 20×20×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0337 | 0.0189 | 0.9474 | 0.9458 | 0.1455 | facet-SE ratio 1.070 | pass |
| 20×20×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0268 | 0.0232 | 0.9500 | 0.9500 | 0.1464 |  | pass |
| 20×20×5 | all-one | normal | G | F interval | 0.7937 | 1.000 | 0.9521 (0.0049) | 0.0216 | 0.0263 | 0.9521 | 0.9332 | 0.3222 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×5 | all-one | normal | Phi | parametric | 0.6579 | 1.000 | 0.9579 (0.0046) | 0.0289 | 0.0132 | 0.9579 | 0.9558 | 0.4660 |  | fail |
| 20×20×5 | all-one | normal | Phi | parametric, attached | 0.6579 | 0.984 | 0.9437 (0.0053) | 0.0284 | 0.0116 | 0.9593 | 0.9421 | 0.4672 |  | fail |
| 20×20×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9258 (0.0060) | 0.0716 | 0.0026 | 0.9258 | 0.9226 | 1.3513 | facet-SE ratio 0.962 | fail |
| 20×20×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0132 | 0.0326 | 0.9542 | 0.9547 | 1.7198 |  | fail |
| 20×20×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.9132 (0.0065) | 0.0863 | 0.0005 | 0.9132 | 0.9163 | 1.3207 | facet-SE ratio 0.995 | fail |
| 20×20×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0216 | 0.0205 | 0.9579 | 0.9563 | 1.6820 |  | pass |
| 20×20×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.8342 (0.0085) | 0.1658 | 0.0000 | 0.8342 | 0.8411 | 0.0358 | facet-SE ratio 0.926 | fail |
| 20×20×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.899 | 0.8553 (0.0081) | 0.0000 | 0.0437 | 0.9514 | 0.8516 | 2039117872.4442 |  | fail |
| 20×20×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0405 | 0.0153 | 0.9442 | 0.9437 | 0.3491 | facet-SE ratio 1.007 | fail |
| 20×20×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0263 | 0.0226 | 0.9511 | 0.9526 | 0.3560 |  | pass |
| 20×20×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9884 (0.0025) | 0.0000 | 0.0116 | 0.9884 | 0.9900 | 0.0393 | facet-SE ratio 0.990 | fail |
| 20×20×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.859 | 0.7353 (0.0101) | 0.0000 | 0.1242 | 0.8555 | 0.7500 | 8827625183.3668 |  | fail |
| 20×20×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9932 (0.0019) | 0.0000 | 0.0068 | 0.9932 | 0.9921 | 0.0389 | facet-SE ratio 0.991 | fail |
| 20×20×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.837 | 0.7368 (0.0101) | 0.0000 | 0.1000 | 0.8805 | 0.7253 | 9475493149.1449 |  | fail |
| 20×20×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0300 | 0.0195 | 0.9505 | 0.9511 | 0.1454 | facet-SE ratio 1.096 | pass |
| 20×20×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0258 | 0.0189 | 0.9553 | 0.9547 | 0.1463 |  | pass |
| 20×20×5 | near-zero-rater | normal | G | F interval | 0.9416 | 1.000 | 0.9532 (0.0048) | 0.0174 | 0.0295 | 0.9532 | 0.9279 | 0.0840 | unfit 0, errors 0, failed fits 0 | pass |
| 20×20×5 | near-zero-rater | normal | Phi | parametric | 0.8976 | 1.000 | 0.9389 (0.0055) | 0.0516 | 0.0095 | 0.9389 | 0.9516 | 0.1682 |  | fail |
| 20×20×5 | near-zero-rater | normal | Phi | parametric, attached | 0.8976 | 0.645 | 0.6032 (0.0112) | 0.0358 | 0.0063 | 0.9347 | 0.6100 | 0.1688 |  | fail |
| 20×20×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9258 (0.0060) | 0.0721 | 0.0021 | 0.9258 | 0.9289 | 5.1435 | facet-SE ratio 1.000 | fail |
| 20×20×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9658 (0.0042) | 0.0153 | 0.0189 | 0.9658 | 0.9642 | 6.4719 |  | fail |
| 20×20×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9232 (0.0061) | 0.0721 | 0.0047 | 0.9232 | 0.9242 | 0.7907 | facet-SE ratio 0.967 | fail |
| 20×20×5 | high-g | normal | i | Satterthwaite | 0.5000 | 1.000 | 0.9637 (0.0043) | 0.0000 | 0.0363 | 0.9637 | 0.9621 | 44.2880 |  | fail |
| 20×20×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8384 (0.0084) | 0.1616 | 0.0000 | 0.8384 | 0.8342 | 1.4904 | facet-SE ratio 0.931 | fail |
| 20×20×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.989 | 0.9537 (0.0048) | 0.0000 | 0.0353 | 0.9643 | 0.9574 | 5787014746.1063 |  | fail |
| 20×20×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9505 (0.0050) | 0.0347 | 0.0147 | 0.9505 | 0.9495 | 0.1753 | facet-SE ratio 1.011 | fail |
| 20×20×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0184 | 0.0274 | 0.9542 | 0.9542 | 0.1788 |  | pass |
| 20×20×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9353 (0.0056) | 0.0532 | 0.0116 | 0.9353 | 0.9353 | 0.3319 | facet-SE ratio 0.953 | fail |
| 20×20×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0263 | 0.0284 | 0.9453 | 0.9453 | 0.3525 |  | pass |
| 20×20×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9395 (0.0055) | 0.0495 | 0.0111 | 0.9395 | 0.9363 | 0.3330 | facet-SE ratio 0.965 | fail |
| 20×20×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0247 | 0.0268 | 0.9484 | 0.9468 | 0.3539 |  | pass |
| 20×20×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9521 (0.0049) | 0.0295 | 0.0184 | 0.9521 | 0.9511 | 0.0726 | facet-SE ratio 1.104 | pass |
| 20×20×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0232 | 0.0221 | 0.9547 | 0.9558 | 0.0732 |  | pass |
| 20×20×5 | high-g | normal | G | F interval | 0.9685 | 1.000 | 0.9611 (0.0044) | 0.0179 | 0.0211 | 0.9611 | 0.9400 | 0.0490 | unfit 0, errors 0, failed fits 0 | fail |
| 20×20×5 | high-g | normal | Phi | parametric | 0.9390 | 1.000 | 0.9632 (0.0043) | 0.0258 | 0.0111 | 0.9632 | 0.9579 | 0.1260 |  | fail |
| 20×20×5 | high-g | normal | Phi | parametric, attached | 0.9390 | 0.989 | 0.9537 (0.0048) | 0.0258 | 0.0095 | 0.9643 | 0.9505 | 0.1268 |  | fail |
| 50×5×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0500 | 0.0084 | 0.9416 | 0.9426 | 1.5043 | facet-SE ratio 0.983 | fail |
| 50×5×2 | all-one | normal | p | Satterthwaite | 1.0000 | 0.998 | 0.9495 (0.0050) | 0.0000 | 0.0484 | 0.9515 | 0.9447 | 10020438370.4463 |  | fail |
| 50×5×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0584 | 0.0000 | 0.9416 | 0.9374 | 4.7848 | facet-SE ratio 0.924 | fail |
| 50×5×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.831 | 0.7695 (0.0097) | 0.0000 | 0.0616 | 0.9259 | 0.7726 | 886075612513.5095 |  | fail |
| 50×5×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7795 (0.0095) | 0.2205 | 0.0000 | 0.7795 | 0.8021 | 6.2446 | facet-SE ratio 1.149 | fail |
| 50×5×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.686 | 0.6405 (0.0110) | 0.0000 | 0.0453 | 0.9340 | 0.6368 | 1031705729399.0643 |  | fail |
| 50×5×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0389 | 0.0174 | 0.9437 | 0.9437 | 0.6218 | facet-SE ratio 0.989 | fail |
| 50×5×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0079 | 0.0400 | 0.9521 | 0.9516 | 0.6568 |  | fail |
| 50×5×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0411 | 0.0095 | 0.9495 | 0.9437 | 0.9582 | facet-SE ratio 1.003 | fail |
| 50×5×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0095 | 0.0347 | 0.9558 | 0.9600 | 1.0802 |  | fail |
| 50×5×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8389 (0.0084) | 0.1611 | 0.0000 | 0.8389 | 0.8363 | 2.7161 | facet-SE ratio 0.942 | fail |
| 50×5×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9700 (0.0039) | 0.0000 | 0.0300 | 0.9700 | 0.9716 | 2752.4216 |  | fail |
| 50×5×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9379 (0.0055) | 0.0463 | 0.0158 | 0.9379 | 0.9389 | 0.3933 | facet-SE ratio 0.965 | fail |
| 50×5×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0284 | 0.0253 | 0.9463 | 0.9468 | 0.4030 |  | pass |
| 50×5×2 | all-one | normal | G | F interval | 0.5556 | 1.000 | 0.9542 (0.0048) | 0.0263 | 0.0195 | 0.9542 | 0.9432 | 0.4685 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×2 | all-one | normal | Phi | parametric | 0.3846 | 1.000 | 0.9600 (0.0045) | 0.0116 | 0.0284 | 0.9600 | 0.9589 | 0.4885 |  | fail |
| 50×5×2 | all-one | normal | Phi | parametric, attached | 0.3846 | 0.579 | 0.5647 (0.0114) | 0.0063 | 0.0084 | 0.9746 | 0.5632 | 0.4965 |  | fail |
| 50×5×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9321 (0.0058) | 0.0621 | 0.0058 | 0.9321 | 0.9326 | 1.0413 | facet-SE ratio 1.015 | fail |
| 50×5×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9684 (0.0040) | 0.0053 | 0.0263 | 0.9684 | 0.9684 | 1.2012 |  | fail |
| 50×5×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8305 (0.0086) | 0.1695 | 0.0000 | 0.8305 | 0.8300 | 2.7538 | facet-SE ratio 0.937 | fail |
| 50×5×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.997 | 0.9716 (0.0038) | 0.0000 | 0.0258 | 0.9741 | 0.9716 | 736056951.4032 |  | fail |
| 50×5×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.1007 | facet-SE ratio 1.083 | fail |
| 50×5×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.566 | 0.5016 (0.0115) | 0.0000 | 0.0647 | 0.8857 | 0.4879 | 35842524704.7935 |  | fail |
| 50×5×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0321 | 0.0168 | 0.9511 | 0.9484 | 0.6249 | facet-SE ratio 1.002 | pass |
| 50×5×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0100 | 0.0311 | 0.9589 | 0.9595 | 0.6592 |  | fail |
| 50×5×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9879 (0.0025) | 0.0000 | 0.0121 | 0.9879 | 0.9889 | 0.1946 | facet-SE ratio 1.006 | fail |
| 50×5×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.588 | 0.3137 (0.0106) | 0.0000 | 0.2747 | 0.5331 | 0.3500 | 99639110716.2240 |  | fail |
| 50×5×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.0876 | facet-SE ratio 0.929 | fail |
| 50×5×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.605 | 0.4900 (0.0115) | 0.0000 | 0.1147 | 0.8103 | 0.4795 | 32559144015.4243 |  | fail |
| 50×5×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0342 | 0.0111 | 0.9547 | 0.9547 | 0.3934 | facet-SE ratio 1.021 | fail |
| 50×5×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0232 | 0.0216 | 0.9553 | 0.9547 | 0.4036 |  | pass |
| 50×5×2 | near-zero-rater | normal | G | F interval | 0.7663 | 1.000 | 0.9474 (0.0051) | 0.0300 | 0.0226 | 0.9474 | 0.9458 | 0.2338 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×2 | near-zero-rater | normal | Phi | parametric | 0.6618 | 1.000 | 0.9563 (0.0047) | 0.0195 | 0.0242 | 0.9563 | 0.9505 | 0.3355 |  | pass |
| 50×5×2 | near-zero-rater | normal | Phi | parametric, attached | 0.6618 | 0.174 | 0.1689 (0.0086) | 0.0032 | 0.0021 | 0.9698 | 0.1684 | 0.3453 |  | fail |
| 50×5×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9326 (0.0058) | 0.0563 | 0.0111 | 0.9326 | 0.9326 | 3.4659 | facet-SE ratio 0.948 | fail |
| 50×5×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9474 (0.0051) | 0.0179 | 0.0347 | 0.9474 | 0.9479 | 3.8347 |  | pass |
| 50×5×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9389 (0.0055) | 0.0611 | 0.0000 | 0.9389 | 0.9368 | 2.3659 | facet-SE ratio 0.936 | fail |
| 50×5×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.833 | 0.7753 (0.0096) | 0.0000 | 0.0579 | 0.9305 | 0.7758 | 403766709304.0802 |  | fail |
| 50×5×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7895 (0.0094) | 0.2105 | 0.0000 | 0.7895 | 0.7953 | 3.2448 | facet-SE ratio 1.161 | fail |
| 50×5×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.689 | 0.6479 (0.0110) | 0.0000 | 0.0416 | 0.9397 | 0.6442 | 487729096260.5543 |  | fail |
| 50×5×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0358 | 0.0158 | 0.9484 | 0.9484 | 0.3125 | facet-SE ratio 1.007 | fail |
| 50×5×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9600 (0.0045) | 0.0084 | 0.0316 | 0.9600 | 0.9600 | 0.3294 |  | fail |
| 50×5×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9389 (0.0055) | 0.0553 | 0.0058 | 0.9389 | 0.9395 | 0.4703 | facet-SE ratio 0.995 | fail |
| 50×5×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9558 (0.0047) | 0.0137 | 0.0305 | 0.9558 | 0.9521 | 0.5313 |  | fail |
| 50×5×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8368 (0.0085) | 0.1632 | 0.0000 | 0.8368 | 0.8358 | 1.3540 | facet-SE ratio 0.954 | fail |
| 50×5×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9732 (0.0037) | 0.0000 | 0.0268 | 0.9732 | 0.9737 | 4.4138 |  | fail |
| 50×5×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9363 (0.0056) | 0.0384 | 0.0253 | 0.9363 | 0.9363 | 0.1977 | facet-SE ratio 0.959 | fail |
| 50×5×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9368 (0.0056) | 0.0258 | 0.0374 | 0.9368 | 0.9368 | 0.2024 |  | fail |
| 50×5×2 | high-g | normal | G | F interval | 0.9091 | 1.000 | 0.9405 (0.0054) | 0.0274 | 0.0321 | 0.9405 | 0.9374 | 0.0985 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×2 | high-g | normal | Phi | parametric | 0.8333 | 1.000 | 0.9547 (0.0048) | 0.0026 | 0.0426 | 0.9547 | 0.9511 | 0.2628 |  | fail |
| 50×5×2 | high-g | normal | Phi | parametric, attached | 0.8333 | 0.585 | 0.5711 (0.0114) | 0.0016 | 0.0126 | 0.9757 | 0.5695 | 0.3044 |  | fail |
| 50×5×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9379 (0.0055) | 0.0547 | 0.0074 | 0.9379 | 0.9384 | 1.2813 | facet-SE ratio 0.991 | fail |
| 50×5×3 | all-one | normal | p | Satterthwaite | 1.0000 | 0.999 | 0.9605 (0.0045) | 0.0000 | 0.0389 | 0.9610 | 0.9600 | 285979224.9372 |  | fail |
| 50×5×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8832 (0.0074) | 0.1168 | 0.0000 | 0.8832 | 0.8784 | 3.8677 | facet-SE ratio 0.936 | fail |
| 50×5×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.901 | 0.8442 (0.0083) | 0.0000 | 0.0563 | 0.9375 | 0.8489 | 296608997050.4019 |  | fail |
| 50×5×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7900 (0.0093) | 0.2100 | 0.0000 | 0.7900 | 0.7905 | 4.6286 | facet-SE ratio 0.977 | fail |
| 50×5×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.828 | 0.7863 (0.0094) | 0.0000 | 0.0416 | 0.9498 | 0.7863 | 292900491914.3301 |  | fail |
| 50×5×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0347 | 0.0153 | 0.9500 | 0.9505 | 0.5317 | facet-SE ratio 1.011 | pass |
| 50×5×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0174 | 0.0274 | 0.9553 | 0.9537 | 0.5536 |  | pass |
| 50×5×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9458 (0.0052) | 0.0421 | 0.0121 | 0.9458 | 0.9458 | 0.6722 | facet-SE ratio 1.002 | fail |
| 50×5×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9574 (0.0046) | 0.0158 | 0.0268 | 0.9574 | 0.9579 | 0.7149 |  | pass |
| 50×5×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8879 (0.0072) | 0.1111 | 0.0011 | 0.8879 | 0.8874 | 1.9578 | facet-SE ratio 0.988 | fail |
| 50×5×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0189 | 0.0242 | 0.9568 | 0.9574 | 3.3603 |  | pass |
| 50×5×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0389 | 0.0226 | 0.9384 | 0.9384 | 0.2786 | facet-SE ratio 0.956 | fail |
| 50×5×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0300 | 0.0311 | 0.9389 | 0.9389 | 0.2826 |  | fail |
| 50×5×3 | all-one | normal | G | F interval | 0.6250 | 1.000 | 0.9505 (0.0050) | 0.0263 | 0.0232 | 0.9505 | 0.9500 | 0.3538 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×3 | all-one | normal | Phi | parametric | 0.4545 | 1.000 | 0.9658 (0.0042) | 0.0121 | 0.0221 | 0.9658 | 0.9637 | 0.4343 |  | fail |
| 50×5×3 | all-one | normal | Phi | parametric, attached | 0.4545 | 0.754 | 0.7326 (0.0102) | 0.0095 | 0.0121 | 0.9714 | 0.7321 | 0.4403 |  | fail |
| 50×5×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9332 (0.0057) | 0.0574 | 0.0095 | 0.9332 | 0.9337 | 1.0052 | facet-SE ratio 0.986 | fail |
| 50×5×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9621 (0.0044) | 0.0074 | 0.0305 | 0.9621 | 0.9621 | 1.1502 |  | fail |
| 50×5×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8326 (0.0086) | 0.1674 | 0.0000 | 0.8326 | 0.8300 | 2.7546 | facet-SE ratio 0.959 | fail |
| 50×5×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.999 | 0.9768 (0.0035) | 0.0000 | 0.0221 | 0.9779 | 0.9779 | 485581029.0813 |  | fail |
| 50×5×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.9968 (0.0013) | 0.0032 | 0.0000 | 0.9968 | 0.9953 | 0.0705 | facet-SE ratio 0.942 | fail |
| 50×5×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.696 | 0.6184 (0.0111) | 0.0000 | 0.0774 | 0.8888 | 0.6237 | 15612752238.0815 |  | fail |
| 50×5×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9463 (0.0052) | 0.0374 | 0.0163 | 0.9463 | 0.9463 | 0.5333 | facet-SE ratio 0.999 | fail |
| 50×5×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0184 | 0.0268 | 0.9547 | 0.9542 | 0.5552 |  | pass |
| 50×5×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9805 (0.0032) | 0.0000 | 0.0195 | 0.9805 | 0.9805 | 0.1342 | facet-SE ratio 0.972 | fail |
| 50×5×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.608 | 0.3674 (0.0111) | 0.0000 | 0.2405 | 0.6043 | 0.3737 | 60885314556.7986 |  | fail |
| 50×5×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.0610 | facet-SE ratio 0.921 | fail |
| 50×5×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.724 | 0.6089 (0.0112) | 0.0000 | 0.1153 | 0.8408 | 0.6116 | 20684363214.7768 |  | fail |
| 50×5×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9489 (0.0050) | 0.0374 | 0.0137 | 0.9489 | 0.9495 | 0.2786 | facet-SE ratio 0.998 | fail |
| 50×5×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9558 (0.0047) | 0.0268 | 0.0174 | 0.9558 | 0.9537 | 0.2823 |  | pass |
| 50×5×3 | near-zero-rater | normal | G | F interval | 0.7874 | 1.000 | 0.9437 (0.0053) | 0.0300 | 0.0263 | 0.9437 | 0.9447 | 0.2022 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×3 | near-zero-rater | normal | Phi | parametric | 0.6784 | 1.000 | 0.9537 (0.0048) | 0.0137 | 0.0326 | 0.9537 | 0.9489 | 0.3247 |  | fail |
| 50×5×3 | near-zero-rater | normal | Phi | parametric, attached | 0.6784 | 0.283 | 0.2705 (0.0102) | 0.0037 | 0.0084 | 0.9572 | 0.2700 | 0.3273 |  | fail |
| 50×5×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9405 (0.0054) | 0.0489 | 0.0105 | 0.9405 | 0.9384 | 3.4069 | facet-SE ratio 0.997 | fail |
| 50×5×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9521 (0.0049) | 0.0147 | 0.0332 | 0.9521 | 0.9521 | 3.7552 |  | fail |
| 50×5×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8832 (0.0074) | 0.1168 | 0.0000 | 0.8832 | 0.8832 | 1.9030 | facet-SE ratio 0.966 | fail |
| 50×5×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.895 | 0.8453 (0.0083) | 0.0000 | 0.0500 | 0.9442 | 0.8453 | 183908966737.0054 |  | fail |
| 50×5×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8021 (0.0091) | 0.1979 | 0.0000 | 0.8021 | 0.7926 | 2.4002 | facet-SE ratio 0.960 | fail |
| 50×5×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.837 | 0.7916 (0.0093) | 0.0000 | 0.0453 | 0.9459 | 0.7968 | 185012442393.3592 |  | fail |
| 50×5×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0416 | 0.0137 | 0.9447 | 0.9453 | 0.2661 | facet-SE ratio 0.992 | fail |
| 50×5×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0195 | 0.0263 | 0.9542 | 0.9537 | 0.2772 |  | pass |
| 50×5×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9395 (0.0055) | 0.0484 | 0.0121 | 0.9395 | 0.9389 | 0.3355 | facet-SE ratio 0.994 | fail |
| 50×5×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9511 (0.0049) | 0.0211 | 0.0279 | 0.9511 | 0.9511 | 0.3573 |  | pass |
| 50×5×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8926 (0.0071) | 0.1063 | 0.0011 | 0.8926 | 0.8921 | 0.9786 | facet-SE ratio 0.953 | fail |
| 50×5×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0158 | 0.0295 | 0.9547 | 0.9547 | 1.6829 |  | pass |
| 50×5×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9463 (0.0052) | 0.0321 | 0.0216 | 0.9463 | 0.9432 | 0.1396 | facet-SE ratio 0.989 | pass |
| 50×5×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0226 | 0.0274 | 0.9500 | 0.9505 | 0.1416 |  | pass |
| 50×5×3 | high-g | normal | G | F interval | 0.9302 | 1.000 | 0.9495 (0.0050) | 0.0232 | 0.0274 | 0.9495 | 0.9447 | 0.0653 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×3 | high-g | normal | Phi | parametric | 0.8696 | 1.000 | 0.9547 (0.0048) | 0.0026 | 0.0426 | 0.9547 | 0.9442 | 0.1921 |  | fail |
| 50×5×3 | high-g | normal | Phi | parametric, attached | 0.8696 | 0.752 | 0.7332 (0.0101) | 0.0026 | 0.0163 | 0.9748 | 0.7279 | 0.2079 |  | fail |
| 50×5×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9374 (0.0056) | 0.0574 | 0.0053 | 0.9374 | 0.9374 | 1.1373 | facet-SE ratio 0.992 | fail |
| 50×5×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9642 (0.0043) | 0.0000 | 0.0358 | 0.9642 | 0.9616 | 1.3617 |  | fail |
| 50×5×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8300 (0.0086) | 0.1700 | 0.0000 | 0.8300 | 0.8300 | 3.3091 | facet-SE ratio 0.945 | fail |
| 50×5×5 | all-one | normal | i | Satterthwaite | 1.0000 | 0.946 | 0.9095 (0.0066) | 0.0000 | 0.0363 | 0.9616 | 0.9100 | 100145088644.8413 |  | fail |
| 50×5×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8411 (0.0084) | 0.1589 | 0.0000 | 0.8411 | 0.8453 | 3.2093 | facet-SE ratio 0.952 | fail |
| 50×5×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.942 | 0.9095 (0.0066) | 0.0000 | 0.0326 | 0.9654 | 0.9053 | 91638887511.8790 |  | fail |
| 50×5×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0342 | 0.0179 | 0.9479 | 0.9474 | 0.4755 | facet-SE ratio 0.993 | pass |
| 50×5×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0174 | 0.0279 | 0.9547 | 0.9547 | 0.4914 |  | pass |
| 50×5×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0326 | 0.0147 | 0.9526 | 0.9526 | 0.4761 | facet-SE ratio 1.024 | fail |
| 50×5×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0205 | 0.0268 | 0.9526 | 0.9547 | 0.4925 |  | pass |
| 50×5×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9168 (0.0063) | 0.0811 | 0.0021 | 0.9168 | 0.9142 | 1.3999 | facet-SE ratio 1.036 | fail |
| 50×5×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9568 (0.0047) | 0.0205 | 0.0226 | 0.9568 | 0.9584 | 1.8254 |  | pass |
| 50×5×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9521 (0.0049) | 0.0305 | 0.0174 | 0.9521 | 0.9484 | 0.1964 | facet-SE ratio 0.978 | pass |
| 50×5×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0232 | 0.0242 | 0.9526 | 0.9532 | 0.1984 |  | pass |
| 50×5×5 | all-one | normal | G | F interval | 0.6944 | 1.000 | 0.9516 (0.0049) | 0.0284 | 0.0200 | 0.9516 | 0.9453 | 0.2740 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×5 | all-one | normal | Phi | parametric | 0.5319 | 1.000 | 0.9589 (0.0046) | 0.0205 | 0.0205 | 0.9589 | 0.9584 | 0.3816 |  | pass |
| 50×5×5 | all-one | normal | Phi | parametric, attached | 0.5319 | 0.891 | 0.8574 (0.0080) | 0.0205 | 0.0126 | 0.9628 | 0.8574 | 0.3846 |  | fail |
| 50×5×5 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9479 (0.0051) | 0.0468 | 0.0053 | 0.9479 | 0.9479 | 0.9828 | facet-SE ratio 1.024 | fail |
| 50×5×5 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9621 (0.0044) | 0.0126 | 0.0253 | 0.9621 | 0.9626 | 1.1197 |  | fail |
| 50×5×5 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8437 (0.0083) | 0.1563 | 0.0000 | 0.8437 | 0.8405 | 2.7385 | facet-SE ratio 0.945 | fail |
| 50×5×5 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 0.998 | 0.9684 (0.0040) | 0.0000 | 0.0295 | 0.9705 | 0.9695 | 66708.2966 |  | fail |
| 50×5×5 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.9516 (0.0049) | 0.0484 | 0.0000 | 0.9516 | 0.9611 | 0.0456 | facet-SE ratio 0.919 | fail |
| 50×5×5 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.801 | 0.7379 (0.0101) | 0.0000 | 0.0626 | 0.9218 | 0.7321 | 6564154601.7765 |  | fail |
| 50×5×5 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0321 | 0.0168 | 0.9511 | 0.9484 | 0.4761 | facet-SE ratio 1.012 | pass |
| 50×5×5 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9547 (0.0048) | 0.0163 | 0.0289 | 0.9547 | 0.9558 | 0.4922 |  | pass |
| 50×5×5 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9821 (0.0030) | 0.0000 | 0.0179 | 0.9821 | 0.9821 | 0.0936 | facet-SE ratio 0.998 | fail |
| 50×5×5 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.657 | 0.4595 (0.0114) | 0.0000 | 0.1979 | 0.6990 | 0.4532 | 36409822554.1336 |  | fail |
| 50×5×5 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9968 (0.0013) | 0.0000 | 0.0032 | 0.9968 | 0.9968 | 0.0417 | facet-SE ratio 0.934 | fail |
| 50×5×5 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.815 | 0.7311 (0.0102) | 0.0000 | 0.0842 | 0.8967 | 0.7237 | 8937150803.9102 |  | fail |
| 50×5×5 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0337 | 0.0247 | 0.9416 | 0.9421 | 0.1973 | facet-SE ratio 0.956 | pass |
| 50×5×5 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9416 (0.0054) | 0.0263 | 0.0321 | 0.9416 | 0.9426 | 0.1992 |  | pass |
| 50×5×5 | near-zero-rater | normal | G | F interval | 0.8052 | 1.000 | 0.9516 (0.0049) | 0.0263 | 0.0221 | 0.9516 | 0.9474 | 0.1823 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×5 | near-zero-rater | normal | Phi | parametric | 0.6923 | 1.000 | 0.9511 (0.0049) | 0.0089 | 0.0400 | 0.9511 | 0.9479 | 0.3179 |  | fail |
| 50×5×5 | near-zero-rater | normal | Phi | parametric, attached | 0.6923 | 0.418 | 0.3963 (0.0112) | 0.0042 | 0.0179 | 0.9472 | 0.3953 | 0.3171 |  | fail |
| 50×5×5 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9474 (0.0051) | 0.0453 | 0.0074 | 0.9474 | 0.9463 | 3.3362 | facet-SE ratio 0.994 | fail |
| 50×5×5 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9553 (0.0047) | 0.0195 | 0.0253 | 0.9553 | 0.9574 | 3.6611 |  | pass |
| 50×5×5 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8384 (0.0084) | 0.1616 | 0.0000 | 0.8384 | 0.8384 | 1.6492 | facet-SE ratio 0.956 | fail |
| 50×5×5 | high-g | normal | i | Satterthwaite | 0.5000 | 0.940 | 0.9053 (0.0067) | 0.0000 | 0.0347 | 0.9630 | 0.9053 | 41240368260.0332 |  | fail |
| 50×5×5 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.8421 (0.0084) | 0.1579 | 0.0000 | 0.8421 | 0.8442 | 1.6183 | facet-SE ratio 0.925 | fail |
| 50×5×5 | high-g | normal | r | Satterthwaite | 0.5000 | 0.939 | 0.9037 (0.0068) | 0.0000 | 0.0358 | 0.9619 | 0.9032 | 57241567570.1971 |  | fail |
| 50×5×5 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9484 (0.0051) | 0.0389 | 0.0126 | 0.9484 | 0.9484 | 0.2370 | facet-SE ratio 1.011 | fail |
| 50×5×5 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9563 (0.0047) | 0.0179 | 0.0258 | 0.9563 | 0.9568 | 0.2451 |  | pass |
| 50×5×5 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9432 (0.0053) | 0.0400 | 0.0168 | 0.9432 | 0.9416 | 0.2374 | facet-SE ratio 0.986 | fail |
| 50×5×5 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9532 (0.0048) | 0.0189 | 0.0279 | 0.9532 | 0.9532 | 0.2453 |  | pass |
| 50×5×5 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9105 (0.0065) | 0.0879 | 0.0016 | 0.9105 | 0.9079 | 0.7012 | facet-SE ratio 0.999 | fail |
| 50×5×5 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9453 (0.0052) | 0.0237 | 0.0311 | 0.9453 | 0.9479 | 0.9137 |  | pass |
| 50×5×5 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9542 (0.0048) | 0.0274 | 0.0184 | 0.9542 | 0.9553 | 0.0986 | facet-SE ratio 0.992 | pass |
| 50×5×5 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9579 (0.0046) | 0.0200 | 0.0221 | 0.9579 | 0.9579 | 0.0995 |  | pass |
| 50×5×5 | high-g | normal | G | F interval | 0.9479 | 1.000 | 0.9542 (0.0048) | 0.0216 | 0.0242 | 0.9542 | 0.9521 | 0.0462 | unfit 0, errors 0, failed fits 0 | pass |
| 50×5×5 | high-g | normal | Phi | parametric | 0.9009 | 1.000 | 0.9521 (0.0049) | 0.0100 | 0.0379 | 0.9521 | 0.9468 | 0.1303 |  | fail |
| 50×5×5 | high-g | normal | Phi | parametric, attached | 0.9009 | 0.884 | 0.8516 (0.0082) | 0.0100 | 0.0221 | 0.9637 | 0.8500 | 0.1342 |  | fail |
| 50×8×2 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9368 (0.0056) | 0.0474 | 0.0158 | 0.9368 | 0.9337 | 1.4247 | facet-SE ratio 0.955 | fail |
| 50×8×2 | all-one | normal | p | Satterthwaite | 1.0000 | 0.999 | 0.9342 (0.0057) | 0.0000 | 0.0653 | 0.9347 | 0.9447 | 2589251019.1808 |  | fail |
| 50×8×2 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.9247 (0.0061) | 0.0747 | 0.0005 | 0.9247 | 0.9253 | 3.4565 | facet-SE ratio 0.953 | fail |
| 50×8×2 | all-one | normal | i | Satterthwaite | 1.0000 | 0.907 | 0.8463 (0.0083) | 0.0000 | 0.0611 | 0.9327 | 0.8463 | 362402271265.5444 |  | fail |
| 50×8×2 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.6789 (0.0107) | 0.3211 | 0.0000 | 0.6789 | 0.6863 | 5.8191 | facet-SE ratio 1.173 | fail |
| 50×8×2 | all-one | normal | r | Satterthwaite | 1.0000 | 0.729 | 0.6905 (0.0106) | 0.0000 | 0.0389 | 0.9466 | 0.6905 | 525863311262.4862 |  | fail |
| 50×8×2 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0321 | 0.0195 | 0.9484 | 0.9484 | 0.4711 | facet-SE ratio 0.979 | pass |
| 50×8×2 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0147 | 0.0342 | 0.9511 | 0.9511 | 0.4868 |  | fail |
| 50×8×2 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0537 | 0.0074 | 0.9389 | 0.9432 | 0.8778 | facet-SE ratio 1.005 | fail |
| 50×8×2 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9589 (0.0046) | 0.0174 | 0.0237 | 0.9589 | 0.9595 | 0.9765 |  | pass |
| 50×8×2 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.8795 (0.0075) | 0.1195 | 0.0011 | 0.8795 | 0.8795 | 2.0778 | facet-SE ratio 0.939 | fail |
| 50×8×2 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9553 (0.0047) | 0.0200 | 0.0247 | 0.9553 | 0.9553 | 3.8762 |  | pass |
| 50×8×2 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0337 | 0.0179 | 0.9484 | 0.9489 | 0.2977 | facet-SE ratio 0.990 | pass |
| 50×8×2 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0232 | 0.0258 | 0.9511 | 0.9505 | 0.3027 |  | pass |
| 50×8×2 | all-one | normal | G | F interval | 0.5926 | 1.000 | 0.9432 (0.0053) | 0.0237 | 0.0332 | 0.9432 | 0.9379 | 0.4358 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×2 | all-one | normal | Phi | parametric | 0.4211 | 1.000 | 0.9421 (0.0054) | 0.0074 | 0.0505 | 0.9421 | 0.9505 | 0.4862 |  | fail |
| 50×8×2 | all-one | normal | Phi | parametric, attached | 0.4211 | 0.668 | 0.6432 (0.0110) | 0.0058 | 0.0195 | 0.9622 | 0.6437 | 0.5044 |  | fail |
| 50×8×2 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0542 | 0.0074 | 0.9384 | 0.9342 | 0.9486 | facet-SE ratio 0.984 | fail |
| 50×8×2 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0163 | 0.0305 | 0.9532 | 0.9558 | 1.0658 |  | pass |
| 50×8×2 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8868 (0.0073) | 0.1132 | 0.0000 | 0.8868 | 0.8847 | 2.1327 | facet-SE ratio 0.937 | fail |
| 50×8×2 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9579 (0.0046) | 0.0079 | 0.0342 | 0.9579 | 0.9605 | 4.0551 |  | fail |
| 50×8×2 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.9837 (0.0029) | 0.0163 | 0.0000 | 0.9837 | 0.9911 | 0.0787 | facet-SE ratio 1.126 | fail |
| 50×8×2 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.599 | 0.5442 (0.0114) | 0.0005 | 0.0542 | 0.9086 | 0.5405 | 21570825477.7215 |  | fail |
| 50×8×2 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9389 (0.0055) | 0.0379 | 0.0232 | 0.9389 | 0.9384 | 0.4708 | facet-SE ratio 0.974 | fail |
| 50×8×2 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9447 (0.0052) | 0.0168 | 0.0384 | 0.9447 | 0.9463 | 0.4863 |  | fail |
| 50×8×2 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9895 (0.0023) | 0.0000 | 0.0105 | 0.9895 | 0.9895 | 0.1171 | facet-SE ratio 0.999 | fail |
| 50×8×2 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.616 | 0.4179 (0.0113) | 0.0000 | 0.1979 | 0.6786 | 0.4063 | 55137218063.5758 |  | fail |
| 50×8×2 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 1.0000 (0.0000) | 0.0000 | 0.0000 | 1.0000 | 1.0000 | 0.0650 | facet-SE ratio 0.951 | fail |
| 50×8×2 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.709 | 0.6074 (0.0112) | 0.0000 | 0.1016 | 0.8567 | 0.6074 | 20620735754.0578 |  | fail |
| 50×8×2 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9363 (0.0056) | 0.0516 | 0.0121 | 0.9363 | 0.9379 | 0.2960 | facet-SE ratio 0.945 | fail |
| 50×8×2 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9379 (0.0055) | 0.0395 | 0.0226 | 0.9379 | 0.9353 | 0.3007 |  | fail |
| 50×8×2 | near-zero-rater | normal | G | F interval | 0.8386 | 1.000 | 0.9532 (0.0048) | 0.0226 | 0.0242 | 0.9532 | 0.9447 | 0.1517 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×2 | near-zero-rater | normal | Phi | parametric | 0.7558 | 1.000 | 0.9521 (0.0049) | 0.0263 | 0.0216 | 0.9521 | 0.9521 | 0.2447 |  | pass |
| 50×8×2 | near-zero-rater | normal | Phi | parametric, attached | 0.7558 | 0.245 | 0.2316 (0.0097) | 0.0089 | 0.0042 | 0.9462 | 0.2316 | 0.2505 |  | fail |
| 50×8×2 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9374 (0.0056) | 0.0537 | 0.0089 | 0.9374 | 0.9374 | 3.4359 | facet-SE ratio 0.968 | fail |
| 50×8×2 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9479 (0.0051) | 0.0200 | 0.0321 | 0.9479 | 0.9479 | 3.7947 |  | pass |
| 50×8×2 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.9416 (0.0054) | 0.0584 | 0.0000 | 0.9416 | 0.9421 | 1.7225 | facet-SE ratio 0.953 | fail |
| 50×8×2 | high-g | normal | i | Satterthwaite | 0.5000 | 0.915 | 0.8479 (0.0082) | 0.0000 | 0.0674 | 0.9264 | 0.8442 | 216211707714.3369 |  | fail |
| 50×8×2 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.6684 (0.0108) | 0.3316 | 0.0000 | 0.6684 | 0.6695 | 2.9518 | facet-SE ratio 1.160 | fail |
| 50×8×2 | high-g | normal | r | Satterthwaite | 0.5000 | 0.723 | 0.6747 (0.0107) | 0.0000 | 0.0479 | 0.9337 | 0.6742 | 326282002974.3917 |  | fail |
| 50×8×2 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9447 (0.0052) | 0.0368 | 0.0184 | 0.9447 | 0.9447 | 0.2356 | facet-SE ratio 0.978 | fail |
| 50×8×2 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9516 (0.0049) | 0.0137 | 0.0347 | 0.9516 | 0.9516 | 0.2435 |  | fail |
| 50×8×2 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9458 (0.0052) | 0.0463 | 0.0079 | 0.9458 | 0.9458 | 0.4445 | facet-SE ratio 0.999 | fail |
| 50×8×2 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9584 (0.0046) | 0.0153 | 0.0263 | 0.9584 | 0.9589 | 0.4939 |  | pass |
| 50×8×2 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.8795 (0.0075) | 0.1205 | 0.0000 | 0.8795 | 0.8784 | 1.0407 | facet-SE ratio 0.964 | fail |
| 50×8×2 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9616 (0.0044) | 0.0158 | 0.0226 | 0.9616 | 0.9616 | 1.9452 |  | fail |
| 50×8×2 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9547 (0.0048) | 0.0279 | 0.0174 | 0.9547 | 0.9526 | 0.1490 | facet-SE ratio 1.004 | pass |
| 50×8×2 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9574 (0.0046) | 0.0168 | 0.0258 | 0.9574 | 0.9579 | 0.1515 |  | pass |
| 50×8×2 | high-g | normal | G | F interval | 0.9209 | 1.000 | 0.9500 (0.0050) | 0.0258 | 0.0242 | 0.9500 | 0.9463 | 0.0885 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×2 | high-g | normal | Phi | parametric | 0.8533 | 1.000 | 0.9079 (0.0066) | 0.0021 | 0.0900 | 0.9079 | 0.9037 | 0.2353 |  | fail |
| 50×8×2 | high-g | normal | Phi | parametric, attached | 0.8533 | 0.662 | 0.6295 (0.0111) | 0.0016 | 0.0311 | 0.9507 | 0.6284 | 0.2778 |  | fail |
| 50×8×3 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0458 | 0.0074 | 0.9468 | 0.9474 | 1.2004 | facet-SE ratio 1.026 | fail |
| 50×8×3 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9637 (0.0043) | 0.0000 | 0.0363 | 0.9637 | 0.9663 | 1.4687 |  | fail |
| 50×8×3 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8863 (0.0073) | 0.1132 | 0.0005 | 0.8863 | 0.8853 | 2.8665 | facet-SE ratio 0.958 | fail |
| 50×8×3 | all-one | normal | i | Satterthwaite | 1.0000 | 0.956 | 0.9163 (0.0064) | 0.0000 | 0.0395 | 0.9587 | 0.9163 | 99903230341.0183 |  | fail |
| 50×8×3 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.7632 (0.0098) | 0.2368 | 0.0000 | 0.7632 | 0.7605 | 4.2936 | facet-SE ratio 0.968 | fail |
| 50×8×3 | all-one | normal | r | Satterthwaite | 1.0000 | 0.886 | 0.8447 (0.0083) | 0.0000 | 0.0416 | 0.9531 | 0.8468 | 149618551355.0761 |  | fail |
| 50×8×3 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9474 (0.0051) | 0.0326 | 0.0200 | 0.9474 | 0.9474 | 0.4030 | facet-SE ratio 0.987 | pass |
| 50×8×3 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0226 | 0.0305 | 0.9468 | 0.9474 | 0.4135 |  | pass |
| 50×8×3 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9405 (0.0054) | 0.0484 | 0.0111 | 0.9405 | 0.9416 | 0.6234 | facet-SE ratio 0.998 | fail |
| 50×8×3 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9505 (0.0050) | 0.0232 | 0.0263 | 0.9505 | 0.9505 | 0.6589 |  | pass |
| 50×8×3 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9142 (0.0064) | 0.0832 | 0.0026 | 0.9142 | 0.9132 | 1.4899 | facet-SE ratio 0.978 | fail |
| 50×8×3 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0216 | 0.0253 | 0.9532 | 0.9532 | 2.0230 |  | pass |
| 50×8×3 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0374 | 0.0205 | 0.9421 | 0.9437 | 0.2107 | facet-SE ratio 0.979 | fail |
| 50×8×3 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9468 (0.0051) | 0.0284 | 0.0247 | 0.9468 | 0.9458 | 0.2130 |  | pass |
| 50×8×3 | all-one | normal | G | F interval | 0.6667 | 1.000 | 0.9532 (0.0048) | 0.0189 | 0.0279 | 0.9532 | 0.9468 | 0.3160 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×3 | all-one | normal | Phi | parametric | 0.5000 | 1.000 | 0.9579 (0.0046) | 0.0079 | 0.0342 | 0.9579 | 0.9574 | 0.4195 |  | fail |
| 50×8×3 | all-one | normal | Phi | parametric, attached | 0.5000 | 0.849 | 0.8189 (0.0088) | 0.0079 | 0.0221 | 0.9647 | 0.8179 | 0.4268 |  | fail |
| 50×8×3 | near-zero-rater | normal | p | parametric | 1.0000 | 1.000 | 0.9421 (0.0054) | 0.0500 | 0.0079 | 0.9421 | 0.9421 | 0.9272 | facet-SE ratio 0.983 | fail |
| 50×8×3 | near-zero-rater | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9516 (0.0049) | 0.0137 | 0.0347 | 0.9516 | 0.9532 | 1.0369 |  | fail |
| 50×8×3 | near-zero-rater | normal | i | parametric | 1.0000 | 1.000 | 0.8789 (0.0075) | 0.1211 | 0.0000 | 0.8789 | 0.8811 | 2.0817 | facet-SE ratio 0.967 | fail |
| 50×8×3 | near-zero-rater | normal | i | Satterthwaite | 1.0000 | 1.000 | 0.9658 (0.0042) | 0.0126 | 0.0216 | 0.9658 | 0.9647 | 3.9363 |  | fail |
| 50×8×3 | near-zero-rater | normal | r | parametric | 0.0100 | 1.000 | 0.9095 (0.0066) | 0.0905 | 0.0000 | 0.9095 | 0.9189 | 0.0542 | facet-SE ratio 0.950 | fail |
| 50×8×3 | near-zero-rater | normal | r | Satterthwaite | 0.0100 | 0.754 | 0.6947 (0.0106) | 0.0000 | 0.0589 | 0.9218 | 0.6916 | 9585061667.3096 |  | fail |
| 50×8×3 | near-zero-rater | normal | pi | parametric | 1.0000 | 1.000 | 0.9437 (0.0053) | 0.0389 | 0.0174 | 0.9437 | 0.9426 | 0.4028 | facet-SE ratio 0.975 | fail |
| 50×8×3 | near-zero-rater | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9526 (0.0049) | 0.0184 | 0.0289 | 0.9526 | 0.9516 | 0.4130 |  | pass |
| 50×8×3 | near-zero-rater | normal | pr | parametric | 0.0100 | 1.000 | 0.9853 (0.0028) | 0.0000 | 0.0147 | 0.9853 | 0.9821 | 0.0813 | facet-SE ratio 0.985 | fail |
| 50×8×3 | near-zero-rater | normal | pr | Satterthwaite | 0.0100 | 0.662 | 0.4921 (0.0115) | 0.0000 | 0.1700 | 0.7432 | 0.4732 | 31733112994.2035 |  | fail |
| 50×8×3 | near-zero-rater | normal | ir | parametric | 0.0100 | 1.000 | 0.9989 (0.0007) | 0.0000 | 0.0011 | 0.9989 | 0.9989 | 0.0456 | facet-SE ratio 0.966 | fail |
| 50×8×3 | near-zero-rater | normal | ir | Satterthwaite | 0.0100 | 0.822 | 0.7111 (0.0104) | 0.0000 | 0.1105 | 0.8655 | 0.7174 | 9314649728.5177 |  | fail |
| 50×8×3 | near-zero-rater | normal | pir | parametric | 1.0000 | 1.000 | 0.9426 (0.0053) | 0.0326 | 0.0247 | 0.9426 | 0.9426 | 0.2106 | facet-SE ratio 0.958 | pass |
| 50×8×3 | near-zero-rater | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0295 | 0.0321 | 0.9384 | 0.9395 | 0.2130 |  | fail |
| 50×8×3 | near-zero-rater | normal | G | F interval | 0.8547 | 1.000 | 0.9458 (0.0052) | 0.0237 | 0.0305 | 0.9458 | 0.9337 | 0.1302 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×3 | near-zero-rater | normal | Phi | parametric | 0.7700 | 1.000 | 0.9521 (0.0049) | 0.0195 | 0.0284 | 0.9521 | 0.9479 | 0.2320 |  | pass |
| 50×8×3 | near-zero-rater | normal | Phi | parametric, attached | 0.7700 | 0.390 | 0.3700 (0.0111) | 0.0084 | 0.0116 | 0.9487 | 0.3674 | 0.2331 |  | fail |
| 50×8×3 | high-g | normal | p | parametric | 4.0000 | 1.000 | 0.9316 (0.0058) | 0.0595 | 0.0089 | 0.9316 | 0.9353 | 3.3086 | facet-SE ratio 0.987 | fail |
| 50×8×3 | high-g | normal | p | Satterthwaite | 4.0000 | 1.000 | 0.9574 (0.0046) | 0.0189 | 0.0237 | 0.9574 | 0.9563 | 3.6406 |  | pass |
| 50×8×3 | high-g | normal | i | parametric | 0.5000 | 1.000 | 0.8937 (0.0071) | 0.1063 | 0.0000 | 0.8937 | 0.8905 | 1.4647 | facet-SE ratio 0.942 | fail |
| 50×8×3 | high-g | normal | i | Satterthwaite | 0.5000 | 0.965 | 0.9174 (0.0063) | 0.0000 | 0.0479 | 0.9504 | 0.9242 | 45543194194.8911 |  | fail |
| 50×8×3 | high-g | normal | r | parametric | 0.5000 | 1.000 | 0.7500 (0.0099) | 0.2500 | 0.0000 | 0.7500 | 0.7563 | 2.0748 | facet-SE ratio 0.966 | fail |
| 50×8×3 | high-g | normal | r | Satterthwaite | 0.5000 | 0.869 | 0.8311 (0.0086) | 0.0000 | 0.0384 | 0.9558 | 0.8284 | 87234335702.7733 |  | fail |
| 50×8×3 | high-g | normal | pi | parametric | 0.5000 | 1.000 | 0.9500 (0.0050) | 0.0316 | 0.0184 | 0.9500 | 0.9516 | 0.2023 | facet-SE ratio 1.016 | pass |
| 50×8×3 | high-g | normal | pi | Satterthwaite | 0.5000 | 1.000 | 0.9526 (0.0049) | 0.0153 | 0.0321 | 0.9526 | 0.9532 | 0.2073 |  | pass |
| 50×8×3 | high-g | normal | pr | parametric | 0.5000 | 1.000 | 0.9358 (0.0056) | 0.0521 | 0.0121 | 0.9358 | 0.9342 | 0.3138 | facet-SE ratio 0.974 | fail |
| 50×8×3 | high-g | normal | pr | Satterthwaite | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0274 | 0.0284 | 0.9442 | 0.9458 | 0.3316 |  | pass |
| 50×8×3 | high-g | normal | ir | parametric | 0.5000 | 1.000 | 0.9053 (0.0067) | 0.0942 | 0.0005 | 0.9053 | 0.9053 | 0.7476 | facet-SE ratio 0.947 | fail |
| 50×8×3 | high-g | normal | ir | Satterthwaite | 0.5000 | 1.000 | 0.9442 (0.0053) | 0.0263 | 0.0295 | 0.9442 | 0.9442 | 1.0133 |  | pass |
| 50×8×3 | high-g | normal | pir | parametric | 0.5000 | 1.000 | 0.9368 (0.0056) | 0.0432 | 0.0200 | 0.9368 | 0.9379 | 0.1053 | facet-SE ratio 0.983 | fail |
| 50×8×3 | high-g | normal | pir | Satterthwaite | 0.5000 | 1.000 | 0.9411 (0.0054) | 0.0326 | 0.0263 | 0.9411 | 0.9411 | 0.1063 |  | pass |
| 50×8×3 | high-g | normal | G | F interval | 0.9412 | 1.000 | 0.9568 (0.0047) | 0.0211 | 0.0221 | 0.9568 | 0.9495 | 0.0568 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×3 | high-g | normal | Phi | parametric | 0.8889 | 1.000 | 0.9400 (0.0054) | 0.0016 | 0.0584 | 0.9400 | 0.9268 | 0.1657 |  | fail |
| 50×8×3 | high-g | normal | Phi | parametric, attached | 0.8889 | 0.838 | 0.8100 (0.0090) | 0.0016 | 0.0268 | 0.9661 | 0.8032 | 0.1769 |  | fail |
| 50×8×5 | all-one | normal | p | parametric | 1.0000 | 1.000 | 0.9384 (0.0055) | 0.0505 | 0.0111 | 0.9384 | 0.9368 | 1.0648 | facet-SE ratio 0.984 | fail |
| 50×8×5 | all-one | normal | p | Satterthwaite | 1.0000 | 1.000 | 0.9621 (0.0044) | 0.0032 | 0.0347 | 0.9621 | 0.9621 | 1.2411 |  | fail |
| 50×8×5 | all-one | normal | i | parametric | 1.0000 | 1.000 | 0.8816 (0.0074) | 0.1174 | 0.0011 | 0.8816 | 0.8811 | 2.5175 | facet-SE ratio 0.944 | fail |
| 50×8×5 | all-one | normal | i | Satterthwaite | 1.0000 | 0.985 | 0.9484 (0.0051) | 0.0000 | 0.0368 | 0.9626 | 0.9489 | 17415558482.2677 |  | fail |
| 50×8×5 | all-one | normal | r | parametric | 1.0000 | 1.000 | 0.8358 (0.0085) | 0.1642 | 0.0000 | 0.8358 | 0.8300 | 3.1035 | facet-SE ratio 0.940 | fail |
| 50×8×5 | all-one | normal | r | Satterthwaite | 1.0000 | 0.972 | 0.9289 (0.0059) | 0.0000 | 0.0432 | 0.9556 | 0.9316 | 32787639301.4892 |  | fail |
| 50×8×5 | all-one | normal | pi | parametric | 1.0000 | 1.000 | 0.9542 (0.0048) | 0.0326 | 0.0132 | 0.9542 | 0.9532 | 0.3587 | facet-SE ratio 1.002 | fail |
| 50×8×5 | all-one | normal | pi | Satterthwaite | 1.0000 | 1.000 | 0.9532 (0.0048) | 0.0221 | 0.0247 | 0.9532 | 0.9516 | 0.3665 |  | pass |
| 50×8×5 | all-one | normal | pr | parametric | 1.0000 | 1.000 | 0.9484 (0.0051) | 0.0337 | 0.0179 | 0.9484 | 0.9489 | 0.4426 | facet-SE ratio 0.977 | pass |
| 50×8×5 | all-one | normal | pr | Satterthwaite | 1.0000 | 1.000 | 0.9495 (0.0050) | 0.0200 | 0.0305 | 0.9495 | 0.9489 | 0.4559 |  | pass |
| 50×8×5 | all-one | normal | ir | parametric | 1.0000 | 1.000 | 0.9232 (0.0061) | 0.0695 | 0.0074 | 0.9232 | 0.9184 | 1.0695 | facet-SE ratio 0.953 | fail |
| 50×8×5 | all-one | normal | ir | Satterthwaite | 1.0000 | 1.000 | 0.9442 (0.0053) | 0.0237 | 0.0321 | 0.9442 | 0.9437 | 1.2454 |  | pass |
| 50×8×5 | all-one | normal | pir | parametric | 1.0000 | 1.000 | 0.9500 (0.0050) | 0.0337 | 0.0163 | 0.9500 | 0.9511 | 0.1488 | facet-SE ratio 0.966 | pass |
| 50×8×5 | all-one | normal | pir | Satterthwaite | 1.0000 | 1.000 | 0.9511 (0.0049) | 0.0284 | 0.0205 | 0.9511 | 0.9511 | 0.1499 |  | pass |
| 50×8×5 | all-one | normal | G | F interval | 0.7407 | 1.000 | 0.9511 (0.0049) | 0.0232 | 0.0258 | 0.9511 | 0.9421 | 0.2286 | unfit 0, errors 0, failed fits 0 | pass |
| 50×8×5 | all-one | normal | Phi | parametric | 0.5882 | 1.000 | 0.9600 (0.0045) | 0.0189 | 0.0211 | 0.9600 | 0.9611 | 0.3477 |  | pass |
| 50×8×5 | all-one | normal | Phi | parametric, attached | 0.5882 | 0.958 | 0.9221 (0.0061) | 0.0189 | 0.0168 | 0.9626 | 0.9237 | 0.3490 |  | fail |
