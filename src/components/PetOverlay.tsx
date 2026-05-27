import React from "react";
import SpeechBubble from "./SpeechBubble";
import catSet1 from "../assets/cat-set-1/cat.svg";
import catSet2 from "../assets/cat-set-2/cat.svg";

interface PetOverlayProps {
  message: string;
  catSet?: "cat-set-1" | "cat-set-2";
  visible: boolean;
}

const CAT_IMAGES: Record<string, string> = {
  "cat-set-1": catSet1,
  "cat-set-2": catSet2,
};

const PetOverlay: React.FC<PetOverlayProps> = ({
  message,
  catSet = "cat-set-1",
  visible,
}) => {
  return (
    <div className={`overlay-container ${visible ? "overlay--visible" : "overlay--hidden"}`}>
      <SpeechBubble message={message} />
      <img
        src={CAT_IMAGES[catSet]}
        alt="Professor Posture Purrfect"
        className="cat-image"
        draggable={false}
      />
    </div>
  );
};

export default PetOverlay;
