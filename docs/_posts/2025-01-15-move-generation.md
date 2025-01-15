## Move generation in progress

After a few days of development, some things are starting to take shape.

I have set up the project, the board, and various debugging functions. I also already added FEN parsing support. This allows me to easily create all kinds of boards for the tests.

I started implementing move generation. So far I have king (without castling) and knights. Generating the attack bitboards isn't very hard so far. More challenging is how to write the `generate_moves()` function. It has a complex logic, but it also needs to be fast and of course we want to have it readable and very solid.

I also have an *update by move* function, which updates the board for a move. This is quite elegant to do with bit boards.

Speaking about moves, how to represent them required some research. For now, I'm not storing them in a [compact version](https://www.chessprogramming.org/Encoding_Moves) (16 or 32 bits), but I might add this later if memory becomes a concern.

### Performance and avoiding early optimization

For a Chess Engine, performance is important, as it is one of the ways to make the engine stronger. The risk there is to fall into the trap of worrying too much about it at the beginning and getting distracted by optimizations that might even not matter or be used in the final solution.

Additionally, since the engine will have complicated logic, it's as important to have a good architecture and design as well as clean code.  
