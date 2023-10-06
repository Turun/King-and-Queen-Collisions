Shuffle a deck of cards. 
- What is the probability of any king and queen being next to each other in the shuffled deck?
- What is the probability of any king and queen being at most one card apart?

from https://www.youtube.com/watch?v=sf5OrthVRPA

This code simulates this test. The results are 48.62% to the first question and 73.597% to the second question. Both a brute force check through every combination (`simulate.rs`) and a monte carlo type simulation (`monte_carlo.rs`) are implemented.

The monte carlo one simply simulates a deck of cards, shuffles it and checks for the two conditions over and over again. The brute force way indeed checks every possible state of the deck. This can be made fast enough (3min on my machine), by representing the position of queens and kings as 1s in an integer that is at least 52 bits long. We can then do all checks with integers, which is really fast on a modern computer. The gist is the following:

Create every possible integer with 52 bits with exactly 4 "1s" in it representing the kings. Then generate every possible integer with exactly 4 "1s" in it, representing the queen positions. 
For every combination thus generated, throw out those that have a one in the same place (1), as this would be a card deck with king and queen in the same position, which is obviously impossible.
For the remaining combinations we can check every single one for their neighborhood status. (2)

(1) this is fast, because the check boils down to `if (kings & queens != 0){/*throw it out*/}`

(2) Similarly, this can be written as `if (kings < 1 & queens != 0) || (kings > 1 & queens != 0)` for the direct neighbor check and `if (kings < 2 & queens != 0) || (kings > 2 & queens != 0)` for the neighborhood check with one spacing.


