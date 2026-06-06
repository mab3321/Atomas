# 🚀 Quick Start - Run on Your Mobile NOW!

Your device is connected: **10FF8L08E200107**

---

## ⚡ Super Quick (3 Steps)

### 1️⃣ Authorize Your Phone
**Look at your phone screen!** There should be a popup:
```
"Allow USB debugging?"
□ Always allow from this computer
[Cancel] [OK]
```

✅ Check the box  
✅ Tap OK

---

### 2️⃣ Run the Test
**Double-click this file:**
```
run_mobile.bat
```

Or in terminal:
```bash
./run_mobile.bat
```

---

### 3️⃣ Choose a Test
```
1 = Quick (10 moves) - SEE IT WORK!
2 = Standard (50 moves) - FULL TEST
3 = Deep search (30 moves, depth 4) - BEST QUALITY
4 = Custom - YOU CHOOSE
```

**Recommendation: Start with Option 1 (quick test)**

---

## 👀 What You'll See

**On your phone:**
- Game launches automatically 🎮
- Atoms get tapped
- Merges happen
- Score increases
- **Watch for Minus atoms!** (red with minus sign)

**In terminal:**
- Detection results
- Solver decisions
- "USE MINUS" messages ← THIS IS THE FIX!
- Move completion status

---

## 📊 Expected Results

✅ **Good (Fix Working):**
- Final score: 500-1500 points (10 moves)
- Minus atoms used (check logs)
- Smooth gameplay
- High success rate

❌ **Problem:**
- Score < 200
- Minus ignored
- Many failures

---

## 🐛 If "Unauthorized" Error

Run this:
```bash
adb devices
```

Should show:
```
10FF8L08E200107    device    ← Must say "device" not "unauthorized"
```

If still "unauthorized":
1. Unplug USB cable
2. Plug back in
3. Authorize popup again
4. Run: `adb devices` to verify

---

## 💬 After Testing

Tell me:
1. Did it launch? ✅/❌
2. Final score? 
3. Minus atoms used? (check terminal logs)
4. Any errors?

---

**Just run: `run_mobile.bat` and choose option 1!** 🎮📱
