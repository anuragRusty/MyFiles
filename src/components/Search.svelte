<script>
    export let searchFile;
    export let currentPath;
</script>

<input class="search" type="text" bind:value={searchFile} placeholder={currentPath} />

<style>
    .search {
    height: 25px;
    width: 360px;
    font-size: medium;
  }
</style>