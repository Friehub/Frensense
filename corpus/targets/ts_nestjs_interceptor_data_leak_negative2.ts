// SAFE: Entity fields are explicitly excluded from serialization using @Exclude()

import { Exclude, Expose } from 'class-transformer';

export class UserEntity {
  id: number;

  @Expose()
  name: string;

  @Expose()
  email: string;

  @Exclude()
  passwordHash: string;

  @Exclude()
  resetToken: string;

  @Expose()
  createdAt: Date;
}
