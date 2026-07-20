// [frensense]
// observation: readable.destroy() called without releasing underlying resources (file handles, sockets, timers).
// impact: File descriptors or socket connections remain open, causing resource leaks and eventual EMFILE errors.
// improvement: Implement the _destroy method to close/release the underlying resource, or use a proper wrapper.

import { Readable } from 'node:stream';
import { open, FileHandle } from 'node:fs/promises';

class FileReader extends Readable {
  private fd: FileHandle | null = null;

  constructor(private path: string) {
    super();
  }

  async _construct(callback: (error?: Error | null) => void): Promise<void> {
    try {
      this.fd = await open(this.path, 'r');
      callback();
    } catch (err) {
      callback(err as Error);
    }
  }

  _read(size: number): void {
    // reading logic would go here
  }
}
