// [frensense]
// observation: A NestJS DTO class lacks validation decorators like @IsString() or @IsEmail(), so request bodies flow into the controller without structural or type validation
// impact: Invalid or malicious data can pass through to business logic, enabling injection attacks, mass assignment, or logic errors
// improvement: Add class-validator decorators to all DTO fields and use ValidationPipe globally

import { Controller, Post, Body } from '@nestjs/common';

class CreateUserDto {
  name: string;
  email: string;
  age: number;
}

@Controller('users')
export class UserController {
  @Post()
  createUser(@Body() dto: CreateUserDto): string {
    return `Created user ${dto.name}`;
  }
}
