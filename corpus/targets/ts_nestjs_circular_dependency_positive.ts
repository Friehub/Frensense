// [frensense]
// observation: Two NestJS modules import each other directly, creating a circular dependency — at runtime one provider resolves to undefined
// impact: The circularly-dependent service is undefined at startup, causing Cannot read properties of undefined errors when the application initializes
// improvement: Use forwardRef(() => Module) in the imports array to break the circular reference, or restructure shared logic into a common module

import { Module, Injectable } from '@nestjs/common';
import { OrdersModule } from './orders.module';
import { PaymentsModule } from './payments.module';

@Injectable()
export class PaymentService {
  processPayment(amount: number): string {
    return `Processed ${amount}`;
  }
}

@Module({
  imports: [OrdersModule, PaymentsModule],
  providers: [PaymentService],
  exports: [PaymentService],
})
export class PaymentsModule {}
