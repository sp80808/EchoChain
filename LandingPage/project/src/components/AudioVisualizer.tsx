import React, { useRef, useEffect } from 'react';

const AudioVisualizer: React.FC = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const analyserRef = useRef<AnalyserNode | null>(null);
  const animationFrameIdRef = useRef<number | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Initialize AudioContext (browser compatibility)
    audioContextRef.current = new (window.AudioContext || (window as any).webkitAudioContext)();
    const audioContext = audioContextRef.current;

    // Create AnalyserNode
    analyserRef.current = audioContext.createAnalyser();
    const analyser = analyserRef.current;
    analyser.fftSize = 256; // Number of data points
    const bufferLength = analyser.frequencyBinCount; // Half of fftSize
    const dataArray = new Uint8Array(bufferLength);

    // Create a dummy audio source (e.g., oscillator) for visualization without actual audio input
    const oscillator = audioContext.createOscillator();
    oscillator.type = 'sine';
    oscillator.frequency.setValueAtTime(440, audioContext.currentTime);
    oscillator.connect(analyser);
    analyser.connect(audioContext.destination);
    oscillator.start();

    const draw = () => {
      animationFrameIdRef.current = requestAnimationFrame(draw);

      analyser.getByteFrequencyData(dataArray);

      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = 'rgba(0, 0, 0, 0)'; // Transparent background
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const barWidth = (canvas.width / bufferLength) * 2.5;
      let x = 0;

      for (let i = 0; i < bufferLength; i++) {
        const barHeight = dataArray[i] / 2; // Scale height

        // Gradient colors for bars
        const gradient = ctx.createLinearGradient(0, canvas.height, 0, 0);
        gradient.addColorStop(0, '#4F46E5'); // Blue
        gradient.addColorStop(1, '#8B5CF6'); // Purple
        ctx.fillStyle = gradient;

        ctx.fillRect(x, canvas.height - barHeight, barWidth, barHeight);

        x += barWidth + 1; // Add a small gap between bars
      }
    };

    draw();

    return () => {
      // Cleanup on unmount
      if (animationFrameIdRef.current) {
        cancelAnimationFrame(animationFrameIdRef.current);
      }
      if (oscillator) {
        oscillator.stop();
        oscillator.disconnect();
      }
      if (analyser) {
        analyser.disconnect();
      }
      if (audioContext) {
        audioContext.close();
      }
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full z-0 opacity-20"
      style={{ pointerEvents: 'none' }}
    />
  );
};

export default AudioVisualizer;
