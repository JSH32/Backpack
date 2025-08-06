/* istanbul ignore file */
/* tslint:disable */

export type UserCreateForm = {
    email: string;
    password: string;
    /**
     * Required when creating a user with password.
     */
    registrationKey?: string | null;
    username: string;
};

