// SAFE: The interceptor wraps the error in a NestJS HttpException after logging

import {
  Injectable,
  NestInterceptor,
  ExecutionContext,
  CallHandler,
  HttpException,
  InternalServerErrorException,
} from '@nestjs/common';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';

@Injectable()
export class ErrorMappingInterceptor implements NestInterceptor {
  intercept(context: ExecutionContext, next: CallHandler): Observable<unknown> {
    return next.handle().pipe(
      catchError((err) => {
        console.error('Error occurred:', err.message);
        if (err instanceof HttpException) {
          return throwError(() => err);
        }
        return throwError(() => new InternalServerErrorException('An unexpected error occurred'));
      }),
    );
  }
}
