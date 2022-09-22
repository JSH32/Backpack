/* istanbul ignore file */
/* tslint:disable */

export type UpdateUserSettings = {
    /**
     * This is required if a password has been set prior.
     */
    currentPassword?: string;
    email?: string;
    newPassword?: string;
    username?: string;
};

