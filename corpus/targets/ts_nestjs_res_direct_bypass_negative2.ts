// SAFE: @Res() is used with { passthrough: true } so NestJS still handles the response

import { Controller, Get, Res } from '@nestjs/common';
import type { Response } from 'express';
import { UserService } from './user.service';

@Controller('users')
export class UserController {
  constructor(private userService: UserService) {}

  @Get()
  async findAll(@Res({ passthrough: true }) res: Response) {
    const users = await this.userService.findAll();
    res.setHeader('X-Total-Count', String(users.length));
    return users;
  }
}
