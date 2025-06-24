Visio Vibe 1 - 3D Object Viewer
================================

A Bevy-based 3D object viewer with interactive controls.

FEATURES:
- View different 3D objects (Cube, Sphere, Cylinder, Torus, Cone)
- Each object has distinct colors and materials
- Mouse controls: Click and drag to rotate, scroll to scale
- Keyboard controls: WASD/Arrow keys to move, Q/E for depth
- UI panel with buttons for precise movement and rotation
- Ground plane with shadow casting
- Real-time lighting with PBR materials

CONTROLS:
- Mouse: Click and drag to rotate object, scroll wheel to scale
- Keyboard: WASD or Arrow keys for XY movement, Q/E for Z movement
- UI Panel: Use buttons for precise control and object selection
- Dropdown: Select different 3D object types

SYSTEM REQUIREMENTS:
- Linux x86_64
- OpenGL 3.3+ or Vulkan support
- Graphics drivers supporting PBR rendering

To run: ./visio-vibe1

For Windows users: You'll need to build from source using Rust and Cargo.
Source code available at the project repository.

Built with Bevy 0.15.0 game engine.