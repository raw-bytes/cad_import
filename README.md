# cad_import - Simple CAD Importer Library
A simple library for importing CAD data from different formats into a uniform in-memory structure.

The goals of this library is:
- Supporting multiple 3D and CAD formats
- Simple design and supporting
- Representation in uniform in-memory structure

## Supported formats
- Object File Format: Extensions=*.off, Mime-Types=model/vnd.off (see https://segeval.cs.princeton.edu/public/off_format.html)
- glTF RUNTIME 3D ASSET DELIVERY: Extensions=\*.gltf,\*.glb, Mime-Types=model/gltf-binary,model/gltf+json (see https://www.khronos.org/gltf/)
- AVEVA PDMS binary RVM: Extensions=\*.rvm, Mime-Types=application/vnd.aveva.pdm.rvm (see https://en.wikipedia.org/wiki/PDMS_(software))

## Changelog
For changes see [Change Log](./CHANGELOG.md)