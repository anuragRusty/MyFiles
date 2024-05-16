<script>
  import {invoke} from "@tauri-apps/api";
  import Item from "./components/Item.svelte";
  import Search from "./components/Search.svelte";
  import Nav from "./components/Nav.svelte";
  import Roots from "./components/Roots.svelte";
  
  let HOME;
  let undoStack = [];
  let redoStack = [];

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
       undoStack.push(currentPath);
       currentPath = path;
     }
    }

  function open_root(name){
    if(name == "Home"){
      init();
    }else{
      let path = HOME + "/" + name;
      open_folder(path,"Folder");
    }
  }

  async function handleUndo(){
    if(undoStack.length > 0){
      let path = undoStack.pop();
      await invoke("get_all_ff",{path:path}).then(newItems => items = newItems);
      redoStack.push(currentPath);
      currentPath = path;
    }
  }

  function handleRedo(){
    if(redoStack.length > 0){
      let path = redoStack.pop();
      open_folder(path,"Folder");
    }
  }

  function trimFileName(name){
    if(name.length > 12){
      return name.slice(0,11) + "...";
    }
    return name;
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
     <Nav handleBack={handleUndo}  handleUndo={handleRedo}/>
    </div>
    <div class="files-folders">
      {#each items as item}
        {#if !item.hidden}
         <Item itemSelected={item.selected} itemPath={item.path} itemIcon={item.file_type} itemName={trimFileName(item.name)} handleDoubleClick={open_folder}/>
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
    width: 260px;
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