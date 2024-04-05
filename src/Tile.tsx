import { invoke } from "@tauri-apps/api";
import { Component } from "solid-js";

export const Tile: Component<{item: string}> = ({item}) => {
  return <div class={`field-tile ${item.toLowerCase()}`}>
    
  </div>;
};