// SAFE: DTO fields have class-validator decorators to validate the request body

import { IsString, IsEmail, IsInt, Min, Max } from 'class-validator';

export class CreateUserDto {
  @IsString()
  name: string;

  @IsEmail()
  email: string;

  @IsInt()
  @Min(0)
  @Max(150)
  age: number;
}
