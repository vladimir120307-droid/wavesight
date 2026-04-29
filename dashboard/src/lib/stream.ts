export interface StreamUpdate {
  timestamp: string;
  node: string;
  sequence: number;
  rssi_dbm: number;
  amplitude: number[];
}

export type StreamListener = (update: StreamUpdate) => void;

export class StreamClient {
  private socket: WebSocket | null = null;
  private listeners = new Set<StreamListener>();
  private retryDelayMs = 1000;
  private maxRetryDelayMs = 15000;

  constructor(private readonly url: string) {}

  connect(): void {
    try {
      this.socket = new WebSocket(this.url);
    } catch (err) {
      console.warn("ws ctor failed:", err);
      this.scheduleReconnect();
      return;
    }

    this.socket.addEventListener("open", () => {
      this.retryDelayMs = 1000;
    });

    this.socket.addEventListener("message", (event) => {
      try {
        const update = JSON.parse(event.data as string) as StreamUpdate;
        for (const listener of this.listeners) {
          listener(update);
        }
      } catch (err) {
        console.warn("ws parse failed:", err);
      }
    });

    this.socket.addEventListener("close", () => {
      this.scheduleReconnect();
    });
    this.socket.addEventListener("error", () => {
      this.socket?.close();
    });
  }

  subscribe(listener: StreamListener): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  close(): void {
    this.socket?.close();
    this.socket = null;
  }

  private scheduleReconnect(): void {
    const delay = this.retryDelayMs;
    this.retryDelayMs = Math.min(delay * 2, this.maxRetryDelayMs);
    window.setTimeout(() => this.connect(), delay);
  }
}
