/* istanbul ignore file */
/* tslint:disable */

export type UpdateUserSettings = {
    /**
     * This is required if a password has been set prior.
     * This is optional if the requesting user is an admin modifying another user.
     */
    currentPassword?: string | null;
    email?: string | null;
    newPassword?: string | null;
    username?: string | null;
};

