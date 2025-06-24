# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bevy 3D object viewer application that allows users to interact with 3D objects through both mouse controls and a UI panel. The application uses Bevy 0.15.0 with bevy_egui for the user interface.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run the application
cargo run

# Check compilation without building
cargo check

# Clean build artifacts
cargo clean
```

## Architecture

The application follows Bevy's Entity Component System (ECS) architecture:

### Key Resources
- `ObjectType`: Tracks the currently selected object type (Cube, Sphere, etc.)
- `MouseControl`: Manages mouse interaction state for object manipulation

### Key Components
- `SelectedObject`: Marks entities that can be manipulated by user input
- `ObjectRotation`: Contains rotation parameters (currently unused but reserved for future features)

### System Organization
The application runs four main systems in the Update schedule:
- `mouse_control_system`: Handles mouse drag rotation and scroll wheel scaling
- `keyboard_control_system`: Processes WASD/arrow key movement and Q/E depth control
- `ui_system`: Manages the egui UI panel with object selection dropdown and control buttons
- `object_selection_system`: Placeholder for future object selection logic

### Object Management
Objects are spawned through `spawn_object()` function which:
- Creates appropriate mesh based on object type (Cube, Sphere, Cylinder, Torus, Cone)
- Applies StandardMaterial with beige/tan coloring
- Attaches SelectedObject and ObjectRotation components

## Bevy Configuration

The project uses selective Bevy features to avoid audio dependencies:
- Excludes default audio features to prevent ALSA compilation issues on Linux
- Includes only necessary rendering, UI, and windowing features
- Uses X11 for Linux window management

## UI Controls

The application provides multiple interaction methods:
- Mouse: Click-drag for rotation, scroll wheel for scaling  
- Keyboard: WASD/arrows for XY movement, Q/E for Z-axis movement
- UI Panel: Buttons for precise movement, rotation, and scaling controls
- Dropdown: Object type selection with immediate switching

## Development Notes

- Object switching immediately despawns the old object and spawns a new one
- All objects with SelectedObject component move together during transformations
- The UI system handles both object selection and real-time manipulation
- Transform operations are applied directly to entity transforms without interpolation