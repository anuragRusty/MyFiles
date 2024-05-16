<script>
    export let handClick;
    let roots = [
     {name:"Home"},
     {name:"Documents"},
     {name:"Downloads"},
     {name:"Music"},
     {name:"Pictures"},
     {name:"Videos"},
    ];
</script>

  {#each roots as root }
    <button on:click={() => handClick(root.name)}> {root.name.toLocaleUpperCase()}</button>
  {/each}

<style>
 button{
  width: 100%;
  height: 48px;
  text-align: left;
  border-radius: 0;
  background-color: #21242b;
  outline: none;
  border: none;
  color:#f2f2f2;
  font-size: 16px;
  font-weight: bolder;
  padding-left: 16px;
}

button:hover{
  scale: 1.025;
  color:#e2e2e2;
  background-color: #111214;
}
</style>