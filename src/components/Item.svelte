<script>
  import { fade } from "svelte/transition";
  import File from "/file.png";
  import Folder from "/folder.png";

//export let isFile;
//export let itemType;

let icons = {
   "File":File,
   "Folder":Folder,
}

export let itemName;
export let itemIcon;
export let itemPath;
export let itemSelected;
export let handleDoubleClick;
</script>

<button in:fade  class="item" draggable="true" on:dblclick={() => {handleDoubleClick(itemPath,itemIcon)}} on:click={() => itemSelected = !itemSelected}>
    <img class:selected-icon={itemSelected} src={icons[itemIcon]} alt="file" class="item-icon">
    <span class:selected-text={itemSelected} class="item-name">{itemName}</span>
</button>

<style>
  .item {
    width: 72px;
    aspect-ratio: 1/1;
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: transparent;
    border: none;
    color: #f2f2f2;
  }

  .item:hover{
    scale: 1.1;
  }

  .item-icon {
    width: 64px;
    height: 64px;
  }

  .item-name {
    text-align: center;
    font-size: small;
  }

  .selected-text{
    background-color: skyblue;
    color:black;
  }
</style>