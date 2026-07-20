// SAFE: Uses a custom DTO with both class-validator and a transformation pipeline

import { IsString, IsEmail, IsOptional, IsBoolean, Transform } from 'class-validator';
import { Trim, ToLowerCase } from './transforms';

export class UpdateUserDto {
  @IsOptional()
  @IsString()
  @Trim()
  name?: string;

  @IsOptional()
  @IsEmail()
  @ToLowerCase()
  email?: string;

  @IsOptional()
  @IsBoolean()
  @Transform(({ value }) => value === 'true')
  isActive?: boolean;
}
