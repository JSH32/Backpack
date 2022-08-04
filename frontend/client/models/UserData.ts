/* istanbul ignore file */
/* tslint:disable */

import type { UserRole } from './UserRole';

export type UserData = {
    username: string;
    verified: boolean;
    email: string;
    id: string;
    role: UserRole;
};

