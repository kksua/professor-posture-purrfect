import React from "react";

interface SpeechBubbleProps {
  message: string;
}

const SpeechBubble: React.FC<SpeechBubbleProps> = ({ message }) => {
  return (
    <div className="speech-bubble">
      <p className="speech-bubble__text">{message}</p>
    </div>
  );
};

export default SpeechBubble;
