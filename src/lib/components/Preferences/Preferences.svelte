<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'
  import { breakSettingsSchema, type BreakSettings } from '$lib/schemas/settings'
  import { z } from 'zod'

  export let onBack: (() => void) | undefined = undefined

  let settings: BreakSettings = {
    microbreak_interval_minutes: 20,
    microbreak_duration_seconds: 20,
    longbreak_interval_microbreaks: 4,
    longbreak_duration_minutes: 5,
    fullscreen_breaks: false,
  }

  let isSaving = false

  // Load settings from backend on mount
  onMount(async () => {
    try {
      const savedSettings = await invoke('get_settings') as BreakSettings
      settings = savedSettings
      console.log('Settings loaded from backend:', savedSettings)
    } catch (error) {
      console.error('Failed to load settings:', error)
    }
  })

  const handleAutoSave = async () => {
    if (isSaving) return // Prevent concurrent saves

    try {
      isSaving = true

      // Validate with Zod
      const validated = breakSettingsSchema.parse(settings)

      // Save to backend (silent)
      await invoke('update_settings', { settings: validated })
      console.log('Settings auto-saved:', validated)

    } catch (error) {
      if (error instanceof z.ZodError) {
        // Validation error
        console.error('Validation errors:', error.issues)
        // Could add visual indicator on invalid field in future
      } else {
        // Backend error
        console.error('Failed to save settings:', error)
      }
    } finally {
      isSaving = false
    }
  }
</script>

<div class="preferences-container">
  <div class="preferences-header">
    <h1 class="preferences-title">Preferences</h1>
    {#if isSaving}
      <span class="save-status">●</span>
    {/if}
    <button type="button" on:click={onBack} class="btn btn-close">✕</button>
  </div>

  <section class="preferences-section">
    <h2>Break Timing</h2>

    <div class="form-row">
      <div class="form-group">
        <label for="microbreak-interval">Microbreak every (min)</label>
        <input
          id="microbreak-interval"
          type="number"
          min="1"
          max="60"
          bind:value={settings.microbreak_interval_minutes}
          on:blur={handleAutoSave}
        />
      </div>

      <div class="form-group">
        <label for="microbreak-duration">Duration (sec)</label>
        <input
          id="microbreak-duration"
          type="number"
          min="5"
          max="300"
          bind:value={settings.microbreak_duration_seconds}
          on:blur={handleAutoSave}
        />
      </div>
    </div>

    <div class="form-row">
      <div class="form-group">
        <label for="longbreak-interval">Long break after (microbreaks)</label>
        <input
          id="longbreak-interval"
          type="number"
          min="1"
          max="10"
          bind:value={settings.longbreak_interval_microbreaks}
          on:blur={handleAutoSave}
        />
      </div>

      <div class="form-group">
        <label for="longbreak-duration">Duration (min)</label>
        <input
          id="longbreak-duration"
          type="number"
          min="1"
          max="60"
          bind:value={settings.longbreak_duration_minutes}
          on:blur={handleAutoSave}
        />
      </div>
    </div>
  </section>

  <section class="preferences-section">
    <h2>Break Window</h2>

    <div class="form-group checkbox">
      <input
        type="checkbox"
        id="fullscreen-breaks"
        bind:checked={settings.fullscreen_breaks}
        on:change={handleAutoSave}
      />
      <label for="fullscreen-breaks">Fullscreen break windows</label>
    </div>
  </section>
</div>

<style>
  .preferences-container {
    padding: 20px;
    background: #242424;
    color: rgba(255, 255, 255, 0.87);
    min-height: 100vh;
  }

  .preferences-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    gap: 12px;
  }

  .preferences-title {
    font-size: 1.75rem;
    margin: 0;
    font-weight: 700;
  }

  .save-status {
    font-size: 0.75rem;
    color: #646cff;
    animation: pulse 1s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .preferences-section {
    margin-bottom: 20px;
    padding: 16px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
  }

  .preferences-section h2 {
    font-size: 1.1rem;
    margin: 0 0 12px 0;
    font-weight: 600;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    margin-bottom: 12px;
  }

  .form-row:last-child {
    margin-bottom: 0;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    margin-bottom: 4px;
    font-size: 0.875rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.7);
  }

  .form-group :global(input[type='number']) {
    padding: 6px 10px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: white;
    font-size: 0.95rem;
  }

  .form-group.checkbox {
    flex-direction: row;
    align-items: center;
    gap: 8px;
  }

  .form-group.checkbox :global(input) {
    width: auto;
    margin: 0;
  }

  .form-group.checkbox label {
    margin: 0;
    cursor: pointer;
    font-size: 0.95rem;
  }

  .btn {
    padding: 8px 20px;
    font-size: 0.95rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }

  .btn-primary {
    background: #646cff;
    color: white;
  }

  .btn-primary:hover {
    background: #535bf2;
  }

  .btn-secondary {
    background: #4a5568;
    color: white;
  }

  .btn-secondary:hover {
    background: #5a6478;
  }

  .btn-close {
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
    font-size: 1.5rem;
    padding: 4px 12px;
    line-height: 1;
  }

  .btn-close:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.87);
  }
</style>
