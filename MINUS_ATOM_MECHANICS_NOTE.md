# Minus Atom - Game Mechanics Verification Needed

**Critical Note:** The Minus atom implementation assumes a specific UI behavior that needs validation on the real device.

---

## 🎯 Current Implementation

### How We Handle Minus Atoms:
```
Player Atom: Minus (-2)
Solver chooses: UseMinus(minus_index=2, target_index=5)
Current action: Tap the atom at target_index=5
Expected result: Game removes both the target atom and places/removes the minus
```

### Code Location:
`crates/atomas-cv/src/action.rs` lines ~64-77

```rust
Decision::UseMinus { minus_index, target_index } => {
    // We tap the TARGET atom coordinates
    get_atom_coordinates(&detection_result.ring_elements, *target_index)
}
```

---

## ❓ Possible Game Behaviors

### Option A: Tap Target (Current Implementation)
**User Action:** Tap the atom you want to remove  
**Game Logic:** Places minus somewhere, removes the tapped atom  
**Our Code:** ✅ Currently implements this

### Option B: Tap Gap (Alternative)
**User Action:** Tap a gap to place the minus atom  
**Game Logic:** Minus removes an adjacent or specified atom  
**Our Code:** ⚠️ Can switch to this if Option A doesn't work

### Option C: Two-Step Interaction
**User Action:** First tap places minus, second tap selects removal target  
**Game Logic:** Requires two separate taps  
**Our Code:** ❌ NOT supported (would need executor changes)

---

## 🧪 How to Verify Which Is Correct

### Manual Test on Device:
1. Start the game normally
2. Wait until you get a Minus atom (red/black atom with - symbol)
3. Observe carefully: What gesture removes an atom?
   - Do you tap the atom you want gone? → Option A ✅
   - Do you tap a gap/space first? → Option B
   - Do you need two taps? → Option C

### Automated Test:
```bash
# Run with verbose to see coordinates being tapped
cargo run --bin milestone2 -- \
  --adb \
  --device emulator-5554 \
  --moves 20 \
  --solver expectimax \
  --verbose 2>&1 | grep -A 2 "UseMinus"

# Watch the screen: Where does it tap?
# Does the minus action succeed?
```

---

## 🔧 If Option A Doesn't Work, Try Option B

### Quick Fix (Switch to Tapping Gap):
Edit `crates/atomas-cv/src/action.rs` line ~74:

**Change FROM:**
```rust
get_atom_coordinates(&detection_result.ring_elements, *target_index)
```

**Change TO:**
```rust
calculate_gap_coordinates(&detection_result.ring_elements, *minus_index)
```

Then rebuild:
```bash
cargo build --release --bin milestone2
```

---

## 🎮 Game Mechanics Context

### How Minus Works in Game Logic:
From `crates/atomas-core/src/game.rs`:
- The minus atom removes TWO atoms from the ring:
  1. The atom at `minus_index` (where minus is "placed")
  2. The atom at `target_index` (the removal target)
- Both are removed, and you get `target_value × 5` points

### Why This Is Tricky:
The game LOGIC is clear: remove two atoms.  
But the UI GESTURE is unknown: How does the player indicate which two?

---

## 📊 Testing Matrix

| Test Scenario | Current Code | If Doesn't Work |
|--------------|--------------|-----------------|
| Minus atom detected | ✅ Detection works | - |
| Solver chooses UseMinus | ✅ Solver logic works | - |
| Coordinates calculated | ✅ Math works | - |
| Tap executes correctly | ❓ NEEDS VERIFICATION | Switch to Option B |
| Atom removed from ring | ❓ NEEDS VERIFICATION | Check game response |
| Score increases | ❓ NEEDS VERIFICATION | Verify points added |

---

## 🚨 Fallback Plan

### If Neither Option A nor B Works:

1. **Check if minus_index and target_index need to be adjacent**
   - Maybe the game only allows removing adjacent atoms?
   - Check solver logic: does it respect adjacency?

2. **Check if there's a drag gesture**
   - Maybe you drag the minus TO the target?
   - Would need to implement drag support in executor

3. **Check game version differences**
   - Maybe emulator game version is different from mobile?
   - Try updating the APK

4. **Check if special atoms work differently in automation**
   - Maybe the game detects automated play?
   - Try increasing delay_ms to make it look more human

---

## ✅ Success Indicators

### If It's Working:
1. ✅ Log shows: "UseMinus: minus at index X, removing target at index Y → tapping target atom"
2. ✅ Visual: Tap occurs on an atom in the ring
3. ✅ Result: That atom disappears from the ring
4. ✅ Result: Ring size decreases by 2 (both minus and target removed)
5. ✅ Result: Score increases by `target_value × 5`
6. ✅ Game continues normally

### If It's Not Working:
1. ❌ Move fails / hangs / does nothing
2. ❌ Wrong atom is tapped
3. ❌ Atom is tapped but nothing happens
4. ❌ Game shows error or rejects move
5. ❌ Minus atom appears to be "placed" but target not removed

---

## 📞 Debugging Commands

### See exactly what's being tapped:
```bash
cargo run --bin milestone2 -- \
  --adb \
  --device emulator-5554 \
  --moves 50 \
  --solver expectimax \
  --delay-ms 2000 \
  --verbose 2>&1 | tee minus_debug.log

# Then search the log:
grep "UseMinus" minus_debug.log
grep "tapping" minus_debug.log
grep "atom_index=" minus_debug.log
```

### Compare with Plus atoms (which should work):
```bash
# Plus atoms tap gaps (confirmed behavior)
grep "UsePlus" minus_debug.log
grep "gap_index=" minus_debug.log
```

---

## 💡 Recommendation

**For now:** Use the current implementation (tap target atom) and TEST on the device.

**If it fails:** Try the one-line fix above (tap gap instead).

**If both fail:** Manual verification of actual game gestures needed.

---

## 🎯 Client Action Required

1. **Run the automation with Minus atoms**
2. **Watch the screen carefully** when UseMinus is executed
3. **Report back:**
   - Does the tap look correct?
   - Does the minus action succeed?
   - Which atom gets removed (if any)?
   - Any error messages or failures?

This will tell us which option is correct, and we can finalize the implementation!

---

**Note:** The bug fix (extending Decision enum) is DEFINITELY correct. The only question is which coordinates to tap for Minus atoms - but this is a minor adjustment once we know the correct game mechanic.
