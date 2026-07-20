// SAFE alternative: Destroy stream after write to ensure closure
import { Duplex } from 'node:stream';

function sendAndDestroy(duplex: Duplex, message: string): void {
  duplex.write(message, () => {
    duplex.destroy();
  });
}
