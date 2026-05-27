import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import PetOverlay from "./components/PetOverlay";
import { getRandomMessage } from "./data/messages";
import "./App.css";

const VISIBLE_DURATION_MS = 15000; // must match the 15-second hide in lib.rs

function App() {
  const [message, setMessage] = useState<string>(() => getRandomMessage());
  const [visible, setVisible] = useState<boolean>(false);
  const hideTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    const unlistenPromise = listen<void>("show-pet", () => {
      setMessage(getRandomMessage());
      setVisible(true);

      // Clear any previous hide timer so rapid "show-now" clicks don't race
      if (hideTimerRef.current !== null) {
        clearTimeout(hideTimerRef.current);
      }

      // Mirror the 15-second window.hide() in Rust so the animation stops cleanly
      hideTimerRef.current = setTimeout(() => {
        setVisible(false);
        hideTimerRef.current = null;
      }, VISIBLE_DURATION_MS);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
      if (hideTimerRef.current !== null) {
        clearTimeout(hideTimerRef.current);
      }
    };
  }, []);

  return <PetOverlay message={message} visible={visible} />;
}

export default App;
