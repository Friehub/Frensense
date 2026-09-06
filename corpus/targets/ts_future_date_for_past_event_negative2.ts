// SAFE alternative: zod date validation
import { z } from 'zod';

const profileSchema = z.object({
  birthDate: z.string().refine(val => {
    const d = new Date(val);
    return !isNaN(d.getTime()) && d <= new Date() && d >= new Date('1900-01-01');
  }, { message: 'Invalid birth date' }),
});

const transactionSchema = z.object({
  amount: z.number().positive(),
  date: z.string().refine(val => {
    const d = new Date(val);
    return !isNaN(d.getTime()) && d <= new Date();
  }),
});
