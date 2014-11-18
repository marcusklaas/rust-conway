rust-conway
===========

This is a concurrent implementation of conway's game of life in rust. The game is played on fixed sized boards with hard borders. The evaluation of the next generation can be done concurrently. Adding a second thread reduces execution time by roughly 35% compared to completely sequential operation. Please note that this is not a high performance project. No clever tricks were used to speed up evaluation other than concurrency.
