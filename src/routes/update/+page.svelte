<script>
	import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
	import { relaunch } from '@tauri-apps/api/process'
	
	async function checkForUpdates() {
	  try {
		const update = await checkUpdate()
		if (update.available) {
		  if (confirm(`Version ${update.manifest.version} is available. Install now?`)) {
			await installUpdate()
			await relaunch()
		  }
		} else {
		  alert('No updates available')
		}
	  } catch (error) {
		console.error(error)
		alert('Error checking for updates')
	  }
	}
	</script>
	
	<button on:click={checkForUpdates}>Check for Updates (V0.0.9)</button>