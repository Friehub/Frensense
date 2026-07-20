// [frensense]
// observation: A NestJS entity class omits @Exclude() on passwordHash (default @Expose() behavior), so class-transformer serializes it into every API response
// impact: Password hashes, security questions, or internal notes leak to API consumers, violating data protection requirements and enabling offline brute force
// improvement: Add @Exclude() to sensitive fields or use @Transform to sanitize before serialization

import { Entity, PrimaryGeneratedColumn, Column } from 'typeorm';

@Entity()
export class User {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  email: string;

  @Column()
  passwordHash: string;

  getDisplayName(): string {
    return this.email.split('@')[0];
  }
}
