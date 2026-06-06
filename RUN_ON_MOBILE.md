# Run Automation on Your Mobile Device

Since you have your mobile device attached, here are the exact commands to run and verify the Minus atom fix works correctly.

---

## 🔍 Step 1: Find Your Device

```bash
adb devices
```

**Example output:**
```
List of devices attached
R5CR80XXXXXX    device
```

Copy your device serial (e.g., `R5CR80XXXXXX`)

---

## 🎮 Step 2: Quick Test (10 moves with verbose)

```bash
# Replace YOUR_DEVICE_SERIAL with the actual serial from step 1
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 10 \
  --solver expectimax \
  --solver-depth 2 \
  --delay-ms 1500 \
  --verbose
```

**What to watch for:**
- Game board updates after each move
- If Minus atom appears, watch if it gets used
- Score increases consistently
- No "failed" messages

---

## 📊 Step 3: Full Test (50 moves, measure performance)

```bash
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 50 \
  --solver expectimax \
  --solver-depth 3 \
  --delay-ms 1000 \
  --verbose 2>&1 | tee mobile_test.log
```

**After completion, check the log:**
```bash
# Check if Minus atoms were used
grep -i "minus" mobile_test.log

# Check if Plus atoms were used
grep -i "plus" mobile_test.log

# Check move success rate
grep -i "completed successfully" mobile_test.log | wc -l
```

---

## 🔬 Step 4: Watch Minus Atom Behavior Specifically

```bash
# Run with extra delay so you can watch the screen
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 30 \
  --solver expectimax \
  --solver-depth 2 \
  --delay-ms 3000 \
  --verbose 2>&1 | grep --line-buffered -E "(Minus|UseMinus|Player atom: -2)"
```

**When you see Minus atom:**
1. Watch the screen carefully
2. Note which atom gets tapped
3. See if it removes correctly
4. Check if score increases

---

## 🎯 Step 5: Compare Solver Depths

### Depth 1 (Fast but basic):
```bash
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 50 \
  --solver expectimax \
  --solver-depth 1 \
  --delay-ms 500
```

### Depth 4 (Slow but strategic):
```bash
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 50 \
  --solver expectimax \
  --solver-depth 4 \
  --delay-ms 800
```

**Compare:**
- Depth 4 should get higher scores
- Depth 4 moves should look more strategic
- Check final scores at the end

---

## 📹 Step 6: Screen Recording (For Verification)

If you want to record gameplay:

```bash
# Start screen recording on device
adb -s YOUR_DEVICE_SERIAL shell screenrecord /sdcard/atomas_test.mp4 &

# Run the automation
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 30 \
  --solver expectimax \
  --solver-depth 3 \
  --delay-ms 1500 \
  --verbose

# Stop recording (Ctrl+C after ~30 seconds)
# Pull the video
adb -s YOUR_DEVICE_SERIAL pull /sdcard/atomas_test.mp4 .
```

---

## 🔍 What to Look For

### ✅ Good Signs (Fix Working):
- Minus atoms appear and ARE used (tapped/activated)
- When Minus used, an atom is removed from ring
- Score increases by `target_value × 5` when Minus used
- Plus atoms are placed and cause merges
- Steady score progression: 20-40 points per move average
- High move success rate (>90%)
- Final score after 50 moves: 1000-2500 points

### ❌ Bad Signs (Something Wrong):
- Minus atoms appear but are ignored/skipped
- Tap happens but nothing changes
- Many "failed to execute" messages
- Score stays very low (<500 after 50 moves)
- Game hangs or becomes unresponsive
- Wrong atoms are tapped

---

## 🐛 If Minus Still Doesn't Work

### Check Which Atom Gets Tapped:

Run with very slow delay and watch:
```bash
cargo run --bin milestone2 -- \
  --adb \
  --device YOUR_DEVICE_SERIAL \
  --moves 10 \
  --solver expectimax \
  --delay-ms 5000 \
  --verbose
```

**When UseMinus appears in logs:**
1. Pause and note the ring state
2. Watch which coordinate gets tapped
3. Is it:
   - ✅ An atom in the ring (target)? → Current implementation
   - ❌ A gap between atoms? → Need to switch to gap mapping

**If tapping target doesn't work, run:**
```bash
# I can guide you through the 1-line fix to tap gaps instead
# Just let me know and I'll provide the exact code change
```

---

## 💬 What to Report Back

After testing, tell me:

1. **Device Info:**
   - Device model?
   - Android version?
   - Serial (first 4 chars)?

2. **Minus Atom Behavior:**
   - Did Minus atoms appear? (check logs)
   - Were they used? (did you see taps?)
   - Did removal work? (atom disappeared?)
   - Score increased correctly?

3. **Overall Performance:**
   - Final score after 50 moves?
   - Approximate success rate?
   - Any errors or failures?

4. **Depth Comparison:**
   - Did depth 4 score better than depth 1?
   - Noticeable difference in play quality?

5. **Visual Observation:**
   - Does gameplay look strategic?
   - Any weird tap locations?
   - Game response smooth?

---

## 📦 Quick Copy-Paste Commands

Replace `YOUR_DEVICE_SERIAL` with your actual serial:

```bash
# Find device serial
adb devices

# Quick 10-move test
cargo run --bin milestone2 -- --adb --device YOUR_DEVICE_SERIAL --moves 10 --solver expectimax --solver-depth 2 --delay-ms 1500 --verbose

# Full 50-move test with log
cargo run --bin milestone2 -- --adb --device YOUR_DEVICE_SERIAL --moves 50 --solver expectimax --solver-depth 3 --delay-ms 1000 --verbose 2>&1 | tee mobile_test.log

# Check for Minus usage
grep -i "minus" mobile_test.log

# Compare depths (run both and compare scores)
cargo run --bin milestone2 -- --adb --device YOUR_DEVICE_SERIAL --moves 50 --solver expectimax --solver-depth 1 --delay-ms 500
cargo run --bin milestone2 -- --adb --device YOUR_DEVICE_SERIAL --moves 50 --solver expectimax --solver-depth 4 --delay-ms 800
```

---

## 🎯 Expected Results

If everything works:
- **Score after 50 moves:** 1000-2500 points
- **Minus atom usage:** 5-10 times (when available)
- **Plus atom usage:** 5-10 times (when available)
- **Success rate:** >90% moves complete
- **Depth effect:** Depth 4 scores 20-40% higher than depth 1

---

**Let's see it in action on your mobile! Run the tests and report back what you observe.** 🚀📱
