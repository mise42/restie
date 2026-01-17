<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window'
  import Preferences from '$lib/components/Preferences/Preferences.svelte'

  const handleClose = async () => {
    await getCurrentWindow().close()
  }
</script>

<Preferences onBack={handleClose} />
