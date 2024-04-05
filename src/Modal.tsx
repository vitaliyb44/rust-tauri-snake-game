import { invoke, window } from "@tauri-apps/api";
import { Component, createSignal, onMount, Show } from "solid-js";
import {Title} from "./Title"
export const Modal: Component<{data: string, startGame: ()=>void}> = (props) => {
  const [score, setScore] = createSignal(0)
    onMount(async ()=>{
    if (props.data == "Win" || props.data == "Lose") {
        await invoke<number>("return_score").then((data:number)=>setScore(data))
    }
  })

  async function handleQuit() {
    await window.appWindow.close();
  }

  return( 
  <div class="modal">
    <Title />
    <Show when={props.data == "Win" || props.data == "Lose"} fallback={<></>}>
    <div><span class={`state-text ${props.data == "Win" ? "won" : "lost"}`}>{`You ${props.data}`.toUpperCase()}</span></div>
    <div class="score">Score: <span>{score()}</span></div>
    </Show>
    <div class="buttons">
    <button class="button button-start" type="button" onClick={()=>props.startGame()}>
        {(props.data != "Win" && props.data != "Lose") ? "Play" : "Play Again"}
    </button>
    <button onClick={handleQuit} class="button button-quit">
        Quit
    </button>
    </div>
  </div>
  );
};