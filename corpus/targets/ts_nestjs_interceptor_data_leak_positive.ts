// [frensense]
// observation: A NestJS interceptor serializes the entire entity for the response, including sensitive fields like passwords or tokens.
// impact: Sensitive data is leaked to the client in API responses, violating security and compliance requirements.
// improvement: Use a response DTO or @Exclude() decorator to strip sensitive fields before serialization.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Injectable, NestInterceptor, ExecutionContext, CallHandler } from '@nestjs/common';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

@Injectable()
export class TransformInterceptor implements NestInterceptor {
  intercept(context: ExecutionContext, next: CallHandler): Observable<any> {
    return next.handle().pipe(map(data => data));
  }
}
