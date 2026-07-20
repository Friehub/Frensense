// [frensense]
// observation: A NestJS controller route handler has no @UseGuards() decorator, leaving the endpoint unprotected.
// impact: Unauthenticated users can access the endpoint, potentially reading or modifying sensitive data.
// improvement: Add @UseGuards(AuthGuard) or a similar guard decorator to all controller routes that require authentication.

import { Controller, Get } from '@nestjs/common';
import { UserService } from './user.service';

@Controller('users')
export class UserController {
  constructor(private userService: UserService) {}

  @Get()
  async findAll() {
    return this.userService.findAll();
  }
}
