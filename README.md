<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.16

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
    Frame;
    Frame --> Start;
    Start --> OD["Collect objects data"]
    OD --> Pipeline;
    Pipeline --> Future;
    Future --> End;
    End --> Frame
```

TODO:
1. [x] Code refactoring;
2. [ ] Create *Render loop*;