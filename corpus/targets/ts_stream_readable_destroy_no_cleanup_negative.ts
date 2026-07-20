// SAFE: _destroy method closes file handle
import { Readable, ReadableOptions } from 'node:stream';
import { open, FileHandle } from 'node:fs/promises';

class FileReader extends Readable {
  private fd: FileHandle | null = null;

  constructor(private path: string, opts?: ReadableOptions) {
    super(opts);
  }

  async _construct(callback: (error?: Error | null) => void): Promise<void> {
    try {
      this.fd = await open(this.path, 'r');
      callback();
    } catch (err) {
      callback(err as Error);
    }
  }

  _destroy(error: Error | null, callback: (error: Error | null) => void): void {
    if (this.fd) {
      this.fd.close().then(() => callback(error)).catch(callback);
    } else {
      callback(error);
    }
  }

  _read(size: number): void {}
}
