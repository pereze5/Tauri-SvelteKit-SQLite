<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import CounterPanel from "$lib/CounterPanel.svelte";
  import SessionPanel from "$lib/SessionPanel.svelte";
  import PatternPanel from "$lib/PatternPanel.svelte";
  import InventoryPanel from "$lib/InventoryPanel.svelte";
  import ProjectInventoryPanel from "$lib/ProjectInventoryPanel.svelte";

  type Project = {
    id: number;
    name: string;
    craft_type: string;
    status: string;
    created_at: string;
  };

  let projects: Project[] = [];
  let activeProject: Project | null = null;

  let name = "";
  let craftType = "knitting";
  let error = "";

  async function loadProjects() {
    projects = await invoke<Project[]>("list_projects");
    activeProject = await invoke<Project | null>("get_active_project");
  }

  async function createProject() {
    error = "";

    if (!name.trim()) {
      error = "Project name is required.";
      return;
    }

    const project = await invoke<Project>("create_project", {
      name: name.trim(),
      craftType
    });

    await invoke("set_active_project", {
      projectId: project.id
    });

    name = "";
    await loadProjects();
  }

  async function selectProject(project: Project) {
    await invoke("set_active_project", {
      projectId: project.id
    });

    activeProject = project;
  }

  onMount(loadProjects);
</script>

<section>
  <h2>Craft Companion</h2>

  <h3>Projects</h3>

  {#if projects.length === 0}
    <p>No projects yet.</p>
  {:else}
    <ul>
      {#each projects as project}
        <li>
          <button on:click={() => selectProject(project)}>
            {project.name} — {project.craft_type}
            {#if activeProject?.id === project.id}
              ✓
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  {/if}

  <hr />

  {#if activeProject}
    <p>
      Active project:
      <strong>{activeProject.name}</strong>
      ({activeProject.craft_type})
    </p>
  {:else}
    <p>No active project selected.</p>
  {/if}

  <form on:submit|preventDefault={createProject}>
    <input bind:value={name} placeholder="Project name" />

    <select bind:value={craftType}>
      <option value="knitting">Knitting</option>
      <option value="crochet">Crochet</option>
      <option value="cross_stitch">Cross stitch</option>
      <option value="sewing">Sewing</option>
      <option value="other">Other</option>
    </select>

    <button type="submit">Create project</button>
  </form>

  {#if error}
    <p>{error}</p>
  {/if}

  {#if activeProject}
    <CounterPanel projectId={activeProject.id} />
    <SessionPanel projectId={activeProject.id} />
    <PatternPanel projectId={activeProject.id} />
    <ProjectInventoryPanel projectId={activeProject.id} />
  {/if}

  <InventoryPanel />
</section>