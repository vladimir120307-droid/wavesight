export type Locale = "en" | "ru";

interface Translations {
  appTitle: string;
  liveCsi: (n: number) => string;
  presenceDetected: string;
  quiet: string;
  uncertain: string;
  emptyTitle: string;
  emptySteps: string[];
  footer: string;
  github: string;
  settings: string;
  honestMode: string;
  honestModeHelp: string;
  language: string;
  serverUrl: string;
  calibrate: string;
  calibrating: string;
  seq: string;
  rssi: string;
  energy: string;
}

const EN: Translations = {
  appTitle: "WaveSight",
  liveCsi: (n) => `Live CSI · ${n} node${n === 1 ? "" : "s"} connected`,
  presenceDetected: "presence detected",
  quiet: "quiet",
  uncertain: "uncertain",
  emptyTitle: "No nodes streaming yet.",
  emptySteps: [
    "Flash firmware/esp32-csi-node on an ESP32-S3.",
    "Set WAVESIGHT_SERVER_URI to ws://<this-machine>:8080/ingest.",
    "Power the board and refresh this page.",
  ],
  footer: "WaveSight alpha · honest mode on · local-first",
  github: "github ↗",
  settings: "Settings",
  honestMode: "Honest Mode",
  honestModeHelp:
    "Show confidence intervals next to every prediction; hide values below confidence threshold.",
  language: "Language",
  serverUrl: "Server URL",
  calibrate: "Calibrate empty room",
  calibrating: "Calibrating…",
  seq: "seq",
  rssi: "rssi",
  energy: "energy",
};

const RU: Translations = {
  appTitle: "WaveSight",
  liveCsi: (n) => `Живой CSI · ${n} ${n === 1 ? "нода" : n < 5 ? "ноды" : "нод"} подключено`,
  presenceDetected: "присутствие",
  quiet: "пусто",
  uncertain: "не уверен",
  emptyTitle: "Ноды ещё не подключены.",
  emptySteps: [
    "Прошейте firmware/esp32-csi-node на ESP32-S3.",
    "Задайте WAVESIGHT_SERVER_URI в ws://<этот-хост>:8080/ingest.",
    "Включите плату и обновите страницу.",
  ],
  footer: "WaveSight alpha · honest mode включён · local-first",
  github: "github ↗",
  settings: "Настройки",
  honestMode: "Honest Mode",
  honestModeHelp:
    "Показывать доверительные интервалы рядом с каждым предсказанием; скрывать значения ниже порога уверенности.",
  language: "Язык",
  serverUrl: "URL сервера",
  calibrate: "Калибровать пустую комнату",
  calibrating: "Калибровка…",
  seq: "seq",
  rssi: "rssi",
  energy: "энергия",
};

const dictionaries: Record<Locale, Translations> = { en: EN, ru: RU };

export function t(locale: Locale): Translations {
  return dictionaries[locale];
}

export function detectLocale(): Locale {
  const stored = window.localStorage.getItem("wavesight.locale") as Locale | null;
  if (stored === "en" || stored === "ru") return stored;
  const nav = window.navigator.language.toLowerCase();
  return nav.startsWith("ru") ? "ru" : "en";
}

export function setLocale(locale: Locale): void {
  window.localStorage.setItem("wavesight.locale", locale);
}
