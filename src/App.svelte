<script>
  import {invoke} from "@tauri-apps/api";
  import Item from "./components/Item.svelte";
  import Search from "./components/Search.svelte";
  import Nav from "./components/Nav.svelte";
  import Roots from "./components/Roots.svelte";
  
  let HOME;
  $: searchFile = "";
  $: currentPath = HOME;
  $: items = [];

  async function init(){
    await invoke("get_home_path").then(home => {
     HOME = home;
     invoke("get_all_ff",{path:home}).then(list => items = list); 
    })
  }

  async function open_folder(path,type){
    if(type == "Folder"){
       await invoke("get_all_ff",{path:path}).then(newItems => items = newItems);
       currentPath = path;
       console.log("OPENED_FOLDER")
     }
    }

  function open_root(name){
    if(name == "Home"){
      init();
    }else{
      open_folder(HOME + "/" + name,"Folder");
    }
  }

init();
</script>

<main>
  <div class="tree">
    <Roots handClick={open_root}/>
  </div>
  <div class="items">
    <div class="navigations">
     <Search searchFile={searchFile} currentPath={currentPath}/>
     <Nav/>
    </div>
    <div class="files-folders">
      {#each items as item}
        {#if !item.hidden}
         <Item itemPath={item.path} itemIcon={item.file_type} itemName={item.name} handleDoubleClick={open_folder}/>
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
  }
</style>