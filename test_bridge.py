#!/usr/bin/env python3
"""
Quick test of the Tauri bridge to verify Python engines are accessible.
"""

import sys
import json
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent))

from md_scanner.tauri_bridge import TaurisBridge


def test_system_info():
    """Test system info retrieval."""
    bridge = TaurisBridge()
    result = bridge.handle_command("get_system_info", {})
    print("System Info:")
    print(json.dumps(result, indent=2))
    assert result.get("success"), "Failed to get system info"
    print("✓ get_system_info passed\n")


def test_validate_index():
    """Test index validation."""
    bridge = TaurisBridge()
    test_index = Path(".test_index")
    result = bridge.handle_command("validate_index", {"index_dir": str(test_index)})
    print("Index Validation:")
    print(json.dumps(result, indent=2))
    assert result.get("success"), "Failed to validate index"
    print("✓ validate_index passed\n")


if __name__ == "__main__":
    print("=" * 50)
    print("Testing Tauri Bridge")
    print("=" * 50 + "\n")

    try:
        test_system_info()
        test_validate_index()
        print("\n" + "=" * 50)
        print("All tests passed! ✓")
        print("=" * 50)
    except Exception as e:
        print(f"\nError: {e}")
        import traceback

        traceback.print_exc()
        sys.exit(1)
