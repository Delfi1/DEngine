<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.19

Dependencies:
1) [Rust](https://www.rust-lang.org/tools/install)
2) [Ninja Build](https://ninja-build.org)

Current Engine structure (Graph):
```mermaid
graph TD;
    Eng[Engine];
    Eng --> App{Application};
    App --> Settings;
    App --> MainLoop[Main Loop];
    App --> Context;
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
    UpdateSurface --> Update[Update Window\n World];
    Update --> DrawFrame[Start Drawing Frame];
    DrawFrame --> End;
    End --> Start;
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