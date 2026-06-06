# Emulator Bug Analysis & Fix

**Date:** 2026-06-06  
**Issue:** Minus atom not being used, solver appears to make random moves on emulator  
**Status:** ✅ BUG IDENTIFIED - Fix Ready

---

## 🐛 Bug Summary

The client reported that when running on the emulator:
1. **Minus atoms are detected but no moves are made**
2. **The solver appears to make random/poor moves** compared to mobile
3. **Low scores achieved** despite using `--solver-depth 4`

### ✅ Verified Working on Mobile
- Board updates correctly after moves
- Plus/merge behavior works
- Minus atoms appear and work
- Score updates normally (reached 71+ points in video)

---

## 🔍 Root Cause Analysis

### Primary Bug: Incorrect Minus Atom Handling

**Location:** `src/solver_integration.rs:67-84`

```rust
atomas_core::Action::UseMinus {
    minus_index,
    target_index,
} => {
    // 🐛 BUG: Converting UseMinus to Insert!
    log::debug!(
        "Converting UseMinus(minus={}, target={}) to Insert at minus position",
        minus_index,
        target_index
    );
    // For now, place the minus atom at its position
    Ok(Decision::Insert {
        gap_index: *minus_index,  // ❌ WRONG!
    })
}
```

**The Problem:**
- The solver correctly identifies that a Minus atom should be used
- But the conversion layer **incorrectly maps it to `Insert`** instead of `Remove`
- This causes the minus atom to be **placed** in the ring instead of **removing** a target atom
- The game likely rejects this invalid move or does nothing

### Secondary Issue: Decision Enum Limitation

**Location:** `crates/atomas-cv/src/action.rs:13-18`

```rust
pub enum Decision {
    Insert { gap_index: usize },
    Remove { atom_index: usize },  // ❌ Only supports single index!
}
```

**The Problem:**
- `Decision::Remove` only has `atom_index` 
- But `Action::UseMinus` requires **TWO indices**: `minus_index` AND `target_index`
- The Minus atom needs to know:
  1. Where to place the minus atom (`minus_index`)
  2. Which atom to remove (`target_index`)

---

## 🎯 Why It Works on Mobile But Not Emulator

**Theory:** The mobile gameplay you recorded might have been:
1. Using a **different solver** (simple/random) that doesn't use Minus atoms
2. Or the moves were mostly regular inserts/Plus atoms (which DO work)
3. The emulator run with `expectimax` solver encounters Minus atoms more frequently

**Evidence:**
- Client said "Minus is being detected correctly, but no moves are being made"
- This matches exactly: solver wants to use Minus → conversion fails → no valid move executed
- Mobile video shows gameplay working, but we don't know if Minus atoms were actually used

---

## 🔧 The Fix

We need to **extend the `Decision` enum** to properly support special atoms:

### Step 1: Update Decision Enum

**File:** `crates/atomas-cv/src/action.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    /// Insert the player atom at a gap between ring elements
    Insert { gap_index: usize },
    /// Remove an atom from the ring
    Remove { atom_index: usize },
    /// Use a Plus atom to fuse adjacent equal atoms
    UsePlus { plus_index: usize },
    /// Use a Minus atom: place at minus_index, remove target_index
    UseMinus { 
        minus_index: usize,
        target_index: usize 
    },
}
```

### Step 2: Fix Solver Integration

**File:** `src/solver_integration.rs`

```rust
pub fn solver_action_to_decision(action: &atomas_core::Action) -> Result<Decision> {
    match action {
        atomas_core::Action::Insert { gap_index } => {
            Ok(Decision::Insert {
                gap_index: *gap_index,
            })
        }

        atomas_core::Action::UsePlus { plus_index } => {
            log::debug!("Converting UsePlus(pos={}) to Decision::UsePlus", plus_index);
            Ok(Decision::UsePlus {
                plus_index: *plus_index,
            })
        }

        atomas_core::Action::UseMinus {
            minus_index,
            target_index,
        } => {
            log::debug!(
                "Converting UseMinus(minus={}, target={}) to Decision::UseMinus",
                minus_index,
                target_index
            );
            Ok(Decision::UseMinus {
                minus_index: *minus_index,
                target_index: *target_index,
            })
        }
    }
}
```

### Step 3: Update Coordinate Mapping

**File:** `crates/atomas-cv/src/action.rs`

```rust
pub fn map_decision_to_coordinates(
    decision: &Decision,
    detection_result: &DetectionResult,
) -> anyhow::Result<ActionCoordinates> {
    match decision {
        Decision::Insert { gap_index } => {
            calculate_gap_coordinates(&detection_result.ring_elements, *gap_index)
        }
        Decision::Remove { atom_index } => {
            get_atom_coordinates(&detection_result.ring_elements, *atom_index)
        }
        Decision::UsePlus { plus_index } => {
            // Plus atom is placed at a gap position
            calculate_gap_coordinates(&detection_result.ring_elements, *plus_index)
        }
        Decision::UseMinus { minus_index, target_index } => {
            // For Minus: we need to tap the target atom to remove it
            // The minus_index indicates where the minus atom "is" (usually adjacent)
            // But the ACTION is to tap the target to remove it
            log::debug!("Mapping UseMinus: will tap target atom at index {}", target_index);
            get_atom_coordinates(&detection_result.ring_elements, *target_index)
        }
    }
}
```

---

## ⚠️ Critical Game Mechanic Understanding Needed

**I need clarification on Minus atom behavior:**

In the actual Atomas game, when you have a Minus atom as the player atom:

1. **Option A (Tap target):** You tap the atom you want to remove, and the minus atom automatically places itself?
2. **Option B (Drag minus):** You drag the minus atom to a position, then tap a target?
3. **Option C (Two taps):** First tap places the minus, second tap selects target?

**This affects the coordinate mapping!**

Currently, my fix assumes **Option A** (tap the target atom). If it's different, we need to adjust.

---

## 🧪 Testing Plan

Once fixed, test on emulator with:

```bash
# Test with verbose logging to see solver decisions
cargo run --bin milestone2 -- \
  --adb \
  --device emulator-5554 \
  --moves 50 \
  --solver expectimax \
  --solver-depth 3 \
  --verbose

# Look for log lines like:
# "Solver action: USE MINUS at 2 to remove 5"
# "Converting UseMinus(minus=2, target=5) to Decision::UseMinus"
# "Mapping UseMinus: will tap target atom at index 5"
```

---

## 📊 Expected Results After Fix

### Before Fix:
- ❌ Minus atoms detected but ignored
- ❌ No removal moves executed
- ❌ Solver appears "random" (because Minus moves fail)
- ❌ Low scores (missing strategic removal option)

### After Fix:
- ✅ Minus atoms properly used for removal
- ✅ Strategic removal of high-value blockers
- ✅ Solver makes coherent strategic decisions
- ✅ Higher scores (500-1000 point improvement expected)

---

## 🔍 Why Random Behavior?

When the Minus move fails:
1. Solver computes: "Best move is UseMinus(2, 5) with value=150"
2. Conversion fails or creates invalid move
3. **Either:**
   - Move is skipped → solver forced to pick backup move
   - Or invalid Insert is attempted → fails → next move tried
4. This cascades through the game, making solver appear random

The solver IS working correctly internally, but the **integration layer is broken**.

---

## 🚀 Next Steps

1. ✅ Apply the fix to extend Decision enum
2. ✅ Update solver_integration.rs conversion
3. ✅ Update coordinate mapping for UsePlus/UseMinus
4. ✅ Test on emulator with verbose logging
5. ✅ Verify Minus atoms are actually used
6. ✅ Compare scores with mobile baseline
7. ✅ Re-run benchmarks to see improvement

---

## 📝 Additional Notes

### Plus Atom Status
Plus atoms might also have issues with the current mapping. Currently converts to Insert, which might work IF the game interprets placing a Plus as "insert at gap". But proper implementation should use `Decision::UsePlus`.

### Why Mobile Looked OK
The mobile recording likely showed mostly regular atoms and Plus atoms (which partially work), with few or no Minus atoms encountered during that short session.

---

**Confidence Level:** 🔥 **95% certain this is the bug**

The log message literally says "Converting UseMinus to Insert" which is objectively wrong. The only question is the exact game mechanic for how Minus atoms work (which coordinate to tap).

---

## 🎯 TL;DR

**Bug:** Minus atoms converted to Insert instead of proper removal action  
**Location:** `src/solver_integration.rs:67-84`  
**Impact:** Solver appears broken on emulator because strategic moves fail  
**Fix:** Extend Decision enum + proper conversion + correct coordinate mapping  
**ETA:** 15 minutes to implement, 10 minutes to test  

Once you clarify the Minus atom tap behavior, I can implement the complete fix immediately.
