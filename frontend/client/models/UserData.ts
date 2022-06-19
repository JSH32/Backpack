/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { UserRole } from './UserRole';

export type UserData = {
    email: string;
    id: string;
    verified: boolean;
    role: UserRole;
    username: string;
};

