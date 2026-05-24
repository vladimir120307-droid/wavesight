import { create } from "zustand";
import { detectLocale, type Locale, setLocale as persistLocale } from "./i18n";

interface SettingsState {
  locale: Locale;
  honestMode: boolean;
  serverUrl: string;
  setLocale(locale: Locale): void;
  setHonestMode(on: boolean): void;
  setServerUrl(url: string): void;
}

const DEFAULT_SERVER_URL = "ws://localhost:8081/api/v1/stream";

function readBool(key: string, fallback: boolean): boolean {
  const v = window.localStorage.getItem(key);
  if (v === null) return fallback;
  return v === "1";
}

function readString(key: string, fallback: string): string {
  return window.localStorage.getItem(key) ?? fallback;
}

export const useSettings = create<SettingsState>((set) => ({
  locale: detectLocale(),
  honestMode: readBool("wavesight.honestMode", true),
  serverUrl: readString("wavesight.serverUrl", DEFAULT_SERVER_URL),
  setLocale(locale) {
    persistLocale(locale);
    set({ locale });
  },
  setHonestMode(on) {
    window.localStorage.setItem("wavesight.honestMode", on ? "1" : "0");
    set({ honestMode: on });
  },
  setServerUrl(url) {
    window.localStorage.setItem("wavesight.serverUrl", url);
    set({ serverUrl: url });
  },
}));
