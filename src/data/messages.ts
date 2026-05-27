export const POSTURE_MESSAGES: string[] = [
  "Your shoulders are doing a sad shrimp pose.",
  "Sit tall, human.",
  "Uncurl your back, bestie.",
  "Neck check. You are not a turtle.",
  "Your spine called. It wants support.",
  "Chin up! No, not like that. Less screen, more sky.",
  "Your future self thanks you for sitting up right now.",
  "Pretend there's a book on your head. Now sit.",
  "Roll those shoulders back. There you go.",
  "Screen too close? Your eyes think so too.",
  "Feet flat on the floor. Back off the chair back.",
  "You are not a question mark. Straighten up.",
  "Deep breath + tall spine = power combo.",
  "Slouching is so last posture check.",
  "The desk is not a pillow. Sit up!",
  "Your neck deserves a break. Look up.",
  "Unclench your jaw while you're at it.",
  "Shoulders: down. Back: straight. You've got this.",
  "Hello from Professor Posture. Please sit correctly.",
  "Align that spine, scholar.",
];

export function getRandomMessage(): string {
  return POSTURE_MESSAGES[Math.floor(Math.random() * POSTURE_MESSAGES.length)];
}
