# Architecture

To handle the communication with the user interface, Kaik uses several threads that communicate over channels.

- UI Handler: Reads the UCI text commands from standard input and converts them into an `UciCommand` message.
- UI Event Handler: Handles `UciEvent` messages by converting them to a string printed to the standard output.
- Command Handler: Processes the `UciCommand` and calls the engine. If the call to the engine is synchronous (i.e. fast) it may send a response to the UI by sending a `UciEvent` message. This runs actually in the main thread and holds a reference onto the game.
- Game Event Handler: The engine and search may send `GameEvent` messages (best move, info), which are processed by the Game Event Handler. This handler converts them to `UciEvent`.

```mermaid
flowchart TD
    ui[/"User Interface"/]
    ui_handler(["UI Handler"])
    ui_event_handler(["UI Event Handler"])
    cmd_handler(["Command Handler"])
    game_event_handler(["Game Event Handler"])
    engine["Game Engine"]
    search["Search"]

    ui-- stdin -->ui_handler
    ui_event_handler-- stdout -->ui
    ui_handler== UciCommand ==>cmd_handler
    cmd_handler== UciEvent ==>ui_event_handler

    cmd_handler<-->engine
    engine-- creates thread -->search
    game_event_handler== UciEvent ==>ui_event_handler
    search== GameEvent ==>game_event_handler
```
