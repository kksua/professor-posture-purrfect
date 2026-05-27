import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import PetOverlay from "./components/PetOverlay";
import { getRandomMessage } from "./data/messages";
import "./App.css";

function App() {
  const [message, setMessage] = useState<string>(() => getRandomMessage());
  const [visible, setVisible] = useState<boolean>(true);

  useEffect(() => {
    const unlistenPromise = listen<void>("show-pet", () => {
      setMessage(getRandomMessage());
      setVisible(true);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return <PetOverlay message={message} visible={visible} />;
}

export default App;
