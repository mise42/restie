<script lang="ts">
  import { onMount } from 'svelte'
  import { breakWindowStore } from '$lib/stores/breakWindowStore'
  import BreakWindow from '$lib/components/BreakWindow/BreakWindow.svelte'

  // Initialize the break window as visible when this window loads
  // Set a 20-second timer for testing
  onMount(() => {
    breakWindowStore.showWindow('Microbreak')
    breakWindowStore.setTimeLeft(20)
  })
</script>

<BreakWindow />
