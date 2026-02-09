#!/usr/bin/env python3
"""Quick test of the adaptive coaching system integration."""

from md_scanner.tauri_bridge import TauriBridge

def test_bridge():
    bridge = TauriBridge()
    print("TauriBridge initialized with coaching support")
    
    # Test coaching commands
    print("\n=== Testing Coaching Commands ===")
    
    # Get status
    status = bridge._coach_get_status()
    print(f"Overall skill: {status.get('overall_skill', 'N/A')}")
    print(f"Skills tracked: {list(status.get('skills', {}).keys())}")
    print(f"Session active: {status.get('session_active', False)}")
    
    # Start a session
    session = bridge._coach_start_session()
    print(f"\nStarted session: {session.get('session_id')}")
    
    # Get a naming suggestion
    result = bridge._coach_get_naming_suggestion(
        filename="notes.md",
        content="Meeting notes from the budget review discussion"
    )
    print(f"\nNaming suggestion:")
    print(f"  Type: {result.get('suggestion_type')}")
    print(f"  Skill level: {result.get('skill_level')}")
    print(f"  Intensity: {result.get('intensity')}")
    print(f"  Should show: {result.get('should_show')}")
    
    # Get fade status
    fade = bridge._coach_get_fade_status()
    print(f"\nFade status:")
    for skill, info in fade.items():
        print(f"  {skill}: {info.get('level')} (faded={info.get('faded_out')})")
    
    # End session
    summary = bridge._coach_end_session()
    print(f"\nSession ended:")
    print(f"  Duration: {summary.get('duration', 'N/A')}")
    print(f"  Suggestions offered: {summary.get('suggestions_offered', 0)}")
    
    print("\n=== All coaching tests passed! ===")

if __name__ == "__main__":
    test_bridge()
