# 🐳 Run on Mobile via Docker

You need Docker since Windows doesn't have OpenCV setup.

---

## 🚀 Quick Start

### Step 1: Start Docker Container with ADB Access

```bash
# Build the Docker image (if not already built)
docker build -f docker/Dockerfile -t atomas-emulator .

# Run container with ADB access and mobile device
docker run -it --rm \
  --privileged \
  -v /dev/bus/usb:/dev/bus/usb \
  -v $(pwd):/workspace \
  -w /workspace \
  atomas-emulator bash
```

**On Windows, use:**
```powershell
docker run -it --rm --privileged -v ${PWD}:/workspace -w /workspace atomas-emulator bash
```

---

### Step 2: Inside Docker, Connect to Your Device

```bash
# Inside the Docker container
adb devices

# Should show:
# 10FF8L08E200107    device
```

---

### Step 3: Run the Automation

```bash
# Inside Docker container
cargo run --bin milestone2 -- \
  --adb \
  --device 10FF8L08E200107 \
  --moves 10 \
  --solver expectimax \
  --solver-depth 2 \
  --delay-ms 1500 \
  --verbose
```

---

## 🎯 Alternative: Build in Docker, Run on Windows

### Option A: Build Binary in Docker

```bash
# In Docker
cargo build --release --bin milestone2

# Binary will be at: target/release/milestone2.exe
```

Then copy it out and run on Windows with ADB.

---

## 🔧 Alternative: Use WSL2 with USB Passthrough

If you have WSL2:

```bash
# In WSL2
usbipd wsl list
usbipd wsl attach --busid <YOUR-DEVICE-BUSID>

# Then run normally in WSL2
cargo run --bin milestone2 -- --adb --device 10FF8L08E200107 ...
```

---

## 💡 Simplest Solution: Use Pre-built Binary

If you have a pre-built binary:

```bash
# Just run the binary directly
./target/release/milestone2.exe --adb --device 10FF8L08E200107 --moves 10 --solver expectimax --solver-depth 2 --delay-ms 1500 --verbose
```

---

## 📦 What You Need

Your mobile device is ready:
- ✅ Device: `10FF8L08E200107`
- ✅ Authorized: Yes
- ✅ ADB working: Yes

Just need to run in an environment with OpenCV (Docker).

---

## 🐛 If Docker Has Issues with ADB

Try this approach:

1. **Run ADB server on Windows:**
```powershell
# On Windows PowerShell
adb -a nodaemon server start
```

2. **Connect from Docker:**
```bash
# In Docker, connect to host ADB
adb -H host.docker.internal devices
```

3. **Run automation with host ADB:**
```bash
cargo run --bin milestone2 -- --adb --device 10FF8L08E200107 ...
```

---

## ✅ Expected Behavior

Once running (in Docker or WSL):
- Game launches on your mobile
- Automation plays 10 moves
- Terminal shows solver decisions
- Watch for "UseMinus" in logs (the fix!)

---

**Use your normal Docker setup and run it there!** 🐳📱
