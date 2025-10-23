import type { Snippet } from "svelte";
import db from "./db";
import { copyFieldsSimple } from "./tools";
import type { GlobalState, UploadHistory } from "./type";

export let globalState: GlobalState = $state({
  alertMessage: "",
  drag: {
    isDragging: false,
    paths: [] as string[],
  },
  modal: {
    isShow: false,
    children: undefined,
    onClose: undefined,
  },
  files: [],
  selectedBucket: undefined,
  activeSelectedBucketId: undefined,
  appSetting: {
    sidebarCollapsed: false,
    useSystemProxy: true,
    locale: "en",
    defaultBucketId: undefined,
    lastActiveBucketId: undefined,
  },
  progress: {},
  bucketsRefreshSignal: 0,
});

export function setAlert(message: string) {
  globalState.alertMessage = message;
}

export function closeModal() {
  globalState.modal.isShow = false;
}

export function showModal(children: Snippet) {
  globalState.modal.children = children;
  globalState.modal.isShow = true;
}

export function setIsDragging(isDragging: boolean) {
  globalState.drag.isDragging = isDragging;
}

export function setDragPaths(paths: string[]) {
  globalState.drag.paths = paths;
}

export function refreshBuckets() {
  globalState.bucketsRefreshSignal++;
}

// initialize app settings from database
export async function initAppSettings() {
  const settings = await db.appSettings.get(1);
  if (settings) {
    copyFieldsSimple(settings, globalState.appSetting);
  }
}
