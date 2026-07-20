// [frensense]
// observation: A NestJS interceptor catches an error inside a catch block without rethrowing it, silently swallowing the exception.
// impact: Errors are suppressed — logging, monitoring, and exception filters never see the failure, making debugging impossible and hiding security-relevant errors.
// improvement: Always rethrow the error (or a wrapped version) after handling side effects like logging, so the exception filter can process it.

import {
  Injectable,
  NestInterceptor,
  ExecutionContext,
  CallHandler,
} from '@nestjs/common';
import { Observable, catchError } from 'rxjs';

@Injectable()
export class LoggingInterceptor implements NestInterceptor {
  intercept(context: ExecutionContext, next: CallHandler): Observable<unknown> {
    return next.handle().pipe(
      catchError((err) => {
        console.error('Error occurred:', err.message);
        throw err;
      }),
    );
  }
}
