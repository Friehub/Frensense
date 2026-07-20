// SAFE: Shared logic is extracted into a common module to eliminate the circular dependency entirely

import { Module } from '@nestjs/common';
import { CommonModule } from './common.module';

@Module({
  imports: [CommonModule],
  exports: [CommonModule],
})
export class PaymentsModule {}
