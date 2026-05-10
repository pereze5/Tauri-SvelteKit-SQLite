<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  type InventoryItem = {
    id: number;
    name: string;
    supply_type: string;
    brand: string;
    color_name: string;
    color_code: string;
    quantity: number;
    unit: string;
    notes: string;
    created_at: string;
    updated_at: string;
  };

  let items: InventoryItem[] = [];
  let error = "";

  let name = "";
  let supplyType = "yarn";
  let brand = "";
  let colorName = "";
  let colorCode = "";
  let quantity = 1;
  let unit = "skein";
  let notes = "";

  async function loadItems() {
    items = await invoke<InventoryItem[]>("list_inventory_items");
  }

  async function addItem() {
    error = "";

    if (!name.trim()) {
      error = "Item name is required.";
      return;
    }

    try {
      await invoke<InventoryItem>("add_inventory_item", {
        name: name.trim(),
        supplyType,
        brand,
        colorName,
        colorCode,
        quantity: Number(quantity),
        unit,
        notes
      });

      name = "";
      brand = "";
      colorName = "";
      colorCode = "";
      quantity = 1;
      unit = supplyType === "floss" ? "skein" : "skein";
      notes = "";

      await loadItems();
    } catch (e) {
      console.error(e);
      error = String(e);
    }
  }

  async function deleteItem(itemId: number) {
    error = "";

    try {
      await invoke("delete_inventory_item", { itemId });
      await loadItems();
    } catch (e) {
      console.error(e);
      error = String(e);
    }
  }

  onMount(loadItems);
</script>

<section>
  <h3>Inventory</h3>

  <form on:submit|preventDefault={addItem}>
    <input bind:value={name} placeholder="Item name" />

    <select bind:value={supplyType}>
      <option value="yarn">Yarn</option>
      <option value="floss">Floss</option>
      <option value="fabric">Fabric</option>
      <option value="needle">Needle</option>
      <option value="notion">Notion</option>
      <option value="other">Other</option>
    </select>

    <input bind:value={brand} placeholder="Brand" />
    <input bind:value={colorName} placeholder="Color name" />
    <input bind:value={colorCode} placeholder="Color code" />

    <input type="number" min="0" step="0.01" bind:value={quantity} />

    <input bind:value={unit} placeholder="Unit, e.g. skein, metre, piece" />

    <input bind:value={notes} placeholder="Notes" />

    <button type="submit">Add inventory item</button>
  </form>

  {#if error}
    <p>{error}</p>
  {/if}

  {#if items.length === 0}
    <p>No inventory items yet.</p>
  {:else}
    <ul>
      {#each items as item}
        <li>
          <strong>{item.name}</strong>
          ({item.supply_type})
          <br />
          {item.brand}
          {#if item.color_name}
            — {item.color_name}
          {/if}
          {#if item.color_code}
            [{item.color_code}]
          {/if}
          <br />
          Quantity: {item.quantity} {item.unit}
          {#if item.notes}
            <br />
            Notes: {item.notes}
          {/if}
          <br />
          <button on:click={() => deleteItem(item.id)}>Remove</button>
        </li>
      {/each}
    </ul>
  {/if}
</section>