// SAFE: Sensitive fields are transformed to a safe representation using @Transform

import { Entity, PrimaryGeneratedColumn, Column } from 'typeorm';
import { Transform } from 'class-transformer';

@Entity()
export class User {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  email: string;

  @Column()
  @Transform(() => undefined)
  passwordHash: string;
}
