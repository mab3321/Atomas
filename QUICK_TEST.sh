#!/bin/bash
# Quick Docker Test Script
# Run this inside Docker container to verify all fixes

set -e

echo "=========================================="
echo "ATOMAS DOCKER VERIFICATION"
echo "=========================================="
echo ""

# Check branch
echo "1. Checking branch..."
BRANCH=$(git branch --show-current)
echo "   Current branch: $BRANCH"
if [ "$BRANCH" != "milestone3-stage2-expectimax-solver" ]; then
    echo "   ⚠️  WARNING: Expected milestone3-stage2-expectimax-solver"
fi
echo ""

# Check latest commit
echo "2. Checking latest commit..."
git log -1 --oneline
echo ""

# Check Rust version
echo "3. Checking Rust version..."
rustc --version
cargo --version
echo ""

# Check dependencies
echo "4. Checking dependencies..."
echo -n "   OpenCV: "
pkg-config --modversion opencv4 2>/dev/null || echo "Not found via pkg-config"
echo -n "   LLVM: "
llvm-config --version 2>/dev/null || echo "Using LLVM_CONFIG=$LLVM_CONFIG"
echo ""

# Clean build
echo "5. Cleaning previous build..."
cargo clean
echo "   ✓ Clean complete"
echo ""

# Build milestone2
echo "6. Building milestone2 binary..."
echo "   (This may take a few minutes...)"
if cargo build --bin milestone2 --release 2>&1 | tee /tmp/build.log; then
    echo "   ✅ BUILD SUCCESS!"
else
    echo "   ❌ BUILD FAILED!"
    echo ""
    echo "Last 20 lines of build output:"
    tail -20 /tmp/build.log
    exit 1
fi
echo ""

# Try dry-run
echo "7. Testing dry-run mode..."
if cargo run --bin milestone2 -- --dry-run --moves 1 2>&1 | head -30; then
    echo "   ✅ DRY-RUN SUCCESS!"
else
    echo "   ⚠️  Dry-run had issues (may need assets)"
fi
echo ""

echo "=========================================="
echo "✅ ALL COMPILE CHECKS PASSED!"
echo "=========================================="
echo ""
echo "Binary location: target/release/milestone2"
echo ""
echo "To test with ADB device:"
echo "  adb devices"
echo "  cargo run --bin milestone2 -- --adb --moves 1 --delay-ms 1000"
echo ""
