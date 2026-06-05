# Docker Environment Verification Guide

## Fixed Issues (Commit: 6cbff71)
✅ OpenCV rectangle API error in overlay.rs
✅ Solver error type conversion in expectimax_solver.rs  
✅ Infinite recursion in loop_controller.rs trait

## Testing in Docker

### 1. Pull Latest Changes
```bash
git fetch origin
git checkout milestone3-stage2-expectimax-solver
git pull origin milestone3-stage2-expectimax-solver
```

### 2. Clean Build
```bash
cargo clean
cargo build --release
```

### 3. Test Milestone 1 (branch: milestone1-move-output)
```bash
git checkout milestone1-move-output
cargo run --bin milestone1
```

### 4. Test Milestone 2 (current branch)
```bash
git checkout milestone3-stage2-expectimax-solver
cargo run --bin milestone2 -- --dry-run --moves 1
```

### 5. Test with ADB (if device connected)
```bash
cargo run --bin milestone2 -- --adb --moves 1 --delay-ms 1000
```

## Expected Output

### Successful Compilation
- No errors during `cargo build`
- Binaries should be in `target/release/`

### Runtime Issues to Watch For
- **"No such file or directory"** → Missing assets/screenshots
- **"ADB device not found"** → Device not connected (normal if no device)
- **"Failed to detect game state"** → Expected if no game screenshot available
- **Segmentation fault** → Possible OpenCV or dependency issue

## If You Still Get Errors

Please provide:
1. **Exact error message** (full output)
2. **Which command** you're running
3. **Which branch** you're on
4. **Docker image** details (`docker --version`, base image)

Run this to capture full diagnostics:
```bash
git branch --show-current > /tmp/docker-debug.txt
rustc --version >> /tmp/docker-debug.txt
cargo --version >> /tmp/docker-debug.txt
echo "---BUILD ATTEMPT---" >> /tmp/docker-debug.txt
cargo build --bin milestone2 2>&1 | tee -a /tmp/docker-debug.txt
```

Then share the contents of `/tmp/docker-debug.txt`
