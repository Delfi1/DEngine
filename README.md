<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.20

Dependencies:
1) [Rust](https://www.rust-lang.org/tools/install)
2) [Ninja Build](https://ninja-build.org)

Controls:
WASD - movement;
F11 - Full Screen;
M - Maximize window;
LShift + Esc - exit;

Current Engine structure (Graph):
```mermaid
graph TD;
    Eng[Engine];
    Eng --> App{Application};
    App --> Settings;
    App --> MainLoop[Main Loop];
    App --> Context;
    
    MainLoop --> ...;
```

How Main Loop works:
```mermaid
graph TD;
    MainLoop[Main Loop];
    MainLoop --> Start;
    Start --> Renderer;
    Renderer --> Window;
    Window --> MatchInput[Match Input];
    MatchInput --> UpdateSurface[Update Surface];
    UpdateSurface --> Update["Update Window,\n World, Context"];
    Update --> DrawFrame[Start Drawing Frame];
    DrawFrame --> End;
    DrawFrame --> ...;
    End --> Start;
```

Context System:
```mermaid
graph TD;
    Context;
    Context --> Time;
    Context --> Graphics;
    Context --> Keyboard;
```

How Render Loop works (Graph):
```mermaid
graph TD;
    Frame[Starting Drawing Frame];
    Frame --> OD[Collect objects data];
    OD --> EP[Engine Pipeline];
    EP --> PP["Physics Pipeline (soon)"];
    PP --> DP[Draw Pipeline];
    DP --> Future;
    Future --> End;
    End --> Frame;
```

TODO:
1. [x] Code refactoring;
2. [x] Create Context system;
3. [x] Create simple *Render loop*;
4. [ ] Create world update and draw logic;