// SAFE: Role-based guard is applied with specific permission checks

import { Controller, Get, UseGuards } from '@nestjs/common';
import { AuthGuard } from '@nestjs/passport';
import { RolesGuard } from './roles.guard';
import { Roles } from './roles.decorator';
import { UserService } from './user.service';

@Controller('admin')
@UseGuards(AuthGuard('jwt'), RolesGuard)
export class AdminController {
  constructor(private userService: UserService) {}

  @Get('users')
  @Roles('admin')
  async findAll() {
    return this.userService.findAll();
  }
}
