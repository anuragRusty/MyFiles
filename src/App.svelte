<script>
  import {invoke} from "@tauri-apps/api";
  import Item from "./components/Item.svelte";
  import Search from "./components/Search.svelte";

  let searchFile = "";
  let items = [1, 2, 3, 4, 5, 7, 8, 9, 10,11,12,13,15,17,18,19];
  let currentPath = "home/name";
  invoke("get_home_path").then((home) => currentPath = home);

</script>

<main>
  <div class="tree">
  </div>
  <div class="items">
    <Search searchFile={searchFile} currentPath={currentPath}/>
    <div class="files-folders">
      {#each items as itemName}
        <Item itemName={itemName}/>
      {/each}
    </div>
  </div>
</main>

<style>
  .tree {
    display: flex;
    width: 160px;
    background-color: #21242b;
  }

  .items {
    height: 100vh;
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