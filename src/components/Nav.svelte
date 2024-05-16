<script>
  export let handleBack;
  export let handleUndo;
</script>

<div class="nav">
  <button on:click={() => handleBack()}>&#x2190;</button>
  <button on:click={() => handleUndo()}>&#x2192;</button>
</div>

<style>
    button{
        width: 30px;
        height: 30px;
        font-size: 18px;
    }
</style>

