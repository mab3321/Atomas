# Milestone 1: Decision → Screen Action Mapping

## Overview

Translates high-level game decisions (INSERT/REMOVE) into concrete screen tap coordinates with visual verification overlays.

## What Changed

### New Modules
- **`crates/atomas-cv/src/action.rs`** - Decision enum and coordinate mapping logic
  - `Decision` enum: `Insert { gap_index }` and `Remove { atom_index }`
  - `map_decision_to_coordinates()` - Maps decisions to pixel coordinates
  - `calculate_gap_coordinates()` - Calculates INSERT gap midpoints
  - `get_atom_coordinates()` - Retrieves REMOVE atom centers

- **`crates/atomas-cv/src/overlay.rs`** - Visual overlay rendering
  - `draw_action_overlay()` - Main overlay drawing function
  - Yellow circle markers for INSERT actions
  - Red X markers for REMOVE actions
  - Text labels with backgrounds

- **`src/milestone1_test.rs`** - Test binary demonstrating Milestone 1
  - Detects game state from screenshot
  - Tests INSERT and REMOVE decisions
  - Generates annotated overlay images

### Modified Files
- **`Cargo.toml`** - Added milestone1 binary and anyhow dependency
- **`crates/atomas-cv/src/lib.rs`** - Exported new action and overlay modules

### Prototypes
- **`milestone1_overlay_pil.py`** - Working Python prototype using PIL
- **`milestone1_overlay.py`** - Alternative OpenCV-based Python prototype

### Output Images
- **`assets/png/outputs/action_overlay_insert.png`** - INSERT example
- **`assets/png/outputs/action_overlay_remove.png`** - REMOVE example
- **`assets/png/outputs/action_overlay_insert_gap8.png`** - Wrap-around test

---

## How to Run

### Python Prototype (Working)

```bash
# Run the prototype
python milestone1_overlay_pil.py

# View outputs
start assets\png\outputs
```

**Requirements:**
- Python 3.x
- PIL/Pillow (usually pre-installed)

### Rust Implementation (When Environment Ready)

```bash
# Build
cargo build --bin milestone1

# Run
cargo run --bin milestone1
```

**Requirements:**
- Rust toolchain (rustup)
- OpenCV libraries (handled by opencv crate)

---

## Output Image Paths

All outputs saved to: `assets/png/outputs/`

1. **`action_overlay_insert.png`** - Yellow circle at INSERT gap
2. **`action_overlay_remove.png`** - Red X at REMOVE atom
3. **`action_overlay_insert_gap8.png`** - Wrap-around gap test

---

## Coordinate Logic

### INSERT - Gap Coordinate

**Goal:** Find the midpoint between two adjacent ring atoms

**Algorithm:**
```rust
fn calculate_gap_coordinates(ring_elements: &[(Element, BBox)], gap_index: usize) 
    -> ActionCoordinates 
{
    let n = ring_elements.len();
    let atom1_idx = gap_index % n;
    let atom2_idx = (gap_index + 1) % n;  // Handles wrap-around
    
    let center1 = ring_elements[atom1_idx].1.center();
    let center2 = ring_elements[atom2_idx].1.center();
    
    ActionCoordinates::new(
        (center1.x + center2.x) / 2,
        (center1.y + center2.y) / 2
    )
}
```

**Example:**
- Gap 3 between atoms [3] and [4]
- Atom 3 center: (380, 450)
- Atom 4 center: (340, 540)
- **Result: (360, 495)**

**Handles wrap-around:**
- Gap N-1 connects atom[N-1] to atom[0]

### REMOVE - Atom Center Coordinate

**Goal:** Get the center of the target atom

**Algorithm:**
```rust
fn get_atom_coordinates(ring_elements: &[(Element, BBox)], atom_index: usize) 
    -> ActionCoordinates 
{
    let center = ring_elements[atom_index].1.center();
    ActionCoordinates::new(center.x, center.y)
}
```

**Example:**
- Atom 5 (Nitrogen)
- BBox center: (280, 595)
- **Result: (280, 595)**

---

## Visual Verification

### INSERT Overlay (Yellow Circle)
- Circle radius: 30 pixels
- Line thickness: 4 pixels
- Color: Yellow (RGB 255, 255, 0)
- Label: "INSERT HERE"

### REMOVE Overlay (Red X)
- X size: 25 pixels
- Line thickness: 4 pixels
- Color: Red (RGB 255, 0, 0)
- Label: "REMOVE"

Both overlays include text labels with black backgrounds for visibility.

---

## Tested Coordinates

| Decision | Index | Coordinates | Target |
|----------|-------|-------------|--------|
| INSERT | Gap 3 | (360, 495) | Between Li and B atoms |
| REMOVE | Atom 5 | (280, 595) | Nitrogen (N) center |
| INSERT | Gap 8 | (85, 405) | Left side wrap-around |

**Verification:**
- ✅ Images have different MD5 hashes
- ✅ Visual markers appear at correct locations
- ✅ INSERT (yellow) and REMOVE (red) are clearly distinct

---

## Out of Scope

**NOT included in Milestone 1:**
- Automation loop (Milestone 2)
- ADB tap integration
- Unattended play loop
- Expectimax solver (Milestone 3)
- Game mechanics port
- CUDA/GPU acceleration
- Deep RL training
- Multi-emulator setup

**Milestone 1 focus:**
- Decision → coordinate mapping only
- Visual verification overlays
- Correct tap location calculation

---

## Integration Points

### For Milestone 2 (Automation):
```rust
use atomas_cv::{Decision, map_decision_to_coordinates};

let coords = map_decision_to_coordinates(&decision, &detection_result)?;
adb_tap(coords.x, coords.y);  // Future ADB integration
```

### For Milestone 3 (Solver):
```rust
let decision = solver.get_best_move(&game_state);
let coords = map_decision_to_coordinates(&decision, &detection_result)?;
execute_move(coords);
```

---

## Data Structures

### Decision Enum
```rust
pub enum Decision {
    Insert { gap_index: usize },
    Remove { atom_index: usize },
}
```

### Action Coordinates
```rust
pub struct ActionCoordinates {
    pub x: i32,
    pub y: i32,
}
```

### Detection Result (Existing)
```rust
pub struct DetectionResult<'a> {
    pub ring_elements: Vec<(Element<'a>, BBox)>,
    pub player_atom: Option<(Element<'a>, BBox)>,
    // ...
}
```

---

## Assumptions

1. Ring elements are sorted clockwise (verified in detector)
2. At least 2 atoms exist for INSERT operations
3. `atom_index < ring_elements.len()` for REMOVE
4. Detection succeeds before coordinate mapping
5. Coordinates scale 1:1 with screenshot resolution

---

## Limitations

1. Prototype uses mock atom positions (real integration needs DetectionResult)
2. No overlap detection with UI elements
3. Fixed visualization style (colors hardcoded)
4. No screen orientation/resolution handling
5. Rust build requires OpenCV system libraries

---

## Testing

Run the Python prototype to verify:
```bash
python milestone1_overlay_pil.py
```

Expected output:
```
============================================================
MILESTONE 1: Decision to Screen Action Mapping
============================================================

INSERT at gap_index=3 → coordinates: (360, 495)
REMOVE at atom_index=5 → coordinates: (280, 595)

✓ Output saved: assets/png/outputs/action_overlay_insert.png
✓ Output saved: assets/png/outputs/action_overlay_remove.png
```

---

## Next Steps

**After Milestone 1 merge:**
1. Set up Rust build environment (if needed)
2. Integrate with real DetectionResult data
3. Begin Milestone 2: Automation loop with ADB

**NOT to be done yet:**
- Solver integration
- Game mechanics
- RL training
- CUDA optimization
