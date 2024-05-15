<script>
    let roots = [
     {name:"RECENT",icon:"&#10226;"},
     {name:"HOME",icon:"&#127968;"},
     {name:"DOCUMENTS",icon:"&#128452;"},
     {name:"DOWLOADS",icon:"&#8681;"},
     {name:"MUSIC",icon:"&#127925;"},
     {name:"PICTURES",icon:"&#127968;"},
     {name:"VIDEOS",icon:"&#127968;"},
     {name:"TRASH",icon:"&#127968;"},
    ];
</script>

  {#each roots as root }
    <button> {root.name}</button>
  {/each}

<style>
 button{
  width: 100%;
  height: 30px;
  text-align: left;
  border-radius: 0;
  background-color: #21242b;
  outline: none;
  border: none;
  color:#f2f2f2;
  font-size: medium;
  font-weight: bolder;
}

button:hover{
  scale: 1.025;
  color:#e2e2e2;
  background-color: #111214;
}
</style>