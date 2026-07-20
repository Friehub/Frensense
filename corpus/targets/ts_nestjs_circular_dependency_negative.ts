// SAFE: forwardRef() is used to break the circular dependency between modules

import { Module, forwardRef } from '@nestjs/common';
import { OrdersModule } from './orders.module';
import { PaymentService } from './payment.service';

@Module({
  imports: [forwardRef(() => OrdersModule)],
  providers: [PaymentService],
  exports: [PaymentService],
})
export class PaymentsModule {}
