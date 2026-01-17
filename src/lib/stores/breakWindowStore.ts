import { writable } from 'svelte/store'

export interface BreakWindow {
  isVisible: boolean
  breakType: 'Microbreak' | 'Longbreak' | null
  timeLeft: number
  isFullscreen: boolean
}

const defaultWindow: BreakWindow = {
  isVisible: false,
  breakType: null,
  timeLeft: 0,
  isFullscreen: true,
}

function createBreakWindowStore() {
  const { subscribe, set, update } = writable<BreakWindow>(defaultWindow)

  return {
    subscribe,
    showWindow: (breakType: 'Microbreak' | 'Longbreak') =>
      update((window) => ({
        ...window,
        isVisible: true,
        breakType,
        timeLeft: breakType === 'Microbreak' ? 20 : 300,
      })),
    hideWindow: () =>
      set({
        isVisible: false,
        breakType: null,
        timeLeft: 0,
        isFullscreen: true,
      }),
    setTimeLeft: (seconds: number) =>
      update((window) => ({
        ...window,
        timeLeft: seconds,
      })),
    toggleFullscreen: () =>
      update((window) => ({
        ...window,
        isFullscreen: !window.isFullscreen,
      })),
  }
}

export const breakWindowStore = createBreakWindowStore()
