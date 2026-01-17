<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { getCurrentWindow } from '@tauri-apps/api/window'
  import { breakWindowStore } from '$lib/stores/breakWindowStore'
  import { onMount, onDestroy } from 'svelte'
  import { tick } from 'svelte'

  const appWindow = getCurrentWindow()

  const MICROBREAK_IDEAS = [
    'Roll your shoulders backward 10 times',
    'Stretch your arms above your head',
    'Look away from your screen for 20 seconds',
    'Stand up and stretch your legs',
    'Rotate your head in circles',
    'Clasp your hands behind your back',
    'Do 10 shoulder shrugs',
    'Stretch your neck side to side',
  ]

  const LONGBREAK_IDEAS = [
    'Take a 5-minute walk outside',
    'Do some light stretching exercises',
    'Drink a glass of water',
    'Practice deep breathing exercises',
    'Rest your eyes by looking at distant objects',
    'Do a quick meditation session',
    'Walk around room or office',
  ]

  export let onClose: (() => void) | undefined = undefined

  let timeLeft = 0
  let idea = ''
  let interval: number | null = null
  let autoHideTimer: number | null = null

  $: if ($breakWindowStore.isVisible && $breakWindowStore.timeLeft !== timeLeft) {
    timeLeft = $breakWindowStore.timeLeft
  }

  $: if ($breakWindowStore.breakType) {
    const ideas = $breakWindowStore.breakType === 'Microbreak' ? MICROBREAK_IDEAS : LONGBREAK_IDEAS
    idea = ideas[Math.floor(Math.random() * ideas.length)]
  }

  $: if ($breakWindowStore.isVisible && timeLeft > 0 && !interval) {
    interval = globalThis.setInterval(() => {
      if (timeLeft <= 1) {
        clearInterval(interval!)
        interval = null
        timeLeft = 0
        breakWindowStore.setTimeLeft(0)
      } else {
        timeLeft--
        breakWindowStore.setTimeLeft(timeLeft)
      }
    }, 1000)
  }

  $: if (!$breakWindowStore.isVisible || timeLeft === 0) {
    if (interval) {
      clearInterval(interval)
      interval = null
    }
  }

  $: if ($breakWindowStore.isVisible && timeLeft === 0 && !autoHideTimer) {
    console.log('Timer reached 0, starting auto-close...')
    autoHideTimer = globalThis.setTimeout(async () => {
      console.log('Auto-close executing...')
      try {
        await invoke('complete_break')
        console.log('Break completed via auto-hide')
      } catch (error) {
        console.error('Complete break failed:', error)
      }
      await tick()
      console.log('Closing window...')
      try {
        await appWindow.close()
        console.log('Window closed successfully')
      } catch (error) {
        console.error('Failed to close window:', error)
      }
      breakWindowStore.hideWindow()
      onClose?.()
      autoHideTimer = null
    }, 500)
  }

  $: if ((!$breakWindowStore.isVisible || timeLeft > 0) && autoHideTimer && timeLeft !== 0) {
    console.log('Clearing auto-hide timer (conditions changed)')
    clearTimeout(autoHideTimer)
    autoHideTimer = null
  }

  const handleSkip = async () => {
    try {
      await invoke('skip_break')
      console.log('Skip break succeeded')
    } catch (error) {
      console.error('Skip break failed:', error)
    }
    breakWindowStore.hideWindow()
    await appWindow.close()
    onClose?.()
  }

  const handlePostpone = async () => {
    try {
      await invoke('postpone_break')
      console.log('Postpone break succeeded')
    } catch (error) {
      console.error('Postpone break failed:', error)
    }
    breakWindowStore.hideWindow()
    await appWindow.close()
    onClose?.()
  }

  const handleComplete = async () => {
    try {
      await invoke('complete_break')
      console.log('Break completed')
    } catch (error) {
      console.error('Complete break failed:', error)
    }
    breakWindowStore.hideWindow()
    await appWindow.close()
    onClose?.()
  }

  const toggleFullscreen = () => {
    breakWindowStore.toggleFullscreen()
  }

  const handleEscape = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      handleSkip()
    }
  }

  onMount(() => {
    globalThis.addEventListener('keydown', handleEscape)
  })

  onDestroy(() => {
    if (interval) clearInterval(interval)
    if (autoHideTimer) clearTimeout(autoHideTimer)
    globalThis.removeEventListener('keydown', handleEscape)
  })
</script>

{#if $breakWindowStore.isVisible}
  <div class="break-window {$breakWindowStore.breakType?.toLowerCase()} {$breakWindowStore.isFullscreen ? 'fullscreen' : ''}">
    <div class="break-content">
      <h1 class="break-title">
        {$breakWindowStore.breakType === 'Microbreak' ? 'Microbreak' : 'Long Break'}
      </h1>

      <p class="break-idea">{idea}</p>

      <div class="break-countdown">
        <span class="countdown-number">{Math.floor(timeLeft / 60)}:</span>
        <span class="countdown-seconds">{String(timeLeft % 60).padStart(2, '0')}</span>
      </div>

      <div class="break-actions">
        <button type="button" on:click={handleComplete} class="btn btn-primary">
          I've taken my break
        </button>

        {#if timeLeft > 5}
          <button type="button" on:click={handlePostpone} class="btn btn-secondary">
            Postpone
          </button>
        {/if}

        <button type="button" on:click={handleSkip} class="btn btn-tertiary">
          Skip
        </button>

        <button
          type="button"
          on:click={toggleFullscreen}
          class="btn btn-icon"
          title="Toggle fullscreen"
        >
          {$breakWindowStore.isFullscreen ? '⛶' : '⛷'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .break-window {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.95);
    backdrop-filter: blur(10px);
    z-index: 9999;
  }

  .break-window.fullscreen {
    position: fixed;
  }

  .break-content {
    background: #242424;
    padding: 40px;
    border-radius: 12px;
    max-width: 600px;
    text-align: center;
    color: rgba(255, 255, 255, 0.87);
  }

  .break-title {
    font-size: 2.5rem;
    margin: 0 0 20px 0;
    font-weight: 700;
  }

  .break-idea {
    font-size: 1.5rem;
    margin: 0 0 40px 0;
    line-height: 1.4;
  }

  .break-countdown {
    font-size: 4rem;
    font-weight: bold;
    margin: 0 0 40px 0;
    font-feature-settings: 'tnum';
  }

  .break-actions {
    display: flex;
    gap: 12px;
    justify-content: center;
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

  .btn-tertiary {
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .btn-tertiary:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .btn-icon {
    font-size: 1.5rem;
    padding: 12px;
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
  }

  .btn-icon:hover {
    color: rgba(255, 255, 255, 0.87);
  }
</style>
