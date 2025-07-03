#!/usr/bin/env python3
"""
Comprehensive audio analysis module for EchoChain P2P file sharing system.
Provides tempo analysis, audio fingerprinting, and compatibility checking.
"""

import os
import hashlib
import numpy as np
import logging
from typing import Dict

# Test imports more carefully
LIBROSA_AVAILABLE = False
AUBIO_AVAILABLE = False

try:
    import librosa

    LIBROSA_AVAILABLE = True
    print("✓ librosa successfully imported in audio_analysis module")
except ImportError as e:
    print(f"❌ librosa import failed: {e}")

try:
    import aubio

    AUBIO_AVAILABLE = True
    print("✓ aubio successfully imported")
except ImportError as e:
    print(f"ℹ️ aubio not available (optional): {e}")

logger = logging.getLogger(__name__)


class AudioAnalyzer:
    """
    Comprehensive audio analysis system for musical content.

    Features:
    # - Musical key detection using chromagram analysis
    - Tempo (BPM) detection with beat tracking
    - Audio fingerprinting for content identification
    - Spectral feature extraction (MFCCs, spectral centroid, etc.)
    - Musical compatibility checking
    """

    def __init__(self, sample_rate: int = 22050):
        """
        Initialize the AudioAnalyzer.

        Args:
            sample_rate: Target sample rate for audio processing
        """
        self.sample_rate = sample_rate
        self.logger = logging.getLogger(
            f"{__name__}.{self.__class__.__name__}"
        )

        if not LIBROSA_AVAILABLE:
            raise ImportError(
                "librosa is required for audio analysis. Install with: pip install librosa."
            )

    def analyze_audio(self, file_path: str) -> Dict:
        """
        Perform comprehensive audio analysis on an audio file.

        Args:
            file_path: Path to the audio file

        Returns:
            Dictionary containing analysis results
        """
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"Audio file not found: {file_path}")

        try:
            # Load audio file
            y, sr = librosa.load(file_path, sr=self.sample_rate)

            # Perform various analyses
            analysis_result = {
                "file_path": file_path,
                "duration": len(y) / sr,
                "sample_rate": sr,
                "tempo": self._detect_tempo(y, sr),
                "fingerprint": self._generate_fingerprint(y),
                "features": self._extract_features(y, sr),
                "energy": float(np.mean(y**2)),
                "spectral_features": self._extract_spectral_features(y, sr),
            }

            self.logger.info(f"Audio analysis completed for {file_path}")
            return analysis_result

        except Exception as e:
            self.logger.error(f"Error analyzing audio file {file_path}: {str(e)}")
            raise

    def _detect_tempo(self, y: np.ndarray, sr: int) -> float:
        """
        Detect the tempo (BPM) of the audio.

        Args:
            y: Audio time series
            sr: Sample rate

        Returns:
            Estimated tempo in BPM
        """
        try:
            if AUBIO_AVAILABLE:
                # Use aubio for tempo detection if available
                return self._detect_tempo_aubio(y, sr)
            else:
                # Use librosa for tempo detection
                tempo, _ = librosa.beat.beat_track(y=y, sr=sr)
                return float(tempo)
        except Exception as e:
            self.logger.warning(f"Tempo detection failed: {str(e)}")
            return 120.0  # Default tempo

    def _detect_tempo_aubio(self, y: np.ndarray, sr: int) -> float:
        """
        Detect tempo using aubio library.

        Args:
            y: Audio time series
            sr: Sample rate

        Returns:
            Estimated tempo in BPM
        """
        # Convert numpy array to aubio format
        hop_s = 512
        o = aubio.tempo("default", 1024, hop_s, sr)

        # Process audio in chunks
        tempos = []
        for i in range(0, len(y) - hop_s, hop_s):
            chunk = y[i : i + hop_s].astype(np.float32)
            is_beat = o(chunk)
            if is_beat[0]:
                tempos.append(o.get_bpm())

        return float(np.mean(tempos)) if tempos else 120.0

    def _generate_fingerprint(self, y: np.ndarray) -> str:
        """
        Generate a unique fingerprint for the audio content.

        Args:
            y: Audio time series

        Returns:
            Hexadecimal fingerprint string
        """
        try:
            # Extract MFCCs for fingerprinting
            mfccs = librosa.feature.mfcc(
                y=y, sr=self.sample_rate, n_mfcc=13
            )

            # Take mean of MFCCs over time
            mfcc_mean = np.mean(mfccs, axis=1)

            # Quantize to create binary fingerprint
            mfcc_binary = (mfcc_mean > np.median(mfcc_mean)).astype(int)

            # Convert to string and hash
            fingerprint_str = "".join(map(str, mfcc_binary))
            fingerprint_hash = hashlib.md5(fingerprint_str.encode()).hexdigest()

            return fingerprint_hash

        except Exception as e:
            self.logger.warning(f"Fingerprint generation failed: {str(e)}")
            return hashlib.md5(str(np.random.random()).encode()).hexdigest()

    def _extract_features(self, y: np.ndarray, sr: int) -> Dict:
        """
        Extract various audio features.

        Args:
            y: Audio time series
            sr: Sample rate

        Returns:
            Dictionary of extracted features
        """
        try:
            features = {}

            # MFCCs
            mfccs = librosa.feature.mfcc(y=y, sr=sr, n_mfcc=13)
            features["mfcc_mean"] = np.mean(mfccs, axis=1).tolist()
            features["mfcc_std"] = np.std(mfccs, axis=1).tolist()

            # Zero crossing rate
            zcr = librosa.feature.zero_crossing_rate(y)
            features["zcr_mean"] = float(np.mean(zcr))
            features["zcr_std"] = float(np.std(zcr))

            # RMS energy
            rms = librosa.feature.rms(y=y)
            features["rms_mean"] = float(np.mean(rms))
            features["rms_std"] = float(np.std(rms))

            return features

        except Exception as e:
            self.logger.warning(f"Feature extraction failed: {str(e)}")
            return {}

    def _extract_spectral_features(self, y: np.ndarray, sr: int) -> Dict:
        """
        Extract spectral features from the audio.

        Args:
            y: Audio time series
            sr: Sample rate

        Returns:
            Dictionary of spectral features
        """
        try:
            spectral_features = {}

            # Spectral centroid
            spectral_centroids = librosa.feature.spectral_centroid(y=y, sr=sr)
            spectral_features["centroid_mean"] = float(np.mean(spectral_centroids))
            spectral_features["centroid_std"] = float(np.std(spectral_centroids))

            # Spectral bandwidth
            spectral_bandwidth = librosa.feature.spectral_bandwidth(y=y, sr=sr)
            spectral_features["bandwidth_mean"] = float(np.mean(spectral_bandwidth))
            spectral_features["bandwidth_std"] = float(np.std(spectral_bandwidth))

            # Spectral rolloff
            spectral_rolloff = librosa.feature.spectral_rolloff(y=y, sr=sr)
            spectral_features["rolloff_mean"] = float(np.mean(spectral_rolloff))
            spectral_features["rolloff_std"] = float(np.std(spectral_rolloff))

            return spectral_features

        except Exception as e:
            self.logger.warning(f"Spectral feature extraction failed: {str(e)}")
            return {}

    def check_compatibility(
        self, analysis1: Dict, analysis2: Dict, tempo_tolerance: float = 10.0
    ) -> Dict:
        """
        Check musical compatibility between two analyzed audio files.

        Args:
            analysis1: Analysis results from first file
            analysis2: Analysis results from second file

            tempo_tolerance: Maximum BPM difference for tempo compatibility

        Returns:
            Dictionary with compatibility information
        """
        try:
            compatibility = {
                "compatible": False,
                "tempo_match": False,
                "tempo_distance": None,
                "overall_score": 0.0,
            }

            # Check tempo compatibility
            tempo1 = analysis1.get("tempo", 120.0)
            tempo2 = analysis2.get("tempo", 120.0)

            tempo_distance = abs(tempo1 - tempo2)
            compatibility["tempo_distance"] = tempo_distance
            compatibility["tempo_match"] = tempo_distance <= tempo_tolerance

            # Calculate overall compatibility score
            tempo_score = max(0, 1 - (tempo_distance / 60))  # Normalize to 0-1

            compatibility["overall_score"] = tempo_score
            compatibility["compatible"] = compatibility["tempo_match"]

            return compatibility

        except Exception as e:
            self.logger.error(f"Compatibility check failed: {str(e)}")
            return {"compatible": False, "error": str(e)}


# Example usage and testing
if __name__ == "__main__":
    # Set up logging
    logging.basicConfig(level=logging.INFO)

    analyzer = AudioAnalyzer()

    # Example usage (requires an actual audio file)
    print("AudioAnalyzer initialized successfully!")
