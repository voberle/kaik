## Perft tests passing

It seems I have a working move generation already\!

### Perft debugging

After I got all the features implemented and got a setup for easy perft debugging, I used this [list of perft test cases](https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9) to debug my code. For each case that didn’t pass, I isolated the buggy position, wrote a dedicated Rust test for it and fixed the bug.

After 6 bug fixes, all those cases passed. I then checked all [interesting positions from the wiki](https://www.chessprogramming.org/Perft_Results), they all pass as well (of course I didn’t try the biggest depths on some).

I also tested all cases from the [Ethereal file](https://github.com/AndyGrant/Ethereal/blob/master/src/perft/standard.epd).