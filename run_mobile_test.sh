#!/bin/bash

# Add ADB to PATH
export PATH="$PATH:/c/Users/RBTGL/AppData/Local/Android/Sdk/platform-tools"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "════════════════════════════════════════════════════"
echo "  ATOMAS Mobile Test Runner"
echo "════════════════════════════════════════════════════"
echo ""

# Check for connected devices
echo -e "${YELLOW}Checking for connected devices...${NC}"
DEVICES=$(adb devices | grep -v "List of devices" | grep "device$" | awk '{print $1}')

if [ -z "$DEVICES" ]; then
    echo -e "${RED}❌ No authorized devices found!${NC}"
    echo ""
    echo "Devices status:"
    adb devices
    echo ""
    echo "👉 Please authorize USB debugging on your phone"
    echo "   Look for popup: 'Allow USB debugging?'"
    echo "   Check 'Always allow' and tap 'OK'"
    echo ""
    echo "📄 See AUTHORIZE_DEVICE.md for detailed instructions"
    exit 1
fi

DEVICE_SERIAL=$(echo "$DEVICES" | head -n 1)
echo -e "${GREEN}✓ Device found: $DEVICE_SERIAL${NC}"
echo ""

# Get device model
MODEL=$(adb -s "$DEVICE_SERIAL" shell getprop ro.product.model 2>/dev/null | tr -d '\r')
ANDROID_VERSION=$(adb -s "$DEVICE_SERIAL" shell getprop ro.build.version.release 2>/dev/null | tr -d '\r')
echo "Device: $MODEL (Android $ANDROID_VERSION)"
echo ""

# Menu
echo "Select test to run:"
echo "  1) Quick test (10 moves, depth 2)"
echo "  2) Standard test (50 moves, depth 3)"
echo "  3) Deep search test (30 moves, depth 4)"
echo "  4) Custom test"
echo ""
read -p "Enter choice (1-4): " choice

case $choice in
    1)
        echo ""
        echo "🎮 Running QUICK TEST (10 moves)..."
        cargo run --bin milestone2 -- \
            --adb \
            --device "$DEVICE_SERIAL" \
            --moves 10 \
            --solver expectimax \
            --solver-depth 2 \
            --delay-ms 1500 \
            --verbose
        ;;
    2)
        echo ""
        echo "🎮 Running STANDARD TEST (50 moves)..."
        cargo run --bin milestone2 -- \
            --adb \
            --device "$DEVICE_SERIAL" \
            --moves 50 \
            --solver expectimax \
            --solver-depth 3 \
            --delay-ms 1000 \
            --verbose 2>&1 | tee mobile_test.log

        echo ""
        echo "════════════════════════════════════════════════════"
        echo "Test complete! Log saved to: mobile_test.log"
        echo ""
        echo "Analysis:"
        echo "  Minus atoms: $(grep -c "UseMinus" mobile_test.log 2>/dev/null || echo 0) times"
        echo "  Plus atoms:  $(grep -c "UsePlus" mobile_test.log 2>/dev/null || echo 0) times"
        echo "  Success:     $(grep -c "completed successfully" mobile_test.log 2>/dev/null || echo 0) moves"
        ;;
    3)
        echo ""
        echo "🎮 Running DEEP SEARCH TEST (30 moves, depth 4)..."
        cargo run --bin milestone2 -- \
            --adb \
            --device "$DEVICE_SERIAL" \
            --moves 30 \
            --solver expectimax \
            --solver-depth 4 \
            --delay-ms 1200 \
            --verbose
        ;;
    4)
        echo ""
        read -p "Number of moves: " moves
        read -p "Solver depth (1-5): " depth
        read -p "Delay between moves (ms): " delay

        echo ""
        echo "🎮 Running CUSTOM TEST..."
        cargo run --bin milestone2 -- \
            --adb \
            --device "$DEVICE_SERIAL" \
            --moves "$moves" \
            --solver expectimax \
            --solver-depth "$depth" \
            --delay-ms "$delay" \
            --verbose
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

echo ""
echo "════════════════════════════════════════════════════"
echo -e "${GREEN}Done!${NC}"
