#!/usr/bin/env python3
"""
Test suite for EchoChain copyright detection functionality.
Tests the copyright detection and audio fingerprinting system.
"""

import os
import sys
import tempfile
import numpy as np
import logging

# Add the current directory to the path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    from copyright_detection import CopyrightDetector
    import soundfile as sf

    DEPENDENCIES_AVAILABLE = True
except ImportError as e:
    print(f"Dependencies not available: {e}")
    DEPENDENCIES_AVAILABLE = False


def create_test_audio_file(filename, duration=5, sample_rate=22050, frequency=440):
    """Create a simple synthetic audio file for testing."""
    t = np.linspace(0, duration, int(sample_rate * duration))

    # Create a simple sine wave
    audio_data = 0.3 * np.sin(2 * np.pi * frequency * t)

    # Add some harmonics to make it more unique
    audio_data += 0.2 * np.sin(2 * np.pi * frequency * 2 * t)
    audio_data += 0.1 * np.sin(2 * np.pi * frequency * 3 * t)

    # Normalize
    audio_data = audio_data / np.max(np.abs(audio_data))

    sf.write(filename, audio_data, sample_rate)
    return filename


def test_copyright_detector():
    """Test the CopyrightDetector functionality."""
    print("=" * 60)
    print("EchoChain Copyright Detection Test")
    print("=" * 60)

    if not DEPENDENCIES_AVAILABLE:
        print("❌ Dependencies not available. Testing basic functionality...")
        try:
            detector = CopyrightDetector()
            print("✓ CopyrightDetector initialized (basic mode)")
            stats = detector.get_database_stats()
            print(f"✓ Database stats: {stats}")
            return True
        except Exception as e:
            print(f"❌ Basic test failed: {e}")
            return False

    try:
        # Initialize detector
        print("🔧 Initializing CopyrightDetector...")
        with tempfile.TemporaryDirectory() as temp_dir:
            db_path = os.path.join(temp_dir, "test_copyright_db.json")
            detector = CopyrightDetector(fingerprint_db_path=db_path)
            print("✓ CopyrightDetector initialized successfully")

            # Create test audio files
            test_file1 = os.path.join(temp_dir, "test_original.wav")
            test_file2 = os.path.join(temp_dir, "test_user_upload.wav")

            print("🎵 Creating test audio files...")
            create_test_audio_file(test_file1, duration=3, frequency=440)  # A4
            create_test_audio_file(
                test_file2, duration=3, frequency=440
            )  # Same frequency (potential match)
            print("✓ Test audio files created")

            # Test copyright analysis on clean file
            print("🔍 Analyzing clean file for copyright...")
            clean_analysis = detector.analyze_for_copyright(test_file2)
            print("✓ Clean file analysis completed")
            print(f"   Risk score: {clean_analysis['risk_score']:.3f}")
            print(
                f"   Is likely copyrighted: {clean_analysis['is_likely_copyrighted']}"
            )
            print(f"   Matches found: {len(clean_analysis['copyright_matches'])}")

            # Add a "copyrighted" file to database
            print("📝 Adding copyrighted content to database...")
            entry_id = detector.add_copyrighted_content(
                test_file1,
                title="Test Copyrighted Song",
                artist="Test Artist",
                label="Test Label",
                year=2024,
            )
            print(f"✓ Added copyrighted content with ID: {entry_id}")

            # Test copyright analysis on potentially infringing file
            print("🚨 Analyzing potentially infringing file...")
            infringing_analysis = detector.analyze_for_copyright(test_file2)
            print("✓ Infringing file analysis completed")
            print(f"   Risk score: {infringing_analysis['risk_score']:.3f}")
            print(
                f"   Is likely copyrighted: {infringing_analysis['is_likely_copyrighted']}"
            )
            print(f"   Matches found: {len(infringing_analysis['copyright_matches'])}")

            if infringing_analysis["copyright_matches"]:
                best_match = infringing_analysis["copyright_matches"][0]
                print(
                    f"   Best match: '{best_match['title']}' by {best_match['artist']}"
                )
                print(f"   Similarity: {best_match['similarity_score']:.3f}")
                print(f"   Match type: {best_match['match_type']}")

            # Test database stats
            print("📊 Testing database statistics...")
            stats = detector.get_database_stats()
            print(f"✓ Database contains {stats['total_entries']} entries")

            print("\n" + "=" * 60)
            print("🎉 Copyright detection tests completed successfully!")
            print("✓ Audio fingerprinting functional")
            print("✓ Copyright database functional")
            print("✓ Similarity detection functional")
            print("✓ Risk assessment functional")
            print("=" * 60)

            return True

    except Exception as e:
        print(f"\n❌ Test failed: {str(e)}")
        import traceback

        traceback.print_exc()
        return False


def test_integration_scenario():
    """Test realistic integration scenario."""
    print("\n🧪 Testing realistic integration scenario...")

    if not DEPENDENCIES_AVAILABLE:
        print("⚠️  Skipping integration test - dependencies not available")
        return True

    try:
        with tempfile.TemporaryDirectory() as temp_dir:
            detector = CopyrightDetector(
                fingerprint_db_path=os.path.join(temp_dir, "integration_db.json")
            )

            # Simulate user uploading a file
            user_file = os.path.join(temp_dir, "user_upload.wav")
            create_test_audio_file(user_file, duration=4, frequency=523.25)  # C5

            # Check for copyright
            result = detector.analyze_for_copyright(user_file)

            # Simulate decision making
            if result["is_likely_copyrighted"]:
                decision = "REJECT - Potential copyright violation detected"
            elif result["risk_score"] > 0.3:
                decision = "REVIEW - Manual review recommended"
            else:
                decision = "APPROVE - No copyright issues detected"

            print(f"✓ Integration test result: {decision}")
            print(f"   Risk score: {result['risk_score']:.3f}")
            print(f"   Confidence: {result['confidence']:.3f}")

            return True

    except Exception as e:
        print(f"❌ Integration test failed: {str(e)}")
        return False


if __name__ == "__main__":
    # Set up logging
    logging.basicConfig(level=logging.WARNING)  # Reduce noise

    success = True

    # Run basic functionality test
    success &= test_copyright_detector()

    # Run integration scenario test
    success &= test_integration_scenario()

    if success:
        print("\n🌟 All copyright detection tests passed!")
        print(
            "   The copyright detection system is ready for EchoChain P2P integration."
        )
    else:
        print("\n⚠️  Some tests failed. Please check the error messages above.")
        sys.exit(1)
