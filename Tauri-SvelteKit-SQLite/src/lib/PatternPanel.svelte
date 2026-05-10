<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { openPath } from "@tauri-apps/plugin-opener";

  export let projectId: number;

  type Pattern = {
    id: number;
    project_id: number;
    display_name: string;
    file_path: string;
    file_type: string;
    current_page: number;
    last_position_note: string;
    last_viewed_at: string;
    metadata_json: string;
  };

  let patterns: Pattern[] = [];
  let error = "";

  async function loadPatterns() {
    patterns = await invoke<Pattern[]>("list_patterns", { projectId });
  }

  function inferFileType(path: string) {
    const lower = path.toLowerCase();

    if (lower.endsWith(".pdf")) return "pdf";
    if (lower.endsWith(".png")) return "image";
    if (lower.endsWith(".jpg")) return "image";
    if (lower.endsWith(".jpeg")) return "image";

    return "unknown";
  }

  function basename(path: string) {
    return path.split(/[\\/]/).pop() ?? path;
  }

  async function choosePatternFile() {
    error = "";

    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "Pattern files",
            extensions: ["pdf", "png", "jpg", "jpeg"]
          }
        ]
      });

      if (!selected || Array.isArray(selected)) return;

      const filePath = selected;
      const displayName = basename(filePath);
      const fileType = inferFileType(filePath);

      if (fileType === "unknown") {
        error = "Unsupported file type.";
        return;
      }

      await invoke<Pattern>("add_pattern", {
        projectId,
        displayName,
        filePath,
        fileType
      });

      await loadPatterns();
    } catch (e) {
      console.error(e);
      error = String(e);
    }
  }

  async function openPattern(filePath: string) {
    error = "";

    try {
      await openPath(filePath);
    } catch (e) {
      console.error(e);
      error = String(e);
    }
  }

  async function deletePattern(patternId: number) {
    error = "";

    try {
      await invoke("delete_pattern", { patternId });
      await loadPatterns();
    } catch (e) {
      console.error(e);
      error = String(e);
    }
  }

  async function updatePatternPosition(pattern: Pattern) {
    error = "";

    try {
      await invoke<Pattern>("update_pattern_position", {
        patternId: pattern.id,
        currentPage: Number(pattern.current_page),
        lastPositionNote: pattern.last_position_note
      });

      await loadPatterns();
    } catch (e) {
      console.error(e);
      error = String(e);
    }
  }

  type PatternMetadata = {
  designer?: string;
  source?: string;
  needle_size?: string;
  gauge?: string;
  yarn_requirement?: string;
  fabric_count?: string;
  size?: string;
  notes?: string;
  };

  function parseMetadata(pattern: Pattern): PatternMetadata {
  try {
    return JSON.parse(pattern.metadata_json || "{}");
  } catch {
    return {};
  }
}

async function savePatternMetadata(pattern: Pattern, metadata: PatternMetadata) {
  error = "";

  try {
    await invoke<Pattern>("update_pattern_metadata", {
      patternId: pattern.id,
      metadataJson: JSON.stringify(metadata)
    });

    await loadPatterns();
  } catch (e) {
    console.error(e);
    error = String(e);
  }
}

  $: if (projectId) {
    loadPatterns();
  }

  onMount(loadPatterns);
</script>

<section>
  <h3>Patterns</h3>

  <button on:click={choosePatternFile}>Add pattern file</button>

  {#if error}
    <p>{error}</p>
  {/if}

  {#if patterns.length === 0}
    <p>No pattern files linked yet.</p>
  {:else}
    <ul>
      {#each patterns as pattern}
      {@const metadata = parseMetadata(pattern)}
        <li>
          <strong>{pattern.display_name}</strong>
          <br />

          Type: {pattern.file_type}
          <br />

          <label>
            Page:
            <input
              type="number"
              min="1"
              bind:value={pattern.current_page}
            />
          </label>

          <br />

          <label>
            Where I stopped:
            <input
              bind:value={pattern.last_position_note}
              placeholder="e.g. row 42, sleeve decreases"
            />
          </label>

          <br />

          <button on:click={() => updatePatternPosition(pattern)}>
            Save position
          </button>

          <h4>Pattern metadata</h4>

          <input
            bind:value={metadata.designer}
            placeholder="Designer"
          />

          <input
            bind:value={metadata.source}
            placeholder="Source"
          />

          <input
            bind:value={metadata.needle_size}
            placeholder="Needle size"
          />

          <input
            bind:value={metadata.gauge}
            placeholder="Gauge"
          />

          <input
            bind:value={metadata.yarn_requirement}
            placeholder="Yarn requirement"
          />

          <input
            bind:value={metadata.fabric_count}
            placeholder="Fabric count"
          />

          <input
            bind:value={metadata.size}
            placeholder="Size"
          />

          <input
            bind:value={metadata.notes}
            placeholder="Metadata notes"
          />

          <br />

          <button on:click={() => savePatternMetadata(pattern, metadata)}>
            Save metadata
          </button>

          <br />
          <br />

          <small>{pattern.file_path}</small>

          <br />

          <button on:click={() => openPattern(pattern.file_path)}>
            Open pattern
          </button>

          <button on:click={() => deletePattern(pattern.id)}>
            Remove pattern
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</section>