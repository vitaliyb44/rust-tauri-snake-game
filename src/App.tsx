import { createSignal, on, onCleanup, onMount, Show } from "solid-js";

import { invoke } from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event"
import { Field } from "./Field";
import { Modal } from "./Modal";
import { window as tauriWindow } from "@tauri-apps/api";
function App() {

  

  const [count, setCount] = createSignal(0)
  const [gameState, setGameState] = createSignal("")
  const [gameEnded, setGameEnded] = createSignal(false)
  const [field, setField] = createSignal<string[][]>([])
  const[currentDirection, setCurrentDirection] =  createSignal('d')

  const myKeys = ['w','a','s','d']
  let myTimer;

  function handleKeyboardInput(event: KeyboardEvent) {
    if (myKeys.includes(event.key)) {
      setCurrentDirection(event.key)
    }
  }

  async function initializeUpdateInterval() {
    let lastTick = performance.now();
    let tickDifference = 75
    
    async function gameLoop() {
      let currentDifference = performance.now() - lastTick;
      if (currentDifference >= tickDifference) {
        lastTick = performance.now()
        let updateSnake = await invoke<string[][]>("update_snake", {direction: currentDirection()})
        let returnGameState = await invoke<string>("return_game_state")
        
        await Promise.all([updateSnake, returnGameState]).then(([newField, newGameState])=>{ 
        setField(newField)
        setGameState(newGameState)
        }).then(()=>{
          
          if (gameState() == "Ongoing") {
            window.requestAnimationFrame(gameLoop);
          }
        })
      } else {
        window.requestAnimationFrame(gameLoop);
        return
      }
  }
  if (gameState() == "Win" || gameState() == "Lose") {
    
    await invoke("setup_game")
    await invoke("start_game")
    await invoke<string>("return_game_state").then((data: string) => {
      setCurrentDirection('d')
      setGameState(data); 
    window.requestAnimationFrame(gameLoop)})
  } else if (gameState() == "Starting") {
    await invoke("start_game").then(()=>window.requestAnimationFrame(gameLoop))
  } else {
    window.requestAnimationFrame(gameLoop)
  }
}
  onMount(async ()=>{
    document.addEventListener("keydown", handleKeyboardInput)
    await invoke("setup_game")
    await invoke<string>("return_game_state").then((data: string) => setGameState(data))
    await invoke<string[][]>("initialize_field").then((data: string[][])=> setField(data))
  })

  return (
    <div draggable={true} onDragStart={async ()=> { await tauriWindow.appWindow.startDragging()}} class="game-container">
      <Show when={gameState()=="Starting"} fallback={null}>
        <Modal data={gameState()} startGame={initializeUpdateInterval} />
      </Show>
      <Show when={gameState() == "Win" || gameState() == "Lose"} fallback={null}><Modal data={gameState()} startGame={initializeUpdateInterval} /></Show>
      <Field field={field()} />
    </div>
  );
}

export default App;
