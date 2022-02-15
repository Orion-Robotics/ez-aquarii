export interface Data {
  sensor_data: number[];
}

export interface LineVector {
  x?: number;
  y?: number;
}

export interface MoveVector {
  x: number;
  y: number;
}

export interface DataObject {
  data: Data;
  line_detections: boolean[];
  line_flipped: boolean;
  line_vector: LineVector;
  move_vector: MoveVector;
}

export interface DataSource {
  next(): DataObject;
  back(): DataObject;
  current(): DataObject;
}

export class TextSource implements DataSource {
  frames: DataObject[];
  frame = 0;

  constructor(text: string) {
    const lines = text.split("\n");
    this.frames = lines.map((line) => JSON.parse(line) as DataObject);
  }

  next(): DataObject {
    if (this.frame >= this.frames.length - 1) this.frame = -1;
    this.frame++;
    return this.current();
  }

  back(): DataObject {
    if (this.frame === 0) this.frame = this.frames.length;
    this.frame--;
    return this.current();
  }

  current(): DataObject {
    return this.frames[this.frame];
  }
}

// some sample data

/*
{"data":{"sensor_data":[]},"line_detections":[true, false, false, true],"line_flipped":false,"line_vector":{"x":null,"y":null},"move_vector":{"x":0.0,"y":0.0}}
{"data":{"sensor_data":[]},"line_detections":[true, false, true, false],"line_flipped":false,"line_vector":{"x":null,"y":null},"move_vector":{"x":0.0,"y":0.0}}
*/
