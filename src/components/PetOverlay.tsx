import React, { useEffect, useRef, useState } from "react";
import SpeechBubble from "./SpeechBubble";

// Cat set 1 — two animation frames
import cat1Frame1 from "../assets/cat-set-1/cat1-1.png";
import cat1Frame2 from "../assets/cat-set-1/cat1-2.png";

// Cat set 2 — two animation frames
import cat2Frame1 from "../assets/cat-set-2/cat2-1.png";
import cat2Frame2 from "../assets/cat-set-2/cat2-2.png";

interface PetOverlayProps {
  message: string;
  catSet?: "cat-set-1" | "cat-set-2";
  visible: boolean;
}

// Each cat set has exactly two frames: [idle, alt]
const CAT_FRAMES: Record<string, [string, string]> = {
  "cat-set-1": [cat1Frame1, cat1Frame2],
  "cat-set-2": [cat2Frame1, cat2Frame2],
};

const FRAME_INTERVAL_MS = 3000; // switch frame every 3 seconds

const PetOverlay: React.FC<PetOverlayProps> = ({
  message,
  catSet = "cat-set-1",
  visible,
}) => {
  const [frameIndex, setFrameIndex] = useState(0);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (visible) {
      // Reset to first frame whenever the overlay becomes visible
      setFrameIndex(0);

      // Alternate between frame 0 and frame 1 every 3 seconds
      intervalRef.current = setInterval(() => {
        setFrameIndex((prev) => (prev === 0 ? 1 : 0));
      }, FRAME_INTERVAL_MS);
    } else {
      // Stop animating and reset when hidden
      if (intervalRef.current !== null) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      setFrameIndex(0);
    }

    return () => {
      if (intervalRef.current !== null) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, [visible]);

  const currentFrame = CAT_FRAMES[catSet][frameIndex];

  return (
    <div
      className={`overlay-container ${
        visible ? "overlay--visible" : "overlay--hidden"
      }`}
    >
      <SpeechBubble message={message} />
      <img
        src={currentFrame}
        alt="Professor Posture Purrfect"
        className="cat-image"
        draggable={false}
      />
    </div>
  );
};

export default PetOverlay;
