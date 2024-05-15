<script>
  import {invoke} from "@tauri-apps/api";
  import Item from "./components/Item.svelte";
  import Search from "./components/Search.svelte";

  $: searchFile = "";
  $: currentPath = "home/name";
  $: items = [];

  async function init(){
    await invoke("get_home_path").then(home => {
     currentPath = home;
     invoke("get_all_ff",{path:home}).then(list => items = list); 
    })
  }

init();
</script>

<main>
  <div class="tree">
  </div>
  <div class="items">
   <Search searchFile={searchFile} currentPath={currentPath}/>
    <div class="files-folders">
      {#each items as item}
        <Item itemName={item.name}/>
      {/each}
    </div>
  </div>
</main>

<style>
  .tree {
    position: sticky;
    top:0;
    width: 160px;
    height: 100vh;
    align-self: flex-start;
    background-color: #21242b;
  }

  .items {
    height: auto;
    width: 100%;
    display: flex;
    gap: 10px;
    flex-direction: column;
    align-items: center;
  }

  .files-folders {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
  }
</style>