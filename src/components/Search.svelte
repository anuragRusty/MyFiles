<script>
    export let searchFile;
    export let currentPath;
</script>

<input class="search" type="text" bind:value={searchFile} placeholder={currentPath} />

<style>
    .search {
    position: sticky;
    top:0;
    height: 24px;
    width: 360px;
    font-size: medium;
  }
</style>