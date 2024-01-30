<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.17

Dependencies:
1) [Rust](https://www.rust-lang.org/tools/install)
2) [Ninja Build](https://ninja-build.org)

Current Engine structure (Graph):
```mermaid
graph TD;
    Eng[Engine];
    Eng --> Wor[World];
    Eng --> Set[Settings];
    Eng --> Win[Window, Surface];
    Wor --> Wor_Name[Name];
    Wor --> Objs{Objects};
    
    Objs --> Cb[Cube];
    Objs --> |Examples| Sph[Sphere];
    Objs --> Another[...];

    Cb --> Data[Position,\n Rotation,\n Scale,\n ... ];
    Sph --> |Object Data| Data;
    Another --> Data;
    
    Data --> |Object Callbacks| Callbacks[on_draw,\n on_update,\n ...];
```

How Render Loop works (Graph):
```mermaid
graph TD;
    Frame[Start Frame];
    Frame --> OD[Collect objects data]
    OD --> EP[Engine Pipeline];
    EP --> DP[Draw Pipeline];
    DP --> End;
    End --> Frame
```

TODO:
1. [x] Code refactoring;
2. [ ] Create Context system;
3. [ ] Create *Render loop*;