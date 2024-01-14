<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.13

Current Engine structure:

```mermaid
graph TD;
    Eng[Engine];
    Eng --> Wor[World];
    Eng --> Set[Settings];
    Eng --> Win[Window, Surface];
    Wor --> Wor_Name[Name];
    Wor --> Objs[Objects];
    
    Objs --> Cb[Cube]
    Objs --> |Examples| Sph[Sphere]
    Objs --> Another[...]

    Cb --> Data[Position, \nRotation, \nScale, \n... ]
    Sph --> |Object Data| Data
    Another --> Data
    
    Data --> |Object Callbacks| Callbacks[on_draw, \non_update, \n...]

```

TODO:
1. [x] Engine, World struct;
2. [x] Simple Object trait;
3. [x] Window drawing;
4. [ ] Create base drawing logic;
5. [ ] Create base physics logic;