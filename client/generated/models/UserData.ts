/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { UserRole } from './UserRole';

export type UserData = {
    verified: boolean;
    id: string;
    username: string;
    role: UserRole;
    email: string;
};

