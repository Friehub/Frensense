// [frensense]
// observation: A NestJS controller method uses @Res() (Express response object) and calls res.send() directly, bypassing NestJS interceptor and response mapping layers.
// impact: NestJS interceptors, exception filters, and serialization logic (class-transformer, class-validator) are skipped, leading to inconsistent error handling, missing transforms, or sensitive data exposure.
// improvement: Avoid injecting @Res() directly. Return values from the handler and let NestJS handle the response, or use @Res({ passthrough: true }) if you must access the response object.

import { Controller, Get, Res } from '@nestjs/common';
import type { Response } from 'express';
import { UserService } from './user.service';

@Controller('users')
export class UserController {
  constructor(private userService: UserService) {}

  @Get()
  async findAll(@Res() res: Response) {
    const users = await this.userService.findAll();
    return res.json(users);
  }
}
