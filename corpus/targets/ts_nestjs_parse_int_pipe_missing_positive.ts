// [frensense]
// observation: A NestJS controller uses a route parameter as a number without @ParseIntPipe or @ParseNumberPipe, so the value is a string at runtime.
// impact: Type confusion occurs — the string is used in numeric operations, database queries, or comparisons, potentially leading to query injection, broken authorization, or unintended behaviour.
// improvement: Add @ParseIntPipe (or @ParseNumberPipe with min/max constraints) to the parameter to ensure type-safe numeric coercion.

import { Controller, Get, Param } from '@nestjs/common';
import { UserService } from './user.service';

@Controller('users')
export class UserController {
  constructor(private userService: UserService) {}

  @Get(':id')
  async findOne(@Param('id') id: number) {
    return this.userService.findById(id);
  }
}
