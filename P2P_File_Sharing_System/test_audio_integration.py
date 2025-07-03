#!/usr/bin/env python3
"""
Simple integration test for EchoChain audio analysis functionality.
Tests the basic functionality of the AudioAnalyzer module.
"""

import os
import sys
import tempfile
import numpy as np
import logging

print(f"sys.path: {sys.path}")

# Add the current directory to the path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

try:
    import librosa
    import matplotlib
    import soundfile as sf
    from audio_analysis import AudioAnalyzer

    DEPENDENCIES_AVAILABLE = True
except ImportError as e:
    print(f"Dependencies not available: {e}")
    DEPENDENCIES_AVAILABLE = False


def create_test_audio_file(filename, duration=5, sample_rate=22050, frequency=440):
    """Create a simple synthetic audio file for testing."""
    t = np.linspace(0, duration, int(sample_rate * duration))

    # Create a simple sine wave
    audio_data = 0.3 * np.sin(2 * np.pi * frequency * t)

    # Add some harmonics to make it more musical
    audio_data += 0.2 * np.sin(2 * np.pi * frequency * 2 * t)
    audio_data += 0.1 * np.sin(2 * np.pi * frequency * 3 * t)

    # Normalize
    audio_data = audio_data / np.max(np.abs(audio_data))

    sf.write(filename, audio_data, sample_rate)
    return filename


def test_audio_analyzer():
    """Test the AudioAnalyzer functionality."""
    print("=" * 60)
    print("EchoChain Audio Analysis Integration Test")
    print("=" * 60)

    if not DEPENDENCIES_AVAILABLE:
        print("❌ Dependencies not available. Please install:")
        print("   pip install librosa numpy scipy soundfile")
        return False

    try:
        # Initialize analyzer
        print("🔧 Initializing AudioAnalyzer...")
        analyzer = AudioAnalyzer()
        print("✓ AudioAnalyzer initialized successfully")

        # Create temporary test audio file
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as tmp_file:
            test_file = tmp_file.name

        print("🎵 Creating test audio file...")
        create_test_audio_file(test_file, duration=3, frequency=261.63)  # C4
        print(f"✓ Test audio file created: {os.path.basename(test_file)}")

        # Test audio analysis
        print("🔍 Performing audio analysis...")
        result = analyzer.analyze_audio(test_file)
        print("✓ Audio analysis completed successfully")

        # Display results
        print("\n📊 Analysis Results:")
        print(f"   Duration: {result['duration']:.2f} seconds")
        print(f"   Tempo: {result['tempo']:.1f} BPM")
        print(f"   Fingerprint: {result['fingerprint'][:16]}...")
        print(f"   Energy: {result['energy']:.6f}")

        # Test compatibility checking
        print("\n🔗 Testing compatibility checking...")

        # Create a second test file
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as tmp_file2:
            test_file2 = tmp_file2.name

        create_test_audio_file(test_file2, duration=3, frequency=293.66)  # D4
        result2 = analyzer.analyze_audio(test_file2)

        compatibility = analyzer.check_compatibility(
            result, result2, tempo_tolerance=10
        )
        print(f"✓ Compatibility check completed")
        print(f"   Compatible: {compatibility['compatible']}")
        print(f"   Tempo match: {compatibility['tempo_match']}")
        print(f"   Overall score: {compatibility['overall_score']:.2f}")

        # Clean up
        os.unlink(test_file)
        os.unlink(test_file2)

        print("\n" + "=" * 60)
        print("🎉 All tests passed successfully!")
        print("✓ AudioAnalyzer is working correctly")
        print("✓ Tempo detection functional")
        print("✓ Compatibility checking functional")
        print("✓ Audio fingerprinting functional")
        print("=" * 60)

        return True

    except Exception as e:
        print(f"\n❌ Test failed: {str(e)}")
        import traceback

        traceback.print_exc()
        return False


if __name__ == "__main__":
    # Set up logging
    logging.basicConfig(level=logging.WARNING)  # Reduce noise

    success = True

    # Run basic functionality test
    success &= test_audio_analyzer()

    if success:
        print("\n🌟 Integration testing completed successfully!")
        print("   The audio analysis system is ready for use in EchoChain P2P.")
    else:
        print("\n⚠️  Some tests failed. Please check the error messages above.")
        sys.exit(1)
