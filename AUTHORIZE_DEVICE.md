# 🔐 Device Authorization Required

Your mobile device is connected but shows as **"unauthorized"**

```
List of devices attached
10FF8L08E200107	unauthorized
```

---

## 📱 How to Authorize

### Step 1: Check Your Phone Screen
Look for a popup that says:
```
"Allow USB debugging?"
From computer: <your-computer-name>
```

### Step 2: Authorize
- ✅ Check the box **"Always allow from this computer"**
- ✅ Tap **"OK"** or **"Allow"**

### Step 3: Verify
After authorizing, run:
```bash
adb devices
```

You should now see:
```
List of devices attached
10FF8L08E200107	device    ← Status changed to "device"
```

---

## ⚠️ If Popup Doesn't Appear

### Option 1: Revoke and Reconnect
```bash
# On your phone: Settings → Developer Options → Revoke USB debugging authorizations
# Then reconnect the USB cable
```

### Option 2: Restart ADB
```bash
adb kill-server
adb start-server
adb devices
```

### Option 3: Check USB Debugging is Enabled
```
Settings → Developer Options → USB debugging (should be ON)
```

---

## 🚀 After Authorization

Once you see `device` status (not `unauthorized`), run:

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

This will launch the game on your phone! 📱🎮
