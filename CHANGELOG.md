# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unpublished]

### Added
- Support for length units (see https://github.com/raw-bytes/cad_import/issues/13)
- Added new metadata concept (see https://github.com/raw-bytes/cad_import/issues/11)

### Changed
- Assembly structure is now represented as arena tree based on node ids. This allows to reference nodes in the assembly structure by their id. However, this is a breaking change in the API!!!.

## [0.3.1]

### Changed
- Updated nalgebra-glm to recent version

## [0.3.0]

### Added
- New resource interface for accessing resources
- Support for non-indexed primitives
- Added export functionality to the X3D format
- Added example program to convert data to X3D
- Added GLTF loader

### Changed
- Changed the resource extension handling

## [0.2.1] - 2023-02-22

### Changed
- Fixed changelog

## [0.2.0] - 2023-02-22

### Added
- Loader for the simple OFF (Object File Format) format 
- More documentation and introduction examples
- Example programs to demonstrate the usage of the library

### Changed
- Extended the in-memory structure
- Restructured the imports to avoid deep import paths

## [0.1.0] - 2023-01-28

### Added

- Added CI/CD pipeline integration
- Added first simple version of in-memory structure
- Simple architecture to register and search different loaders