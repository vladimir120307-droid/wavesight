import { t, type Locale } from "../lib/i18n";
import type { StreamUpdate } from "../lib/stream";
import { Spectrogram } from "./Spectrogram";

interface Props {
  node: string;
  lastUpdate: StreamUpdate;
  history: StreamUpdate[];
  locale: Locale;
}

function presenceFrom(history: StreamUpdate[]): { present: boolean; energy: number } {
  if (history.length < 4) return { present: false, energy: 0 };
  const recent = history.slice(-32);
  const means = recent.map((u) =>
    u.amplitude.length === 0
      ? 0
      : u.amplitude.reduce((a, b) => a + b, 0) / u.amplitude.length,
  );
  const avg = means.reduce((a, b) => a + b, 0) / means.length;
  const variance =
    means.reduce((a, b) => a + (b - avg) ** 2, 0) / means.length;
  const energy = Math.sqrt(variance);
  return { present: energy > 0.04, energy };
}

export function NodeCard({ node, lastUpdate, history, locale }: Props) {
  const tr = t(locale);
  const { present, energy } = presenceFrom(history);
  return (
    <section className="panel space-y-3">
      <header className="flex items-center justify-between">
        <h2 className="text-lg font-semibold">{node}</h2>
        <span
          className={`pill ${
            present
              ? "bg-accent/15 text-accent"
              : "bg-zinc-700/40 text-zinc-300"
          }`}
        >
          <span
            className={`w-2 h-2 rounded-full ${
              present ? "bg-accent" : "bg-zinc-500"
            }`}
          />
          {present ? tr.presenceDetected : tr.quiet}
        </span>
      </header>

      <Spectrogram history={history} />

      <dl className="grid grid-cols-3 gap-3 text-xs">
        <div>
          <dt className="text-zinc-500">{tr.seq}</dt>
          <dd>{lastUpdate.sequence}</dd>
        </div>
        <div>
          <dt className="text-zinc-500">{tr.rssi}</dt>
          <dd>{lastUpdate.rssi_dbm} dBm</dd>
        </div>
        <div>
          <dt className="text-zinc-500">{tr.energy}</dt>
          <dd>{energy.toFixed(3)}</dd>
        </div>
      </dl>
    </section>
  );
}
