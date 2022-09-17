/* istanbul ignore file */
/* tslint:disable */

export type UpdateUserSettings = {
    /**
     * Always require old password to change options.
     */
    currentPassword: string;
    email?: string;
    newPassword?: string;
    username?: string;
};

