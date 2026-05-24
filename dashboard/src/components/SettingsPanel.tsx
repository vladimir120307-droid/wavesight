import { useSettings } from "../lib/settings";
import { t, type Locale } from "../lib/i18n";

interface Props {
  onClose: () => void;
}

export function SettingsPanel({ onClose }: Props) {
  const { locale, honestMode, serverUrl, setLocale, setHonestMode, setServerUrl } =
    useSettings();
  const tr = t(locale);

  return (
    <div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4">
      <div className="panel w-full max-w-md space-y-4">
        <header className="flex items-center justify-between">
          <h2 className="text-lg font-semibold">{tr.settings}</h2>
          <button
            onClick={onClose}
            className="text-zinc-400 hover:text-zinc-100"
            aria-label="close"
          >
            ✕
          </button>
        </header>

        <label className="block space-y-1">
          <span className="text-sm text-zinc-300">{tr.language}</span>
          <div className="flex gap-2">
            {(["en", "ru"] as Locale[]).map((l) => (
              <button
                key={l}
                onClick={() => setLocale(l)}
                className={`px-3 py-1 rounded text-sm ${
                  l === locale
                    ? "bg-accent text-canvas"
                    : "bg-line text-zinc-300 hover:bg-zinc-700"
                }`}
              >
                {l.toUpperCase()}
              </button>
            ))}
          </div>
        </label>

        <label className="flex items-start gap-3">
          <input
            type="checkbox"
            checked={honestMode}
            onChange={(e) => setHonestMode(e.target.checked)}
            className="mt-1"
          />
          <span>
            <span className="text-sm font-medium text-zinc-200">{tr.honestMode}</span>
            <span className="block text-xs text-zinc-400">{tr.honestModeHelp}</span>
          </span>
        </label>

        <label className="block space-y-1">
          <span className="text-sm text-zinc-300">{tr.serverUrl}</span>
          <input
            value={serverUrl}
            onChange={(e) => setServerUrl(e.target.value)}
            className="w-full bg-canvas border border-line rounded px-2 py-1 text-sm"
          />
        </label>
      </div>
    </div>
  );
}
