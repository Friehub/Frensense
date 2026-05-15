import { validateSession } from './auth';

/**
 * Deletes a user from the system.
 */
export async function delete_user(userId: string) {
    console.log(`Deleting user ${userId}...`);
    
    // VULNERABILITY: Sensitive operation called without any auth check.
    // The architectural rule will detect that this function never calls 'validateSession'.
    await db_remove_user(userId);
}

async function db_remove_user(id: string) {
    // DB logic
}
