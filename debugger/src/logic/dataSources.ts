export interface Data {
  sensor_data: number[];
}

export interface Vec2 {
  x: number;
  y: number;
}

export interface DataObject {
  data: Data;
  line_detections: boolean[];
  line_flipped: boolean;
  line_vector?: Vec2;
  previous_vec?: Vec2;

  orbit_offset: number;
  dampen_amount: number;
  orbit_angle: number;
  ball_follow_vector: Vec2;
  move_vector: Vec2;
}

export type Module =
  | {
      line: {
        sensor_count: number;
        pickup_threshold: number;
        trigger_threshold: number;
        uart_path: string;
        baud_rate: number;
      };
    }
  | {
      server: {
        addr: string;
      };
    }
  | {
      camera: {
        path: string;
      };
    };

export interface Config {
  modules: Module[];
}

export type JSONValue =
  | string
  | number
  | boolean
  | { [x: string]: JSONValue }
  | Array<JSONValue>;

export interface JSONObject {
  [x: string]: JSONValue;
}

export interface JSONArray extends Array<JSONValue> {}

export interface DataSource {
  stop(): void;
  next(): DataObject;
  back(): DataObject;
  goTo(frame: number): DataObject;
  clear(): void;
  current(): DataObject;
  currentConfig(): Promise<Config>;
  currentFrame(): number;
  numFrames(): number;
  onFrame(handler: (frame: DataObject) => void): void;
  sendConfig(config: Config): void;
}

type FrameCallback = (frame: DataObject) => void;

class BasicDataSource implements DataSource {
  frames: DataObject[] = [];
  frame = 0;
  handler: FrameCallback | undefined = undefined;
  config = { modules: [] };

  clear() {
    this.frames = [];
    this.frame = 0;
    this.handler?.(this.current());
  }

  stop() {}

  sendConfig(config: Config) {}

  goTo(frame: number): DataObject {
    this.frame = frame;
    this.handler?.(this.current());
    return this.current();
  }

  next(): DataObject {
    if (this.frame >= this.frames.length - 1) this.frame = -1;
    this.frame++;
    this.handler?.(this.current());
    return this.current();
  }

  back(): DataObject {
    if (this.frame === 0) this.frame = this.frames.length;
    this.frame--;
    this.handler?.(this.current());
    return this.current();
  }

  current(): DataObject {
    return this.frames[this.frame];
  }
  async currentConfig() {
    return { modules: [] };
  }

  currentFrame(): number {
    return this.frame;
  }

  numFrames(): number {
    return this.frames.length;
  }

  onFrame(handler: FrameCallback): void {
    this.handler = handler;
  }
}

export class ServerSource extends BasicDataSource {
  ws: WebSocket;

  constructor(private url: string) {
    super();

    const ws = new WebSocket(`ws://${url}/state`);
    this.ws = ws;
    ws.addEventListener("message", (ev) => {
      this.frames.push(JSON.parse(ev.data));
      this.handler?.(this.next());
    });
    ws.addEventListener("close", (ev) => console.log(ev));
  }

  sendConfig(config: Config) {
    this.ws.send(JSON.stringify(config));
  }

  async currentConfig() {
    return fetch(`http://${this.url}/config`).then((res) => res.json());
  }

  stop() {
    this.ws.close();
  }
}

export class TextSource extends BasicDataSource {
  constructor(text: string) {
    super();
    const lines = text.split("\n");
    this.frames = lines.map((line) => JSON.parse(line) as DataObject);
  }
}

// some sample data

/*
{"data":{"sensor_data":[]},"line_detections":[true, false, false, true],"line_flipped":false,"line_vector":{"x":null,"y":null},"move_vector":{"x":0.0,"y":0.0}}
{"data":{"sensor_data":[]},"line_detections":[true, false, true, false],"line_flipped":false,"line_vector":{"x":null,"y":null},"move_vector":{"x":0.0,"y":0.0}}
*/
