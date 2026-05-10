<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import CounterPanel from "$lib/CounterPanel.svelte";
  import SessionPanel from "$lib/SessionPanel.svelte";
  import PatternPanel from "$lib/PatternPanel.svelte";
  import InventoryPanel from "$lib/InventoryPanel.svelte";
  import ProjectInventoryPanel from "$lib/ProjectInventoryPanel.svelte";
  import ProjectSummaryPanel from "$lib/ProjectSummaryPanel.svelte";
  import ShoppingListPanel from "$lib/ShoppingListPanel.svelte";

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

  let editingProjectId: number | null = null;
  let editName = "";
  let editCraftType = "knitting";
  let editStatus = "active";

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

  function startEditingProject(project: Project) {
  editingProjectId = project.id;
  editName = project.name;
  editCraftType = project.craft_type;
  editStatus = project.status;
}

function cancelEditingProject() {
  editingProjectId = null;
  editName = "";
  editCraftType = "knitting";
  editStatus = "active";
}



async function saveProjectEdit(projectId: number) {
  error = "";

  if (!editName.trim()) {
    error = "Project name is required.";
    return;
  }

  await invoke("update_project", {
    projectId,
    name: editName.trim(),
    craftType: editCraftType,
    status: editStatus
  });

  cancelEditingProject();
  await loadProjects();
}

async function deleteProject(projectId: number) {
  error = "";

  const confirmed = confirm(
    "Delete this project? This will also remove its counters, sessions, patterns, and supply links."
  );

  if (!confirmed) return;

  await invoke("delete_project", { projectId });
  await loadProjects();
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
    {#if editingProjectId === project.id}
      <input bind:value={editName} />

      <select bind:value={editCraftType}>
        <option value="knitting">Knitting</option>
        <option value="crochet">Crochet</option>
        <option value="cross_stitch">Cross stitch</option>
        <option value="sewing">Sewing</option>
        <option value="other">Other</option>
      </select>

      <select bind:value={editStatus}>
        <option value="active">Active</option>
        <option value="paused">Paused</option>
        <option value="finished">Finished</option>
        <option value="archived">Archived</option>
      </select>

      <button on:click={() => saveProjectEdit(project.id)}>Save</button>
      <button on:click={cancelEditingProject}>Cancel</button>
    {:else}
      <button on:click={() => selectProject(project)}>
        {project.name} — {project.craft_type}
        {#if activeProject?.id === project.id}
          ✓
        {/if}
      </button>

      <button on:click={() => startEditingProject(project)}>Edit</button>
      <button on:click={() => deleteProject(project.id)}>Delete</button>
    {/if}
  </li>
{/each}
    </ul>
  {/if}

  <h3>Add project</h3>

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

  <hr />

  {#if activeProject}
    <p>
      Active project:
      <strong>{activeProject.name}</strong>
      ({activeProject.craft_type})
    </p>

    <ProjectSummaryPanel projectId={activeProject.id} />
    <CounterPanel projectId={activeProject.id} />
    <SessionPanel projectId={activeProject.id} />
    <PatternPanel projectId={activeProject.id} />
    <ProjectInventoryPanel projectId={activeProject.id} />
  {:else}
    <p>No active project selected.</p>
  {/if}

  <InventoryPanel />
  <ShoppingListPanel />
</section>