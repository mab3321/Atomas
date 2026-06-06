# 🚀 RUN IT NOW - Your Device is Ready!

Your device is **AUTHORIZED** and ready!

Device: `10FF8L08E200107` ✅

---

## ⚡ Quick Run (Copy-Paste This)

Open your terminal and run:

```bash
cargo run --bin milestone2 -- \
  --adb \
  --device 10FF8L08E200107 \
  --moves 10 \
  --solver expectimax \
  --solver-depth 2 \
  --delay-ms 1500 \
  --verbose
```

**OR** just double-click: `run_mobile.bat` and choose option 1

---

## 📱 What Will Happen

1. **Compiling...** (first time might take 1-2 minutes)
2. **Game launches on your phone** 🎮
3. **Automation starts** - watch your phone!
4. **Terminal shows** - detection, solver decisions, moves

---

## 👀 Watch Your Phone Screen!

You should see:
- Game board with atoms
- Atoms being tapped automatically
- Merges happening
- Score increasing
- **Minus atoms (red with -)** being used!

---

## 📊 What to Report

After 10 moves complete, tell me:

1. ✅ **Did it work?** Game launched and played?
2. ✅ **Final score?** (should be 300-800 points)
3. ✅ **Minus atoms?** Check terminal for "UseMinus" messages
4. ✅ **Any errors?** In terminal or on phone?

---

## 🔍 If You See "UseMinus" in Terminal

That means the fix is working! The Minus atom bug is fixed!

Example log you should see:
```
[INFO] Solver action: USE MINUS at 2 to remove 5 (value=150.23, nodes=243)
[DEBUG] Converting UseMinus(minus=2, target=5) to Decision::UseMinus
UseMinus: minus at index 2, removing target at index 5 → tapping target atom
```

---

## 💬 Having Issues?

**Option 1:** Use Docker (if you usually use it):
```bash
# Your Docker setup command here
```

**Option 2:** Run the batch file:
```
run_mobile.bat
```

**Option 3:** Tell me the error and I'll help!

---

**Your device is ready. Just run the command and watch your phone!** 📱🎮
