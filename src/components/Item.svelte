<script>
//export let isFile;
//export let itemType;
//export let itemIcon;
export let itemName;
</script>

<div class="item">
    <div class="item-icon"></div>
    <span class="item-name">File{itemName}</span>
</div>

<style>
  .item {
    width: 84px;
    margin: 8px 0% 0% 8px;
    aspect-ratio: 1/1;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .item-icon {
    border: 1px solid black;
    width: 80%;
    aspect-ratio: 1/1;
  }

  .item-name {
    text-align: center;
    font-size: small;
  }
</style>