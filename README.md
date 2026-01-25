# Avoidance Steering - Bevy Game Project

A Bevy game engine example demonstrating autonomous agent movement with avoidance steering behavior. This project showcases entities that navigate toward targets while avoiding obstacles and each other in real-time.

## Features

- **Goal-Seeking Behavior**: Entities autonomously move toward designated target positions
- **Obstacle Avoidance**: Agents detect and avoid nearby obstacles using collision detection
- **Wall Collision**: Entities bounce off screen boundaries to stay within play area
- **Visual Debugging**: Gizmo rendering for visualizing agent detection radius and behavior
- **Smooth Movement**: Acceleration and deceleration systems for natural motion
- **Multi-Agent Simulation**: Multiple entities with independent steering behaviors

## Project Structure

```
src/bin/avoidance_steering/
├── main.rs           # Application entry point and setup
├── components.rs     # Entity components (Mover, Target, Obstacle)
├── systems.rs        # System exports and module organization
└── systems/          # Individual system implementations
    ├── movement.rs         # Velocity and acceleration updates
    ├── avoidance.rs        # Obstacle avoidance logic
    ├── target_reached.rs   # Target arrival detection
    └── wall_collision.rs   # Boundary collision handling
```

## Components

### Mover
Represents an autonomous agent that can move and seek targets.
- `acceleration`: Current acceleration vector
- `velocity`: Current velocity vector
- `target`: Optional reference to target entity

### Target
Marks an entity as a navigation goal for movers.

### Obstacle
Defines collision avoidance properties for an entity.
- `radius`: Detection/collision radius for avoidance

## Key Parameters

```rust
ENTITY_SPEED: 30.0                           // Maximum movement speed
ENTITY_ACCELERATION_TIME_TOWARD_TARGET: 0.2 // Acceleration duration
ENTITY_DESELERATION_TIME_AFTER_TARGET_REACHED: 0.1 // Deceleration duration
AVOID_FORCE: 200.0                           // Avoidance force magnitude
AVOID_RADIUS: 12.0                           // Detection radius for obstacles
TARGET_REACHED_DISTANCE: 50.0                // Distance threshold for goal arrival
WALL_PADDING: 20.0                           // Boundary collision padding
```

## Technical Details

### Movement System
Entities accelerate toward their target using a time-based acceleration curve, providing smooth, natural-looking movement rather than instant velocity changes.

### Avoidance System
Uses circular detection radius around each entity to identify nearby obstacles. When obstacles are detected, a repulsive force is applied perpendicular to movement, causing agents to steer away.

### Target System
When an agent reaches within `TARGET_REACHED_DISTANCE` of its target, it begins decelerating.

### Wall Collision
Screen boundaries are enforced with padding to keep entities visible and prevent jittering at edges.
