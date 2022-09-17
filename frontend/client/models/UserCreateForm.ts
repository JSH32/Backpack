/* istanbul ignore file */
/* tslint:disable */

export type UserCreateForm = {
    email: string;
    password: string;
    /**
     * Only needed when service is invite_only.
     */
    registrationKey?: string;
    username: string;
};

