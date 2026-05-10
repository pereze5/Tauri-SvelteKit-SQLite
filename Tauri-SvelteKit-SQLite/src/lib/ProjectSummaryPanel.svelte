<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let projectId: number;

  type Counter = {
    id: number;
    label: string;
    value: number;
  };

  type Session = {
    id: number;
    started_at: string;
    ended_at: string | null;
    notes: string;
  };

  type Pattern = {
    id: number;
    display_name: string;
    current_page: number;
    last_position_note: string;
    metadata_json: string;
  };

  type PatternMetadata = {
    designer?: string;
    needle_size?: string;
    gauge?: string;
    yarn_requirement?: string;
    fabric_count?: string;
    size?: string;
  };

  type ProjectInventoryItem = {
    id: number;
    item_name: string;
    quantity_allocated: number;
    unit: string;
  };

  let counters: Counter[] = [];
  let sessions: Session[] = [];
  let patterns: Pattern[] = [];
  let supplies: ProjectInventoryItem[] = [];

  async function loadSummary() {
    counters = await invoke<Counter[]>("list_counters", { projectId });
    sessions = await invoke<Session[]>("list_sessions", { projectId });
    patterns = await invoke<Pattern[]>("list_patterns", { projectId });
    supplies = await invoke<ProjectInventoryItem[]>("list_project_inventory", {
      projectId
    });
  }

  function parseMetadata(pattern: Pattern): PatternMetadata {
    try {
      return JSON.parse(pattern.metadata_json || "{}");
    } catch {
      return {};
    }
  }

  $: latestSession = sessions[0] ?? null;

  $: if (projectId) {
    loadSummary();
  }

  onMount(loadSummary);
</script>

<section>
  <h3>Where I stopped</h3>

  <h4>Patterns</h4>
  {#if patterns.length === 0}
    <p>No pattern linked.</p>
  {:else}
    <ul>
      {#each patterns as pattern}
        {@const metadata = parseMetadata(pattern)}

        <li>
          <div class="pattern-name">{pattern.display_name}</div>

          <div class="stopping-point">
            <div class="page">Page {pattern.current_page}</div>

            {#if pattern.last_position_note}
              <div class="line">{pattern.last_position_note}</div>
            {/if}
          </div>

          <div class="metadata">
            {#if metadata.needle_size}
              <div>Needle: {metadata.needle_size}</div>
            {/if}

            {#if metadata.gauge}
              <div>Gauge: {metadata.gauge}</div>
            {/if}

            {#if metadata.yarn_requirement}
              <div>Yarn: {metadata.yarn_requirement}</div>
            {/if}

            {#if metadata.fabric_count}
              <div>Fabric count: {metadata.fabric_count}</div>
            {/if}

            {#if metadata.size}
              <div>Size: {metadata.size}</div>
            {/if}
          </div>
        </li>
      {/each}
    </ul>
  {/if}

  <h4>Counters</h4>
  {#if counters.length === 0}
    <p>No counters yet.</p>
  {:else}
    <ul>
      {#each counters as counter}
        <li>{counter.label}: {counter.value}</li>
      {/each}
    </ul>
  {/if}

  <h4>Latest session</h4>
  {#if latestSession}
    <p>
      {latestSession.started_at}
      {#if latestSession.ended_at}
        → {latestSession.ended_at}
      {:else}
        → active
      {/if}
    </p>

    {#if latestSession.notes}
      <p>{latestSession.notes}</p>
    {/if}
  {:else}
    <p>No sessions yet.</p>
  {/if}

  <h4>Project supplies</h4>
  {#if supplies.length === 0}
    <p>No supplies linked.</p>
  {:else}
    <ul>
      {#each supplies as supply}
        <li>
          {supply.item_name}: {supply.quantity_allocated} {supply.unit}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .pattern-name {
    font-weight: 600;
    margin-bottom: 0.4rem;
  }

  .stopping-point {
    border: 1px solid currentColor;
    padding: 0.75rem;
    margin: 0.5rem 0;
  }

  .page {
    font-size: 1.25rem;
    font-weight: 700;
    margin-bottom: 0.4rem;
  }

  .line {
    font-size: 1.1rem;
    font-weight: 700;
  }

  .metadata {
    font-size: 0.9rem;
    opacity: 0.75;
    margin-top: 0.5rem;
  }
</style>