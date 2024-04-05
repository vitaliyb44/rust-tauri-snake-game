import { Component, createSignal, For } from "solid-js";
import { Tile } from "./Tile";

export const Field: Component<{field: string[][]}> = (props) => {
  
  return (
    <>
    <div class="field-wrapper">
      </div>
  <div class="field">
  <For each={props.field} fallback={<div>Hmm</div>}>
    {(row, row_index)=>(
      <For each={row}>
        {(tile, tile_index)=><Tile data-index={`${row_index()}-${tile_index}`} item={tile} />}
    </For>
    )}
  </For>
  
  </div>
  </>);
};