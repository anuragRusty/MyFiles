<script>
  import {invoke} from "@tauri-apps/api";
  import Item from "./components/Item.svelte";
  import Search from "./components/Search.svelte";
  import Nav from "./components/Nav.svelte";
  import Roots from "./components/Roots.svelte";

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
    <Roots/>
  </div>
  <div class="items">
    <div class="navigations">
     <Search searchFile={searchFile} currentPath={currentPath}/>
     <Nav/>
    </div>
    <div class="files-folders">
      {#each items as item}
        {#if !item.hidden}
         <Item itemIcon={item.file_type} itemName={item.name}/>
        {/if}
      {/each}
    </div>
  </div>
</main>

<style>
  .tree {
    position: sticky;
    top:0;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 160px;
    height: 100vh;
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

  .navigations{
    position: sticky;
    top:0;
    width: 100%;
    display: flex;
    gap: 12px;
    justify-content: center;
    background-color: #282c34;
  }

  .files-folders {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
  }
</style>