## Sliding pieces, Perft(3)

### Hyperbola Quintessence

For generating the movements of sliding pieces (bishops, rooks, queens) there [are really many different techniques](https://www.chessprogramming.org/Sliding_Piece_Attacks). Using [the transcript of a talk on efficient generation](https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks) of sliding piece attacks, I chose to go with an approach by calculation, using the Hyperbola Quintessence approach. Another very nice and popular approach is Magic Bitboards ([good and clear explanation](https://stackoverflow.com/questions/16925204/sliding-move-generation-using-magic-bitboard)).

Hyperbola Quintessence does bit operations, for which modern processors should have dedicated instructions for all. Diagonals and file attacks work the same and it’s quite logical. For ranks (lines) however the o^(o-2r) trick doesn’t work and I must confess I don’t really understand why. But [the wiki offers a clear alternative](https://www.chessprogramming.org/First_Rank_Attacks#Attacks_on_all_Ranks) based on precalculated masks.

So at this state I have basic move generation for all pieces, but the “advanced” stuff is missing:

- En passant  
- Castling  
- Promotion  
- In check and pinning

### Make, unmake

Once we have the list of moves, we need to apply them to the board. With bit boards, there is a fast and easy method for make, [update by move](https://www.chessprogramming.org/General_Setwise_Operations#UpdateByMove).

I didn’t bother with unmake for now. While it seems to help with performance, it’s not trivial to do and I really don’t need it for now. I’m just copying the full board when needed.

### Perft

With basic move generation working, I could implement a perft function and run it on the initial position for depth of up to 3, since at that depth, there cannot be en passant, etc.

    Perft results for depth 3: 8902 nodes.

Cool milestone!