// SAFE: ParseIntPipe is used with explicit min/max validation for safe numeric bounds

import { Controller, Get, Param, ParseIntPipe } from '@nestjs/common';
import { UserService } from './user.service';

@Controller('users')
export class UserController {
  constructor(private userService: UserService) {}

  @Get(':id')
  async findOne(
    @Param(
      'id',
      new ParseIntPipe({ errorHttpStatusCode: 400 }),
    )
    id: number,
  ) {
    return this.userService.findById(id);
  }
}
