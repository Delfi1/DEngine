<h1 align="center">Delfi Engine</h1>

Simple physics engine. Current version: v0.0.11

Current Engine structure:

flowchart TD
    Eng[Engine]
    Eng --> Wor[World]
    Eng --> Set[Settings]
    Eng --> Win[Window; Surface]
    Wor --> Wor_Name[Name]
    Wor --> Objs[Objects]

TODO:
1. [x] Engine, World struct;
2. [x] Simple Object trait;
3. [x] Window drawing;
4. [x] 