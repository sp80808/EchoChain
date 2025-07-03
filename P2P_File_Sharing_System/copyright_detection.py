#!/usr/bin/env python3
"""
Copyright Detection and Audio Fingerprinting Module for EchoChain P2P File Sharing System.
Focuses on detecting copyrighted materials through audio fingerprinting and comparison.
"""

import os
import hashlib
import numpy as np
import logging
from typing import Dict, List, Optional, Tuple
import json
import time

try:
    import librosa
    import scipy.signal

    LIBROSA_AVAILABLE = True
except ImportError:
    LIBROSA_AVAILABLE = False
    logging.warning("librosa not available - copyright detection will be limited")

logger = logging.getLogger(__name__)


class CopyrightDetector:
    """
    Copyright detection system using audio fingerprinting.

    Features:
    - Audio fingerprinting for content identification
    - Spectral hash generation for efficient comparison
    - Copyright database management
    - Similarity detection and matching
    """

    def __init__(
        self, sample_rate: int = 22050, fingerprint_db_path: str = "copyright_db.json"
    ):
        """
        Initialize the CopyrightDetector.

        Args:
            sample_rate: Target sample rate for audio processing
            fingerprint_db_path: Path to copyright fingerprint database
        """
        self.sample_rate = sample_rate
        self.db_path = fingerprint_db_path
        self.logger = logging.getLogger(f"{__name__}.{self.__class__.__name__}")

        # Load existing copyright database
        self.copyright_db = self._load_copyright_database()

        if not LIBROSA_AVAILABLE:
            self.logger.warning("librosa not available - some features will be limited")

    def analyze_for_copyright(self, file_path: str) -> Dict:
        """
        Analyze an audio file for potential copyright violations.

        Args:
            file_path: Path to the audio file

        Returns:
            Dictionary containing copyright analysis results
        """
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"Audio file not found: {file_path}")

        try:
            # Generate fingerprint for the audio file
            fingerprint_data = self._generate_audio_fingerprint(file_path)

            # Check against known copyrighted content
            copyright_matches = self._check_copyright_database(fingerprint_data)

            # Calculate risk score
            risk_score = self._calculate_copyright_risk(
                copyright_matches, fingerprint_data
            )

            analysis_result = {
                "file_path": file_path,
                "timestamp": time.time(),
                "fingerprint": fingerprint_data["fingerprint_hash"],
                "spectral_hash": fingerprint_data["spectral_hash"],
                "duration": fingerprint_data["duration"],
                "copyright_matches": copyright_matches,
                "risk_score": risk_score,
                "is_likely_copyrighted": risk_score > 0.7,
                "confidence": self._calculate_confidence(copyright_matches),
            }

            self.logger.info(f"Copyright analysis completed for {file_path}")
            return analysis_result

        except Exception as e:
            self.logger.error(
                f"Error analyzing file for copyright {file_path}: {str(e)}"
            )
            raise

    def _generate_audio_fingerprint(self, file_path: str) -> Dict:
        """
        Generate audio fingerprint for copyright detection.

        Args:
            file_path: Path to the audio file

        Returns:
            Dictionary containing fingerprint data
        """
        if not LIBROSA_AVAILABLE:
            # Fallback to basic file hash if librosa not available
            return self._generate_basic_fingerprint(file_path)

        try:
            # Load audio file
            y, sr = librosa.load(file_path, sr=self.sample_rate)

            # Generate multiple types of fingerprints for robust detection
            fingerprint_data = {
                "duration": len(y) / sr,
                "sample_rate": sr,
                "file_size": os.path.getsize(file_path),
            }

            # 1. Spectral centroid fingerprint
            spectral_centroids = librosa.feature.spectral_centroid(y=y, sr=sr)
            fingerprint_data["spectral_centroid_hash"] = self._hash_feature(
                spectral_centroids
            )

            # 2. Chromagram fingerprint
            chroma = librosa.feature.chroma_stft(y=y, sr=sr)
            fingerprint_data["chroma_hash"] = self._hash_feature(chroma)

            # 3. MFCC fingerprint
            mfccs = librosa.feature.mfcc(y=y, sr=sr, n_mfcc=13)
            fingerprint_data["mfcc_hash"] = self._hash_feature(mfccs)

            # 4. Spectral rolloff fingerprint
            spectral_rolloff = librosa.feature.spectral_rolloff(y=y, sr=sr)
            fingerprint_data["rolloff_hash"] = self._hash_feature(spectral_rolloff)

            # 5. Zero crossing rate fingerprint
            zcr = librosa.feature.zero_crossing_rate(y)
            fingerprint_data["zcr_hash"] = self._hash_feature(zcr)

            # 6. Onset detection fingerprint
            onset_frames = librosa.onset.onset_detect(y=y, sr=sr)
            fingerprint_data["onset_hash"] = self._hash_array(onset_frames)

            # Combine all hashes into a composite fingerprint
            composite_features = [
                fingerprint_data["spectral_centroid_hash"],
                fingerprint_data["chroma_hash"],
                fingerprint_data["mfcc_hash"],
                fingerprint_data["rolloff_hash"],
                fingerprint_data["zcr_hash"],
                fingerprint_data["onset_hash"],
            ]

            fingerprint_data["fingerprint_hash"] = hashlib.sha256(
                "".join(composite_features).encode()
            ).hexdigest()

            # Generate spectral hash for efficient comparison
            fingerprint_data["spectral_hash"] = self._generate_spectral_hash(y, sr)

            return fingerprint_data

        except Exception as e:
            self.logger.warning(
                f"Advanced fingerprinting failed, using basic: {str(e)}"
            )
            return self._generate_basic_fingerprint(file_path)

    def _generate_basic_fingerprint(self, file_path: str) -> Dict:
        """
        Generate basic fingerprint when librosa is not available.

        Args:
            file_path: Path to the audio file

        Returns:
            Basic fingerprint data
        """
        with open(file_path, "rb") as f:
            file_content = f.read()

        file_hash = hashlib.sha256(file_content).hexdigest()
        file_size = len(file_content)

        return {
            "fingerprint_hash": file_hash,
            "spectral_hash": file_hash[:32],  # Use part of file hash
            "duration": 0,  # Unknown without audio analysis
            "file_size": file_size,
            "basic_fingerprint": True,
        }

    def _generate_spectral_hash(self, y: np.ndarray, sr: int) -> str:
        """
        Generate spectral hash for efficient similarity comparison.

        Args:
            y: Audio time series
            sr: Sample rate

        Returns:
            Spectral hash string
        """
        try:
            # Compute spectrogram
            stft = librosa.stft(y)
            magnitude = np.abs(stft)

            # Downsample to reduce size
            magnitude_downsampled = magnitude[::4, ::4]  # Take every 4th sample

            # Convert to binary based on median threshold
            median_val = np.median(magnitude_downsampled)
            binary_matrix = magnitude_downsampled > median_val

            # Convert binary matrix to hash
            binary_string = "".join(binary_matrix.flatten().astype(int).astype(str))

            # Hash the binary string
            spectral_hash = hashlib.md5(binary_string.encode()).hexdigest()

            return spectral_hash

        except Exception as e:
            self.logger.warning(f"Spectral hash generation failed: {str(e)}")
            return hashlib.md5(str(np.random.random()).encode()).hexdigest()

    def _hash_feature(self, feature: np.ndarray) -> str:
        """
        Hash a feature array for fingerprinting.

        Args:
            feature: Feature array

        Returns:
            Hash string
        """
        # Take mean over time axis
        feature_mean = np.mean(feature, axis=1) if feature.ndim > 1 else feature

        # Quantize to create stable hash
        quantized = np.round(feature_mean * 1000).astype(int)

        # Convert to string and hash
        feature_str = ",".join(map(str, quantized))
        return hashlib.md5(feature_str.encode()).hexdigest()

    def _hash_array(self, arr: np.ndarray) -> str:
        """
        Hash a numpy array.

        Args:
            arr: Numpy array

        Returns:
            Hash string
        """
        arr_str = ",".join(map(str, arr.flatten()))
        return hashlib.md5(arr_str.encode()).hexdigest()

    def _check_copyright_database(self, fingerprint_data: Dict) -> List[Dict]:
        """
        Check fingerprint against copyright database.

        Args:
            fingerprint_data: Fingerprint data to check

        Returns:
            List of potential copyright matches
        """
        matches = []

        for entry_id, entry in self.copyright_db.items():
            similarity_score = self._calculate_similarity(fingerprint_data, entry)

            if similarity_score > 0.5:  # Threshold for potential match
                matches.append(
                    {
                        "entry_id": entry_id,
                        "title": entry.get("title", "Unknown"),
                        "artist": entry.get("artist", "Unknown"),
                        "similarity_score": similarity_score,
                        "match_type": self._determine_match_type(similarity_score),
                    }
                )

        # Sort by similarity score (highest first)
        matches.sort(key=lambda x: x["similarity_score"], reverse=True)

        return matches

    def _calculate_similarity(self, fingerprint1: Dict, fingerprint2: Dict) -> float:
        """
        Calculate similarity between two fingerprints.

        Args:
            fingerprint1: First fingerprint
            fingerprint2: Second fingerprint

        Returns:
            Similarity score (0-1)
        """
        similarities = []

        # Compare different hash types
        hash_types = [
            "spectral_centroid_hash",
            "chroma_hash",
            "mfcc_hash",
            "rolloff_hash",
            "zcr_hash",
            "onset_hash",
            "spectral_hash",
        ]

        for hash_type in hash_types:
            if hash_type in fingerprint1 and hash_type in fingerprint2:
                # Calculate Hamming distance for hash comparison
                hash1 = fingerprint1[hash_type]
                hash2 = fingerprint2[hash_type]

                if len(hash1) == len(hash2):
                    hamming_distance = sum(c1 != c2 for c1, c2 in zip(hash1, hash2))
                    similarity = 1 - (hamming_distance / len(hash1))
                    similarities.append(similarity)

        # Return average similarity if we have comparisons
        return np.mean(similarities) if similarities else 0.0

    def _calculate_copyright_risk(
        self, matches: List[Dict], fingerprint_data: Dict
    ) -> float:
        """
        Calculate copyright risk score.

        Args:
            matches: List of copyright matches
            fingerprint_data: Fingerprint data

        Returns:
            Risk score (0-1)
        """
        if not matches:
            return 0.0

        # Base risk on highest similarity match
        max_similarity = max(match["similarity_score"] for match in matches)

        # Adjust based on number of matches
        num_matches = len(matches)
        match_factor = min(num_matches / 10, 1.0)  # Cap at 1.0

        # Combine factors
        risk_score = (max_similarity * 0.7) + (match_factor * 0.3)

        return min(risk_score, 1.0)

    def _calculate_confidence(self, matches: List[Dict]) -> float:
        """
        Calculate confidence in copyright detection.

        Args:
            matches: List of matches

        Returns:
            Confidence score (0-1)
        """
        if not matches:
            return 0.95  # High confidence in "no copyright"

        # Confidence based on match quality and consistency
        avg_similarity = np.mean([match["similarity_score"] for match in matches])

        # Higher similarity = higher confidence
        return min(avg_similarity + 0.2, 1.0)

    def _determine_match_type(self, similarity_score: float) -> str:
        """
        Determine the type of copyright match.

        Args:
            similarity_score: Similarity score

        Returns:
            Match type string
        """
        if similarity_score > 0.9:
            return "exact_match"
        elif similarity_score > 0.8:
            return "strong_match"
        elif similarity_score > 0.6:
            return "potential_match"
        else:
            return "weak_match"

    def add_copyrighted_content(
        self, file_path: str, title: str, artist: str, label: str = "", year: int = None
    ) -> str:
        """
        Add a file to the copyright database.

        Args:
            file_path: Path to copyrighted audio file
            title: Title of the work
            artist: Artist name
            label: Record label
            year: Release year

        Returns:
            Entry ID in database
        """
        try:
            fingerprint_data = self._generate_audio_fingerprint(file_path)

            entry_id = hashlib.sha256(
                f"{title}_{artist}_{time.time()}".encode()
            ).hexdigest()[:16]

            self.copyright_db[entry_id] = {
                "title": title,
                "artist": artist,
                "label": label,
                "year": year,
                "added_timestamp": time.time(),
                "file_path": file_path,
                **fingerprint_data,
            }

            # Save updated database
            self._save_copyright_database()

            self.logger.info(f"Added copyrighted content: {title} by {artist}")
            return entry_id

        except Exception as e:
            self.logger.error(f"Error adding copyrighted content: {str(e)}")
            raise

    def _load_copyright_database(self) -> Dict:
        """
        Load copyright database from file.

        Returns:
            Copyright database dictionary
        """
        if os.path.exists(self.db_path):
            try:
                with open(self.db_path, "r") as f:
                    return json.load(f)
            except Exception as e:
                self.logger.warning(f"Could not load copyright database: {str(e)}")

        return {}

    def _save_copyright_database(self):
        """Save copyright database to file."""
        try:
            with open(self.db_path, "w") as f:
                json.dump(self.copyright_db, f, indent=2)
        except Exception as e:
            self.logger.error(f"Could not save copyright database: {str(e)}")

    def get_database_stats(self) -> Dict:
        """
        Get statistics about the copyright database.

        Returns:
            Database statistics
        """
        return {
            "total_entries": len(self.copyright_db),
            "database_path": self.db_path,
            "database_size_mb": (
                os.path.getsize(self.db_path) / (1024 * 1024)
                if os.path.exists(self.db_path)
                else 0
            ),
        }


# Example usage
if __name__ == "__main__":
    # Set up logging
    logging.basicConfig(level=logging.INFO)

    detector = CopyrightDetector()
    print("CopyrightDetector initialized successfully!")
    print("Database stats:", detector.get_database_stats())
