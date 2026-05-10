<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let projectId: number;

  type Counter = {
    id: number;
    project_id: number;
    label: string;
    value: number;
    counter_type: string;
    sort_order: number;
    updated_at: string;
  };

  let counters: Counter[] = [];
  let label = "Row";
  let error = "";

  async function loadCounters() {
    counters = await invoke<Counter[]>("list_counters", {
      projectId
    });
  }

  async function createCounter() {
    error = "";

    if (!label.trim()) {
      error = "Counter label is required.";
      return;
    }

    await invoke<Counter>("create_counter", {
      projectId,
      label: label.trim(),
      counterType: "row"
    });

    label = "Row";
    await loadCounters();
  }

  async function changeCounter(counterId: number, delta: number) {
    const updated = await invoke<Counter>("increment_counter", {
      counterId,
      delta
    });

    counters = counters.map((counter) =>
      counter.id === updated.id ? updated : counter
    );
  }

  $: if (projectId) {
    loadCounters();
  }

  onMount(loadCounters);
</script>

<section>
  <h3>Counters</h3>

  <form on:submit|preventDefault={createCounter}>
    <input bind:value={label} placeholder="Counter label" />
    <button type="submit">Add counter</button>
  </form>

  {#if error}
    <p>{error}</p>
  {/if}

  {#if counters.length === 0}
    <p>No counters yet.</p>
  {:else}
    <ul>
      {#each counters as counter}
        <li>
          <strong>{counter.label}</strong>: {counter.value}

          <button on:click={() => changeCounter(counter.id, -1)}>-</button>
          <button on:click={() => changeCounter(counter.id, 1)}>+</button>
        </li>
      {/each}
    </ul>
  {/if}
</section>