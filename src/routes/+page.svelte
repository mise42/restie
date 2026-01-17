<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  let message = ''
  let isTesting = false

  const testApp = async () => {
    isTesting = true
    try {
      const result = await invoke('get_settings') as { microbreak_interval_minutes: number, longbreak_duration_minutes: number }
      message = `✓ Connected to Tauri! Microbreak interval: ${result.microbreak_interval_minutes}min, Long break duration: ${result.longbreak_duration_minutes}min`
    } catch (error) {
      message = `✗ Error: ${error}`
    }
    isTesting = false
  }
</script>

<svelte:head>
  <title>Restie - Testing</title>
</svelte:head>

<main>
  <div class="app">
    <h1 class="app-title">Restie - Break Time Reminder</h1>
    <p class="subtitle">
      Break time reminder app - Tauri + Svelte + TypeScript
    </p>

    <div class="app-actions">
      <button
        type="button"
        class="btn btn-tertiary"
        on:click={testApp}
        disabled={isTesting}
      >
        {isTesting ? 'Testing...' : 'Test App'}
      </button>
    </div>

    {#if message}
      <div class="test-result">
        <p>{message}</p>
      </div>
    {/if}

    <div class="info">
      <p>Backend: Rust (Tauri 2.0)</p>
      <p>Frontend: Svelte 5 + TypeScript 5</p>
      <p>State: Svelte Stores</p>
      <p>Status: MVP Features Implemented</p>
      <p class="note">Note: Use system tray menu to access Preferences and Test Break windows</p>
    </div>
  </div>
</main>

<style>
  main {
    padding: 40px;
    background: #1a1a1a;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .app {
    max-width: 600px;
    text-align: center;
    color: rgba(255, 255, 255, 0.87);
  }

  .app-title {
    font-size: 2.5rem;
    margin: 0 0 12px 0;
    font-weight: 700;
    color: #646cff;
  }

  .subtitle {
    font-size: 1.25rem;
    margin: 0 0 32px 0;
    color: rgba(255, 255, 255, 0.6);
  }

  .app-actions {
    margin-bottom: 24px;
  }

  .test-result {
    background: rgba(100, 108, 255, 0.1);
    border: 1px solid rgba(100, 108, 255, 0.3);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 24px;
  }

  .test-result p {
    margin: 0;
    font-size: 1rem;
  }

  .info {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    padding: 24px;
    text-align: left;
  }

  .info p {
    margin: 8px 0;
    font-size: 0.95rem;
  }

  .note {
    margin-top: 16px !important;
    color: rgba(255, 255, 255, 0.5);
    font-style: italic;
  }

  .btn {
    padding: 12px 24px;
    font-size: 1rem;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-tertiary {
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .btn-tertiary:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
