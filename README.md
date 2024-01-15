<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.15

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
graph LR;
    Frame_S[Frame Start] --> WE[Window Events] --> Update[Update \nWorld]; 
    Update --> Draw[Draw World\n objects] --> Wait[Waiting for\n next frame] --> Frame_E[Frame End] --> Frame_S;
```

TODO:
1. [x] Engine, World struct;
2. [x] Simple Object trait;
3. [x] Window drawing;
4. [ ] Code refactoring;