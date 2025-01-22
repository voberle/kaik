## Special moves, legal moves

I continued the move generation development.

### Special moves: En-passant, castling, promotions

For en passant, I store the en passant square in the board whenever a pawn is double pushed. Based on this, it’s just a matter of generating extra moves when this field is set.

With castling I used the approach of a 4 bits field to track castling abilities.

### Check

To handle the case of the king being in check there are two approaches.

One would be to do it in the moves generation. While this might be the better solution in terms of performance, it’s not so easy (here is a good overview of [how it can be done with bit boards](https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/)).

So for now I went with the easier solution of detecting if the king is in check after updating the board. If the king is in check, I just cancel the update. For the check detection, I use the [method described in the wiki](https://www.chessprogramming.org/Checks_and_Pinned_Pieces_\(Bitboards\)), which imagines other pieces being at the king’s position and checks which squares are attacked.

### Perft progress

With those features, I could go further on perft testing, up to depth 5 from initial position.

I also developed a small perft\_cmp.sh script that makes it easy to compare my perft results with the Stockfish ones. This allowed me to isolate a few problematic positions and fix some bugs.

### Removing the BitBoard struct.

Initially I had decided to wrap the u64 of the BitBoard in a struct

```rust
pub struct BitBoard(u64);
```

In practice however this proved annoying to use for limited benefits. I had to overload all the bit manipulation operators, and even then some constructions looked awkward, such as doing an AND on two BitBoard and checking if the result was null.

So I went back to a simple type definition.

```rust
pub type BitBoard = u64;
```

NB: Performance wise I believe the struct would have been fine, since Rust aims for zero-cost abstractions, meaning that high-level features should compile down to efficient code without runtime overhead. Additionally the compiler should see that BitBoard has the same memory layout as a u64, so it can eliminate any unnecessary indirection.  
