import "./App.css";

import { createSignal, onMount } from "solid-js";
import { invoke } from '@tauri-apps/api/core';


function App() {
  const [sceneDescription, setSceneDescription] = createSignal("");
  const [health, setHealth] = createSignal(100);
  const [isEndOfDungeon, setIsEndOfDungeon] = createSignal(false);

  const startGame = async () => {
    console.log("Starting game...");
    await invoke("start_game");
    initialRoomDescription();
  };

  const initialRoomDescription = async () => {
    const updatedDescription = await invoke("get_room_description") as string;
    setSceneDescription(updatedDescription);
  };

  const handleMove = async () => {
    const moveAction = await invoke("player_action", { action: "move" }) as string;
    setSceneDescription(moveAction);
    if (moveAction.includes("You have reached the end")) {
      setIsEndOfDungeon(true);
    }
  };

  const handleAttack = async () => {
    let attackOutcome = await invoke("player_action", { action: "attack" }) as string;
    if (attackOutcome.includes("damage")) {
      const healthMatch = attackOutcome.match(/Health: (\d+)/);
      if (healthMatch) setHealth(parseInt(healthMatch[1]));
      attackOutcome = attackOutcome.replace(/Health: \d+/, '').trim();
    }
    if (health() <= 0) {
      setSceneDescription("You have been defeated!");
      setIsEndOfDungeon(true);
      return;
    }
    setSceneDescription(attackOutcome);
  };

  //TODO discuss
  const handleCloseGame = async () => {
    //await exit(1);
    await invoke("exit_game");
  };

  onMount(startGame);

  return (
    <div>
      <h1>Wraithspire</h1>
      <p>{sceneDescription()}</p>
      <p>Health: {health()}</p>
      {isEndOfDungeon() ? (
          <button onClick={handleCloseGame}>Close Game</button>
      ) : (
        <>
          <button onClick={handleMove}>Move Forward</button>
          <button onClick={handleAttack}>Attack</button>
        </>
      )}
    </div>
  );
}

export default App;
