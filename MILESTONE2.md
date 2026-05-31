# Milestone 2: Automation Loop

## Overview

Implements the automation loop for playing Atomas:
**Capture → Detect → Decide → Execute → Repeat**

**Phase 1 (Current):** Dry-run mode using sample screenshots  
**Phase 2 (Future):** ADB integration for real device control

---

## Phase 1: Dry-Run Mode

### Features

- ✅ Load screenshots from disk (no emulator required)
- ✅ Detect game state using existing CV (Milestone 1)
- ✅ Simple placeholder decision logic (NOT expectimax solver)
- ✅ Convert decisions to tap coordinates
- ✅ Mock executor prints ADB commands without executing
- ✅ Configurable number of moves
- ✅ Detailed logging for every step
- ✅ Graceful error handling

### NOT Included (Milestone 3)

- ❌ Expectimax solver
- ❌ Game mechanics simulation
- ❌ Game tree search
- ❌ CUDA/GPU acceleration
- ❌ Advanced AI strategies

---

## How to Run

### Basic Usage (Dry-Run Mode)

```bash
# Run with default settings (10 moves, prefer-insert strategy)
cargo run --bin milestone2 -- --dry-run

# Run with custom screenshot
cargo run --bin milestone2 -- \
    --dry-run \
    --screenshot assets/jpg/board.jpg \
    --moves 10

# Try different strategies
cargo run --bin milestone2 -- \
    --dry-run \
    --strategy random \
    --moves 5

# Verbose logging
cargo run --bin milestone2 -- \
    --dry-run \
    --moves 3 \
    --verbose
```

### Command-Line Options

```
Options:
  --dry-run              Run in dry-run mode (no ADB required) [default: true]
  --screenshot <PATH>    Path to screenshot [default: assets/jpg/board.jpg]
  --moves <N>            Number of moves to execute [default: 10]
  --strategy <STRATEGY>  Decision strategy [default: prefer-insert]
                         Valid: random, prefer-insert, prefer-remove
  -v, --verbose          Enable verbose logging
  -h, --help             Print help
  -V, --version          Print version
```

---

## Decision Strategies

### Simple Placeholder Strategies (Phase 1)

These are **NOT** the expectimax solver - they're simple heuristics for testing:

1. **`random`**
   - Randomly choose INSERT or REMOVE
   - 50/50 split when both are valid

2. **`prefer-insert`** (default)
   - Always choose INSERT when possible
   - Fallback to REMOVE if only 1 atom

3. **`prefer-remove`**
   - Prefer REMOVE when >= 2 atoms
   - Fallback to INSERT otherwise

**Note:** The real solver (expectimax) will be implemented in Milestone 3.

---

## Example Output

```
==========================================================
MILESTONE 2: Automation Loop
Mode: dry-run (Phase 1)
Max moves: 3
==========================================================

=================================================
Using PLACEHOLDER decision logic: PreferInsert
Expectimax solver will be Milestone 3
=================================================

----------------------------------------------------------
[Move 1/3] Capturing screenshot...
  Source: "assets/jpg/board.jpg" (dry-run mode)
[Move 1/3] Detecting game state...
  Detected: 12 ring atoms, 1 player atom
[Move 1/3] Choosing move...
  Decision: INSERT at gap_index=7 (strategy: PreferInsert)
[Move 1/3] Mapping to coordinates...
  Coordinate: (190, 480)
[Move 1/3] Executing tap (DRY-RUN)...
  Would execute: adb shell input tap 190 480
  Status: Skipped (dry-run mode)
[Move 1/3] ✓ Complete

----------------------------------------------------------
[Move 2/3] Capturing screenshot...
  Source: "assets/jpg/board.jpg" (dry-run mode)
[Move 2/3] Detecting game state...
  Detected: 12 ring atoms, 1 player atom
[Move 2/3] Choosing move...
  Decision: INSERT at gap_index=3 (strategy: PreferInsert)
[Move 2/3] Mapping to coordinates...
  Coordinate: (360, 495)
[Move 2/3] Executing tap (DRY-RUN)...
  Would execute: adb shell input tap 360 495
  Status: Skipped (dry-run mode)
[Move 2/3] ✓ Complete

----------------------------------------------------------
[Move 3/3] Capturing screenshot...
  Source: "assets/jpg/board.jpg" (dry-run mode)
[Move 3/3] Detecting game state...
  Detected: 12 ring atoms, 1 player atom
[Move 3/3] Choosing move...
  Decision: INSERT at gap_index=9 (strategy: PreferInsert)
[Move 3/3] Mapping to coordinates...
  Coordinate: (120, 417)
[Move 3/3] Executing tap (DRY-RUN)...
  Would execute: adb shell input tap 120 417
  Status: Skipped (dry-run mode)
[Move 3/3] ✓ Complete

==========================================================
AUTOMATION LOOP SUMMARY
==========================================================
Total moves attempted: 3
Successful: 3
Failed: 0
Detection failures: 0
Success rate: 100.0%
==========================================================
```

---

## Architecture

### Data Flow

```
1. Screenshot Source (dry-run: load from disk)
           ↓
2. Game State Detector (CV from Milestone 1)
           ↓
3. Simple Solver (placeholder decision logic)
           ↓
4. Coordinate Mapper (Milestone 1 action.rs)
           ↓
5. Action Executor (dry-run: print command)
           ↓
6. Loop Controller (repeat N times)
```

### Module Structure

```
src/automation/
├── mod.rs              # Module exports
├── capture.rs          # Screenshot loading
├── decision.rs         # Simple decision strategies
├── executor.rs         # Tap execution (mock/ADB)
└── loop_controller.rs  # Main loop orchestration

src/milestone2_test.rs  # CLI binary
```

---

## Requirements

### Phase 1 (Current)

- ✅ **No emulator required**
- ✅ **No ADB required**
- ✅ Existing screenshot from Milestone 1
- ✅ Rust toolchain
- ✅ OpenCV (handled by opencv crate)

### Phase 2 (Future)

- 🔄 Android emulator running Atomas
- 🔄 ADB configured and device connected
- 🔄 Screen resolution handling

---

## Error Handling

### Screenshot Not Found

```
Error: Screenshot source not available

Caused by:
    Screenshot file not found: "assets/jpg/board.jpg"

Solution: Ensure the screenshot file exists or provide a different path:
  --screenshot path/to/your/screenshot.jpg
```

### Detection Failed

```
[Move 2/5] ✗ Failed: Failed to detect game state

Caused by:
    No ring atoms detected - cannot choose move

The loop continues with remaining moves.
```

### Invalid Strategy

```
Error: Invalid strategy: 'invalid-name'
Valid options: random, prefer-insert, prefer-remove
```

---

## Testing

### Test 1: Basic Dry-Run (3 moves)

```bash
cargo run --bin milestone2 -- --dry-run --moves 3
```

**Expected:** 3 moves executed, all successful, no ADB calls

### Test 2: Different Screenshot

```bash
cargo run --bin milestone2 -- \
    --dry-run \
    --screenshot path/to/another/screenshot.jpg \
    --moves 5
```

**Expected:** Uses custom screenshot

### Test 3: Different Strategy

```bash
cargo run --bin milestone2 -- \
    --dry-run \
    --strategy random \
    --moves 10
```

**Expected:** Random INSERT/REMOVE decisions

### Test 4: Error Handling

```bash
# Invalid screenshot path
cargo run --bin milestone2 -- \
    --dry-run \
    --screenshot nonexistent.jpg

# Invalid strategy
cargo run --bin milestone2 -- \
    --strategy invalid-name
```

**Expected:** Clear error messages, graceful failures

---

## Assumptions

1. **Same screenshot for all moves** (Phase 1 limitation)
   - In dry-run mode, detection uses the same screenshot each iteration
   - Real game state changes will be handled in Phase 2 with ADB capture

2. **No game state validation between moves**
   - Cannot detect desync in Phase 1 (same screenshot)
   - Phase 2 will add desync detection

3. **Simple decision logic only**
   - Uses basic heuristics, not game tree search
   - Good enough for testing automation loop structure

4. **No timing/delays**
   - Phase 1 runs moves immediately
   - Phase 2 will add configurable delays between moves

---

## Limitations

### Phase 1 Limitations

- ❌ Cannot capture live screenshots (no ADB)
- ❌ Cannot execute real taps (mock only)
- ❌ Cannot detect desync (same screenshot)
- ❌ Cannot adapt to changing game state

### Fixed in Phase 2

- ✅ ADB screenshot capture
- ✅ Real tap execution
- ✅ Dynamic game state tracking
- ✅ Desync detection

---

## Next Steps

### Phase 2: ADB Integration

1. **ADB device detection**
   - Check `adb devices`
   - Validate device connection

2. **Screenshot capture**
   - `adb exec-out screencap -p > screenshot.png`
   - Save to temp directory

3. **Tap execution**
   - `adb shell input tap x y`
   - Add configurable delay

4. **Desync detection**
   - Compare expected vs actual atom count
   - Stop loop on significant deviation

5. **Error recovery**
   - Retry on transient failures
   - Graceful degradation

### Phase 3: Solver Integration (Milestone 3)

- Replace simple heuristics with expectimax
- Game tree search
- Value estimation
- Advanced move selection

---

## Dependencies

```toml
clap = "4.5"           # CLI argument parsing
log = "0.4"            # Logging interface
env_logger = "0.11"    # Log formatting
rand = "0.9"           # Random strategy
anyhow = "1.0"         # Error handling
```

---

## Code Quality

- ✅ Modular structure (separate concerns)
- ✅ Comprehensive error handling
- ✅ Detailed logging at every step
- ✅ Unit tests for key components
- ✅ Documentation comments
- ✅ Type-safe coordinate handling

---

## Out of Scope

**NOT in Milestone 2:**

- ❌ Expectimax solver
- ❌ Game mechanics port
- ❌ CUDA/GPU acceleration
- ❌ Multi-emulator orchestration
- ❌ Deep RL training
- ❌ Advanced desync recovery
- ❌ Performance optimization

These are Milestone 3 or beyond.

---

## Troubleshooting

### Build Errors

```bash
# Format code
cargo fmt

# Check for errors
cargo check --bin milestone2

# Build
cargo build --bin milestone2
```

### Runtime Errors

Check logs with `--verbose` flag:
```bash
cargo run --bin milestone2 -- --dry-run --verbose
```

### Missing Screenshot

Ensure the default screenshot exists:
```bash
ls -la assets/jpg/board.jpg
```

Or provide custom path:
```bash
--screenshot path/to/your/screenshot.jpg
```

---

## Summary

**Phase 1 Deliverables:**

- ✅ Dry-run automation loop
- ✅ Simple decision strategies
- ✅ Detailed logging
- ✅ CLI interface
- ✅ Error handling
- ✅ No emulator required

**Status:** Ready for testing and Phase 2 ADB integration.
