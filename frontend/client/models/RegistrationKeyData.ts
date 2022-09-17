/* istanbul ignore file */
/* tslint:disable */

export type RegistrationKeyData = {
    /**
     * Registration key.
     */
    code: string;
    /**
     * Key invalidation date.
     */
    expiryDate: string;
    id: string;
    /**
     * Admin which issued this registration key.
     */
    issuer: string;
    /**
     * Amount of uses left.
     */
    usesLeft?: number;
};

