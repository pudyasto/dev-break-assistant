#!/bin/bash
# DevBreak — Setup Script for Ubuntu 24.04 / GNOME / Wayland
# Run this once before starting development.

set -e

echo "🌿 DevBreak Setup Script"
echo "========================"

# ─── 1. System dependencies (requires sudo)
echo ""
echo "📦 Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libsoup-3.0-dev \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config \
    build-essential

echo "✅ System dependencies installed"

# ─── 2. Rust (if not installed)
if ! command -v cargo &> /dev/null; then
    echo ""
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed: $(rustc --version)"
else
    echo "✅ Rust already installed: $(cargo --version)"
fi

# ─── 3. npm dependencies
echo ""
echo "📦 Installing npm dependencies..."
npm install
echo "✅ npm packages installed"

# ─── 4. TypeScript check
echo ""
echo "🔍 Running TypeScript type-check..."
npm run type-check && echo "✅ TypeScript OK" || echo "❌ TypeScript errors"

# ─── 5. Rust domain tests (no system deps needed)
echo ""
echo "🧪 Running Rust domain tests..."
cd src-tauri && cargo test --lib 2>&1 | tail -10
cd ..

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Setup complete!"
echo ""
echo "To start DevBreak in development mode:"
echo "  npm run tauri dev"
echo ""
echo "Make sure you are running GNOME on Wayland for idle detection."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
