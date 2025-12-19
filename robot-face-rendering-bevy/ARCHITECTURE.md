# Architecture Documentation

## Project Structure

The Robot Face Rendering project follows clean architecture principles with clear separation of concerns:

```
src/
├── main.rs              # Application entry point and module orchestration
├── components.rs        # Pure data structures (ECS Components)
├── resources.rs         # Global state (ECS Resources)
├── constants.rs         # Configuration constants
├── utils.rs             # Helper functions (Bezier mesh generation)
└── systems/             # Game logic (ECS Systems)
    ├── mod.rs           # System module exports
    ├── setup.rs         # Startup systems (scene initialization)
    ├── animation.rs     # Animation systems (blink, emotion, pupil)
    ├── input.rs         # Input handling (keyboard)
    └── ui.rs            # UI updates
```

## Design Patterns

### 1. **Entity Component System (ECS)**
Bevy's core pattern that separates:
- **Entities**: Unique IDs (implicit)
- **Components**: Pure data (in `components.rs`)
- **Systems**: Logic that operates on components (in `systems/`)

### 2. **Module Organization**
Each module has a single, clear responsibility:

#### `components.rs` - Data Only
```rust
pub struct Pupil {
    pub side: Eye,
    pub look_offset: Vec2,
}
```
✅ No methods, just data
✅ Public fields for ECS access
✅ Clear documentation

#### `resources.rs` - Global State
```rust
#[derive(Resource)]
pub struct Emotion {
    pub happiness: f32,
}
```
✅ Singleton pattern via Bevy's Resource system
✅ Default implementations for initialization

#### `systems/` - Pure Logic
```rust
pub fn blink_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut Blinking, &mut Transform)>,
) {
    // Logic here
}
```
✅ No state, only parameters
✅ Operates on queries
✅ Side-effect free (except mutations)

### 3. **Constants Centralization**
All magic numbers live in `constants.rs`:
```rust
pub const EYE_RADIUS: f32 = 60.0;
pub const MOUTH_WIDTH: f32 = 280.0;
```
✅ Easy to tweak behavior
✅ Self-documenting
✅ Type-safe

### 4. **Utility Functions**
Complex operations extracted to `utils.rs`:
```rust
pub fn create_bezier_curve_mesh(start: Vec2, control: Vec2, end: Vec2) -> Mesh
```
✅ Pure functions
✅ Reusable
✅ Well-documented

## System Flow

### Startup Phase
```
main()
  ↓
setup_camera()      → Spawns Camera2d
  ↓
setup_robot_face()  → Creates face entities (eyes, mouth)
  ↓
setup_ui()          → Creates UI text
```

### Update Loop (Every Frame)
```
keyboard_input_system      → Read input, update emotion/trigger actions
  ↓
auto_blink_system          → Timer-based blink triggers
  ↓
blink_animation_system     → Animate eye closing/opening
  ↓
emotion_update_system      → Update mouth mesh based on emotion
  ↓
pupil_look_system          → Smooth pupil movement
  ↓
update_ui_system           → Update UI text with current state
```

## Key Design Decisions

### 1. **Why Separate Systems Directory?**
- **Clarity**: Each system type has its own file
- **Maintainability**: Easy to find and modify specific behaviors
- **Testing**: Can test systems in isolation
- **Growth**: Easy to add new systems without cluttering main.rs

### 2. **Why Public Fields in Components?**
Bevy's ECS requires direct field access for queries:
```rust
Query<(&Pupil, &mut Transform)>  // Needs pub fields
```

### 3. **Why Constants Instead of Config File?**
- **Compile-time**: No runtime parsing overhead
- **Type-safe**: Rust compiler catches errors
- **Simple**: For this use case, constants are sufficient
- **Future**: Can migrate to config files later if needed

### 4. **Why Inline Eye Spawning?**
Due to Rust's type system and Bevy's command builders, helper functions for entity spawning introduce complex lifetime issues. Inlining is more explicit and compiler-friendly.

## Adding New Features

### New Component
1. Define in `components.rs`:
   ```rust
   #[derive(Component)]
   pub struct Eyebrow {
       pub angle: f32,
   }
   ```
2. Use in systems

### New System
1. Create in appropriate `systems/*.rs`:
   ```rust
   pub fn eyebrow_system(query: Query<&mut Eyebrow>) {
       // Logic
   }
   ```
2. Export in `systems/mod.rs`
3. Add to `main.rs` app builder

### New Resource
1. Define in `resources.rs`:
   ```rust
   #[derive(Resource)]
   pub struct FaceColor(pub Color);
   ```
2. Initialize in `main.rs`

## Best Practices

### ✅ Do's
- Keep components as simple data structures
- Use queries to access components
- Document public interfaces
- Use constants for configuration
- Organize systems by functionality
- Use change detection (`Res<T>::is_changed()`) for optimization

### ❌ Don'ts
- Put logic in components
- Use global mutable state (use Resources instead)
- Hardcode magic numbers
- Create circular dependencies between modules
- Mix setup and update logic

## Performance Considerations

### Query Optimization
```rust
// Good: Specific query
Query<(&Pupil, &mut Transform), With<Eye>>

// Bad: Overly broad
Query<(&mut Transform)>  // Matches everything!
```

### Change Detection
```rust
// Good: Only run when needed
if emotion.is_changed() {
    // Update mouth
}

// Bad: Every frame
// Update mouth unconditionally
```

### System Ordering
Systems in a tuple run in parallel when possible:
```rust
.add_systems(Update, (
    input,      // Can run in parallel
    animation,  // with these
    physics,    // systems
))
```

## Module Dependencies

```
main.rs
  ├─→ components.rs (no dependencies)
  ├─→ resources.rs (no dependencies)
  ├─→ constants.rs (no dependencies)
  ├─→ utils.rs → constants
  └─→ systems/
       ├─→ setup.rs → components, constants, utils
       ├─→ animation.rs → components, resources, constants, utils
       ├─→ input.rs → components, resources
       └─→ ui.rs → components, resources
```

✅ Clean dependency graph
✅ No circular dependencies
✅ Easy to reason about

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier_curve() {
        // Test utils
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
#[test]
fn test_emotion_system() {
    // Test system behavior
}
```

## Future Enhancements

1. **Plugin Architecture**: Extract robot face into a Bevy plugin
2. **Component Traits**: Add behavior traits for components
3. **Event System**: Use Bevy events for communication
4. **Asset Loading**: Load configurations from files
5. **State Machine**: Formal state management for emotions

---

**Last Updated**: 2025-12-19
**Bevy Version**: 0.17.3
**Architecture**: ECS (Entity Component System)
