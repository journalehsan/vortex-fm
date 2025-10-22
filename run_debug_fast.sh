#!/bin/bash
# Fast debug runner - uses pre-built binary

BINARY="./target/release/vortex_fm"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found at $BINARY"
    echo "Building first..."
    cargo build --release
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Running with CLEAN DEBUG LOGS (ribbon only)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "When you see this, try:"
echo "  1. Click the VIEW button [🔲]"
echo "  2. Click the SORT button [⇅]"
echo ""
echo "Watch for these markers in the logs:"
echo "  📥 = Button clicked"
echo "  🔄 = State changed"
echo "  ✅ = Processing OK"
echo "  📤 = Message sent"
echo "  📌 = Handler running"
echo ""
echo "Press Ctrl+C to exit"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

RUST_LOG=debug $BINARY 2>&1 | grep -E "📥|🔧|🔄|📖|✅|📤|📌|⚠️|RibbonMessage|ToggleView|ToggleSort"
